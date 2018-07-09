//! Floating point traits
//!
//! This module is work-around for the lack of floating point operations under `#![no_std]`. If you
//! haven't disabled the `std` feature, it just re-exports `num_traits::Float`.
//!
//! However, without `std`, it's a custom trait with a subset of the methods from
//!`num_traits::Float`, implemented for `f32` and `f64`.
//!
//! If you are using this on `no_std`, there are two options to supply the necessary floating point
//! operations:
//!
//! * By enabling the `soft_float` feature, these functions are supplied using the `mish` and `m`
//!   crates. This comes with the caveat that as software implementations, they're likely to be
//!   slower and less precise than potential hardware versions.
//! * Without the `soft_float` feature, it is necessary to supply the required operations via
//!   `extern "C"` declarations. This might be beneficial if, for example, you're using a platform
//!   that has hard float operations and you need it to be faster. It's recommended to enable LTO in
//!   this case, to minimize any overhead. These are the functions it expects to link against:
//!   ```rust,ignore
//!   extern "C" {
//!       // Computes the square root of `x`
//!       // Should be compatible with `x.sqrt()` from the standard library
//!       fn sqrtf32(x: f32) -> f32;
//!       fn sqrtf64(x: f64) -> f64;
//!       // Computes `x` to the power of `y`
//!       // Should be compatible with `x.powf(y)` from the standard library
//!       fn powf32(x: f32, y: f32) -> f32;
//!       fn powf64(x: f64, y: f64) -> f64;
//!       // Computes the sine of `x` radians
//!       // Should be compatible with `x.sin()` from the standard library
//!       fn sinf32(x: f32) -> f32;
//!       fn sinf64(x: f64) -> f64;
//!       // Computes the cosine of `x` radians
//!       // Should be compatible with `x.cos()` from the standard library
//!       fn cosf32(x: f32) -> f32;
//!       fn cosf64(x: f64) -> f64;
//!       // Computes the inverse tangent of `y / x`, in the corresponding quadrant
//!       // Should be compatible with `y.atan2(x)` from the standard library
//!       fn atan2f32(y: f32, x: f32) -> f32;
//!       fn atan2f64(y: f64, x: f64) -> f64;
//!   }
//!   ```
//!
//! Because new floating point functions may be needed in patch releases, the specifics of which
//! operations are included in the trait (and likewise in the list of required `extern "C"` functions)
//! are semver-exempt on `no_std`.

#[cfg(feature = "std")]
pub use num_traits::Float;

#[cfg(not(feature = "std"))]
pub use self::no_std_float_trait::Float;

#[cfg(not(feature = "std"))]
mod no_std_float_trait {
    #[cfg(feature = "soft_float")]
    extern crate m;
    #[cfg(feature = "soft_float")]
    extern crate mish;

    use num_traits::float::FloatCore;

    /// This is the trait that represents a floating-point number under `no_std`. It has a subset
    /// of the operations that are in `num_traits::Float`, including all of the 
    /// `num_traits::float::FloatCore` opterations. For documentation of specific functions in this
    /// trait, see the [`num_traits::Float` docs][num_traits].
    ///
    /// It's implemented for `f32` and `f64`. See the [module docs][module] for details.
    ///
    /// # Compatibility between versions
    ///
    /// Because of the possibility of needing more floating point operations in point releases, this
    /// trait is semver-exempt with respect to adding new functions. (If you really need to
    /// implement it for your own `MyFloat` type, pin a specific version in your `Cargo.toml`.)
    /// However, removing methods from this trait is still considered a breaking change.
    ///
    /// [num_traits]: https://docs.rs/num-traits/0.2.5/num_traits/float/trait.Float.html
    /// [module]: index.html
    pub trait Float: FloatCore {
        fn sqrt(self) -> Self;
        fn powf(self, other: Self) -> Self;
        fn sin(self) -> Self;
        fn cos(self) -> Self;
        fn atan2(self, other: Self) -> Self;
    }

    #[cfg(feature = "soft_float")]
    impl Float for f32 {
        fn sqrt(self) -> f32 {
            mish::sqrt(self)
        }
        fn powf(self, other: f32) -> f32 {
            mish::powf(self, other)
        }
        fn sin(self) -> f32 {
            mish::sin(self)
        }
        fn cos(self) -> f32 {
            mish::cos(self)
        }
        fn atan2(self, other: f32) -> f32 {
            m::Float::atan2(self, other)
        }
    }

    #[cfg(feature = "soft_float")]
    impl Float for f64 {
        fn sqrt(self) -> f64 {
            mish::sqrt(self)
        }
        fn powf(self, other: f64) -> f64 {
            mish::powf(self, other)
        }
        fn sin(self) -> f64 {
            mish::sin(self)
        }
        fn cos(self) -> f64 {
            mish::cos(self)
        }
        fn atan2(self, other: f64) -> f64 {
            // f64 atan2 isn't implemented in `m` yet
            m::Float::atan2(self as f32, other as f32).into()
        }
    }

    #[cfg(not(feature = "soft_float"))]
    extern "C" {
        // Computes the square root of `x`
        // Should be compatible with `x.sqrt()` from the standard library
        fn sqrtf32(x: f32) -> f32;
        fn sqrtf64(x: f64) -> f64;
        // Computes `x` to the power of `y`
        // Should be compatible with `x.powf(y)` from the standard library
        fn powf32(x: f32, y: f32) -> f32;
        fn powf64(x: f64, y: f64) -> f64;
        // Computes the sine of `x` radians
        // Should be compatible with `x.sin()` from the standard library
        fn sinf32(x: f32) -> f32;
        fn sinf64(x: f64) -> f64;
        // Computes the cosine of `x` radians
        // Should be compatible with `x.cos()` from the standard library
        fn cosf32(x: f32) -> f32;
        fn cosf64(x: f64) -> f64;
        // Computes the inverse tangent of `y / x`, in the corresponding quadrant
        // Should be compatible with `y.atan2(x)` from the standard library
        fn atan2f32(y: f32, x: f32) -> f32;
        fn atan2f64(y: f64, x: f64) -> f64;
    }

    #[cfg(not(feature = "soft_float"))]
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

    #[cfg(not(feature = "soft_float"))]
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
