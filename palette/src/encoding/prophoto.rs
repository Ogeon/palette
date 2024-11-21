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
            T::from_f64(0.28804),
        )
    }
    fn green() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.1596),
            T::from_f64(0.8404),
            T::from_f64(0.71187),
        )
    }
    fn blue() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.0366),
            T::from_f64(0.0001),
            T::from_f64(0.000085663),
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

#[cfg(test)]
mod test {
    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{
            convert::IntoColorUnclamped,
            encoding::ProPhotoRgb,
            matrix::{matrix_inverse, rgb_to_xyz_matrix},
            rgb::{Primaries, RgbSpace},
            white_point::{Any, WhitePoint, D50},
            Xyz,
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

        #[test]
        fn primaries_prophoto() {
            let red: Xyz<Any, f64> = ProPhotoRgb::red().into_color_unclamped();
            let green: Xyz<Any, f64> = ProPhotoRgb::green().into_color_unclamped();
            let blue: Xyz<Any, f64> = ProPhotoRgb::blue().into_color_unclamped();
            // Compare sum of primaries to white point.
            assert_relative_eq!(red + green + blue, D50::get_xyz(), epsilon = 0.0001);
        }
    }

    #[cfg(feature = "approx")]
    mod transfer {
        use crate::encoding::{FromLinear, IntoLinear, ProPhotoRgb};

        #[test]
        fn lin_to_enc_to_lin() {
            for i in 0..=100 {
                let linear = i as f64 / 100.0;
                let encoded: f64 = ProPhotoRgb::from_linear(linear);
                assert_relative_eq!(
                    linear,
                    ProPhotoRgb::into_linear(encoded),
                    epsilon = 0.0000001
                );
            }
        }

        #[test]
        fn enc_to_lin_to_enc() {
            for i in 0..=100 {
                let encoded = i as f64 / 100.0;
                let linear: f64 = ProPhotoRgb::into_linear(encoded);
                assert_relative_eq!(
                    encoded,
                    ProPhotoRgb::from_linear(linear),
                    epsilon = 0.0000001
                );
            }
        }
    }

    #[cfg(feature = "gamma_lut_u16")]
    mod lut {
        use crate::encoding::{FromLinear, IntoLinear, ProPhotoRgb};

        #[test]
        #[cfg(feature = "approx")]
        fn test_u16_f32_into_impl() {
            for i in 0..=65535u16 {
                let u16_impl: f32 = ProPhotoRgb::into_linear(i);
                let f32_impl = ProPhotoRgb::into_linear(i as f32 / 65535.0);
                assert_relative_eq!(u16_impl, f32_impl, epsilon = 0.000001);
            }
        }

        #[test]
        #[cfg(feature = "approx")]
        fn test_u16_f64_into_impl() {
            for i in 0..=65535u16 {
                let u16_impl: f64 = ProPhotoRgb::into_linear(i);
                let f64_impl = ProPhotoRgb::into_linear(i as f64 / 65535.0);
                assert_relative_eq!(u16_impl, f64_impl, epsilon = 0.0000001);
            }
        }

        #[test]
        fn u16_to_f32_to_u16() {
            for expected in 0..=65535u16 {
                let linear: f32 = ProPhotoRgb::into_linear(expected);
                let result: u16 = ProPhotoRgb::from_linear(linear);
                assert_eq!(result, expected);
            }
        }

        #[test]
        fn u16_to_f64_to_u16() {
            for expected in 0..=65535u16 {
                let linear: f64 = ProPhotoRgb::into_linear(expected);
                let result: u16 = ProPhotoRgb::from_linear(linear);
                assert_eq!(result, expected);
            }
        }
    }
}
