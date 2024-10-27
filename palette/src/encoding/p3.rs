//! The P3 color space(s) and standards.

use core::marker::PhantomData;

use crate::{
    encoding::{FromLinear, IntoLinear, Srgb},
    luma::LumaStandard,
    num::{Powf, Real},
    rgb::{Primaries, RgbSpace, RgbStandard},
    white_point::{Any, WhitePoint, D65},
    Mat3, Xyz, Yxy,
};

/// The theatrical DCI-P3 standard.
///
/// This standard uses a gamma 2.6 transfer function and a white point of ~6300K that
/// matches the color of xenon bulbs used in theater projectors
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DciP3;

impl<T: Real> Primaries<T> for DciP3 {
    fn red() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.680),
            T::from_f64(0.320),
            T::from_f64(0.209492),
        )
    }
    fn green() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.265),
            T::from_f64(0.690),
            T::from_f64(0.721595),
        )
    }
    fn blue() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.150),
            T::from_f64(0.060),
            T::from_f64(0.068913),
        )
    }
}

impl<T: Real> WhitePoint<T> for DciP3 {
    fn get_xyz() -> Xyz<Any, T> {
        Xyz::new(
            T::from_f64(0.314 / 0.351),
            T::from_f64(1.0),
            T::from_f64(0.335 / 0.351),
        )
    }
}

impl RgbSpace for DciP3 {
    type Primaries = DciP3;
    type WhitePoint = DciP3;

    #[rustfmt::skip]
    #[inline(always)]
    fn rgb_to_xyz_matrix() -> Option<Mat3<f64>> {
        // Matrix calculated using https://www.russellcottrell.com/photo/matrixCalculator.htm
        Some([
            0.4451698, 0.2771344, 0.1722827,
            0.2094917, 0.7215953, 0.0689131,
            0.0000000, 0.0470606, 0.9073554,
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn xyz_to_rgb_matrix() -> Option<Mat3<f64>> {
        // Matrix calculated using https://www.russellcottrell.com/photo/matrixCalculator.htm
        Some([
             2.7253940, -1.0180030, -0.4401632,
            -0.7951680,  1.6897321,  0.0226472,
             0.0412419, -0.0876390,  1.1009294,
        ])
    }
}

impl RgbStandard for DciP3 {
    type Space = DciP3;
    type TransferFn = P3Gamma;
}

impl LumaStandard for DciP3 {
    type WhitePoint = DciP3;
    type TransferFn = P3Gamma;
}

/// The Canon DCI-P3+ color space and standard.
///
/// This standard has the same white point as [`DciP3`], but has a much wider gamut and
/// no standardized transfer function (left to user preference). The generic `F` in
/// this struct represents the chosen transfer function.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DciP3Plus<F>(PhantomData<F>);

impl<T: Real, F> Primaries<T> for DciP3Plus<F> {
    fn red() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.740),
            T::from_f64(0.270),
            T::from_f64(0.203986),
        )
    }
    fn green() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.220),
            T::from_f64(0.780),
            T::from_f64(0.882591),
        )
    }
    fn blue() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.090),
            T::from_f64(-0.090),
            T::from_f64(-0.086577),
        )
    }
}

impl<F> RgbSpace for DciP3Plus<F> {
    type Primaries = DciP3Plus<F>;
    type WhitePoint = DciP3;

