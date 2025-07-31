//! The Adobe RGB (1998) standard.

use palette_math::{
    gamma::lut::GammaLutBuilder,
    lut::{ArrayTable, SliceTable},
};

use crate::{
    encoding::{lut::adobe::*, FromLinear, IntoLinear},
    luma::LumaStandard,
    num::{Powf, Real},
    rgb::{Primaries, RgbSpace, RgbStandard},
    white_point::{Any, D65},
    Mat3, Yxy,
};

use super::{FromLinearLut, GetLutBuilder, IntoLinearLut};

/// The Adobe RGB (1998) (a.k.a. opRGB) color space and standard.
///
/// This color space was designed to encompass most colors achievable by CMYK
/// printers using RGB primaries. It has a wider color gamut than sRGB, primarily
/// in cyan-green hues.
///
/// The Adobe RGB standard uses a gamma 2.2 transfer function.
///
///# As transfer function
///
/// `AdobeRgb` will not use any kind of approximation when converting from `T` to
/// `T`. This involves calls to `powf`, which may make it too slow for certain
/// applications.
///
/// There are some specialized cases where it has been optimized:
///
/// * When converting from `u8` to `f32` or `f64`, while converting to linear
///   space. This uses lookup tables with precomputed values.
/// * When converting from `f32` or `f64` to `u8`, while converting from linear
///   space. This uses a fast algorithm that guarantees a maximum error in the
///   result of less than 0.6 in line with [this DirectX spec](<https://microsoft.github.io/DirectX-Specs/d3d/archive/D3D11_3_FunctionalSpec.htm#FLOATtoSRGB>).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AdobeRgb;

impl AdobeRgb {
    /// Access the pre-generated lookup table for non-linear `u8` to linear `f32` conversion.
    pub fn get_u8_to_f32_lut() -> IntoLinearLut<u8, f32, Self, &'static ArrayTable<256>> {
        IntoLinearLut::from(ADOBE_RGB_U8_TO_F32.get_ref())
    }

    /// Access the pre-generated lookup table for non-linear `u8` to linear `f64` conversion.
    pub fn get_u8_to_f64_lut() -> IntoLinearLut<u8, f64, Self, &'static ArrayTable<256>> {
        IntoLinearLut::from(ADOBE_RGB_U8_TO_F64.get_ref())
    }

    /// Access the pre-generated lookup table for linear `f32` to non-linear `u8` conversion.
    pub fn get_f32_to_u8_lut() -> FromLinearLut<f32, u8, Self, &'static SliceTable> {
        FromLinearLut::from_table(ADOBE_RGB_F32_TO_U8.get_slice())
    }
}

impl<T: Real> Primaries<T> for AdobeRgb {
    // Primary values from https://www.adobe.com/digitalimag/pdfs/AdobeRGB1998.pdf with
    // `luma` values taken from the conversion matrix in `RgbSpace` implementation.
    fn red() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.6400),
            T::from_f64(0.3300),
            T::from_f64(0.2974),
        )
    }
    fn green() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.2100),
            T::from_f64(0.7100),
            T::from_f64(0.6273),
        )
    }
    fn blue() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.1500),
            T::from_f64(0.0600),
            T::from_f64(0.0753),
        )
    }
}

impl RgbSpace for AdobeRgb {
    type Primaries = AdobeRgb;
    type WhitePoint = D65;

