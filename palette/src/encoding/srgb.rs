//! The sRGB standard.

use palette_math::{
    gamma::lut::GammaLutBuilder,
    lut::{ArrayTable, SliceTable},
};

use crate::{
    bool_mask::LazySelect,
    encoding::{lut::srgb::*, FromLinear, IntoLinear},
    luma::LumaStandard,
    num::{Arithmetics, MulAdd, MulSub, PartialCmp, Powf, Real},
    rgb::{Primaries, RgbSpace, RgbStandard},
    white_point::{Any, D65},
    Mat3, Yxy,
};

use super::{FromLinearLut, GetLutBuilder, IntoLinearLut};

/// The sRGB standard, color space, and transfer function.
///
/// # As transfer function
///
/// `Srgb` will not use any kind of approximation when converting from `T` to
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
pub struct Srgb;

impl Srgb {
    /// Access the pre-generated lookup table for non-linear `u8` to linear `f32` conversion.
    pub fn get_u8_to_f32_lut() -> IntoLinearLut<u8, f32, Self, &'static ArrayTable<256>> {
        IntoLinearLut::from(SRGB_U8_TO_F32.get_ref())
    }

    /// Access the pre-generated lookup table for non-linear `u8` to linear `f64` conversion.
    pub fn get_u8_to_f64_lut() -> IntoLinearLut<u8, f64, Self, &'static ArrayTable<256>> {
        IntoLinearLut::from(SRGB_U8_TO_F64.get_ref())
    }

    /// Access the pre-generated lookup table for linear `f32` to non-linear `u8` conversion.
    pub fn get_f32_to_u8_lut() -> FromLinearLut<f32, u8, Self, &'static SliceTable> {
        FromLinearLut::from_table(SRGB_F32_TO_U8.get_slice())
    }
}

impl<T: Real> Primaries<T> for Srgb {
    fn red() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.6400),
            T::from_f64(0.3300),
            T::from_f64(0.212656),
        )
    }
    fn green() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.3000),
            T::from_f64(0.6000),
            T::from_f64(0.715158),
        )
    }
    fn blue() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.1500),
            T::from_f64(0.0600),
            T::from_f64(0.072186),
        )
    }
}

impl RgbSpace for Srgb {
    type Primaries = Srgb;
    type WhitePoint = D65;

    #[rustfmt::skip]
    #[inline(always)]
    fn rgb_to_xyz_matrix() -> Option<Mat3<f64>> {
        // Matrix from http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
        Some([
            0.4124564, 0.3575761, 0.1804375,
            0.2126729, 0.7151522, 0.0721750,
            0.0193339, 0.1191920, 0.9503041,
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn xyz_to_rgb_matrix() -> Option<Mat3<f64>> {
        // Matrix from http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
        Some([
             3.2404542, -1.5371385, -0.4985314,
            -0.9692660,  1.8760108,  0.0415560,
             0.0556434, -0.2040259,  1.0572252,
        ])
    }
}

impl RgbStandard for Srgb {
    type Space = Srgb;
    type TransferFn = Srgb;
}

impl LumaStandard for Srgb {
    type WhitePoint = D65;
    type TransferFn = Srgb;
}

impl GetLutBuilder for Srgb {
    fn get_lut_builder() -> GammaLutBuilder {
        palette_math::gamma::adobe_rgb_builder()
    }
}

impl<T> IntoLinear<T, T> for Srgb
where
    T: Real + Powf + MulAdd + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    #[inline]
    fn into_linear(x: T) -> T {
        // Dividing the constants directly shows performance benefits in benchmarks for this function
        lazy_select! {
            if x.lt_eq(&T::from_f64(0.04045)) => T::from_f64(1.0 / 12.92) * &x,
            else => x.clone().mul_add(T::from_f64(1.0 / 1.055), T::from_f64(0.055 / 1.055)).powf(T::from_f64(2.4)),
        }
    }
}

impl<T> FromLinear<T, T> for Srgb
where
    T: Real + Powf + MulSub + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    #[inline]
    fn from_linear(x: T) -> T {
        lazy_select! {
            if x.lt_eq(&T::from_f64(0.0031308)) => T::from_f64(12.92) * &x,
            else => x.clone().powf(T::from_f64(1.0 / 2.4)).mul_sub(T::from_f64(1.055), T::from_f64(0.055)),
        }
    }
}

impl IntoLinear<f32, u8> for Srgb {
    #[inline]
    fn into_linear(encoded: u8) -> f32 {
        *SRGB_U8_TO_F32.lookup(encoded)
    }
}

impl FromLinear<f32, u8> for Srgb {
    #[inline]
    fn from_linear(linear: f32) -> u8 {
        SRGB_F32_TO_U8.lookup(linear)
    }
}

impl IntoLinear<f64, u8> for Srgb {
    #[inline]
    fn into_linear(encoded: u8) -> f64 {
        *SRGB_U8_TO_F64.lookup(encoded)
    }
}

impl FromLinear<f64, u8> for Srgb {
    #[inline]
    fn from_linear(linear: f64) -> u8 {
        <Srgb>::from_linear(linear as f32)
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{
            encoding::Srgb,
            matrix::{matrix_inverse, rgb_to_xyz_matrix},
            rgb::RgbSpace,
        };

        #[test]
        fn rgb_to_xyz() {
            let dynamic = rgb_to_xyz_matrix::<Srgb, f64>();
            let constant = Srgb::rgb_to_xyz_matrix().unwrap();
            assert_relative_eq!(dynamic[..], constant[..], epsilon = 0.0000001);
        }

        #[test]
        fn xyz_to_rgb() {
            let dynamic = matrix_inverse(rgb_to_xyz_matrix::<Srgb, f64>());
            let constant = Srgb::xyz_to_rgb_matrix().unwrap();
            assert_relative_eq!(dynamic[..], constant[..], epsilon = 0.0000001);
        }
    }

