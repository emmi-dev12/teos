#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};

// rustc UEFI target has no libc; uefi 0.33 still references wcslen.
#[no_mangle]
pub unsafe extern "C" fn wcslen(s: *const u16) -> usize {
    let mut n = 0;
    while *s.add(n) != 0 {
        n += 1;
    }
    n
}

const COL: BltPixel = BltPixel::new(0x20, 0xc8, 0xff);
const BG: BltPixel = BltPixel::new(0x08, 0x08, 0x0c);

// 5x7 caps, bit0 = left
const FONT: [[u8; 7]; 4] = [
    // T
    [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
    // E
    [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
    // O
    [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
    // S
    [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
];

fn glyph(gop: &mut GraphicsOutput, gx: usize, gy: usize, bits: &[u8; 7], scale: usize) {
    for (row, mask) in bits.iter().enumerate() {
        for col in 0..5 {
            if mask & (1 << (4 - col)) != 0 {
                let x = gx + col * scale;
                let y = gy + row * scale;
                let _ = gop.blt(BltOp::VideoFill {
                    color: COL,
                    dest: (x, y),
                    dims: (scale, scale),
                });
            }
        }
    }
}

fn paint() {
    let handle = match boot::get_handle_for_protocol::<GraphicsOutput>() {
        Ok(h) => h,
        Err(_) => return,
    };
    let mut gop = match boot::open_protocol_exclusive::<GraphicsOutput>(handle) {
        Ok(g) => g,
        Err(_) => return,
    };
    let (w, h) = gop.current_mode_info().resolution();
    let _ = gop.blt(BltOp::VideoFill {
        color: BG,
        dest: (0, 0),
        dims: (w, h),
    });
    let scale = 12usize;
    let letter_w = 6 * scale;
    let total = 4 * letter_w;
    let x0 = w.saturating_sub(total) / 2;
    let y0 = h.saturating_sub(7 * scale) / 2;
    for (i, g) in FONT.iter().enumerate() {
        glyph(&mut gop, x0 + i * letter_w, y0, g, scale);
    }
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().ok();
    paint();
    uefi::system::with_stdout(|out| {
        let _ = out.output_string(cstr16!("\r\nTeOS\r\nUSB only. Read DISCLAIMER.md\r\n"));
    });
    loop {
        boot::stall(1_000_000);
    }
}
