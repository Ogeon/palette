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

use lookup_tables::*;

mod lookup_tables;

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
            else => linear.clone().powf(T::from_f64(0.45)).mul_sub(T::from_f64(ALPHA), T::from_f64(ALPHA - 1.0))
        }
    }
}

impl IntoLinear<f32, u8> for RecOetf {
    #[inline]
    fn into_linear(encoded: u8) -> f32 {
        REC_OETF_U8_TO_F32[encoded as usize]
    }
}

impl FromLinear<f32, u8> for RecOetf {
    #[inline]
    fn from_linear(linear: f32) -> u8 {
        // Algorithm modeled closely off of `f32_to_srgb8` from fast-srgb8 crate
        const MAX_FLOAT_BITS: u32 = 0x3f7fffff;
        const MIN_FLOAT_BITS: u32 = 0x39000000;
        let max_float = f32::from_bits(MAX_FLOAT_BITS);
        let min_float = f32::from_bits(MIN_FLOAT_BITS);

        let mut input = linear;
        // Implemented this way to map NaN to `min_float`
        if input.partial_cmp(&min_float) != Some(core::cmp::Ordering::Greater) {
            input = min_float;
        } else if input > max_float {
            input = max_float;
        }
        let input_bits = input.to_bits();
        #[cfg(all(not(bench), test))]
        {
            debug_assert!((MIN_FLOAT_BITS..=MAX_FLOAT_BITS).contains(&input_bits));
        }
        // Safety: all input floats are clamped into the {min_float, max_float} range,
        // which turns out in this case to guarantee that their bitwise reprs are
        // clamped to the {MIN_FLOAT_BITS, MAX_FLOAT_BITS} range (guaranteed by the
        // fact that min_float/max_float are the normal, finite, the same sign, and
        // not zero).
        //
        // Because of that, the smallest result of `input_bits - MIN_FLOAT_BITS` is 0
        // (when `input_bits` is `MIN_FLOAT_BITS`), and the largest is `0x067fffff`,
        // (when `input_bits` is `MAX_FLOAT_BITS`). `0x067fffff >> 20` is 0x67, e.g. 103,
        // and thus all possible results are inbounds for the (104 item) table.
        // This is all verified in test code.
        //
        // Note that the compiler can't figure this out on it's own, so the
        // get_unchecked does help some.
        let entry = {
            let i = ((input_bits - MIN_FLOAT_BITS) >> 20) as usize;
            #[cfg(all(not(bench), test))]
            {
                debug_assert!(TO_REC_OETF_U8.get(i).is_some());
            }
            unsafe { *TO_REC_OETF_U8.get_unchecked(i) }
        };

        let bias = (entry >> 16) << 9;
        let scale = entry & 0xffff;

        let t = (input_bits >> 12) & 0xff;
        let res = (bias + scale * t) >> 16;
        #[cfg(all(not(bench), test))]
        {
            debug_assert!(res < 256, "{}", res);
        }
        res as u8
    }
}

impl IntoLinear<f64, u8> for RecOetf {
    #[inline]
    fn into_linear(encoded: u8) -> f64 {
        REC_OETF_U8_TO_F64[encoded as usize]
    }
}

impl FromLinear<f64, u8> for RecOetf {
    #[inline]
    fn from_linear(linear: f64) -> u8 {
        RecOetf::from_linear(linear as f32)
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
        use crate::encoding::{rec_standards::RecOetf, FromLinear, IntoLinear};

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

        #[test]
        fn test_u8_f32_into_impl() {
            for i in 0..=255u8 {
                let u8_impl: f32 = RecOetf::into_linear(i);
                let f32_impl = RecOetf::into_linear(i as f32 / 255.0);
                assert_relative_eq!(u8_impl, f32_impl, epsilon = 0.0000001);
            }
        }

        #[test]
        fn test_u8_f64_into_impl() {
            for i in 0..=255u8 {
                let u8_impl: f64 = RecOetf::into_linear(i);
                let f64_impl = RecOetf::into_linear(i as f64 / 255.0);
                assert_relative_eq!(u8_impl, f64_impl, epsilon = 0.0000001);
            }
        }

        #[test]
        fn u8_to_f32_to_u8() {
            for expected in 0u8..=255u8 {
                let linear: f32 = RecOetf::into_linear(expected);
                let result: u8 = RecOetf::from_linear(linear);
                assert_eq!(result, expected);
            }
        }

        #[test]
        fn u8_to_f64_to_u8() {
            for expected in 0u8..=255u8 {
                let linear: f64 = RecOetf::into_linear(expected);
                let result: u8 = RecOetf::from_linear(linear);
                assert_eq!(result, expected);
            }
        }
    }
}
