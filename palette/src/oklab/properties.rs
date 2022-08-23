#[cfg(feature = "approx")]
use core::borrow::Borrow;
use core::ops::BitOr;
#[cfg(feature = "approx")]
use core::ops::Neg;
use core::ops::{Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(feature = "approx")]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

#[cfg(feature = "approx")]
use crate::angle::{AngleEq, HalfRotation, SignedAngle};
use crate::color_difference::{get_ciede_difference, LabColorDiff};
use crate::num::{Abs, Exp, Hypot, Powi, Sqrt};
#[cfg(feature = "approx")]
use crate::visual::{VisualColor, VisuallyEqual};
#[cfg(feature = "approx")]
use crate::HasBoolMask;
use crate::{
    angle::RealAngle,
    blend::{PreAlpha, Premultiply},
    bool_mask::LazySelect,
    clamp, clamp_assign, contrast_ratio,
    num::{
        self, Arithmetics, FromScalarArray, IntoScalarArray, IsValidDivisor, MinMax, One,
        PartialCmp, Real, Trigonometry, Zero,
    },
    stimulus::Stimulus,
    white_point::D65,
    Alpha, Clamp, ClampAssign, ColorDifference, FromColor, GetHue, IsWithinBounds, Lighten,
    LightenAssign, Mix, MixAssign, OklabHue, RelativeContrast, Xyz,
};

use super::Oklab;

impl_is_within_bounds! {
    Oklab {
        l => [Self::min_l(), Self::max_l()],
        a => [Self::min_a(), Self::max_a()],
        b => [Self::min_b(), Self::max_b()]
    }
    where T: Real
}

impl<T> Clamp for Oklab<T>
where
    T: Real + Arithmetics + Hypot + Clone + num::Clamp,
{
    #[inline]
    fn clamp(self) -> Self {
        // lightness is limited and thus can be clamped.
        let l = clamp(self.l, Self::min_l(), Self::max_l());
        // a and b are unlimited
        Self::new(l, self.a, self.b)
    }
}

impl<T> ClampAssign for Oklab<T>
where
    T: Real + num::ClampAssign,
{
    #[inline]
    fn clamp_assign(&mut self) {
        clamp_assign(&mut self.l, Self::min_l(), Self::max_l());
    }
}

impl_mix!(Oklab);
impl_lighten!(Oklab increase {l => [Self::min_l(), Self::max_l()]} other {a, b});
impl_premultiply!(Oklab { l, a, b });

impl<T> GetHue for Oklab<T>
where
    T: RealAngle + Zero + Arithmetics + Trigonometry + Clone + Default + PartialEq,
{
    type Hue = OklabHue<T>;

    fn get_hue(&self) -> OklabHue<T> {
        self.try_hue().unwrap_or_default()
    }
}

impl<T> ColorDifference for Oklab<T>
where
    T: Real
        + RealAngle
        + One
        + Zero
        + Powi
        + Exp
        + Trigonometry
        + Abs
        + Sqrt
        + Arithmetics
        + PartialCmp
        + Clone,
    T::Mask: LazySelect<T> + BitAnd<Output = T::Mask> + BitOr<Output = T::Mask>,
    Self: Into<LabColorDiff<T>>,
{
    type Scalar = T;

    #[inline]
    fn get_color_difference(self, other: Oklab<T>) -> Self::Scalar {
        get_ciede_difference(self.into(), other.into())
    }
}

impl_color_add!(Oklab<T>, [l, a, b]);
impl_color_sub!(Oklab<T>, [l, a, b]);
impl_color_mul!(Oklab<T>, [l, a, b]);
impl_color_div!(Oklab<T>, [l, a, b]);

impl_array_casts!(Oklab<T>, [T; 3]);
impl_simd_array_conversion!(Oklab, [l, a, b]);

impl_eq!(Oklab, [l, a, b]);

#[cfg(feature = "approx")]
impl<T> VisualColor<T> for Oklab<T>
where
    T: PartialOrd
        + HasBoolMask<Mask = bool>
        + AbsDiffEq<Epsilon = T>
        + One
        + Zero
        + Neg<Output = T>,
    T::Epsilon: Clone,
    OklabHue<T>: AbsDiffEq<Epsilon = T::Epsilon>,
{
    /// Returns true, if `chroma == 0`
    #[allow(dead_code)]
    fn is_grey(&self, epsilon: T::Epsilon) -> bool {
        self.a.abs_diff_eq(&T::zero(), epsilon.clone()) && self.b.abs_diff_eq(&T::zero(), epsilon)
    }

    /// Returns true, if `lightness >= 1`
    ///
    /// **Note:** `sRGB` to `Oklab` conversion uses `f32` constants.
    /// A tolerance `epsilon >= 1e-8` is required to reliably detect white.
    /// Conversion of `sRGB` via XYZ requires `epsilon >= 1e-5`
    fn is_white(&self, epsilon: T::Epsilon) -> bool {
        self.l >= T::one() || self.l.abs_diff_eq(&T::one(), epsilon)
    }

    /// Returns true, if `lightness == 0`
    fn is_black(&self, epsilon: T::Epsilon) -> bool {
        self.l.abs_diff_eq(&T::zero(), epsilon)
    }
}
#[cfg(feature = "approx")]
impl<S, O, T> VisuallyEqual<O, S, T> for Oklab<T>
where
    T: PartialOrd
        + HasBoolMask<Mask = bool>
        + RealAngle
        + SignedAngle
        + Zero
        + One
        + AngleEq<Mask = bool>
        + Sub<Output = T>
        + AbsDiffEq<Epsilon = T>
        + Neg<Output = T>
        + Clone,
    T::Epsilon: Clone + HalfRotation + Mul<Output = T::Epsilon>,
    S: Borrow<Self> + Copy,
    O: Borrow<Self> + Copy,
{
    fn visually_eq(s: S, o: O, epsilon: T::Epsilon) -> bool {
        VisuallyEqual::both_black_or_both_white(s, o, epsilon.clone())
            || s.borrow().l.abs_diff_eq(&o.borrow().l, epsilon.clone())
                && s.borrow().a.abs_diff_eq(&o.borrow().a, epsilon.clone())
                && s.borrow().b.abs_diff_eq(&o.borrow().b, epsilon)
    }
}

impl<T> RelativeContrast for Oklab<T>
where
    T: Real + Arithmetics + PartialCmp,
    T::Mask: LazySelect<T>,
    Xyz<D65, T>: FromColor<Self>,
{
    type Scalar = T;

    #[inline]
    fn get_contrast_ratio(self, other: Self) -> T {
        let xyz1 = Xyz::from_color(self);
        let xyz2 = Xyz::from_color(other);

        contrast_ratio(xyz1.y, xyz2.y)
    }
}
