// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use super::Lab;

#[automatically_derived]
unsafe impl<Wp, T> crate::cast::ArrayCast for Lab<Wp, T> {
    type Array = [T; 3usize];
}

