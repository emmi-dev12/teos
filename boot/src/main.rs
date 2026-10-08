#![no_main]
#![no_std]

extern crate alloc;

use alloc::format;
use alloc::vec::Vec;
use uefi::boot::LoadImageSource;
use uefi::CString16;
use uefi::fs::FileSystem;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};
use uefi::proto::console::pointer::Pointer;
use uefi::proto::console::text::{Key, ScanCode};
use uefi::proto::loaded_image::LoadedImage;

#[no_mangle]
pub unsafe extern "C" fn wcslen(s: *const u16) -> usize {
    let mut n = 0;
    while *s.add(n) != 0 {
        n += 1;
    }
    n
}

struct Cfg {
    animation: u8,
    color: u8,
    length: u8,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            animation: 0,
            color: 0,
            length: 0,
        }
    }
}

const PAL: [BltPixel; 4] = [
    BltPixel::new(0x20, 0xc8, 0xff),
    BltPixel::new(0x3d, 0xdc, 0x84),
    BltPixel::new(0xff, 0x9f, 0x0a),
    BltPixel::new(0xff, 0x5f, 0xa8),
];
const BG: BltPixel = BltPixel::new(0x0c, 0x0c, 0x10);
const PANEL: BltPixel = BltPixel::new(0x1c, 0x1c, 0x22);
const WHITE: BltPixel = BltPixel::new(0xf5, 0xf5, 0xf7);
const MUTE: BltPixel = BltPixel::new(0x8e, 0x8e, 0x93);
const ON: BltPixel = BltPixel::new(0xff, 0xff, 0xff);

fn ink(c: &Cfg) -> BltPixel {
    PAL[c.color as usize % 4]
}

fn duration_ms(c: &Cfg) -> u32 {
    match c.length {
        0 => 700,
        2 => 2800,
        _ => 1400,
    }
}

fn anim_name(a: u8) -> &'static str {
    match a {
        0 => "none",
        1 => "pulse",
        2 => "orbit",
        3 => "rain",
        4 => "bounce",
        5 => "clip",
        _ => "orbit",
    }
}

fn color_hex(c: u8) -> &'static str {
    match c {
        1 => "3ddc84",
        2 => "ff9f0a",
        3 => "ff5fa8",
        _ => "20c8ff",
    }
}

fn parse_cfg(text: &str) -> Cfg {
    let mut c = Cfg::default();
    for raw in text.lines() {
        let line = raw.trim();
        if let Some((k, v)) = line.split_once('=') {
            match k.trim() {
                "animation" => {
                    c.animation = match v.trim() {
                        "none" => 0,
                        "pulse" => 1,
                        "orbit" => 2,
                        "rain" => 3,
                        "bounce" => 4,
                        "clip" => 5,
                        _ => 2,
                    }
                }
                "color" | "fg" => {
                    let h = v.trim().trim_start_matches('#');
                    c.color = match h {
                        "3ddc84" => 1,
                        "ff9f0a" => 2,
                        "ff5fa8" => 3,
                        _ => 0,
                    };
                }
                "duration_ms" => {
                    let n: u32 = v.trim().parse().unwrap_or(700);
                    c.length = if n < 1100 {
                        0
                    } else if n > 2000 {
                        2
                    } else {
                        1
                    };
                }
                _ => {}
            }
        }
    }
    c
}

fn load_cfg() -> Cfg {
    let Ok(proto) = boot::get_image_file_system(boot::image_handle()) else {
        return Cfg::default();
    };
    let mut fs = FileSystem::new(proto);
    for name in ["splash.cfg", "\\splash.cfg"] {
        if let Ok(path) = CString16::try_from(name) {
            if let Ok(s) = fs.read_to_string(&*path) {
                return parse_cfg(&s);
            }
        }
    }
    Cfg::default()
}

fn save_cfg(c: &Cfg) -> bool {
    let Ok(proto) = boot::get_image_file_system(boot::image_handle()) else {
        return false;
    };
    let mut fs = FileSystem::new(proto);
    let body = format!(
        "animation={}\ncolor=#{}\nbg=#0c0c10\nscale=12\nduration_ms={}\n",
        anim_name(c.animation),
        color_hex(c.color),
        duration_ms(c)
    );
    if let Ok(path) = CString16::try_from("splash.cfg") {
        return fs.write(&*path, body.as_bytes()).is_ok();
    }
    false
}

