//! The low level color math behind the more high level
//! [`palette`](https://crates.io/crates/palette) crate. This create's primary
//! purpose is to facilitate `palette`, but its content is also useable on its
//! own.
//!
//! # Getting Started
//!
//! Add the following lines to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! palette_math = "0.7.6"
//! ```
//!
//! or this if you want to opt out of `std`:
//!
//! ```toml
//! # Uses libm instead of std for floating point math:
//! palette_math = { version = "0.7.6", features = ["libm"], default-features = false }
//! ```
//!
//! ## Cargo Features
//!
//! These features are enabled by default:
//!
//! * `"std"` - Enables use of the `std` library, primarily for floating point
//!   math. Also enables `"alloc"`.
//! * `"alloc"` - Enables the use of types that may allocate memory, such as
//!   [`Vec`][alloc::vec::Vec].
//!
//! These features are disabled by default:
//!
//! * `"libm"` - Uses the [`libm`](https://crates.io/crates/libm) floating point
//!   math library (for when the `std` feature is disabled).
//! * `"wide"` - Enables support for using SIMD types from
//!   [`wide`](https://crates.io/crates/wide).
//!
//! ## Embedded And `#![no_std]`
//!
//! `palette_math` supports `#![no_std]` environments by disabling the `"std"`
//! feature. It uses [`libm`], via the `"libm"` feature, to provide the
//! floating-point operations that are typically in `std`, and the `"alloc"`
//! feature to provide features that use allocating types.

#![no_std]
#![doc(html_root_url = "https://docs.rs/palette_math/0.7.6/")]
#![warn(missing_docs)]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

// Utilities
pub mod num;

pub mod gamma;
pub mod lut;
