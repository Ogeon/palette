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

use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

#[cfg(all(not(feature = "std"), feature = "libm"))]
mod libm;

/// Numbers that belong to the real number set. It's both a semantic marker and
/// provides a constructor for number constants.
pub trait Real {
    /// Create a number from an `f64` value, mainly for converting constants.
    #[must_use]
    fn from_f64(n: f64) -> Self;
}

/// Methods for the value `0`.
pub trait Zero {
    /// Create a new `0` value.
    #[must_use]
    fn zero() -> Self;
}

/// Methods for the value `1`.
pub trait One {
    /// Create a new `1` value.
    #[must_use]
    fn one() -> Self;
}

/// A helper trait that collects arithmetic traits under one name.
pub trait Arithmetics
where
    Self: Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Neg<Output = Self>
        + Rem<Output = Self>
        + Sized,
    for<'a> Self: Add<&'a Self, Output = Self>
        + Sub<&'a Self, Output = Self>
        + Mul<&'a Self, Output = Self>
        + Div<&'a Self, Output = Self>,
{
}

impl<T> Arithmetics for T
where
    T: Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Neg<Output = Self>
        + Rem<Output = Self>
        + Sized,
    for<'a> Self: Add<&'a Self, Output = Self>
        + Sub<&'a Self, Output = Self>
        + Mul<&'a Self, Output = Self>
        + Div<&'a Self, Output = Self>,
{
}

/// Methods for getting the largest or smallest of two values.
pub trait MinMax: Sized {
    /// Return the smallest of `self` and `other`.
    #[must_use]
    fn min(self, other: Self) -> Self;

    /// Return the largest of `self` and `other`.
    #[must_use]
    fn max(self, other: Self) -> Self;

    /// Return a pair of `self` and `other`, where the smallest is the first
    /// value and the largest is the second.
    #[must_use]
    fn min_max(self, other: Self) -> (Self, Self);
}

/// Trigonometry methods and their inverses.
pub trait Trigonometry: Sized {
    /// Compute the sine of `self` (in radians).
    #[must_use]
    fn sin(self) -> Self;

    /// Compute the cosine of `self` (in radians).
    #[must_use]
    fn cos(self) -> Self;

    /// Simultaneously compute the sine and cosine of `self` (in radians).
    /// Returns `(sin(self), cos(self))`.
    #[must_use]
    fn sin_cos(self) -> (Self, Self);

    /// Compute the tangent of `self` (in radians).
    #[must_use]
    fn tan(self) -> Self;

    /// Compute the arcsine in radians of `self`.
    #[must_use]
    fn asin(self) -> Self;

    /// Compute the arccosine in radians of `self`.
    #[must_use]
    fn acos(self) -> Self;

    /// Compute the arctangent in radians of `self`.
    #[must_use]
    fn atan(self) -> Self;

    /// Compute the arctangent in radians of `self` (y) and `other` (x).
    #[must_use]
    fn atan2(self, other: Self) -> Self;
}

/// Method for getting the absolute value of a number.
pub trait Abs {
    /// Returns the absolute value of `self`.
    #[must_use]
    fn abs(self) -> Self;
}

/// Method for getting the square root of a number.
pub trait Sqrt {
    /// Returns the square root of `self`.
    #[must_use]
    fn sqrt(self) -> Self;
}

/// Method for getting the cube root of a number.
pub trait Cbrt {
    /// Returns the cube root of `self`.
    #[must_use]
    fn cbrt(self) -> Self;
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

/// Methods for calculating `e ^ x`,
pub trait Exp {
    /// Return `e ^ self`.
    #[must_use]
    fn exp(self) -> Self;
}

/// Methods for checking if a number can be used as a divisor.
pub trait IsValidDivisor {
    /// Return `true` if `self` can be used as a divisor in `x / self`.
    ///
    /// This checks that division by `self` will result in a finite and defined
    /// value. Integers check for `self != 0`, while floating point types call
    /// [`is_normal`][std::primitive::f32::is_normal].
    #[must_use]
    fn is_valid_divisor(&self) -> bool;
}

/// Methods for calculating the lengths of a hypotenuse.
pub trait Hypot {
    /// Returns the length of the hypotenuse formed by `self` and `other`, i.e.
    /// `sqrt(self * self + other * other)`.
    #[must_use]
    fn hypot(self, other: Self) -> Self;
}

/// Methods for rounding numbers to integers.
pub trait Round {
    /// Return the nearest integer to `self`. Round half-way cases away from 0.0.
    #[must_use]
    fn round(self) -> Self;

    /// Return the largest integer less than or equal to `self`.
    #[must_use]
    fn floor(self) -> Self;