struct Clip {
    w: usize,
    h: usize,
    n: usize,
    pix: Vec<BltPixel>,
}

fn load_clip() -> Option<Clip> {
    let proto = boot::get_image_file_system(boot::image_handle()).ok()?;
    let mut fs = FileSystem::new(proto);
    let path = CString16::try_from("CLIP.VID").ok()?;
    let bytes = fs.read(&*path).ok()?;
    if bytes.len() < 14 || &bytes[0..8] != b"TEOSCLIP" {
        return None;
    }
    let w = u16::from_le_bytes([bytes[8], bytes[9]]) as usize;
    let h = u16::from_le_bytes([bytes[10], bytes[11]]) as usize;
    let n = u16::from_le_bytes([bytes[12], bytes[13]]) as usize;
    let need = 14 + w.checked_mul(h)?.checked_mul(3)?.checked_mul(n)?;
    if w == 0 || h == 0 || n == 0 || w > 640 || h > 400 || n > 90 || bytes.len() < need {
        return None;
    }
    let mut pix = Vec::with_capacity(w * h * n);
    for chunk in bytes[14..need].chunks_exact(3) {
        pix.push(BltPixel::new(chunk[0], chunk[1], chunk[2]));
    }
    Some(Clip { w, h, n, pix })
}

fn apple_set_os() {
    // Public Mac EFI protocol (0xbb / t2linux hybrid-graphics). No T2 key dump.
    #[repr(C)]
    struct AppleSetOs {
        version: u64,
        set_os_version: Option<unsafe extern "efiapi" fn(*const u8) -> Status>,
        set_os_vendor: Option<unsafe extern "efiapi" fn(*const u8) -> Status>,
    }
    let Some(st) = uefi::table::system_table_raw() else {
        return;
    };
    let st = unsafe { st.as_ref() };
    let Some(bs) = (unsafe { st.boot_services.as_ref() }) else {
        return;
    };
    let guid = uefi::Guid::parse_or_panic("c5c5da95-7d5c-45e6-b2f1-3fd52bb10077");
    let mut iface: *mut AppleSetOs = core::ptr::null_mut();
    let stt = unsafe { (bs.locate_protocol)(&guid, core::ptr::null_mut(), core::ptr::addr_of_mut!(iface).cast()) };
    if stt.is_error() || iface.is_null() {
        return;
    }
    let set_os = unsafe { &*iface };
    let ver = b"Mac OS X 10.9\0";
    let vendor = b"Apple Inc.\0";
    if set_os.version != 0 {
        if let Some(f) = set_os.set_os_version {
            unsafe { let _ = f(ver.as_ptr()); };
        }
    }
    if let Some(f) = set_os.set_os_vendor {
        unsafe { let _ = f(vendor.as_ptr()); };
    }
}

fn start_linux() {
    use uefi::boot::{OpenProtocolAttributes, OpenProtocolParams};
    use uefi_raw::protocol::loaded_image::LoadedImageProtocol;
    apple_set_os();

    let Ok(proto) = boot::get_image_file_system(boot::image_handle()) else {
        return;
    };
    let mut fs = FileSystem::new(proto);
    let Ok(kpath) = CString16::try_from("EFI\\TEOS\\vmlinuz") else {
        return;
    };
    let Ok(kbuf) = fs.read(&*kpath) else {
        return;
    };
    let Ok(kh) = boot::load_image(
        boot::image_handle(),
        LoadImageSource::FromBuffer {
            buffer: &kbuf,
            file_path: None,
        },
    ) else {
        return;
    };
    let our_dev = unsafe {
        boot::open_protocol::<LoadedImage>(
            OpenProtocolParams {
                handle: boot::image_handle(),
                agent: boot::image_handle(),
                controller: None,
            },
            OpenProtocolAttributes::GetProtocol,
        )
        .ok()
        .and_then(|li| li.device())
    };
    let opts = CString16::try_from("console=ttyS0 console=tty0 intel_iommu=on iommu=pt pm_async=off rdinit=/init initrd=\\EFI\\TEOS\\initrd.gz")
        .ok();
    if let Some(opts) = opts {
        let leaked: &'static CString16 = alloc::boxed::Box::leak(alloc::boxed::Box::new(opts));
        if let Ok(mut li) = boot::open_protocol_exclusive::<LoadedImage>(kh) {
            let bytes = leaked.num_chars() * 2 + 2;
            unsafe {
                li.set_load_options(leaked.as_ptr().cast::<u8>(), bytes as u32);
                if let Some(dev) = our_dev {
                    let raw = &mut *(&mut *li as *mut LoadedImage as *mut LoadedImageProtocol);
                    raw.device_handle = dev.as_ptr();
                }
            }
        }
    }
    let _ = boot::start_image(kh);
}

