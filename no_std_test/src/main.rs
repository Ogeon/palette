#![cfg_attr(feature = "nightly", feature(lang_items, start))]
#![no_std]

#[cfg(feature = "nightly")]
use core::panic::PanicInfo;

extern crate libc;

#[cfg(feature = "nightly")]
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let _magenta = palette::Srgb::new(255u8, 0, 255);

    0
}

#[cfg(feature = "nightly")]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(feature = "nightly")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(feature = "nightly"))]
fn main() {}
