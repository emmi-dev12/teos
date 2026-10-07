#![no_main]
#![no_std]

extern crate alloc;

use alloc::format;
use alloc::vec::Vec;
use uefi::CString16;
use uefi::fs::FileSystem;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};
use uefi::proto::console::pointer::Pointer;
use uefi::proto::console::text::{Key, ScanCode};

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
            animation: 2,
            color: 0,
            length: 1,
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
        0 => 1200,
        2 => 5000,
        _ => 2800,
    }
}

fn anim_name(a: u8) -> &'static str {
    match a {
        1 => "pulse",
        2 => "orbit",
        3 => "rain",
        4 => "bounce",
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
                        "pulse" => 1,
                        "orbit" => 2,
                        "rain" => 3,
                        "bounce" => 4,
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
                    let n: u32 = v.trim().parse().unwrap_or(2800);
                    c.length = if n < 1800 {
                        0
                    } else if n > 3600 {
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

fn glyph_bits(ch: u8) -> [u8; 7] {
    match ch {
        b'A' => [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        b'B' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
        b'C' => [0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111],
        b'D' => [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
        b'E' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
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

fn preview(gop: &mut GraphicsOutput, cfg: &Cfg, ox: usize, oy: usize, pw: usize, ph: usize, t: u32) {
    fill(gop, BG, ox, oy, pw, ph);
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
    let handle = match boot::get_handle_for_protocol::<GraphicsOutput>() {
        Ok(h) => h,
        Err(_) => return Status::ABORTED,
    };
    let mut gop = match boot::open_protocol_exclusive::<GraphicsOutput>(handle) {
        Ok(g) => g,
        Err(_) => return Status::ABORTED,
    };
    let (w, h) = gop.current_mode_info().resolution();
    let mut ptr = boot::get_handle_for_protocol::<Pointer>()
        .ok()
        .and_then(|hh| boot::open_protocol_exclusive::<Pointer>(hh).ok());
    let mut mx = w / 2;
    let mut my = h / 2;
    let mut t = 0u32;
    let mut saved = false;
    let looks = ["CIRCLES", "GROW", "RAIN", "BOUNCE"];
    let colors = ["BLUE", "GREEN", "ORANGE", "PINK"];
    let times = ["SHORT", "MEDIUM", "LONG"];

    loop {
        let preview_h = h * 40 / 100;
        let gap = 10usize;
        let tw = w.saturating_sub(gap * 5) / 4;
        let th = 48usize;
        let row_y = preview_h + 32;
        let mut hits: Vec<Hit> = Vec::new();
        for i in 0..4 {
            hits.push(Hit {
                x: gap + i * (tw + gap),
                y: row_y,
                w: tw,
                h: th,
                kind: 1,
                id: [2, 1, 3, 4][i],
            });
        }
        let row2 = row_y + th + gap;
        for i in 0..4 {
            hits.push(Hit {
                x: gap + i * (tw + gap),
                y: row2,
                w: tw,
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
            x: w / 2 - 90,
            y: row3 + th + gap,
            w: 180,
            h: 52,
            kind: 4,
            id: 0,
        });

        fill(&mut gop, BG, 0, 0, w, h);
        preview(&mut gop, &cfg, 0, 0, w, preview_h, t);
        text(&mut gop, 16, preview_h + 8, "LOOK", 2, MUTE, w, h);
        for i in 0..4 {
            tile(&mut gop, &hits[i], cfg.animation == hits[i].id, looks[i], w, h);
        }
        for i in 0..4 {
            let ht = &hits[4 + i];
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
            tile(&mut gop, &hits[8 + i], cfg.length == i as u8, times[i], w, h);
        }
        let sv = &hits[11];
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
        fill(&mut gop, WHITE, mx.saturating_sub(2), my.saturating_sub(2), 8, 8);

        if let Some(p) = ptr.as_mut() {
            if let Ok(Some(st)) = p.read_state() {
                mx = (mx as i32 + st.relative_movement[0] / 2048).clamp(0, w as i32 - 1) as usize;
                my = (my as i32 + st.relative_movement[1] / 2048).clamp(0, h as i32 - 1) as usize;
                if st.button[0] {
                    for hit in hits.iter() {
                        if in_hit(hit, mx, my) {
                            if apply_hit(&mut cfg, hit) {
                                saved = save_cfg(&cfg);
                            } else {
                                saved = false;
                            }
                        }
                    }
                }
            }
        }

        let pressed = core::cell::RefCell::new(None);
        uefi::system::with_stdin(|inp| {
            if let Ok(k) = inp.read_key() {
                *pressed.borrow_mut() = k;
            }
        });
        if let Some(k) = pressed.into_inner() {
            match k {
                Key::Special(ScanCode::LEFT) => {
                    cfg.animation = match cfg.animation {
                        1 => 4,
                        2 => 1,
                        3 => 2,
                        4 => 3,
                        _ => 2,
                    };
                    saved = false;
                }
                Key::Special(ScanCode::RIGHT) => {
                    cfg.animation = match cfg.animation {
                        1 => 2,
                        2 => 3,
                        3 => 4,
                        4 => 1,
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
                }
                _ => {}
            }
        }

        t = t.wrapping_add(1);
        boot::stall(16_000);
    }
}
