//! The Adobe RGB (1998) standard.

use crate::{
    encoding::gamma::GammaFn,
    luma::LumaStandard,
    num::Real,
    rgb::{Primaries, RgbSpace, RgbStandard},
    white_point::{Any, D65},
    Mat3, Yxy,
};

/// The Adobe RGB (1998) (a.k.a. opRGB) color space and standard.
///
/// This color space was designed to encompass most colors achievable by CMYK
/// printers using RGB primaries. It has a wider color gamut than sRGB, primarily
/// in cyan-green hues.
///
/// The Adobe RGB standard uses a gamma 2.2 transfer function.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AdobeRgb;

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
    type TransferFn = GammaFn;
}

impl LumaStandard for AdobeRgb {
    type WhitePoint = D65;
    type TransferFn = GammaFn;
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
}
