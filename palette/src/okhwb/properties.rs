use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::stimulus::Stimulus;
use crate::white_point::D65;
use crate::{
    angle::{RealAngle, SignedAngle},
    hues::OklabHueIter,
};
use crate::{
    bool_mask::{LazySelect, Select},
    clamp, clamp_min, clamp_min_assign, FromColor, Lighten, LightenAssign, Mix, MixAssign,
    OklabHue, Xyz,
};
use crate::{
    num::{self, Arithmetics, FromScalarArray, IntoScalarArray, One, PartialCmp, Real, Zero},
    Alpha,
};

use super::Okhwb;

impl_is_within_bounds_hwb!(Okhwb where T: Stimulus);
impl_clamp_hwb!(Okhwb where T: Stimulus);

impl_mix_hue!(Okhwb {
    whiteness,
    blackness
});
impl_lighten_hwb!(Okhwb where T: Stimulus);
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