    /// Return the smallest integer greater than or equal to `self`.
    #[must_use]
    fn ceil(self) -> Self;
}

macro_rules! impl_uint {
    ($($ty: ident),+) => {
        $(
            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0
                }
            }

            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1
                }
            }

            impl MinMax for $ty {
                #[inline]
                fn min(self, other: Self) -> Self {
                    core::cmp::Ord::min(self, other)
                }

                #[inline]
                fn max(self, other: Self) -> Self {
                    core::cmp::Ord::max(self, other)
                }

                #[inline]
                fn min_max(self, other: Self) -> (Self, Self) {
                    if self > other {
                        (other, self)
                    } else {
                        (self, other)
                    }
                }
            }

            impl Powu for $ty {
                #[inline]
                fn powu(self, exp: u32) -> Self {
                    pow(self, exp)
                }
            }

            impl IsValidDivisor for $ty {
                #[inline]
                fn is_valid_divisor(&self) -> bool {
                    *self != 0
                }
            }
        )+
    };
}

macro_rules! impl_float {
    ($($ty: ident),+) => {
        $(
            impl Real for $ty {
                #[inline]
                fn from_f64(n: f64) -> $ty {
                    n as $ty
                }
            }

            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0.0
                }
            }

            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1.0
                }
            }

            impl MinMax for $ty {
                #[inline]
                fn max(self, other: Self) -> Self {
                    $ty::max(self, other)
                }

                #[inline]
                fn min(self, other: Self) -> Self {
                    $ty::min(self, other)
                }

                #[inline]
                fn min_max(self, other: Self) -> (Self, Self) {
                    if self > other {
                        (other, self)
                    } else {
                        (self, other)
                    }
                }
            }

            impl Powu for $ty {
                #[inline]
                fn powu(self, exp: u32) -> Self {
                    pow(self, exp)
                }
            }

            impl IsValidDivisor for $ty {
                #[inline]
                fn is_valid_divisor(&self) -> bool {
                    $ty::is_normal(*self)
                }
            }

            #[cfg(feature = "std")]
            impl Trigonometry for $ty {
                #[inline]
                fn sin(self) -> Self {
                    $ty::sin(self)
                }

                #[inline]
                fn cos(self) -> Self {
                    $ty::cos(self)
                }

                #[inline]
                fn sin_cos(self) -> (Self, Self) {
                    $ty::sin_cos(self)
                }

                #[inline]
                fn tan(self) -> Self {
                    $ty::tan(self)
                }

                #[inline]
                fn asin(self) -> Self {
                    $ty::asin(self)
                }

                #[inline]
                fn acos(self) -> Self {
                    $ty::acos(self)
                }

                #[inline]
                fn atan(self) -> Self {
                    $ty::atan(self)
                }

                #[inline]
                fn atan2(self, other: Self) -> Self {
                    $ty::atan2(self, other)
                }
            }

            #[cfg(feature = "std")]
            impl Abs for $ty {
                #[inline]
                fn abs(self) -> Self {
                    $ty::abs(self)
                }
            }

            #[cfg(feature = "std")]
            impl Sqrt for $ty {
                #[inline]
                fn sqrt(self) -> Self {
                    $ty::sqrt(self)
                }
            }

            #[cfg(feature = "std")]
            impl Cbrt for $ty {
                #[inline]
                fn cbrt(self) -> Self {
                    $ty::cbrt(self)
                }
            }

            #[cfg(feature = "std")]
            impl Powf for $ty {
                #[inline]
                fn powf(self, exp: Self) -> Self {
                    $ty::powf(self, exp)
                }
            }

            #[cfg(feature = "std")]
            impl Powi for $ty {
                #[inline]
                fn powi(self, exp: i32) -> Self {
                    $ty::powi(self, exp)
                }
            }

            #[cfg(feature = "std")]
            impl Recip for $ty {
                #[inline]
                fn recip(self) -> Self {
                    $ty::recip(self)
                }
            }

            #[cfg(feature = "std")]
            impl Exp for $ty {
                #[inline]
                fn exp(self) -> Self {
                    $ty::exp(self)
                }
            }

            #[cfg(feature = "std")]
            impl Hypot for $ty {
                #[inline]
                fn hypot(self, other: Self) -> Self {
                    $ty::hypot(self, other)
                }
            }

            #[cfg(feature = "std")]
            impl Round for $ty {
                #[inline]
                fn round(self) -> Self {
                    $ty::round(self)
                }

                #[inline]
                fn floor(self) -> Self {
                    $ty::floor(self)
                }

                #[inline]
                fn ceil(self) -> Self {
                    $ty::ceil(self)
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
