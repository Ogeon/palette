use crate::{
    bool_mask::LazySelect,
    hues::OklabHueIter,
    num::{Arithmetics, One, PartialCmp, Real, Zero},
    white_point::D65,
    FromColor, OklabHue, Xyz,
};

use super::Oklrch;

impl_is_within_bounds! {
    Oklrch {
        l => [Self::min_l(), Self::max_l()],
        chroma => [Self::min_chroma(), None]
    }
    where T: Zero + One
}
impl_clamp! {
    Oklrch {
        l => [Self::min_l(), Self::max_l()],
        chroma => [Self::min_chroma()]
    }
    other {hue}
    where T: Zero + One
}

impl_mix_hue!(Oklrch { l, chroma });
impl_lighten!(Oklrch increase {l => [Self::min_l(), Self::max_l()]} other {hue, chroma} where T: Zero + One);
impl_hue_ops!(Oklrch, OklabHue);

impl_color_add!(Oklrch, [l, chroma, hue]);
impl_color_sub!(Oklrch, [l, chroma, hue]);

impl_array_casts!(Oklrch<T>, [T; 3]);
impl_simd_array_conversion_hue!(Oklrch, [l, chroma]);
impl_struct_of_array_traits_hue!(Oklrch, OklabHueIter, [l, chroma]);

impl_eq_hue!(Oklrch, OklabHue, [l, chroma, hue]);

#[allow(deprecated)]
impl<T> crate::RelativeContrast for Oklrch<T>
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
