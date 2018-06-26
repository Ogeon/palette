//!Floating point traits
//! 
//!This module is work-around for the lack of floating point operations under `#![no_std]`. If you
//!haven't disabled the `std` feature, it just re-exports `num_traits::Float`.
//!
//!However, without `std`, it's a custom trait that's implemented for `f32` and `f64` using the
//!`mish` and `m` crates.
//!
//!Because new floating point functions may be needed in patch releases, the specifics of which
//!operations are included in the trait are semver-exempt on `no_std`.

#[cfg(feature = "std")]
pub use num_traits::Float;

#[cfg(not(feature = "std"))]
pub use self::no_std_float_hack::Float;

#[cfg(not(feature = "std"))]
mod no_std_float_hack {
    use m;
    use mish;

    pub trait Float: ::num_traits::float::FloatCore {
        fn sqrt(self) -> Self;
        fn powf(self, other: Self) -> Self;
        fn sin(self) -> Self;
        fn cos(self) -> Self;
        fn atan2(self, other: Self) -> Self;
    }

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
            <f32 as m::Float>::atan2(self, other)
        }
    }

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
            <f64 as m::Float>::atan2(self, other)
        }
    }
}
