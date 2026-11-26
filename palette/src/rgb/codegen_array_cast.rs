// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use super::Rgb;

#[automatically_derived]
unsafe impl<S, T> crate::cast::ArrayCast for Rgb<S, T> {
    type Array = [T; 3usize];
}

