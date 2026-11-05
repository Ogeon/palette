// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use super::Hsv;

#[automatically_derived]
unsafe impl<S, T> crate::cast::ArrayCast for Hsv<S, T> {
    type Array = [T; 3usize];
}