fn blit_clip(gop: &mut GraphicsOutput, clip: &Clip, ox: usize, oy: usize, pw: usize, ph: usize, t: u32) {
    // One frame only — looping GOP blits strobe on T2 panels.
    let _ = t;
    let fi = 0usize;
    let off = fi * clip.w * clip.h;
    let end = off + clip.w * clip.h;
    if end > clip.pix.len() {
        return;
    }
    let dw = clip.w.min(pw);
    let dh = clip.h.min(ph);
    let dx = ox + pw.saturating_sub(dw) / 2;
    let dy = oy + ph.saturating_sub(dh) / 2;
    let _ = gop.blt(BltOp::BufferToVideo {
        buffer: &clip.pix[off..end],
        src: BltRegion::Full,
        dest: (dx, dy),
        dims: (dw, dh),
    });
}

fn glyph_bits(ch: u8) -> [u8; 7] {
    match ch {
        b'A' => [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        b'B' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
        b'C' => [0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111],
        b'D' => [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
        b'E' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
        b'F' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
        b'G' => [0b01111, 0b10000, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110],
        b'H' => [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        b'I' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111],
        b'K' => [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001],
        b'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        b'M' => [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
        b'N' => [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
        b'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        b'P' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
        b'R' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
        b'S' => [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
        b'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        b'U' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        b'V' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
        b'W' => [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010],
        b'Y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        b' ' => [0; 7],
        _ => [0b11111, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11111],
    }
}

fn fill(gop: &mut GraphicsOutput, c: BltPixel, x: usize, y: usize, w: usize, h: usize) {
    if w == 0 || h == 0 {
        return;
    }
    let _ = gop.blt(BltOp::VideoFill {
        color: c,
        dest: (x, y),
        dims: (w, h),
    });
}

fn plot_glyph(
    gop: &mut GraphicsOutput,
    gx: usize,
    gy: usize,
    bits: [u8; 7],
    s: usize,
    col: BltPixel,
    sw: usize,
    sh: usize,
) {
    for (row, mask) in bits.iter().enumerate() {
        for coln in 0..5 {
            if mask & (1 << (4 - coln)) != 0 {
                let x = gx + coln * s;
                let y = gy + row * s;
                if x + s < sw && y + s < sh {
                    fill(gop, col, x, y, s, s);
                }
            }
        }
    }
}

fn text(gop: &mut GraphicsOutput, x: usize, y: usize, msg: &str, s: usize, col: BltPixel, sw: usize, sh: usize) {
    let mut cx = x;
    for ch in msg.bytes() {
        let up = if (b'a'..=b'z').contains(&ch) { ch - 32 } else { ch };
        plot_glyph(gop, cx, y, glyph_bits(up), s, col, sw, sh);
        cx += 6 * s;
    }
}

fn logo(gop: &mut GraphicsOutput, x0: isize, y0: isize, scale: usize, col: BltPixel, w: usize, h: usize) {
    text(gop, x0.max(0) as usize, y0.max(0) as usize, "TEOS", scale, col, w, h);
}

fn sin_cos(deg: i32) -> (i32, i32) {
    let d = ((deg % 360) + 360) % 360;
    let q = d / 90;
    let r = d % 90;
    let s = r * 256 / 90;
    match q {
        0 => (s, 256 - s),
        1 => (256 - s, -s),
        2 => (-s, -256 + s),
        _ => (-256 + s, s),
    }
}

fn preview(
    gop: &mut GraphicsOutput,
    cfg: &Cfg,
    clip: Option<&Clip>,
    ox: usize,
    oy: usize,
    pw: usize,
    ph: usize,
    t: u32,
) {
    if cfg.animation == 5 {
        if let Some(c) = clip {
            blit_clip(gop, c, ox, oy, pw, ph, t);
            return;
        }
        fill(gop, BG, ox, oy, pw, ph);
        text(gop, ox + 16, oy + ph / 2, "NO CLIP YET", 3, MUTE, ox + pw, oy + ph);
        return;
    }
    fill(gop, BG, ox, oy, pw, ph);
    if cfg.animation == 0 {
        let sc = 8usize;
        let x0 = ox as isize + (pw.saturating_sub(4 * 6 * sc) / 2) as isize;
        let y0 = oy as isize + (ph.saturating_sub(7 * sc) / 2) as isize;
        logo(gop, x0, y0, sc, ink(cfg), ox + pw, oy + ph);
        return;
    }
    let base = 8usize;
    let col = ink(cfg);
    let (x0, y0, sc) = match cfg.animation {
        1 => {
            let p = ((t / 8) % 40) as isize;
            let d = if p < 20 { p } else { 40 - p };
            let sc = (base as isize + d / 5).max(4) as usize;
            (
                ox as isize + (pw.saturating_sub(4 * 6 * sc) / 2) as isize,
                oy as isize + (ph.saturating_sub(7 * sc) / 2) as isize,
                sc,
            )
        }
        4 => {
            let x = ox as isize + ((t / 2) as usize % pw.saturating_sub(80).max(1)) as isize;
            (x, oy as isize + ph as isize / 3, base)
        }
        _ => (
            ox as isize + (pw.saturating_sub(4 * 6 * base) / 2) as isize,
            oy as isize + (ph.saturating_sub(7 * base) / 2) as isize,
            base,
        ),
    };
    logo(gop, x0, y0, sc, col, ox + pw, oy + ph);
    if cfg.animation == 2 {
        let cx = (ox + pw / 2) as i32;
        let cy = (oy + ph / 2) as i32;
        let r = (core::cmp::min(pw, ph) / 4) as i32;
        for i in 0..8 {
            let a = t as i32 / 4 + i * 45;
            let (s, c) = sin_cos(a);
            let x = cx + (r * c) / 256;
            let y = cy + (r * s) / 256;
            if (x as usize) > ox && (y as usize) > oy {
                fill(gop, col, x as usize, y as usize, 6, 6);
            }
        }
    }
    if cfg.animation == 3 {
        for i in 0..18u32 {
            let x = ox + ((i * 97 + t / 3) as usize * 11) % pw.saturating_sub(4).max(1);
            let y = oy + ((i * 53 + t) as usize * 7) % ph.saturating_sub(8).max(1);
            fill(gop, col, x, y, 2, 6);
        }
    }
}

struct Hit {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    kind: u8,
    id: u8,
}

fn in_hit(h: &Hit, x: usize, y: usize) -> bool {
    x >= h.x && y >= h.y && x < h.x + h.w && y < h.y + h.h
}

fn tile(gop: &mut GraphicsOutput, h: &Hit, on: bool, label: &str, sw: usize, sh: usize) {
    fill(gop, if on { ON } else { PANEL }, h.x, h.y, h.w, h.h);
    if on {
        fill(
            gop,
            PANEL,
            h.x + 4,
            h.y + 4,
            h.w.saturating_sub(8),
            h.h.saturating_sub(8),
        );
    }
    let ts = 2;
    let tw = label.len() * 6 * ts;
    let tx = h.x + h.w.saturating_sub(tw) / 2;
    let ty = h.y + h.h.saturating_sub(7 * ts) / 2;
    text(gop, tx, ty, label, ts, WHITE, sw, sh);
}

fn apply_hit(cfg: &mut Cfg, h: &Hit) -> bool {
    match h.kind {
        1 => cfg.animation = h.id,
        2 => cfg.color = h.id,
        3 => cfg.length = h.id,
        4 => return true,
        _ => {}
    }
    false
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().ok();
    let mut cfg = load_cfg();
    let clip = load_clip();
    let has_clip = clip.is_some();
    if cfg.animation == 5 && !has_clip {
        cfg.animation = 2;
    }
    let handle = match boot::get_handle_for_protocol::<GraphicsOutput>() {
        Ok(h) => h,
        Err(_) => return Status::ABORTED,
    };
    let mut gop = match boot::open_protocol_exclusive::<GraphicsOutput>(handle) {
        Ok(g) => g,
        Err(_) => return Status::ABORTED,
    };
    let (w, h) = gop.current_mode_info().resolution();
    let mut t = 0u32;
    let mut saved = false;
    let mut upload = false;
    let mut go_linux = false;
    let mut dirty = true;
    let looks = ["CIRCLES", "GROW", "RAIN", "BOUNCE", "MY CLIP"];
    let colors = ["BLUE", "GREEN", "ORANGE", "PINK"];
    let times = ["SHORT", "MEDIUM", "LONG"];

    loop {
        let preview_h = h * 40 / 100;
        let gap = 10usize;
        let tw = w.saturating_sub(gap * 6) / 5;
        let th = 48usize;
        let row_y = preview_h + 32;
        let mut hits: Vec<Hit> = Vec::new();
        for i in 0..5 {
            hits.push(Hit {
                x: gap + i * (tw + gap),
                y: row_y,
                w: tw,
                h: th,
                kind: 1,
                id: [2, 1, 3, 4, 5][i],
            });
        }
        let row2 = row_y + th + gap;
        for i in 0..4 {
            hits.push(Hit {
                x: gap + i * ((w.saturating_sub(gap * 5) / 4) + gap),
                y: row2,
                w: w.saturating_sub(gap * 5) / 4,
                h: th,
                kind: 2,
                id: i as u8,
            });
        }
        let row3 = row2 + th + gap;
        let tw3 = w.saturating_sub(gap * 4) / 3;
        for i in 0..3 {
            hits.push(Hit {
                x: gap + i * (tw3 + gap),
                y: row3,
                w: tw3,
                h: th,
                kind: 3,
                id: i as u8,
            });
        }
        hits.push(Hit {
            x: w / 2 - 200,
            y: row3 + th + gap,
            w: 160,
            h: 52,
            kind: 4,
            id: 0,
        });
        hits.push(Hit {
            x: w / 2 - 20,
            y: row3 + th + gap,
            w: 160,
            h: 52,
            kind: 9,
            id: 0,
        });
        hits.push(Hit {
            x: w / 2 + 160,
            y: row3 + th + gap,
            w: 160,
            h: 52,
            kind: 6,
            id: 0,
        });

        if upload {
            hits.clear();
            hits.push(Hit {
                x: 24,
                y: h.saturating_sub(70),
                w: 140,
                h: 48,
                kind: 7,
                id: 0,
            });
            if has_clip {
                hits.push(Hit {
                    x: w.saturating_sub(220),
                    y: h.saturating_sub(70),
                    w: 200,
                    h: 48,
                    kind: 8,
                    id: 0,
                });
            }
        }

        if dirty {
        fill(&mut gop, BG, 0, 0, w, h);
        preview(&mut gop, &cfg, clip.as_ref(), 0, 0, w, preview_h, t);
        if upload {
            fill(&mut gop, PANEL, 20, preview_h + 8, w.saturating_sub(40), h.saturating_sub(preview_h + 16));
            text(&mut gop, 40, preview_h + 28, "UPLOAD", 3, WHITE, w, h);
            text(&mut gop, 40, preview_h + 70, "COPY A VIDEO ONTO THIS STICK", 2, WHITE, w, h);
            text(&mut gop, 40, preview_h + 100, "IN FINDER  THEN REBOOT", 2, MUTE, w, h);
            if has_clip {
                text(&mut gop, 40, preview_h + 140, "CLIP IS READY", 3, ink(&cfg), w, h);
            } else {
                text(&mut gop, 40, preview_h + 140, "NO CLIP YET", 3, MUTE, w, h);
            }
            tile(&mut gop, &hits[0], false, "BACK", w, h);
            if hits.len() > 1 {
                tile(&mut gop, &hits[1], true, "USE CLIP", w, h);
            }
        } else {
        text(&mut gop, 16, preview_h + 8, "LOOK", 2, MUTE, w, h);
        for i in 0..5 {
            let on = cfg.animation == hits[i].id;
            let label = if i == 4 && !has_clip { "NO CLIP" } else { looks[i] };
            tile(&mut gop, &hits[i], on && !(i == 4 && !has_clip), label, w, h);
        }
        for i in 0..4 {
            let ht = &hits[5 + i];
            fill(&mut gop, if cfg.color == i as u8 { ON } else { PANEL }, ht.x, ht.y, ht.w, ht.h);
            fill(
                &mut gop,
                PAL[i],
                ht.x + 8,
                ht.y + 8,
                ht.w.saturating_sub(16),
                ht.h.saturating_sub(16),
            );
            text(&mut gop, ht.x + 12, ht.y + ht.h / 2 - 7, colors[i], 2, WHITE, w, h);
        }
        for i in 0..3 {
            tile(&mut gop, &hits[9 + i], cfg.length == i as u8, times[i], w, h);
        }
        let sv = &hits[12];
        fill(&mut gop, ink(&cfg), sv.x, sv.y, sv.w, sv.h);
        text(
            &mut gop,
            sv.x + 36,
            sv.y + 16,
            if saved { "SAVED" } else { "SAVE" },
            3,
            BG,
            w,
            h,
        );
        let stt = &hits[13];
        fill(&mut gop, ink(&cfg), stt.x, stt.y, stt.w, stt.h);
        text(&mut gop, stt.x + 28, stt.y + 16, "START", 3, BG, w, h);
        let up = &hits[14];
        fill(&mut gop, WHITE, up.x, up.y, up.w, up.h);
        text(&mut gop, up.x + 24, up.y + 16, "UPLOAD", 3, BG, w, h);
        text(
            &mut gop,
            16,
            h.saturating_sub(28),
            "CLICK A BOX   OR ARROWS THEN ENTER",
            2,
            MUTE,
            w,
            h,
        );
        }
        dirty = false;
        }

        let pressed = core::cell::RefCell::new(None);
        uefi::system::with_stdin(|inp| {
            if let Ok(k) = inp.read_key() {
                *pressed.borrow_mut() = k;
            }
        });
        if let Some(k) = pressed.into_inner() {
            dirty = true;
            match k {
                Key::Special(ScanCode::LEFT) => {
                    cfg.animation = match cfg.animation {
                        1 => 4,
                        2 => 1,
                        3 => 2,
                        4 => {
                            if has_clip {
                                5
                            } else {
                                3
                            }
                        }
                        5 => 4,
                        _ => 2,
                    };
                    saved = false;
                }
                Key::Special(ScanCode::RIGHT) => {
                    cfg.animation = match cfg.animation {
                        1 => 2,
                        2 => 3,
                        3 => 4,
                        4 => {
                            if has_clip {
                                5
                            } else {
                                1
                            }
                        }
                        5 => 1,
                        _ => 2,
                    };
                    saved = false;
                }
                Key::Special(ScanCode::UP) => {
                    cfg.color = (cfg.color + 3) % 4;
                    saved = false;
                }
                Key::Special(ScanCode::DOWN) => {
                    cfg.color = (cfg.color + 1) % 4;
                    saved = false;
                }
                Key::Printable(ch) => {
                    let c: char = ch.into();
                    if c == '\r' {
                        saved = save_cfg(&cfg);
                    }
                    if c == ' ' {
                        go_linux = true;
                    }
                }
                _ => {}
            }
        }

        t = t.wrapping_add(1);
        boot::stall(80_000);
        if go_linux {
            break;
        }
    }
    drop(gop);
    start_linux();
    loop {
        boot::stall(1_000_000);
    }
}
