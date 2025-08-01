//! The ITU-R Recommendation BT.2020 (Rec. 2020) and BT.709 (Rec. 709) standards and their
//! associated transfer function.

use palette_math::{
    gamma::lut::GammaLutBuilder,
    lut::{ArrayTable, SliceTable},
};

use crate::{
    bool_mask::LazySelect,
    encoding::{lut::rec_standards::*, FromLinear, IntoLinear, Srgb},
    luma::LumaStandard,
    num::{Arithmetics, MulAdd, MulSub, PartialCmp, Powf, Real},
    rgb::{Primaries, RgbSpace, RgbStandard},
    white_point::{Any, D65},
    Mat3, Yxy,
};

use super::{FromLinearLut, GetLutBuilder, IntoLinearLut};

/// The Rec. 2020 standard, color space, and transfer function ([`RecOetf`]).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rec2020;

impl<T: Real> Primaries<T> for Rec2020 {
    // Primary values taken from ITU specification:
    // https://www.itu.int/dms_pubrec/itu-r/rec/bt/R-REC-BT.2020-2-201510-I!!PDF-E.pdf
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
        // Matrix calculated using specified primary values and white point
        // using formulas from http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
        Some([
            0.6370102, 0.1446150, 0.1688448,
            0.2627217, 0.6779893, 0.0592890,
            0.0000000, 0.0280723, 1.0607577,
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn xyz_to_rgb_matrix() -> Option<Mat3<f64>> {
        // Matrix calculated using specified primary values and white point
        // using formulas from http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
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

/// The Rec. 709 standard, color space, and transfer function ([`RecOetf`]).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
///
/// `RecOetf` will not use any kind of approximation when converting from `T` to
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
pub struct RecOetf;

impl RecOetf {
    /// Access the pre-generated lookup table for non-linear `u8` to linear `f32` conversion.
    pub fn get_u8_to_f32_lut() -> IntoLinearLut<u8, f32, Self, &'static ArrayTable<256>> {
        IntoLinearLut::from(REC_OETF_U8_TO_F32.get_ref())
    }

    /// Access the pre-generated lookup table for non-linear `u8` to linear `f64` conversion.
    pub fn get_u8_to_f64_lut() -> IntoLinearLut<u8, f64, Self, &'static ArrayTable<256>> {
        IntoLinearLut::from(REC_OETF_U8_TO_F64.get_ref())
    }

    /// Access the pre-generated lookup table for linear `f32` to non-linear `u8` conversion.
    pub fn get_f32_to_u8_lut() -> FromLinearLut<f32, u8, Self, &'static SliceTable> {
        FromLinearLut::from_table(REC_OETF_F32_TO_U8.get_slice())
    }
}

impl GetLutBuilder for RecOetf {
    fn get_lut_builder() -> GammaLutBuilder {
        palette_math::gamma::rec_oetf_builder()
    }
}

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
            else => linear.clone().powf(T::from_f64(0.45)).mul_sub(T::from_f64(ALPHA), T::from_f64(ALPHA - 1.0))
        }
    }
}

impl IntoLinear<f32, u8> for RecOetf {
    #[inline]
    fn into_linear(encoded: u8) -> f32 {
        *REC_OETF_U8_TO_F32.lookup(encoded)
    }
}

impl FromLinear<f32, u8> for RecOetf {
    #[inline]
    fn from_linear(linear: f32) -> u8 {
        REC_OETF_F32_TO_U8.lookup(linear)
    }
}

impl IntoLinear<f64, u8> for RecOetf {
    #[inline]
    fn into_linear(encoded: u8) -> f64 {
        *REC_OETF_U8_TO_F64.lookup(encoded)
    }
}

impl FromLinear<f64, u8> for RecOetf {
    #[inline]
    fn from_linear(linear: f64) -> u8 {
        <RecOetf>::from_linear(linear as f32)
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

    #[cfg(feature = "approx")]
    mod transfer {
        use crate::encoding::{FromLinear, IntoLinear, RecOetf};

        #[test]
        fn lin_to_enc_to_lin() {
            for i in 0..=100 {
                let linear = i as f64 / 100.0;
                let encoded: f64 = RecOetf::from_linear(linear);
                assert_relative_eq!(linear, RecOetf::into_linear(encoded), epsilon = 0.0000001);
            }
        }

        #[test]
        fn enc_to_lin_to_enc() {
            for i in 0..=100 {
                let encoded = i as f64 / 100.0;
                let linear: f64 = RecOetf::into_linear(encoded);
                assert_relative_eq!(encoded, RecOetf::from_linear(linear), epsilon = 0.0000001);
            }
        }
    }

    mod lut {
        use crate::{
            encoding::{FromLinear, IntoLinear, RecOetf},
            rgb,
        };

        #[test]
        #[cfg_attr(miri, ignore)]
        #[cfg(feature = "approx")]
        fn test_u8_f32_into_impl() {
            for i in 0..=255u8 {
                let u8_impl: f32 = RecOetf::into_linear(i);
                let f32_impl = RecOetf::into_linear(i as f32 / 255.0);
                assert_relative_eq!(u8_impl, f32_impl, epsilon = 0.000001);
            }
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        #[cfg(feature = "approx")]
        fn test_u8_f64_into_impl() {
            for i in 0..=255u8 {
                let u8_impl: f64 = RecOetf::into_linear(i);
                let f64_impl = RecOetf::into_linear(i as f64 / 255.0);
                assert_relative_eq!(u8_impl, f64_impl, epsilon = 0.0000001);
            }
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn u8_to_f32_to_u8() {
            for expected in 0u8..=255u8 {
                let linear: f32 = RecOetf::into_linear(expected);
                let result: u8 = RecOetf::from_linear(linear);
                assert_eq!(result, expected);
            }
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn u8_to_f64_to_u8() {
            for expected in 0u8..=255u8 {
                let linear: f64 = RecOetf::into_linear(expected);
                let result: u8 = RecOetf::from_linear(linear);
                assert_eq!(result, expected);
            }
        }

        #[test]
        fn constant_lut() {
            let decode_lut = RecOetf::get_u8_to_f32_lut();
            let decode_lut_64 = RecOetf::get_u8_to_f64_lut();
            let encode_lut = RecOetf::get_f32_to_u8_lut();

            let linear: rgb::LinRec2020<f32> =
                decode_lut.lookup_rgb(rgb::Rec2020::new(23, 198, 76));
            let _: rgb::Rec2020<u8> = encode_lut.lookup_rgb(linear);

            let linear: rgb::LinRec709<f32> = decode_lut.lookup_rgb(rgb::Rec709::new(23, 198, 76));
            let _: rgb::Rec709<u8> = encode_lut.lookup_rgb(linear);

            let _: rgb::LinRec2020<f64> = decode_lut_64.lookup_rgb(rgb::Rec2020::new(23, 198, 76));
            let _: rgb::LinRec709<f64> = decode_lut_64.lookup_rgb(rgb::Rec709::new(23, 198, 76));
        }
    }
}
