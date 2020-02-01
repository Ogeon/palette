//! Derives traits from the [palette](https://crates.io/crates/palette) crate.

#![cfg_attr(feature = "strict", deny(warnings))]
#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;

mod convert;
mod encoding;
mod meta;
mod util;

const COLOR_TYPES: &[&str] = &[
    "Rgb", "Luma", "Hsl", "Hsv", "Hwb", "Lab", "Lch", "Xyz", "Yxy",
];

#[proc_macro_derive(
    FromColor,
    attributes(
        palette_internal,
        palette_white_point,
        palette_component,
        palette_manual_from,
        palette_rgb_space,
        palette_alpha
    )
)]
pub fn derive_from_color(tokens: TokenStream) -> TokenStream {
    convert::derive_from_color(tokens)
}

#[proc_macro_derive(
    IntoColor,
    attributes(
        palette_internal,
        palette_white_point,
        palette_component,
        palette_manual_into,
        palette_rgb_space,
        palette_alpha
    )
)]
pub fn derive_into_color(tokens: TokenStream) -> TokenStream {
    convert::derive_into_color(tokens)
}

#[proc_macro_derive(
    Pixel,
    attributes(
        palette_internal,
        palette_unsafe_same_layout_as,
        palette_unsafe_zero_sized
    )
)]
pub fn derive_pixel(tokens: TokenStream) -> TokenStream {
    encoding::derive_pixel(tokens)
}
