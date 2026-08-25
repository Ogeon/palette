use core::ops::{Add, Neg};

use crate::{
    angle::RealAngle,
    bool_mask::LazySelect,
    num::{Arithmetics, One, PartialCmp, Real, Trigonometry, Zero},
    white_point::D65,
    FromColor, GetHue, OklabHue, Xyz,
};

use super::Oklrab;

impl_is_within_bounds! {
    Oklrab {
        l => [Self::min_l(), Self::max_l()]
    }
    where T: Zero + One
}
impl_clamp! {
    Oklrab {
        l => [Self::min_l(), Self::max_l()]
    }
    other {a, b}
    where T: Zero + One
}

impl_mix!(Oklrab);
impl_lighten!(Oklrab increase {l => [Self::min_l(), Self::max_l()]} other {a, b} where T:  One);
impl_premultiply!(Oklrab { l, a, b });
impl_euclidean_distance!(Oklrab { l, a, b });
impl_hyab!(Oklrab {
    lightness: l,
    chroma1: a,
    chroma2: b
});
impl_lab_color_schemes!(Oklrab[l]);

impl<T> GetHue for Oklrab<T>
where
    T: RealAngle + Trigonometry + Add<T, Output = T> + Neg<Output = T> + Clone,
{
    type Hue = OklabHue<T>;

    fn get_hue(&self) -> OklabHue<T> {
        OklabHue::from_cartesian(self.a.clone(), self.b.clone())
    }
}

impl_color_add!(Oklrab, [l, a, b]);
impl_color_sub!(Oklrab, [l, a, b]);
impl_color_mul!(Oklrab, [l, a, b]);
impl_color_div!(Oklrab, [l, a, b]);

impl_array_casts!(Oklrab<T>, [T; 3]);
impl_simd_array_conversion!(Oklrab, [l, a, b]);
impl_struct_of_array_traits!(Oklrab, [l, a, b]);

impl_eq!(Oklrab, [l, a, b]);

#[allow(deprecated)]
impl<T> crate::RelativeContrast for Oklrab<T>
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

#[cfg(test)]
mod test {
    #[cfg(feature = "approx")]
    use crate::{Oklrab, Oklrch};

    test_lab_color_schemes!(Oklrab / Oklrch[l]);
}
