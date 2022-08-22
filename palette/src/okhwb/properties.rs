use core::ops::{Add, AddAssign, BitAnd, DivAssign, Sub, SubAssign};

use crate::angle::{RealAngle, SignedAngle};
use crate::stimulus::Stimulus;
use crate::white_point::D65;
use crate::HasBoolMask;
use crate::{
    bool_mask::{LazySelect, Select},
    clamp, clamp_min, clamp_min_assign, contrast_ratio, ClampAssign, FromColor, GetHue,
    IsWithinBounds, Lighten, LightenAssign, Mix, MixAssign, OklabHue, RelativeContrast, SetHue,
    ShiftHue, ShiftHueAssign, WithHue, Xyz,
};
use crate::{
    num::{
        self, Arithmetics, FromScalarArray, IntoScalarArray, MinMax, One, PartialCmp, Real, Zero,
    },
    Alpha, Clamp,
};

use super::Okhwb;
#[cfg(feature = "approx")]
use crate::angle::{AngleEq, HalfRotation};
#[cfg(feature = "approx")]
use crate::visual::{VisualColor, VisuallyEqual};
#[cfg(feature = "approx")]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
#[cfg(feature = "approx")]
use core::borrow::Borrow;
impl<T> IsWithinBounds for Okhwb<T>
where
    T: Real + Stimulus + PartialCmp + Add<Output = T> + HasBoolMask + Clone,
    T::Mask: BitAnd<Output = T::Mask>,
{
    #[rustfmt::skip]
    #[inline]
    fn is_within_bounds(&self) -> T::Mask {
        self.blackness.gt_eq(&Self::min_blackness()) & self.blackness.lt_eq(&Self::max_blackness()) &
            self.whiteness.gt_eq(&Self::min_whiteness()) & self.whiteness.lt_eq(&Self::max_blackness()) &
            (self.whiteness.clone() + self.blackness.clone()).lt_eq(&T::max_intensity())
    }
}

impl<T> Clamp for Okhwb<T>
where
    T: Real + Stimulus + One + num::Clamp + PartialCmp + Add<Output = T> + DivAssign + Clone,
    T::Mask: Select<T>,
{
    #[inline]
    fn clamp(self) -> Self {
        let mut whiteness = clamp_min(self.whiteness.clone(), Self::min_whiteness());
        let mut blackness = clamp_min(self.blackness.clone(), Self::min_blackness());

        let sum = self.blackness + self.whiteness;
        let divisor = sum.gt(&T::max_intensity()).select(sum, T::one());
        whiteness /= divisor.clone();
        blackness /= divisor;

        Self::new(self.hue, whiteness, blackness)
    }
}

impl<T> ClampAssign for Okhwb<T>
where
    T: Real + Stimulus + One + num::ClampAssign + PartialCmp + Add<Output = T> + DivAssign + Clone,
    T::Mask: Select<T>,
{
    #[inline]
    fn clamp_assign(&mut self) {
        clamp_min_assign(&mut self.whiteness, Self::min_whiteness());
        clamp_min_assign(&mut self.blackness, Self::min_blackness());

        let sum = self.blackness.clone() + self.whiteness.clone();
        let divisor = sum.gt(&T::max_intensity()).select(sum, T::one());
        self.whiteness /= divisor.clone();
        self.blackness /= divisor;
    }
}

impl_mix_hue!(Okhwb {
    whiteness,
    blackness
});

impl<T> Lighten for Okhwb<T>
where
    T: Stimulus + Real + Zero + MinMax + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    type Scalar = T;

    #[inline]
    fn lighten(self, factor: T) -> Self {
        let difference_whiteness = lazy_select! {
            if factor.gt_eq(&T::zero()) => Self::max_whiteness() - &self.whiteness,
            else => self.whiteness.clone(),
        };
        let delta_whiteness = difference_whiteness.max(T::zero()) * &factor;

        let difference_blackness = lazy_select! {
            if factor.gt_eq(&T::zero()) => self.blackness.clone(),
            else => Self::max_blackness() - &self.blackness,
        };
        let delta_blackness = difference_blackness.max(T::zero()) * factor;

        Okhwb {
            hue: self.hue,
            whiteness: (self.whiteness + delta_whiteness).max(Self::min_whiteness()),
            blackness: (self.blackness - delta_blackness).max(Self::min_blackness()),
        }
    }

    #[inline]
    fn lighten_fixed(self, amount: T) -> Self {
        Okhwb {
            hue: self.hue,
            whiteness: (self.whiteness + Self::max_whiteness() * &amount)
                .max(Self::min_whiteness()),
            blackness: (self.blackness - Self::max_blackness() * amount).max(Self::min_blackness()),
        }
    }
}

