//! Traits for abstracting over numeric types.
//!
//! These traits describe various numeric properties and operations. They are
//! similar in purpose to the immensely helpful traits in
//! [`num-traits`](https://crates.io/crates/num-traits/), but the structure is
//! different. The philosophy behind this module is to focus on capabilities,
//! rather than categories, and to assume as little as possible. Within reason.
//!
//! Instead of having large traits with a lot of methods and dependencies, each
//! operation (or group of operations), are separated into their own traits.
//! This allows number types to have partial compatibility by only implementing
//! some of the traits, and new methods can be added as new traits without
//! affecting old functionality.

use core::ops::Mul;

#[cfg(all(not(feature = "std"), feature = "libm"))]
mod libm;
#[cfg(feature = "wide")]
mod wide;

/// Methods for the value `1`.
pub trait One {
    /// Create a new `1` value.
    #[must_use]
    fn one() -> Self;
}

/// Method for raising a number by a real number exponent.
///
/// The name "powf" is kept for familiarity, even though the exponent doesn't
/// have to be a floating point number.
pub trait Powf {
    /// Return `self` raised to the power of `exp`.
    #[must_use]
    fn powf(self, exp: Self) -> Self;
}

/// Method for raising a number by a signed integer exponent.
pub trait Powi {
    /// Return `self` raised to the power of `exp`.
    #[must_use]
    fn powi(self, exp: i32) -> Self;
}

/// Method for raising a number by a n unsigned integer exponent.
pub trait Powu {
    /// Return `self` raised to the power of `exp`.
    #[must_use]
    fn powu(self, exp: u32) -> Self;
}

/// Method for calculating `1 / x`.
pub trait Recip {
    /// Return `1 / self`.
    #[must_use]
    fn recip(self) -> Self;
}

macro_rules! impl_uint {
    ($($ty: ident),+) => {
        $(
            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1
                }
            }

            impl Powu for $ty {
                #[inline]
                fn powu(self, exp: u32) -> Self {
                    pow(self, exp)
                }
            }
        )+
    };
}

macro_rules! impl_float {
    ($($ty: ident),+) => {
        $(
            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1.0
                }
            }

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl Powf for $ty {
                #[inline]
                fn powf(self, exp: Self) -> Self {
                    $ty::powf(self, exp)
                }
            }

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl Powi for $ty {
                #[inline]
                fn powi(self, exp: i32) -> Self {
                    $ty::powi(self, exp)
                }
            }

            impl Powu for $ty {
                #[inline]
                fn powu(self, exp: u32) -> Self {
                    pow(self, exp)
                }
            }

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl Recip for $ty {
                #[inline]
                fn recip(self) -> Self {
                    $ty::recip(self)
                }
            }
        )+
    };
}

impl_uint!(u8, u16, u32, u64, u128);
impl_float!(f32, f64);

/// "borrowed" from num_traits
///
/// Raises a value to the power of exp, using exponentiation by squaring.
///
/// Note that `0⁰` (`pow(0, 0)`) returns `1`. Mathematically this is undefined.
//
// # Example
//
// ```rust
// use num_traits::pow;
//
// assert_eq!(pow(2i8, 4), 16);
// assert_eq!(pow(6u8, 3), 216);
// assert_eq!(pow(0u8, 0), 1); // Be aware if this case affects you
// ```
#[inline]
fn pow<T: Clone + One + Mul<T, Output = T>>(mut base: T, mut exp: u32) -> T {
    if exp == 0 {
        return T::one();
    }

    while exp & 1 == 0 {
        base = base.clone() * base;
        exp >>= 1;
    }
    if exp == 1 {
        return base;
    }

    let mut acc = base.clone();
    while exp > 1 {
        exp >>= 1;
        base = base.clone() * base;
        if exp & 1 == 1 {
            acc = acc * base.clone();
        }
    }
    acc
}
