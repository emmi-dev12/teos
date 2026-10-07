#![no_std]

#[cfg(test)]
extern crate std;

/// Own boot handoff. Not multiboot, not Linux boot_params.
#[repr(C)]
pub struct TeosBootInfo {
    pub magic: u64,
    pub fb_ptr: u64,
    pub fb_width: u32,
    pub fb_height: u32,
    pub fb_stride: u32,
}

pub const TEOS_MAGIC: u64 = 0x5445_4F53_7630_2E31; // "TEOSv0.1"

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_is_stable() {
        assert_eq!(TEOS_MAGIC, 0x5445_4F53_7630_2E31);
    }
}
