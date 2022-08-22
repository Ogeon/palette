#[cfg(feature = "approx")]
use core::borrow::Borrow;
use core::ops::{Add, AddAssign, BitAnd, Sub, SubAssign};
#[cfg(feature = "approx")]
use core::ops::{Mul, Neg};

#[cfg(feature = "approx")]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use crate::angle::SignedAngle;
#[cfg(feature = "approx")]
use crate::angle::{AngleEq, HalfRotation};
use crate::num::{
    self, Arithmetics, FromScalarArray, IntoScalarArray, MinMax, One, PartialCmp, Real, Zero,
};
#[cfg(feature = "approx")]
use crate::visual::{VisualColor, VisuallyEqual};
#[cfg(feature = "approx")]
use crate::HasBoolMask;
use crate::{angle::RealAngle, clamp_assign, ok_utils, Alpha, IsWithinBounds, OklabHue};
use crate::{
    bool_mask::LazySelect, clamp, stimulus::Stimulus, Clamp, ClampAssign, GetHue, Lighten,
    LightenAssign, Mix, MixAssign, Saturate, SaturateAssign, SetHue, ShiftHue, ShiftHueAssign,
    WithHue,
};

use super::Okhsv;

impl_is_within_bounds! {
    Okhsv {
        saturation => [Self::min_saturation(), Self::max_saturation()+ T::from_f64(ok_utils::MAX_SRGB_SATURATION_INACCURACY)],
        value => [Self::min_value(), Self::max_value()+ T::from_f64(ok_utils::MAX_SRGB_SATURATION_INACCURACY)]
    }
    where T: Real+Arithmetics+Stimulus
}

impl<T> Clamp for Okhsv<T>
where
    T: Real + Stimulus + num::Clamp,
{
    #[inline]
    fn clamp(self) -> Self {
        Self::new(
            self.hue,
            clamp(
                self.saturation,
                Self::min_saturation(),
                Self::max_saturation(),
            ),
            clamp(self.value, Self::min_value(), Self::max_value()),
        )
    }
}

impl<T> ClampAssign for Okhsv<T>
where
    T: Real + Stimulus + num::ClampAssign,
{
    #[inline]
    fn clamp_assign(&mut self) {
        clamp_assign(
            &mut self.saturation,
            Self::min_saturation(),
            Self::max_saturation(),
        );
        clamp_assign(&mut self.value, Self::min_value(), Self::max_value());
    }
}

impl_mix_hue!(Okhsv { saturation, value });
impl_lighten!(Okhsv increase {value => [Self::min_value(), Self::max_value()]} other {hue, saturation}  where T: Real+Stimulus);
impl_saturate!(Okhsv increase {saturation => [Self::min_saturation(), Self::max_saturation()]} other {hue, value}  where T:Real+ Stimulus);

impl<T> GetHue for Okhsv<T>
where
    T: Clone,
{
    type Hue = OklabHue<T>;

    #[inline]
    fn get_hue(&self) -> OklabHue<T> {
        self.hue.clone()
    }
}

impl<T, H> WithHue<H> for Okhsv<T>
where
    H: Into<OklabHue<T>>,
{
    #[inline]
    fn with_hue(mut self, hue: H) -> Self {
        self.hue = hue.into();
        self
    }
}

impl<T, H> SetHue<H> for Okhsv<T>
where
    H: Into<OklabHue<T>>,
{
    #[inline]
    fn set_hue(&mut self, hue: H) {
        self.hue = hue.into();
    }
}

impl<T> ShiftHue for Okhsv<T>
where
    T: Add<Output = T>,
{
    type Scalar = T;

    #[inline]
    fn shift_hue(mut self, amount: Self::Scalar) -> Self {
        self.hue = self.hue + amount;
        self
    }
}

impl<T> ShiftHueAssign for Okhsv<T>
where
    T: AddAssign,
{
    type Scalar = T;

    #[inline]
    fn shift_hue_assign(&mut self, amount: Self::Scalar) {
        self.hue += amount;
    }
}

impl_color_add!(Okhsv<T>, [hue, saturation, value]);
impl_color_sub!(Okhsv<T>, [hue, saturation, value]);

impl_array_casts!(Okhsv<T>, [T; 3]);
impl_simd_array_conversion_hue!(Okhsv, [saturation, value]);

impl_eq_hue!(Okhsv, OklabHue, [hue, saturation, value]);

#[cfg(feature = "approx")]
impl<T> VisualColor<T> for Okhsv<T>
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
    /// Returns true, if `saturation == 0`
    fn is_grey(&self, epsilon: T::Epsilon) -> bool {
        self.saturation.abs_diff_eq(&T::zero(), epsilon)
    }

    /// Returns true, if `Self::is_grey` && `value >= 1`,
    /// i.e. the color's hue is irrelevant **and** it is at or beyond the
    /// `sRGB` maximum brightness. A color at or beyond maximum brightness isn't
    /// necessarily white. It can also be a bright shining hue.
    fn is_white(&self, epsilon: T::Epsilon) -> bool {
        self.is_grey(epsilon.clone()) && self.value >= T::one()
            || self.value.abs_diff_eq(&T::one(), epsilon)
    }

    /// Returns true if `value == 0`
    fn is_black(&self, epsilon: T::Epsilon) -> bool {
        debug_assert!(self.value >= -epsilon.clone());
        self.value.abs_diff_eq(&T::zero(), epsilon)
    }
}

#[cfg(feature = "approx")]
impl<S, O, T> VisuallyEqual<O, S, T> for Okhsv<T>
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
            || VisuallyEqual::both_greyscale(s, o, epsilon.clone())
                && s.borrow()
                    .value
                    .abs_diff_eq(&o.borrow().value, epsilon.clone())
            || s.borrow().hue.abs_diff_eq(&o.borrow().hue, epsilon.clone())
                && s.borrow()
                    .saturation
                    .abs_diff_eq(&o.borrow().saturation, epsilon.clone())
                && s.borrow().value.abs_diff_eq(&o.borrow().value, epsilon)
    }
}