impl<T> LightenAssign for Okhwb<T>
where
    T: Stimulus
        + Real
        + Zero
        + MinMax
        + num::ClampAssign
        + AddAssign
        + SubAssign
        + Arithmetics
        + PartialCmp
        + Clone,
    T::Mask: LazySelect<T>,
{
    type Scalar = T;

    #[inline]
    fn lighten_assign(&mut self, factor: T) {
        let difference_whiteness = lazy_select! {
            if factor.gt_eq(&T::zero()) => Self::max_whiteness() - &self.whiteness,
            else => self.whiteness.clone(),
        };
        self.whiteness += difference_whiteness.max(T::zero()) * &factor;
        clamp_min_assign(&mut self.whiteness, Self::min_whiteness());

        let difference_blackness = lazy_select! {
            if factor.gt_eq(&T::zero()) => self.blackness.clone(),
            else => Self::max_blackness() - &self.blackness,
        };
        self.blackness -= difference_blackness.max(T::zero()) * factor;
        clamp_min_assign(&mut self.blackness, Self::min_blackness());
    }

    #[inline]
    fn lighten_fixed_assign(&mut self, amount: T) {
        self.whiteness += Self::max_whiteness() * &amount;
        clamp_min_assign(&mut self.whiteness, Self::min_whiteness());

        self.blackness -= Self::max_blackness() * amount;
        clamp_min_assign(&mut self.blackness, Self::min_blackness());
    }
}

impl<T> GetHue for Okhwb<T>
where
    T: Clone,
{
    type Hue = OklabHue<T>;

    #[inline]
    fn get_hue(&self) -> OklabHue<T> {
        self.hue.clone()
    }
}

impl<T, H> WithHue<H> for Okhwb<T>
where
    H: Into<OklabHue<T>>,
{
    #[inline]
    fn with_hue(mut self, hue: H) -> Self {
        self.hue = hue.into();
        self
    }
}

impl<T, H> SetHue<H> for Okhwb<T>
where
    H: Into<OklabHue<T>>,
{
    #[inline]
    fn set_hue(&mut self, hue: H) {
        self.hue = hue.into();
    }
}

impl<T> ShiftHue for Okhwb<T>
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

impl<T> ShiftHueAssign for Okhwb<T>
where
    T: AddAssign,
{
    type Scalar = T;

    #[inline]
    fn shift_hue_assign(&mut self, amount: Self::Scalar) {
        self.hue += amount;
    }
}

impl_color_add!(Okhwb<T>, [hue, whiteness, blackness]);
impl_color_sub!(Okhwb<T>, [hue, whiteness, blackness]);

impl_array_casts!(Okhwb<T>, [T; 3]);
impl_simd_array_conversion_hue!(Okhwb, [whiteness, blackness]);

impl<T> RelativeContrast for Okhwb<T>
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

impl_eq_hue!(Okhwb, OklabHue, [hue, whiteness, blackness]);

#[cfg(feature = "approx")]
impl<T> VisualColor<T> for Okhwb<T>
where
    T: PartialOrd + HasBoolMask<Mask = bool> + AbsDiffEq<Epsilon = T> + One + Zero + Arithmetics,
    T::Epsilon: Clone,
    OklabHue<T>: AbsDiffEq<Epsilon = T::Epsilon>,
{
    /// Returns `true`, if `self.blackness + self.whiteness >= 1`,
    /// assuming (but not asserting) that neither
    /// `blackness` nor `whiteness` can be negative.
    fn is_grey(&self, epsilon: T::Epsilon) -> bool {
        let wb_sum = self.blackness.clone() + self.whiteness.clone();
        wb_sum > T::one() || wb_sum.abs_diff_eq(&T::one(), epsilon)
    }

    /// Returns `true`, if `Self::is_grey && blackness == 0`,
    /// i.e. the color's hue is irrelevant **and** the color contains
    /// no black component it must be white.
    fn is_white(&self, epsilon: T::Epsilon) -> bool {
        self.is_grey(epsilon.clone()) && self.blackness < epsilon
    }

    /// Returns `true` if `Self::is_grey && whiteness == 0`
    fn is_black(&self, epsilon: T::Epsilon) -> bool {
        self.is_grey(epsilon.clone()) && self.whiteness < epsilon
    }
}

#[cfg(feature = "approx")]
impl<S, O, T> VisuallyEqual<O, S, T> for Okhwb<T>
where
    T: PartialOrd
        + HasBoolMask<Mask = bool>
        + RealAngle
        + SignedAngle
        + Zero
        + One
        + AngleEq<Mask = bool>
        + Arithmetics
        + AbsDiffEq<Epsilon = T>
        + Clone,
    T::Epsilon: Clone + HalfRotation,
    S: Borrow<Self> + Copy,
    O: Borrow<Self> + Copy,
{
    fn visually_eq(s: S, o: O, epsilon: T::Epsilon) -> bool {
        VisuallyEqual::both_black_or_both_white(s, o, epsilon.clone())
            || VisuallyEqual::both_greyscale(s, o, epsilon.clone())
                && s.borrow()
                    .whiteness
                    .abs_diff_eq(&o.borrow().whiteness, epsilon.clone())
                && s.borrow()
                    .blackness
                    .abs_diff_eq(&o.borrow().blackness, epsilon.clone())
            || s.borrow().hue.abs_diff_eq(&o.borrow().hue, epsilon.clone())
                && s.borrow()
                    .blackness
                    .abs_diff_eq(&o.borrow().blackness, epsilon.clone())
                && s.borrow()
                    .whiteness
                    .abs_diff_eq(&o.borrow().whiteness, epsilon)
    }
}
