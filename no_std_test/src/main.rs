#![feature(lang_items, start)]
#![no_std]

extern crate libc;

extern crate palette;

use core::panic::PanicInfo;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let _magenta = palette::Srgb::new(255u8, 0, 255);

    0
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
