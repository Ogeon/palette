//! Derives traits from the [palette](https://crates.io/crates/palette) crate.

#![cfg_attr(feature = "strict", deny(warnings))]
#![recursion_limit = "128"]

extern crate proc_macro2;
extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

mod util;
mod meta;
mod from_color;

const COLOR_TYPES: &[&str] = &[
    "Rgb", "Luma", "Hsl", "Hsv", "Hwb", "Lab", "Lch", "Xyz", "Yxy"
];

#[proc_macro_derive(FromColor,
                    attributes(palette_internal, palette_white_point, palette_component,
                               palette_manual_from, palette_rgb_space))]
pub fn derive_from_color(tokens: TokenStream) -> TokenStream {
    from_color::derive(tokens)
}