    #[cfg(feature = "approx")]
    mod transfer {
        use crate::encoding::{FromLinear, IntoLinear, Srgb};

        #[test]
        fn lin_to_enc_to_lin() {
            for i in 0..=100 {
                let linear = i as f64 / 100.0;
                let encoded: f64 = Srgb::from_linear(linear);
                assert_relative_eq!(linear, Srgb::into_linear(encoded), epsilon = 0.0000001);
            }
        }

        #[test]
        fn enc_to_lin_to_enc() {
            for i in 0..=100 {
                let encoded = i as f64 / 100.0;
                let linear: f64 = Srgb::into_linear(encoded);
                assert_relative_eq!(encoded, Srgb::from_linear(linear), epsilon = 0.0000001);
            }
        }
    }

    mod lut {
        use crate::{
            encoding::{FromLinear, IntoLinear, Srgb},
            rgb,
        };

        #[test]
        #[cfg_attr(miri, ignore)]
        #[cfg(feature = "approx")]
        fn test_u8_f32_into_impl() {
            for i in 0..=255u8 {
                let u8_impl: f32 = Srgb::into_linear(i);
                let f32_impl = Srgb::into_linear(i as f32 / 255.0);
                assert_relative_eq!(u8_impl, f32_impl, epsilon = 0.000001);
            }
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        #[cfg(feature = "approx")]
        fn test_u8_f64_into_impl() {
            for i in 0..=255u8 {
                let u8_impl: f64 = Srgb::into_linear(i);
                let f64_impl = Srgb::into_linear(i as f64 / 255.0);
                assert_relative_eq!(u8_impl, f64_impl, epsilon = 0.0000001);
            }
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn u8_to_f32_to_u8() {
            for expected in 0..=255u8 {
                let linear: f32 = Srgb::into_linear(expected);
                let result: u8 = Srgb::from_linear(linear);
                assert_eq!(result, expected);
            }
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn u8_to_f64_to_u8() {
            for expected in 0..=255u8 {
                let linear: f64 = Srgb::into_linear(expected);
                let result: u8 = Srgb::from_linear(linear);
                assert_eq!(result, expected);
            }
        }

        #[test]
        fn constant_lut() {
            let decode_lut = Srgb::get_u8_to_f32_lut();
            let decode_lut_64 = Srgb::get_u8_to_f64_lut();
            let encode_lut = Srgb::get_f32_to_u8_lut();

            let linear: rgb::LinSrgb<f32> = decode_lut.lookup_rgb(rgb::Srgb::new(23, 198, 76));
            let _: rgb::Srgb<u8> = encode_lut.lookup_rgb(linear);

            let linear: rgb::LinDisplayP3<f32> =
                decode_lut.lookup_rgb(rgb::DisplayP3::new(23, 198, 76));
            let _: rgb::DisplayP3<u8> = encode_lut.lookup_rgb(linear);

            let _: rgb::LinSrgb<f64> = decode_lut_64.lookup_rgb(rgb::Srgb::new(23, 198, 76));
            let _: rgb::LinDisplayP3<f64> =
                decode_lut_64.lookup_rgb(rgb::DisplayP3::new(23, 198, 76));
        }
    }
}