    #[rustfmt::skip]
    #[inline(always)]
    fn rgb_to_xyz_matrix() -> Option<Mat3<f64>> {
        // Matrix calculated using https://www.russellcottrell.com/photo/matrixCalculator.htm
        Some([
            0.5590736, 0.2489359,  0.0865774,
            0.2039863, 0.8825911, -0.0865774,
           -0.0075550, 0.0000000,  0.9619710,
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn xyz_to_rgb_matrix() -> Option<Mat3<f64>> {
        // Matrix calculated using https://www.russellcottrell.com/photo/matrixCalculator.htm
        Some([
             1.9904035, -0.5613959, -0.2296619,
            -0.4584928,  1.2623460,  0.1548755,
             0.0156321, -0.0044090,  1.0377287,
        ])
    }
}

impl<F> RgbStandard for DciP3Plus<F> {
    type Space = DciP3Plus<F>;
    type TransferFn = F;
}

impl<F> LumaStandard for DciP3Plus<F> {
    type WhitePoint = DciP3;
    type TransferFn = F;
}

/// A gamma 2.6 transfer function used by some P3 variants
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct P3Gamma;

impl<T> IntoLinear<T, T> for P3Gamma
where
    T: Real + Powf,
{
    #[inline]
    fn into_linear(encoded: T) -> T {
        encoded.powf(T::from_f64(2.6))
    }
}

impl<T> FromLinear<T, T> for P3Gamma
where
    T: Real + Powf,
{
    #[inline]
    fn from_linear(linear: T) -> T {
        linear.powf(T::from_f64(1.0 / 2.6))
    }
}

/// The Display P3 standard.
///
/// This standard uses the same primaries as [`DciP3`] but with a [`D65`] white point
/// and the [`Srgb`] transfer function.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DisplayP3;

impl<T: Real> Primaries<T> for DisplayP3 {
    fn red() -> Yxy<Any, T> {
        Yxy::new(T::from_f64(0.680), T::from_f64(0.320), T::from_f64(0.22900))
    }
    fn green() -> Yxy<Any, T> {
        Yxy::new(T::from_f64(0.265), T::from_f64(0.690), T::from_f64(0.69173))
    }
    fn blue() -> Yxy<Any, T> {
        Yxy::new(T::from_f64(0.150), T::from_f64(0.060), T::from_f64(0.07927))
    }
}
impl RgbSpace for DisplayP3 {
    type Primaries = DisplayP3;
    type WhitePoint = D65;

    #[rustfmt::skip]
    #[inline(always)]
    fn rgb_to_xyz_matrix() -> Option<Mat3<f64>> {
        // Matrix calculated using https://www.russellcottrell.com/photo/matrixCalculator.htm
        Some([
            0.4866327, 0.2656632, 0.1981742,
            0.2290036, 0.6917267, 0.0792697,
            0.0000000, 0.0451126, 1.0437174,
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn xyz_to_rgb_matrix() -> Option<Mat3<f64>> {
        // Matrix calculated using https://www.russellcottrell.com/photo/matrixCalculator.htm
        Some([
             2.4931808, -0.9312655, -0.4026597,
            -0.8295031,  1.7626941,  0.0236251,
             0.0358536, -0.0761890,  0.9570926,
        ])
    }
}

impl RgbStandard for DisplayP3 {
    type Space = DisplayP3;
    type TransferFn = Srgb;
}

impl LumaStandard for DisplayP3 {
    type WhitePoint = D65;
    type TransferFn = Srgb;
}

#[cfg(test)]
mod test {
    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{
            convert::IntoColorUnclamped,
            encoding::p3::{DciP3, DciP3Plus, DisplayP3, P3Gamma},
            matrix::{matrix_inverse, rgb_to_xyz_matrix},
            rgb::{Primaries, RgbSpace},
            white_point::{Any, WhitePoint, D65},
            Xyz,
        };

        #[test]
        fn rgb_to_xyz_display_p3() {
            let dynamic = rgb_to_xyz_matrix::<DisplayP3, f64>();
            let constant = DisplayP3::rgb_to_xyz_matrix().unwrap();
            assert_relative_eq!(dynamic[..], constant[..], epsilon = 0.0000001);
        }

        #[test]
        fn xyz_to_rgb_display_p3() {
            let dynamic = matrix_inverse(rgb_to_xyz_matrix::<DisplayP3, f64>());
            let constant = DisplayP3::xyz_to_rgb_matrix().unwrap();
            assert_relative_eq!(dynamic[..], constant[..], epsilon = 0.0000001);
        }

        #[test]
        fn rgb_to_xyz_dci_p3() {
            let dynamic = rgb_to_xyz_matrix::<DciP3, f64>();
            let constant = DciP3::rgb_to_xyz_matrix().unwrap();
            assert_relative_eq!(dynamic[..], constant[..], epsilon = 0.0000001);
        }

        #[test]
        fn xyz_to_rgb_dci_p3() {
            let dynamic = matrix_inverse(rgb_to_xyz_matrix::<DciP3, f64>());
            let constant = DciP3::xyz_to_rgb_matrix().unwrap();
            assert_relative_eq!(dynamic[..], constant[..], epsilon = 0.0000001);
        }

        #[test]
        fn rgb_to_xyz_dci_p3_plus() {
            let dynamic = rgb_to_xyz_matrix::<DciP3Plus<P3Gamma>, f64>();
            let constant = DciP3Plus::<P3Gamma>::rgb_to_xyz_matrix().unwrap();
            assert_relative_eq!(dynamic[..], constant[..], epsilon = 0.0000001);
        }

        #[test]
        fn xyz_to_rgb_dci_p3_plus() {
            let dynamic = matrix_inverse(rgb_to_xyz_matrix::<DciP3Plus<P3Gamma>, f64>());
            let constant = DciP3Plus::<P3Gamma>::xyz_to_rgb_matrix().unwrap();
            assert_relative_eq!(dynamic[..], constant[..], epsilon = 0.0000001);
        }

        #[test]
        fn primaries_display_p3() {
            let red: Xyz<Any, f64> = DisplayP3::red().into_color_unclamped();
            let green: Xyz<Any, f64> = DisplayP3::green().into_color_unclamped();
            let blue: Xyz<Any, f64> = DisplayP3::blue().into_color_unclamped();
            // Compare sum of primaries to white point.
            assert_relative_eq!(red + green + blue, D65::get_xyz(), epsilon = 0.00001)
        }

        #[test]
        fn primaries_dci_p3() {
            let red: Xyz<Any, f64> = DciP3::red().into_color_unclamped();
            let green: Xyz<Any, f64> = DciP3::green().into_color_unclamped();
            let blue: Xyz<Any, f64> = DciP3::blue().into_color_unclamped();
            // Compare sum of primaries to white point.
            assert_relative_eq!(red + green + blue, DciP3::get_xyz(), epsilon = 0.00001)
        }

        #[test]
        fn primaries_dci_p3_plus() {
            let red: Xyz<Any, f64> = DciP3Plus::<P3Gamma>::red().into_color_unclamped();
            let green: Xyz<Any, f64> = DciP3Plus::<P3Gamma>::green().into_color_unclamped();
            let blue: Xyz<Any, f64> = DciP3Plus::<P3Gamma>::blue().into_color_unclamped();
            // Compare sum of primaries to white point.
            assert_relative_eq!(red + green + blue, DciP3::get_xyz(), epsilon = 0.00001)
        }
    }

    #[cfg(feature = "approx")]
    mod transfer {
        use crate::encoding::{FromLinear, IntoLinear, P3Gamma};

        #[test]
        fn lin_to_enc_to_lin() {
            for i in 0..=100 {
                let linear = i as f64 / 100.0;
                let encoded: f64 = P3Gamma::from_linear(linear);
                assert_relative_eq!(linear, P3Gamma::into_linear(encoded), epsilon = 0.0000001);
            }
        }

        #[test]
        fn enc_to_lin_to_enc() {
            for i in 0..=100 {
                let encoded = i as f64 / 100.0;
                let linear: f64 = P3Gamma::into_linear(encoded);
                assert_relative_eq!(encoded, P3Gamma::from_linear(linear), epsilon = 0.0000001);
            }
        }
    }

    mod lut {
        use crate::encoding::{FromLinear, IntoLinear, P3Gamma};

        #[test]
        #[cfg(feature = "approx")]
        fn test_u8_f32_into_impl() {
            for i in 0..=255u8 {
                let u8_impl: f32 = P3Gamma::into_linear(i);
                let f32_impl = P3Gamma::into_linear(i as f32 / 255.0);
                assert_relative_eq!(u8_impl, f32_impl, epsilon = 0.000001);
            }
        }

        #[test]
        #[cfg(feature = "approx")]
        fn test_u8_f64_into_impl() {
            for i in 0..=255u8 {
                let u8_impl: f64 = P3Gamma::into_linear(i);
                let f64_impl = P3Gamma::into_linear(i as f64 / 255.0);
                assert_relative_eq!(u8_impl, f64_impl, epsilon = 0.0000001);
            }
        }

        #[test]
        fn u8_to_f32_to_u8() {
            for expected in 0u8..=255u8 {
                let linear: f32 = P3Gamma::into_linear(expected);
                let result: u8 = P3Gamma::from_linear(linear);
                assert_eq!(result, expected);
            }
        }

        #[test]
        fn u8_to_f64_to_u8() {
            for expected in 0u8..=255u8 {
                let linear: f64 = P3Gamma::into_linear(expected);
                let result: u8 = P3Gamma::from_linear(linear);
                assert_eq!(result, expected);
            }
        }
    }
}
