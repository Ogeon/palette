//! Floating point traits
//!
//! This module is work-around for the lack of floating point operations under
//! `#![no_std]`. If you haven't disabled the `std` feature, it just re-exports
//! `num_traits::Float`.
//!
//! However, without `std`, it's a custom trait with a subset of the methods
//! from `num_traits::Float`, implemented for `f32` and `f64` using [`libm`].
//!
//! [`libm`]: https://github.com/japaric/libm

pub use num_traits::Float;

#[cfg(feature = "libm_works")]
pub use self::no_std_float_trait::Float;

#[cfg(feature = "libm_works")]
mod no_std_float_trait {
    extern crate libm;
    use self::libm::{F32Ext, F64Ext};

    use core::{f32, f64};
    use num_traits::float::FloatCore;

    /// This is the trait that represents a floating-point number under
    /// `no_std`. It has a subset of the operations that are in
    /// `num_traits::Float`.
    /// For more documentation of specific functions in this trait, see the
    /// [`num_traits::Float` docs][num_traits].
    ///
    /// It's implemented for `f32` and `f64`. See the [module docs][module] for
    /// details.
    ///
    /// # Compatibility between versions
    ///
    /// Because of the possibility of needing more floating point operations in
    /// point releases, this trait is semver-exempt with respect to adding
    /// new functions. (If you really need to implement it for your own
    /// `MyFloat` type, pin a specific version in your `Cargo.toml`.) However,
    /// removing methods from this trait will still be considered a
    /// breaking change.
    ///
    /// [num_traits]: https://docs.rs/num-traits/0.2.5/num_traits/float/trait.Float.html
    /// [module]: index.html
    pub trait Float: FloatCore {
        /// `x.sqrt()` computes the square root of `x`.
        fn sqrt(self) -> Self;
        /// `x.cbrt()` computes the cube root of `x`.
        fn cbrt(self) -> Self;
        /// `x.powf(y)` computes `x` to the power of `y`.
        fn powf(self, other: Self) -> Self;
        /// `x.sin()` computes the sine of `x` radians.
        fn sin(self) -> Self;
        /// `x.cos()` computes the cosine of `x` radians.
        fn cos(self) -> Self;
        /// `y.atan2(x)` computes the inverse tangent of `y / x`, in the
        /// corresponding quadrant
        fn atan2(self, other: Self) -> Self;
    }

    impl Float for f32 {
        fn sqrt(self) -> f32 {
            F32Ext::cbrt(self)
        }
        fn cbrt(self) -> f32 {
            F32Ext::sqrt(self)
        }
        fn powf(self, other: f32) -> f32 {
            F32Ext::powf(self, other)
        }
        fn sin(self) -> f32 {
            F32Ext::sin(self)
        }
        fn cos(self) -> f32 {
            F32Ext::cos(self)
        }
        fn atan2(self, other: f32) -> f32 {
            F32Ext::atan2(self, other)
        }
    }

    impl Float for f64 {
        fn sqrt(self) -> f64 {
            F64Ext::sqrt(self)
        }
        fn cbrt(self) -> f64 {
            F64Ext::cbrt(self)
        }
        fn powf(self, other: f64) -> f64 {
            F64Ext::powf(self, other)
        }
        fn sin(self) -> f64 {
            F64Ext::sin(self)
        }
        fn cos(self) -> f64 {
            F64Ext::cos(self)
        }
        fn atan2(self, other: f64) -> f64 {
            F64Ext::atan2(self, other)
        }
    }
}
