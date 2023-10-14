use core::ops::{Add, AddAssign, BitAnd, DivAssign, Sub, SubAssign};

use crate::stimulus::Stimulus;
use crate::white_point::D65;
use crate::HasBoolMask;
use crate::{
    angle::{RealAngle, SignedAngle},
    hues::OklabHueIter,
};
use crate::{
    bool_mask::{LazySelect, Select},
    clamp, clamp_min, clamp_min_assign, ClampAssign, FromColor, IsWithinBounds, Lighten,
    LightenAssign, Mix, MixAssign, OklabHue, Xyz,
};
use crate::{
    num::{
        self, Arithmetics, FromScalarArray, IntoScalarArray, MinMax, One, PartialCmp, Real, Zero,
    },
    Alpha, Clamp,
};

use super::Okhwb;

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

impl_hue_ops!(Okhwb, OklabHue);

impl_color_add!(Okhwb<T>, [hue, whiteness, blackness]);
impl_color_sub!(Okhwb<T>, [hue, whiteness, blackness]);

impl_array_casts!(Okhwb<T>, [T; 3]);
impl_simd_array_conversion_hue!(Okhwb, [whiteness, blackness]);
impl_struct_of_array_traits_hue!(Okhwb, OklabHueIter, [whiteness, blackness]);

#[allow(deprecated)]
impl<T> crate::RelativeContrast for Okhwb<T>
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

        crate::contrast_ratio(xyz1.y, xyz2.y)
    }
}

impl_eq_hue!(Okhwb, OklabHue, [hue, whiteness, blackness]);
