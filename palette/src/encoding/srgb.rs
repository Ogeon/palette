//! The sRGB standard.

use crate::{
    bool_mask::LazySelect,
    encoding::{FromLinear, IntoLinear},
    luma::LumaStandard,
    num::{Arithmetics, MulAdd, MulSub, PartialCmp, Powf, Real},
    rgb::{Primaries, RgbSpace, RgbStandard},
    white_point::{Any, D65},
    Mat3, Yxy,
};

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
///   space. This uses lookup tables with precomputed values provided that the
///   `float_lut` feature (enabled by default) is being used.
/// * When converting from `f32` or `f64` to `u8`, while converting from linear
///   space if the `fast_uint_lut` feature (enabled by default) is being used.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Srgb;

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
        use crate::encoding::{FromLinear, IntoLinear, Srgb};

        #[test]
        #[cfg(feature = "approx")]
        fn test_u8_f32_into_impl() {
            for i in 0..=255u8 {
                let u8_impl: f32 = Srgb::into_linear(i);
                let f32_impl = Srgb::into_linear(i as f32 / 255.0);
                assert_relative_eq!(u8_impl, f32_impl, epsilon = 0.000001);
            }
        }

        #[test]
        #[cfg(feature = "approx")]
        fn test_u8_f64_into_impl() {
            for i in 0..=255u8 {
                let u8_impl: f64 = Srgb::into_linear(i);
                let f64_impl = Srgb::into_linear(i as f64 / 255.0);
                assert_relative_eq!(u8_impl, f64_impl, epsilon = 0.0000001);
            }
        }

        #[test]
        fn u8_to_f32_to_u8() {
            for expected in 0..=255u8 {
                let linear: f32 = Srgb::into_linear(expected);
                let result: u8 = Srgb::from_linear(linear);
                assert_eq!(result, expected);
            }
        }

        #[test]
        fn u8_to_f64_to_u8() {
            for expected in 0..=255u8 {
                let linear: f64 = Srgb::into_linear(expected);
                let result: u8 = Srgb::from_linear(linear);
                assert_eq!(result, expected);
            }
        }
    }
}
