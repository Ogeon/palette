#[cfg(feature = "approx")]
use core::borrow::Borrow;
use core::ops::{Add, AddAssign, BitAnd, Sub, SubAssign};
#[cfg(feature = "approx")]
use core::ops::{Mul, Neg};

#[cfg(feature = "approx")]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

#[cfg(feature = "approx")]
use crate::angle::{AngleEq, HalfRotation};
#[cfg(feature = "approx")]
use crate::visual::{VisualColor, VisuallyEqual};
use crate::white_point::D65;
#[cfg(feature = "approx")]
use crate::HasBoolMask;
use crate::{
    angle::{RealAngle, SignedAngle},
    bool_mask::LazySelect,
    clamp, clamp_assign, contrast_ratio,
    num::{
        self, Arithmetics, FromScalarArray, IntoScalarArray, MinMax, One, PartialCmp, Real, Zero,
    },
    stimulus::Stimulus,
    Alpha, Clamp, ClampAssign, FromColor, GetHue, IsWithinBounds, Lighten, LightenAssign, Mix,
    MixAssign, OklabHue, RelativeContrast, Saturate, SaturateAssign, SetHue, ShiftHue,
    ShiftHueAssign, WithHue, Xyz,
};

use super::Okhsl;

impl_is_within_bounds! {
    Okhsl {
        saturation => [Self::min_saturation(), Self::max_saturation()],
        lightness => [Self::min_lightness(), Self::max_lightness()]
    }
    where T: Stimulus
}

impl<T> Clamp for Okhsl<T>
where
    T: Stimulus + num::Clamp,
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
            clamp(self.lightness, Self::min_lightness(), Self::max_lightness()),
        )
    }
}

impl<T> ClampAssign for Okhsl<T>
where
    T: Stimulus + num::ClampAssign,
{
    #[inline]
    fn clamp_assign(&mut self) {
        clamp_assign(
            &mut self.saturation,
            Self::min_saturation(),
            Self::max_saturation(),
        );
        clamp_assign(
            &mut self.lightness,
            Self::min_lightness(),
            Self::max_lightness(),
        );
    }
}

impl_mix_hue!(Okhsl {
    saturation,
    lightness
});
impl_lighten!(Okhsl increase {lightness => [Self::min_lightness(), Self::max_lightness()]} other {hue, saturation}  where T: Real+Stimulus);
impl_saturate!(Okhsl increase {saturation => [Self::min_saturation(), Self::max_saturation()]} other {hue, lightness}  where T: Real+Stimulus);

impl<T> GetHue for Okhsl<T>
where
    T: Clone,
{
    type Hue = OklabHue<T>;

    #[inline]
    fn get_hue(&self) -> OklabHue<T> {
        self.hue.clone()
    }
}

impl<T, H> WithHue<H> for Okhsl<T>
where
    H: Into<OklabHue<T>>,
{
    #[inline]
    fn with_hue(mut self, hue: H) -> Self {
        self.hue = hue.into();
        self
    }
}

impl<T, H> SetHue<H> for Okhsl<T>
where
    H: Into<OklabHue<T>>,
{
    #[inline]
    fn set_hue(&mut self, hue: H) {
        self.hue = hue.into();
    }
}

impl<T> ShiftHue for Okhsl<T>
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

impl<T> ShiftHueAssign for Okhsl<T>
where
    T: AddAssign,
{
    type Scalar = T;

    #[inline]
    fn shift_hue_assign(&mut self, amount: Self::Scalar) {
        self.hue += amount;
    }
}

impl_color_add!(Okhsl<T>, [hue, saturation, lightness]);
impl_color_sub!(Okhsl<T>, [hue, saturation, lightness]);

impl_array_casts!(Okhsl<T>, [T; 3]);
impl_simd_array_conversion_hue!(Okhsl, [saturation, lightness]);

impl_eq_hue!(Okhsl, OklabHue, [hue, saturation, lightness]);

impl<T> RelativeContrast for Okhsl<T>
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

#[cfg(feature = "approx")]
impl<T> VisualColor<T> for Okhsl<T>
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
        debug_assert!(self.saturation >= -epsilon.clone());
        self.saturation.abs_diff_eq(&T::zero(), epsilon)
    }

    /// Returns true, if `Self::is_grey` && `lightness >= 1`,
    /// i.e. the color's hue is irrelevant **and** it is at or beyond the
    /// `sRGB` maximum luminance. A color at or beyond maximum brightness isn't
    /// necessarily white. It may also be a bright shining hue.
    fn is_white(&self, epsilon: T::Epsilon) -> bool {
        self.is_grey(epsilon.clone()) && self.lightness > T::one()
            || self.lightness.abs_diff_eq(&T::one(), epsilon)
    }

    /// Returns true if `lightness == 0`
    fn is_black(&self, epsilon: T::Epsilon) -> bool {
        debug_assert!(self.lightness >= -epsilon.clone());
        self.lightness <= epsilon
    }
}

#[cfg(feature = "approx")]
impl<S, O, T> VisuallyEqual<O, S, T> for Okhsl<T>
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
                    .lightness
                    .abs_diff_eq(&o.borrow().lightness, epsilon.clone())
            || s.borrow().hue.abs_diff_eq(&o.borrow().hue, epsilon.clone())
                && s.borrow()
                    .saturation
                    .abs_diff_eq(&o.borrow().saturation, epsilon.clone())
                && s.borrow()
                    .lightness
                    .abs_diff_eq(&o.borrow().lightness, epsilon)
    }
}
