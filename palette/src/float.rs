//!Floating point traits
//! 
//!This module is work-around for the lack of floating point operations under `#![no_std]`. If you
//!haven't disabled the `std` feature, it shouldn't affect you.
//!
//!Because conversions between different color encodings requrire floating point functions such as
//!`powf`, `sin`, `cos`, etc. which are unavailable without the standard library, it looks for
//!externally linked symbols that implement these functions.
//!
//!These external functions are needed without the standard library:
//!```rust
//!extern "C" {
//!    // The square root of `x`
//!    // Should be compatible with `x.sqrt()` from the standard library
//!    fn sqrtf32(x: f32) -> f32;
//!    fn sqrtf64(x: f64) -> f64;
//!    // `x` to the power of `y`
//!    // Should be compatible with `x.powf(y)` from the standard library
//!    fn powf32(x: f32, y: f32) -> f32;
//!    fn powf64(x: f64, y: f64) -> f64;
//!    // The sine of `x` radians
//!    // Should be compatible with `x.sin()` from the standard library
//!    fn sinf32(x: f32) -> f32;
//!    fn sinf64(x: f64) -> f64;
//!    // The cosine of `x` radians
//!    // Should be compatible with `x.cos()` from the standard library
//!    fn cosf32(x: f32) -> f32;
//!    fn cosf64(x: f64) -> f64;
//!    // Inverse tangent
//!    // Should be compatible with `y.atan2(x)` from the standard library
//!    fn atan2f32(y: f32, x: f32) -> f32;
//!    fn atan2f64(y: f64, x: f64) -> f64;
//!}
//!```
//!
//!There are different ways to deal with it:
//! * Implement it yourself
//! * Use `core::intrinsics`
//! * Provide interfaces to a different math library (e.g. Julia's `libm`)
//! * Don't use the features that need these functions and enable LTO (this is not guaranteed to be
//!   stable between patch releases)

#[cfg(feature = "std")]
pub use num_traits::Float;

#[cfg(not(feature = "std"))]
pub use self::no_std_float_hack::Float;

#[cfg(not(feature = "std"))]
mod no_std_float_hack {
    pub trait Float: ::num_traits::float::FloatCore {
        fn sqrt(self) -> Self;
        fn powf(self, other: Self) -> Self;
        fn sin(self) -> Self;
        fn cos(self) -> Self;
        fn atan2(self, other: Self) -> Self;
    }

    extern "C" {
        // The square root of `x`
        // Should be compatible with `x.sqrt()` from the standard library
        fn sqrtf32(x: f32) -> f32;
        fn sqrtf64(x: f64) -> f64;
        // `x` to the power of `y`
        // Should be compatible with `x.powf(y)` from the standard library
        fn powf32(x: f32, y: f32) -> f32;
        fn powf64(x: f64, y: f64) -> f64;
        // The sine of `x` radians
        // Should be compatible with `x.sin()` from the standard library
        fn sinf32(x: f32) -> f32;
        fn sinf64(x: f64) -> f64;
        // The cosine of `x` radians
        // Should be compatible with `x.cos()` from the standard library
        fn cosf32(x: f32) -> f32;
        fn cosf64(x: f64) -> f64;
        // Inverse tangent
        // Should be compatible with `y.atan2(x)` from the standard library
        fn atan2f32(y: f32, x: f32) -> f32;
        fn atan2f64(y: f64, x: f64) -> f64;
    }

    impl Float for f32 {
        fn sqrt(self) -> f32 {
            unsafe { sqrtf32(self) }
        }
        fn powf(self, other: f32) -> f32 {
            unsafe { powf32(self, other) }
        }
        fn sin(self) -> f32 {
            unsafe { sinf32(self) }
        }
        fn cos(self) -> f32 {
            unsafe { cosf32(self) }
        }
        fn atan2(self, other: f32) -> f32 {
            unsafe { atan2f32(self, other) }
        }
    }

    impl Float for f64 {
        fn sqrt(self) -> f64 {
            unsafe { sqrtf64(self) }
        }
        fn powf(self, other: f64) -> f64 {
            unsafe { powf64(self, other) }
        }
        fn sin(self) -> f64 {
            unsafe { sinf64(self) }
        }
        fn cos(self) -> f64 {
            unsafe { cosf64(self) }
        }
        fn atan2(self, other: f64) -> f64 {
            unsafe { atan2f64(self, other) }
        }
    }
}
