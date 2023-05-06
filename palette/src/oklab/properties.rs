use core::ops::{Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[cfg(feature = "approx")]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use crate::{
    angle::RealAngle,
    blend::{PreAlpha, Premultiply},
    bool_mask::LazySelect,
    clamp, clamp_assign,
    num::{
        self, Arithmetics, FromScalarArray, IntoScalarArray, IsValidDivisor, MinMax, One,
        PartialCmp, Real, Trigonometry, Zero,
    },
    stimulus::Stimulus,
    white_point::D65,
    Alpha, Clamp, ClampAssign, FromColor, GetHue, IsWithinBounds, Lighten, LightenAssign, Mix,
    MixAssign, OklabHue, Xyz,
};

use super::Oklab;

impl_is_within_bounds! {
    Oklab {
        l => [Self::min_l(), Self::max_l()]
    }
    where T: Zero + One
}

impl<T> Clamp for Oklab<T>
where
    T: num::Clamp + Zero + One,
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
    T: num::ClampAssign + Zero + One,
{
    #[inline]
    fn clamp_assign(&mut self) {
        clamp_assign(&mut self.l, Self::min_l(), Self::max_l());
    }
}

impl_mix!(Oklab);
impl_lighten!(Oklab increase {l => [Self::min_l(), Self::max_l()]} other {a, b} where T:  One);
impl_premultiply!(Oklab { l, a, b });
impl_euclidean_distance!(Oklab { l, a, b });

impl<T> GetHue for Oklab<T>
where
    T: RealAngle + Trigonometry + Add<T, Output = T> + Neg<Output = T> + Clone,
{
    type Hue = OklabHue<T>;

    fn get_hue(&self) -> OklabHue<T> {
        OklabHue::from_cartesian(self.a.clone(), self.b.clone())
    }
}

impl_color_add!(Oklab<T>, [l, a, b]);
impl_color_sub!(Oklab<T>, [l, a, b]);
impl_color_mul!(Oklab<T>, [l, a, b]);
impl_color_div!(Oklab<T>, [l, a, b]);

impl_array_casts!(Oklab<T>, [T; 3]);
impl_simd_array_conversion!(Oklab, [l, a, b]);
impl_struct_of_array_traits!(Oklab, [l, a, b]);

impl_eq!(Oklab, [l, a, b]);

#[allow(deprecated)]
impl<T> crate::RelativeContrast for Oklab<T>
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
