// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use super::Lms;

#[automatically_derived]
unsafe impl<M, T> crate::cast::ArrayCast for Lms<M, T> {
    type Array = [T; 3usize];
}

