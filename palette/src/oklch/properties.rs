use core::ops::{Add, AddAssign, BitAnd, Sub, SubAssign};

use crate::{
    angle::{RealAngle, SignedAngle},
    bool_mask::LazySelect,
    clamp, clamp_assign, clamp_min, clamp_min_assign,
    hues::OklabHueIter,
    num::{
        self, Arithmetics, FromScalarArray, IntoScalarArray, MinMax, One, PartialCmp, Real, Zero,
    },
    white_point::D65,
    Alpha, Clamp, ClampAssign, FromColor, IsWithinBounds, Lighten, LightenAssign, Mix, MixAssign,
    OklabHue, Xyz,
};

use super::Oklch;

impl_is_within_bounds! {
    Oklch {
        l => [Self::min_l(), Self::max_l()],
        chroma => [Self::min_chroma(),None]
    }
    where T: Zero + One
}

impl<T> Clamp for Oklch<T>
where
    T: num::Clamp + Zero + One,
{
    #[inline]
    fn clamp(self) -> Self {
        Self::new(
            clamp(self.l, Self::min_l(), Self::max_l()),
            clamp_min(self.chroma, Self::min_chroma()),
            self.hue,
        )
    }
}

impl<T> ClampAssign for Oklch<T>
where
    T: num::ClampAssign + Zero + One,
{
    #[inline]
    fn clamp_assign(&mut self) {
        clamp_assign(&mut self.l, Self::min_l(), Self::max_l());
        clamp_min_assign(&mut self.chroma, Self::min_chroma());
    }
}

impl_mix_hue!(Oklch { l, chroma });
impl_lighten!(Oklch increase {l => [Self::min_l(), Self::max_l()]} other {hue, chroma} where T: Zero + One);
impl_hue_ops!(Oklch, OklabHue);

impl_color_add!(Oklch<T>, [l, chroma, hue]);
impl_color_sub!(Oklch<T>, [l, chroma, hue]);

impl_array_casts!(Oklch<T>, [T; 3]);
impl_simd_array_conversion_hue!(Oklch, [l, chroma]);
impl_struct_of_array_traits_hue!(Oklch, OklabHueIter, [l, chroma]);

impl_eq_hue!(Oklch, OklabHue, [l, chroma, hue]);

#[allow(deprecated)]
impl<T> crate::RelativeContrast for Oklch<T>
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
