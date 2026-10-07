#![no_main]
#![no_std]

extern crate alloc;

use uefi::CString16;
use uefi::fs::FileSystem;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};

#[no_mangle]
pub unsafe extern "C" fn wcslen(s: *const u16) -> usize {
    let mut n = 0;
    while *s.add(n) != 0 {
        n += 1;
    }
    n
}

struct Cfg {
    animation: u8, // 0 none 1 pulse 2 orbit 3 rain 4 bounce
    fg: BltPixel,
    bg: BltPixel,
    scale: usize,
    duration_ms: u32,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            animation: 2,
            fg: BltPixel::new(0x20, 0xc8, 0xff),
            bg: BltPixel::new(0x08, 0x08, 0x0c),
            scale: 12,
            duration_ms: 2800,
        }
    }
}

fn hex_byte(s: &[u8]) -> u8 {
    fn n(c: u8) -> u8 {
        match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            b'A'..=b'F' => c - b'A' + 10,
            _ => 0,
        }
    }
    if s.len() < 2 {
        return 0;
    }
    n(s[0]) * 16 + n(s[1])
}

fn parse_cfg(text: &str) -> Cfg {
    let mut c = Cfg::default();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        let k = k.trim();
        let v = v.trim();
        match k {
            "animation" => {
                c.animation = match v {
                    "none" => 0,
                    "pulse" => 1,
                    "orbit" => 2,
                    "rain" => 3,
                    "bounce" => 4,
                    _ => 2,
                }
            }
            "color" | "fg" => {
                let h = v.trim_start_matches('#');
                if h.len() >= 6 {
                    let b = h.as_bytes();
                    c.fg = BltPixel::new(hex_byte(&b[0..2]), hex_byte(&b[2..4]), hex_byte(&b[4..6]));
                }
            }
            "bg" => {
                let h = v.trim_start_matches('#');
                if h.len() >= 6 {
                    let b = h.as_bytes();
                    c.bg = BltPixel::new(hex_byte(&b[0..2]), hex_byte(&b[2..4]), hex_byte(&b[4..6]));
                }
            }
            "scale" => c.scale = v.parse().unwrap_or(12).clamp(4, 40),
            "duration_ms" => c.duration_ms = v.parse().unwrap_or(2800).clamp(400, 20000),
            _ => {}
        }
    }
    c
}

fn load_cfg() -> Cfg {
    let Ok(proto) = boot::get_image_file_system(boot::image_handle()) else {
        return Cfg::default();
    };
    let mut fs = FileSystem::new(proto);
    for name in ["splash.cfg", "\\splash.cfg", "EFI\\TEOS\\splash.cfg"] {
        if let Ok(path) = CString16::try_from(name) {
            if let Ok(s) = fs.read_to_string(&*path) {
                return parse_cfg(&s);
            }
        }
    }
    Cfg::default()
}

const FONT: [[u8; 7]; 4] = [
    [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
    [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
    [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
    [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
];

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

fn glyph(
    gop: &mut GraphicsOutput,
    gx: isize,
    gy: isize,
    bits: &[u8; 7],
    scale: usize,
    fg: BltPixel,
    sw: usize,
    sh: usize,
) {
    for (row, mask) in bits.iter().enumerate() {
        for col in 0..5 {
            if mask & (1 << (4 - col)) != 0 {
                let x = gx + (col * scale) as isize;
                let y = gy + (row * scale) as isize;
                if x >= 0 && y >= 0 && (x as usize) + scale < sw && (y as usize) + scale < sh {
                    fill(gop, fg, x as usize, y as usize, scale, scale);
                }
            }
        }
    }
}

fn logo(
    gop: &mut GraphicsOutput,
    x0: isize,
    y0: isize,
    scale: usize,
    fg: BltPixel,
    w: usize,
    h: usize,
) {
    let letter_w = (6 * scale) as isize;
    for (i, g) in FONT.iter().enumerate() {
        glyph(gop, x0 + i as isize * letter_w, y0, g, scale, fg, w, h);
    }
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

fn frame(gop: &mut GraphicsOutput, cfg: &Cfg, w: usize, h: usize, t: u32) {
    fill(gop, cfg.bg, 0, 0, w, h);
    let base = cfg.scale;
    let (x0, y0, sc) = match cfg.animation {
        1 => {
            let p = ((t / 8) % 40) as isize;
            let d = if p < 20 { p } else { 40 - p };
            let sc = (base as isize + d / 4).max(4) as usize;
            let lw = 4 * 6 * sc;
            (
                (w.saturating_sub(lw) / 2) as isize,
                (h.saturating_sub(7 * sc) / 2) as isize,
                sc,
            )
        }
        4 => {
            let period = (w / 2 + 40) as u32;
            let phase = (t / 2) % (period * 2);
            let x = if phase < period {
                phase as isize
            } else {
                (period * 2 - phase) as isize
            };
            let y = (h / 3) as isize + (((t / 3) % 40) as i32 - 20).unsigned_abs() as isize;
            (x, y, base)
        }
        _ => {
            let lw = 4 * 6 * base;
            (
                (w.saturating_sub(lw) / 2) as isize,
                (h.saturating_sub(7 * base) / 2) as isize,
                base,
            )
        }
    };
    logo(gop, x0, y0, sc, cfg.fg, w, h);

    if cfg.animation == 2 {
        let cx = w as i32 / 2;
        let cy = h as i32 / 2;
        let r = (core::cmp::min(w, h) / 4) as i32;
        for i in 0..8 {
            let a = t as i32 / 4 + i * 45;
            let (s, c) = sin_cos(a);
            let x = cx + (r * c) / 256;
            let y = cy + (r * s) / 256;
            if x > 4 && y > 4 && (x as usize) + 4 < w && (y as usize) + 4 < h {
                fill(gop, cfg.fg, x as usize - 4, y as usize - 4, 8, 8);
            }
        }
    }
    if cfg.animation == 3 {
        for i in 0..24u32 {
            let x = ((i * 97 + t / 3) as usize * 13) % w.saturating_sub(2).max(1);
            let y = ((i * 53 + t) as usize * 7) % h.saturating_sub(2).max(1);
            fill(gop, cfg.fg, x, y, 2, 6);
        }
    }
}

fn play(cfg: &Cfg) {
    let handle = match boot::get_handle_for_protocol::<GraphicsOutput>() {
        Ok(h) => h,
        Err(_) => return,
    };
    let mut gop = match boot::open_protocol_exclusive::<GraphicsOutput>(handle) {
        Ok(g) => g,
        Err(_) => return,
    };
    let (w, h) = gop.current_mode_info().resolution();
    let frames = (cfg.duration_ms / 16).max(1);
    for t in 0..frames {
        frame(&mut gop, cfg, w, h, t);
        boot::stall(16_000);
    }
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().ok();
    let cfg = load_cfg();
    play(&cfg);
    uefi::system::with_stdout(|out| {
        let _ = out.output_string(cstr16!("\r\nTeOS\r\nLinux userspace. USB only.\r\n"));
    });
    loop {
        boot::stall(1_000_000);
    }
}
