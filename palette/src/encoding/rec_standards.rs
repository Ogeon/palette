//! The ITU-R Recommendation BT.2020 (aka Rec. 2020) standard.

use crate::{
    bool_mask::LazySelect,
    encoding::{FromLinear, IntoLinear, Srgb},
    luma::LumaStandard,
    num::{Arithmetics, MulAdd, MulSub, PartialCmp, Powf, Real},
    rgb::{Primaries, RgbSpace, RgbStandard},
    white_point::{Any, D65},
    Mat3, Yxy,
};

/// The Rec. 2020 standard, color space, and transfer function.
///
/// # As transfer function
///
/// `Rec2020` will not use any kind of approximation when converting from `T` to
/// `T`. This involves calls to `powf`, which may make it too slow for certain
/// applications.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rec2020;

impl<T: Real> Primaries<T> for Rec2020 {
    fn red() -> Yxy<Any, T> {
        Yxy::new(T::from_f64(0.708), T::from_f64(0.292), T::from_f64(0.2627))
    }
    fn green() -> Yxy<Any, T> {
        Yxy::new(T::from_f64(0.170), T::from_f64(0.797), T::from_f64(0.6780))
    }
    fn blue() -> Yxy<Any, T> {
        Yxy::new(T::from_f64(0.131), T::from_f64(0.046), T::from_f64(0.0593))
    }
}

impl RgbSpace for Rec2020 {
    type Primaries = Rec2020;
    type WhitePoint = D65;

    #[rustfmt::skip]
    #[inline(always)]
    fn rgb_to_xyz_matrix() -> Option<Mat3<f64>> {
        Some([
            0.6370102, 0.1446150, 0.1688448,
            0.2627217, 0.6779893, 0.0592890,
            0.0000000, 0.0280723, 1.0607577,
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn xyz_to_rgb_matrix() -> Option<Mat3<f64>> {
        Some([
             1.7165107, -0.3556417, -0.2533455,
            -0.6666930,  1.6165022,  0.0157688,
             0.0176436, -0.0427798,  0.9423051,
        ])
    }
}

impl RgbStandard for Rec2020 {
    type Space = Rec2020;
    type TransferFn = RecOetf;
}

impl LumaStandard for Rec2020 {
    type WhitePoint = D65;
    type TransferFn = RecOetf;
}

/// The Rec. 709 standard, color space, and transfer function.
///
/// # As transfer function
///
/// `Rec709` will not use any kind of approximation when converting from `T` to
/// `T`. This involves calls to `powf`, which may make it too slow for certain
/// applications.
pub struct Rec709;

impl RgbStandard for Rec709 {
    type Space = Srgb;
    type TransferFn = RecOetf;
}

impl LumaStandard for Rec709 {
    type WhitePoint = D65;
    type TransferFn = RecOetf;
}

/// The opto-electronic transfer function used in standard dynamic range (SDR)
/// standards by the ITU-R such as [`Rec709`] and [`Rec2020`].
pub struct RecOetf;

const ALPHA: f64 = 1.09929682680944;
const BETA: f64 = 0.018053968510807;

impl<T> IntoLinear<T, T> for RecOetf
where
    T: Real + Powf + MulAdd + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    #[inline]
    fn into_linear(encoded: T) -> T {
        lazy_select! {
            if encoded.lt(&T::from_f64(4.5*BETA)) => T::from_f64(1.0 / 4.5) * &encoded,
            else => encoded.clone().mul_add(T::from_f64(1.0 / ALPHA), T::from_f64(1.0 - 1.0 / ALPHA)).powf(T::from_f64(1.0 / 0.45))
        }
    }
}

impl<T> FromLinear<T, T> for RecOetf
where
    T: Real + Powf + MulSub + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    #[inline]
    fn from_linear(linear: T) -> T {
        lazy_select! {
            if linear.lt(&T::from_f64(BETA)) => T::from_f64(4.5) * &linear,
            else => linear.clone().powf(T::from_f64(0.45)).mul_sub(T::from_f64(ALPHA), T::from_f64(1.0 - ALPHA))
        }
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{
            encoding::Rec2020,
            matrix::{matrix_inverse, rgb_to_xyz_matrix},
            rgb::RgbSpace,
        };

        #[test]
        fn rgb_to_xyz() {
            let dynamic = rgb_to_xyz_matrix::<Rec2020, f64>();
            let constant = Rec2020::rgb_to_xyz_matrix().unwrap();
            assert_relative_eq!(dynamic[..], constant[..], epsilon = 0.0000001);
        }

        #[test]
        fn xyz_to_rgb() {
            let dynamic = matrix_inverse(rgb_to_xyz_matrix::<Rec2020, f64>());
            let constant = Rec2020::xyz_to_rgb_matrix().unwrap();
            assert_relative_eq!(dynamic[..], constant[..], epsilon = 0.0000001);
        }
    }
}
