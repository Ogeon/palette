use core::ops::{Add, AddAssign, BitAnd, Sub, SubAssign};

#[cfg(feature = "approx")]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use crate::{
    angle::{RealAngle, SignedAngle},
    bool_mask::LazySelect,
    clamp, clamp_assign, contrast_ratio,
    num::{
        self, Arithmetics, FromScalarArray, IntoScalarArray, MinMax, One, PartialCmp, Real, Zero,
    },
    white_point::D65,
    Alpha, Clamp, ClampAssign, FromColor, GetHue, IsWithinBounds, Lighten, LightenAssign, Mix,
    MixAssign, OklabHue, RelativeContrast, Saturate, SaturateAssign, SetHue, ShiftHue,
    ShiftHueAssign, WithHue, Xyz,
};

use super::Oklch;

impl_is_within_bounds! {
    Oklch {
        l => [Self::min_l(), Self::max_l()],
        chroma => [Self::min_chroma(), Self::max_chroma()]
    }
    where T: Real+Zero + One
}

impl<T> Clamp for Oklch<T>
where
    T: Real + Zero + One + num::Clamp,
{
    #[inline]
    fn clamp(self) -> Self {
        Self::new(
            clamp(self.l, Self::min_l(), Self::max_l()),
            clamp(self.chroma, Self::min_chroma(), Self::max_chroma()),
            self.hue,
        )
    }
}

impl<T> ClampAssign for Oklch<T>
where
    T: Real + Zero + One + num::ClampAssign,
{
    #[inline]
    fn clamp_assign(&mut self) {
        clamp_assign(&mut self.l, Self::min_l(), Self::max_l());
        clamp_assign(&mut self.chroma, Self::min_chroma(), Self::max_chroma());
    }
}

impl_mix_hue!(Oklch { l, chroma });
impl_lighten!(Oklch increase {l => [Self::min_l(), Self::max_l()]} other {hue, chroma} where T: One);
impl_saturate!(Oklch increase {chroma => [Self::min_chroma(), Self::max_chroma()]} other {hue, l} where T: One);

impl<T> GetHue for Oklch<T>
where
    T: Clone,
{
    type Hue = OklabHue<T>;

    #[inline]
    fn get_hue(&self) -> OklabHue<T> {
        self.hue.clone()
    }
}

impl<T, H> WithHue<H> for Oklch<T>
where
    H: Into<OklabHue<T>>,
{
    #[inline]
    fn with_hue(mut self, hue: H) -> Self {
        self.hue = hue.into();
        self
    }
}

impl<T, H> SetHue<H> for Oklch<T>
where
    H: Into<OklabHue<T>>,
{
    #[inline]
    fn set_hue(&mut self, hue: H) {
        self.hue = hue.into();
    }
}

impl<T> ShiftHue for Oklch<T>
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

impl<T> ShiftHueAssign for Oklch<T>
where
    T: AddAssign,
{
    type Scalar = T;

    #[inline]
    fn shift_hue_assign(&mut self, amount: Self::Scalar) {
        self.hue += amount;
    }
}

impl_color_add!(Oklch<T>, [l, chroma, hue]);
impl_color_sub!(Oklch<T>, [l, chroma, hue]);

impl_array_casts!(Oklch<T>, [T; 3]);
impl_simd_array_conversion_hue!(Oklch, [l, chroma]);

impl_eq_hue!(Oklch, OklabHue, [l, chroma, hue]);

impl<T> RelativeContrast for Oklch<T>
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
