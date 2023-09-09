use core::ops::Mul;

use crate::{
    angle::RealAngle,
    color_difference::{DeltaE, EuclideanDistance, ImprovedDeltaE},
    convert::FromColorUnclamped,
    num::{MinMax, Powf, Real, Sqrt, Trigonometry, Zero},
};

use super::Cam16UcsJmh;

/// The Cartesian form of CAM16-UCS, or J'a'b'.
#[derive(Clone, Copy, Debug, WithAlpha, ArrayCast, FromColorUnclamped)]
#[palette(
    palette_internal,
    component = "T",
    skip_derives(Cam16UcsJmh, Cam16UcsJab)
)]
#[repr(C)]
pub struct Cam16UcsJab<T> {
    /// The [lightness](https://en.wikipedia.org/wiki/Lightness) (J') of the color.
    pub lightness: T,

    /// The redness/greenness (a') of the color.
    pub a: T,

    /// The yellowness/blueness (b') of the color.
    pub b: T,
}

impl<T> Cam16UcsJab<T> {
    /// Create a CIE L\*a\*b\* color.
    pub const fn new(lightness: T, a: T, b: T) -> Self {
        Self { lightness, a, b }
    }

    /// Convert to a `(L\*, a\*, b\*)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.lightness, self.a, self.b)
    }

    /// Convert from a `(L\*, a\*, b\*)` tuple.
    pub fn from_components((lightness, a, b): (T, T, T)) -> Self {
        Self::new(lightness, a, b)
    }
}

impl<T> FromColorUnclamped<Cam16UcsJab<T>> for Cam16UcsJab<T> {
    fn from_color_unclamped(val: Cam16UcsJab<T>) -> Self {
        val
    }
}

impl<T> FromColorUnclamped<Cam16UcsJmh<T>> for Cam16UcsJab<T>
where
    T: RealAngle + Zero + Mul<Output = T> + Trigonometry + MinMax + Clone,
{
    fn from_color_unclamped(val: Cam16UcsJmh<T>) -> Self {
        let (a, b) = val.hue.into_cartesian();
        let colorfulness = val.colorfulness.max(T::zero());

        Self {
            lightness: val.lightness,
            a: a * colorfulness.clone(),
            b: b * colorfulness,
        }
    }
}

impl_color_add!(Cam16UcsJab, [lightness, a, b]);
impl_color_sub!(Cam16UcsJab, [lightness, a, b]);
impl_color_mul!(Cam16UcsJab, [lightness, a, b]);
impl_color_div!(Cam16UcsJab, [lightness, a, b]);
impl_euclidean_distance!(Cam16UcsJab { lightness, a, b });

impl<T> DeltaE for Cam16UcsJab<T>
where
    Self: EuclideanDistance<Scalar = T>,
    T: Sqrt,
{
    type Scalar = T;

    #[inline]
    fn delta_e(self, other: Self) -> Self::Scalar {
        self.distance(other)
    }
}

impl<T> ImprovedDeltaE for Cam16UcsJab<T>
where
    Self: DeltaE<Scalar = T> + EuclideanDistance<Scalar = T>,
    T: Real + Mul<T, Output = T> + Powf,
{
    #[inline]
    fn improved_delta_e(self, other: Self) -> Self::Scalar {
        // Coefficients from "Power functions improving the performance of
        // color-difference formulas" by Huang et al.
        // https://opg.optica.org/oe/fulltext.cfm?uri=oe-23-1-597&id=307643
        //
        // The multiplication of 0.5 in the exponent makes it square root the
        // squared distance.
        T::from_f64(1.41) * self.distance_squared(other).powf(T::from_f64(0.63 * 0.5))
    }
}