    #[rustfmt::skip]
    #[inline(always)]
    fn rgb_to_xyz_matrix() -> Option<Mat3<f64>> {
        // Matrix from http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
        Some([
            0.5767309, 0.1855540, 0.1881852,
            0.2973769, 0.6273491, 0.0752741,
            0.0270343, 0.0706872, 0.9911085,
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn xyz_to_rgb_matrix() -> Option<Mat3<f64>> {
        // Matrix from http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
        Some([
             2.0413690, -0.5649464, -0.3446944,
            -0.9692660,  1.8760108,  0.0415560,
             0.0134474, -0.1183897,  1.0154096,
        ])
    }
}

impl RgbStandard for AdobeRgb {
    type Space = AdobeRgb;
    type TransferFn = AdobeRgb;
}

impl LumaStandard for AdobeRgb {
    type WhitePoint = D65;
    type TransferFn = AdobeRgb;
}

impl GetLutBuilder for AdobeRgb {
    fn get_lut_builder() -> GammaLutBuilder {
        palette_math::gamma::adobe_rgb_builder()
    }
}

impl<T> IntoLinear<T, T> for AdobeRgb
where
    T: Real + Powf,
{
    fn into_linear(encoded: T) -> T {
        encoded.powf(T::from_f64(563.0 / 256.0))
    }
}

impl<T> FromLinear<T, T> for AdobeRgb
where
    T: Real + Powf,
{
    fn from_linear(linear: T) -> T {
        linear.powf(T::from_f64(256.0 / 563.0))
    }
}

impl IntoLinear<f32, u8> for AdobeRgb {
    #[inline]
    fn into_linear(encoded: u8) -> f32 {
        *ADOBE_RGB_U8_TO_F32.lookup(encoded)
    }
}

impl FromLinear<f32, u8> for AdobeRgb {
    #[inline]
    fn from_linear(linear: f32) -> u8 {
        ADOBE_RGB_F32_TO_U8.lookup(linear)
    }
}

impl IntoLinear<f64, u8> for AdobeRgb {
    #[inline]
    fn into_linear(encoded: u8) -> f64 {
        *ADOBE_RGB_U8_TO_F64.lookup(encoded)
    }
}

impl FromLinear<f64, u8> for AdobeRgb {
    #[inline]
    fn from_linear(linear: f64) -> u8 {
        <AdobeRgb>::from_linear(linear as f32)
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{
            encoding::adobe::AdobeRgb,
            matrix::{matrix_inverse, rgb_to_xyz_matrix},
            rgb::RgbSpace,
        };

        #[test]
        fn rgb_to_xyz() {
            let dynamic = rgb_to_xyz_matrix::<AdobeRgb, f64>();
            let constant = AdobeRgb::rgb_to_xyz_matrix().unwrap();
            assert_relative_eq!(dynamic[..], constant[..], epsilon = 0.0000001);
        }

        #[test]
        fn xyz_to_rgb() {
            let dynamic = matrix_inverse(rgb_to_xyz_matrix::<AdobeRgb, f64>());
            let constant = AdobeRgb::xyz_to_rgb_matrix().unwrap();
            assert_relative_eq!(dynamic[..], constant[..], epsilon = 0.0000001);
        }
    }

    #[cfg(feature = "approx")]
    mod transfer {
        use crate::encoding::{AdobeRgb, FromLinear, IntoLinear};

        #[test]
        fn lin_to_enc_to_lin() {
            for i in 0..=100 {
                let linear = i as f64 / 100.0;
                let encoded: f64 = AdobeRgb::from_linear(linear);
                assert_relative_eq!(linear, AdobeRgb::into_linear(encoded), epsilon = 0.0000001);
            }
        }

        #[test]
        fn enc_to_lin_to_enc() {
            for i in 0..=100 {
                let encoded = i as f64 / 100.0;
                let linear: f64 = AdobeRgb::into_linear(encoded);
                assert_relative_eq!(encoded, AdobeRgb::from_linear(linear), epsilon = 0.0000001);
            }
        }

        #[test]
        fn correct_values() {
            let half_to_encoded: f64 = AdobeRgb::from_linear(0.5);
            assert_relative_eq!(half_to_encoded, 0.72965838, epsilon = 0.0000001);
            let half_to_linear = AdobeRgb::into_linear(0.5);
            assert_relative_eq!(half_to_linear, 0.21775552, epsilon = 0.0000001);
        }
    }

    mod lut {
        use crate::{
            encoding::{AdobeRgb, FromLinear, IntoLinear},
            rgb,
        };

        #[test]
        #[cfg_attr(miri, ignore)]
        #[cfg(feature = "approx")]
        fn test_u8_f32_into_impl() {
            for i in 0..=255u8 {
                let u8_impl: f32 = AdobeRgb::into_linear(i);
                let f32_impl = AdobeRgb::into_linear(i as f32 / 255.0);
                assert_relative_eq!(u8_impl, f32_impl, epsilon = 0.000001);
            }
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        #[cfg(feature = "approx")]
        fn test_u8_f64_into_impl() {
            for i in 0..=255u8 {
                let u8_impl: f64 = AdobeRgb::into_linear(i);
                let f64_impl = AdobeRgb::into_linear(i as f64 / 255.0);
                assert_relative_eq!(u8_impl, f64_impl, epsilon = 0.0000001);
            }
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn u8_to_f32_to_u8() {
            for expected in 0u8..=255u8 {
                let linear: f32 = AdobeRgb::into_linear(expected);
                let result: u8 = AdobeRgb::from_linear(linear);
                assert_eq!(result, expected);
            }
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn u8_to_f64_to_u8() {
            for expected in 0u8..=255u8 {
                let linear: f64 = AdobeRgb::into_linear(expected);
                let result: u8 = AdobeRgb::from_linear(linear);
                assert_eq!(result, expected);
            }
        }

        #[test]
        fn constant_lut() {
            let decode_lut = AdobeRgb::get_u8_to_f32_lut();
            let decode_lut_64 = AdobeRgb::get_u8_to_f64_lut();
            let encode_lut = AdobeRgb::get_f32_to_u8_lut();

            let linear: rgb::LinAdobeRgb<f32> =
                decode_lut.lookup_rgb(rgb::AdobeRgb::new(23, 198, 76));
            let _: rgb::AdobeRgb<u8> = encode_lut.lookup_rgb(linear);

            let _: rgb::LinAdobeRgb<f64> =
                decode_lut_64.lookup_rgb(rgb::AdobeRgb::new(23, 198, 76));
        }
    }
}
