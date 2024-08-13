//! The ProPhoto RGB standard.

use crate::{
    bool_mask::LazySelect,
    encoding::{FromLinear, IntoLinear},
    luma::LumaStandard,
    num::{Arithmetics, PartialCmp, Powf, Real},
    rgb::{Primaries, RgbSpace, RgbStandard},
    white_point::{Any, D50},
    Mat3, Yxy,
};

use lookup_tables::*;

mod lookup_tables;

/// The ProPhoto RGB standard and color space with gamma 2.2 transfer function.
///
/// About 13% of the colors in this space are "[impossible colors](https://en.wikipedia.org/wiki/Impossible_color)"
/// meaning that they model cone responses that are, in practice, impossible to
/// achieve.
///
/// # As transfer function
///
/// `ProPhotoRgb` will not use any kind of approximation when converting from `T` to
/// `T`. This involves a call to `powf`, which may make it too slow for certain
/// applications.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ProPhotoRgb;

impl<T: Real> Primaries<T> for ProPhotoRgb {
    fn red() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.7347),
            T::from_f64(0.2653),
            T::from_f64(0.2881),
        )
    }
    fn green() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.1596),
            T::from_f64(0.8404),
            T::from_f64(0.7118),
        )
    }
    fn blue() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.0366),
            T::from_f64(0.0001),
            T::from_f64(0.0001),
        )
    }
}

impl RgbSpace for ProPhotoRgb {
    type Primaries = ProPhotoRgb;
    type WhitePoint = D50;

    #[rustfmt::skip]
    #[inline(always)]
    fn rgb_to_xyz_matrix() -> Option<Mat3<f64>> {
        // Matrix from http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
        Some([
            0.7976749, 0.1351917, 0.0313534,
            0.2880402, 0.7118741, 0.0000857,
            0.0000000, 0.0000000, 0.8252100,
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn xyz_to_rgb_matrix() -> Option<Mat3<f64>> {
        // Matrix from http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
        Some([
             1.3459433, -0.2556075, -0.0511118,
            -0.5445989,  1.5081673,  0.0205351,
             0.0000000,  0.0000000,  1.2118128,
        ])
    }
}

impl RgbStandard for ProPhotoRgb {
    type Space = ProPhotoRgb;
    type TransferFn = ProPhotoRgb;
}

impl LumaStandard for ProPhotoRgb {
    type WhitePoint = D50;
    type TransferFn = ProPhotoRgb;
}

impl<T> IntoLinear<T, T> for ProPhotoRgb
where
    T: Real + Powf + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    fn into_linear(encoded: T) -> T {
        lazy_select! {
            if encoded.lt(&T::from_f64(0.03125)) => T::from_f64(1.0 / 16.0) * &encoded,
            else => encoded.clone().powf(T::from_f64(1.8)),
        }
    }
}

impl<T> FromLinear<T, T> for ProPhotoRgb
where
    T: Real + Powf + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    fn from_linear(linear: T) -> T {
        lazy_select! {
            if linear.lt(&T::from_f64(0.001953125)) => T::from_f64(16.0) * &linear,
            else => linear.clone().powf(T::from_f64(1.0 / 1.8)),
        }
    }
}

impl IntoLinear<f64, u16> for ProPhotoRgb {
    fn into_linear(encoded: u16) -> f64 {
        PROPHOTO_U16_TO_F64[encoded as usize]
    }
}

impl IntoLinear<f64, u8> for ProPhotoRgb {
    fn into_linear(encoded: u8) -> f64 {
        // 65535 / 255 = 257
        PROPHOTO_U16_TO_F64[encoded as usize * 257]
    }
}
impl IntoLinear<f32, u16> for ProPhotoRgb {
    fn into_linear(encoded: u16) -> f32 {
        PROPHOTO_U16_TO_F64[encoded as usize] as f32
    }
}

impl IntoLinear<f32, u8> for ProPhotoRgb {
    fn into_linear(encoded: u8) -> f32 {
        // 65535 / 255 = 257
        PROPHOTO_U16_TO_F64[encoded as usize * 257] as f32
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{
            encoding::prophoto::ProPhotoRgb,
            matrix::{matrix_inverse, rgb_to_xyz_matrix},
            rgb::RgbSpace,
        };

        #[test]
        fn rgb_to_xyz() {
            let dynamic = rgb_to_xyz_matrix::<ProPhotoRgb, f64>();
            let constant = ProPhotoRgb::rgb_to_xyz_matrix().unwrap();
            assert_relative_eq!(dynamic[..], constant[..], epsilon = 0.0000001);
        }

        #[test]
        fn xyz_to_rgb() {
            let dynamic = matrix_inverse(rgb_to_xyz_matrix::<ProPhotoRgb, f64>());
            let constant = ProPhotoRgb::xyz_to_rgb_matrix().unwrap();
            assert_relative_eq!(dynamic[..], constant[..], epsilon = 0.0000001);
        }
    }

    mod transfer {
        use crate::encoding::{FromLinear, IntoLinear, ProPhotoRgb};

        #[test]
        fn u16_to_f64_to_u16() {
            for i in 0..=65535u16 {
                let linear: f64 = ProPhotoRgb::into_linear(i);
                let encoded: f64 = ProPhotoRgb::from_linear(linear);
                assert_eq!(i, (65535.0 * encoded + 0.5) as u16);
            }
        }

        #[test]
        fn u16_to_f32_to_u16() {
            for i in 0..=65535u16 {
                let linear: f32 = ProPhotoRgb::into_linear(i);
                let encoded: f32 = ProPhotoRgb::from_linear(linear);
                assert_eq!(i, (65535.0 * encoded + 0.5) as u16);
            }
        }

        #[test]
        fn u8_to_f64_to_u8() {
            for i in 0..=255u8 {
                let linear: f64 = ProPhotoRgb::into_linear(i);
                let encoded: f64 = ProPhotoRgb::from_linear(linear);
                assert_eq!(i, (255.0 * encoded + 0.5) as u8);
            }
        }

        #[test]
        fn u8_to_f32_to_u8() {
            for i in 0..=255u8 {
                let linear: f32 = ProPhotoRgb::into_linear(i);
                let encoded: f32 = ProPhotoRgb::from_linear(linear);
                assert_eq!(i, (255.0 * encoded + 0.5) as u8);
            }
        }
    }
}
