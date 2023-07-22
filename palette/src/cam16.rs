//! Types for the CIE CAM16 color appearance model.

use crate::{
    angle::{RealAngle, SignedAngle},
    bool_mask::LazySelect,
    convert::IntoColorUnclamped,
    num::{Abs, Arithmetics, One, PartialCmp, Powf, Real, Signum, Sqrt, Trigonometry, Zero},
    white_point::{self},
    Xyz,
};

pub use full::*;
pub use parameters::*;
pub use partial::*;

mod full;
mod math;
mod parameters;
mod partial;

/// Converts a color to CAM16, using a set of parameters.
pub trait IntoCam16<Wp, T> {
    /// Convert `self` into CAM16, with `parameters` that describe the viewing
    /// conditions.
    fn into_cam16(self, parameters: BakedParameters<Wp, T>) -> Cam16<Wp, T>;
}

impl<C, Wp, T> IntoCam16<Wp, T> for C
where
    C: IntoColorUnclamped<Xyz<Wp, T>>,
    T: Real + Arithmetics + Powf + Sqrt + Abs + Signum + Trigonometry + RealAngle + Clone,
{
    fn into_cam16(self, parameters: BakedParameters<Wp, T>) -> Cam16<Wp, T> {
        math::xyz_to_cam16(
            self.into_color_unclamped().with_white_point(),
            parameters.inner,
        )
        .with_white_point()
    }
}

/// Converts CAM16 to a color, using a set of parameters.
pub trait FromCam16<Wp, T> {
    /// Convert `cam16` into `Self`, with `parameters` that describe the viewing
    /// conditions.
    fn from_cam16(cam16: PartialCam16<Wp, T>, parameters: BakedParameters<Wp, T>) -> Self;
}

impl<C, Wp, T> FromCam16<Wp, T> for C
where
    T: Real
        + One
        + Zero
        + Sqrt
        + Powf
        + Abs
        + Signum
        + Arithmetics
        + Trigonometry
        + RealAngle
        + SignedAngle
        + PartialCmp
        + Clone,
    T::Mask: LazySelect<Xyz<white_point::Any, T>>,
    Xyz<Wp, T>: IntoColorUnclamped<C>,
{
    fn from_cam16(cam16: PartialCam16<Wp, T>, parameters: BakedParameters<Wp, T>) -> Self {
        math::cam16_to_xyz(cam16.with_white_point(), parameters.inner)
            .with_white_point()
            .into_color_unclamped()
    }
}

#[cfg(test)]
mod test {
    use crate::{convert::IntoColor, Srgb};

    use super::{Cam16, ChromaticityType, FromCam16, LuminanceType, PartialCam16};

    macro_rules! assert_cam16_to_rgb {
        ($cam16:expr, $rgb:expr, $($params:tt)*) => {
            let cam16 = $cam16;

            let rgb: Srgb<f64> = cam16.into_color();
            assert_relative_eq!(rgb, $rgb, $($params)*);

            let chromaticities = [
                ChromaticityType::Chroma(cam16.chroma),
                ChromaticityType::Colorfulness(cam16.colorfulness),
                ChromaticityType::Saturation(cam16.saturation),
            ];
            let luminances = [
                LuminanceType::Lightness(cam16.lightness),
                LuminanceType::Brightness(cam16.brightness),
            ];

            for chromaticity in chromaticities {
                for luminance in luminances {
                    let partial = PartialCam16 {
                        hue: cam16.hue,
                        chromaticity,
                        luminance,
                        white_point: cam16.white_point,
                    };
                    assert_relative_eq!(
                        Srgb::<f64>::from_cam16(dbg!(partial), Default::default()),
                        $rgb,
                        $($params)*
                    );
                }
            }
        };
    }

    #[test]
    fn example_blue() {
        // Uses the example color from https://observablehq.com/@jrus/cam16
        let mut cam16: Cam16<_, f64> = Srgb::from(0x5588cc).into_linear().into_color();
        cam16.hue = cam16.hue.into_positive_degrees().into();

        assert_relative_eq!(
            cam16,
            Cam16 {
                lightness: 45.544264720360346,
                chroma: 45.07001048293764,
                hue: 259.225345298129.into(),
                brightness: 132.96974182692045,
                colorfulness: 39.4130607870103,
                saturation: 54.4432031413259,
                white_point: core::marker::PhantomData
            },
            epsilon = 0.01
        );

        assert_cam16_to_rgb!(
            cam16,
            Srgb::from(0x5588cc).into_format(),
            epsilon = 0.0000001
        );
    }

    #[test]
    fn black() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let mut cam16: Cam16<_, f64> = Srgb::from(0x000000).into_linear().into_color();
        cam16.hue = cam16.hue.into_positive_degrees().into();

        assert_relative_eq!(
            cam16,
            Cam16 {
                lightness: 0.0,
                chroma: 0.0,
                hue: 0.0.into(),
                brightness: 0.0,
                colorfulness: 0.0,
                saturation: 0.0,
                white_point: core::marker::PhantomData
            },
            epsilon = 0.01
        );

        assert_cam16_to_rgb!(
            cam16,
            Srgb::from(0x000000).into_format(),
            epsilon = 0.0000001
        );
    }

    #[test]
    fn white() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let mut cam16: Cam16<_, f64> = Srgb::from(0xffffff).into_linear().into_color();
        cam16.hue = cam16.hue.into_positive_degrees().into();

        assert_relative_eq!(
            cam16,
            Cam16 {
                lightness: 99.99955537650459,
                chroma: 2.1815254387079435,
                hue: 209.49854407518228.into(),
                brightness: 197.03120459014184,
                colorfulness: 1.9077118865271965,
                saturation: 9.839859256901553,
                white_point: core::marker::PhantomData
            },
            epsilon = 0.1
        );

        assert_cam16_to_rgb!(
            cam16,
            Srgb::from(0xffffff).into_format(),
            epsilon = 0.0000001
        );
    }

    #[test]
    fn red() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let mut cam16: Cam16<_, f64> = Srgb::from(0xff0000).into_linear().into_color();
        cam16.hue = cam16.hue.into_positive_degrees().into();

        assert_relative_eq!(
            cam16,
            Cam16 {
                lightness: 46.23623443823762,
                chroma: 113.27879472174797,
                hue: 27.412485587695937.into(),
                brightness: 133.9760614641257,
                colorfulness: 99.06063864657237,
                saturation: 85.98782392745971,
                white_point: core::marker::PhantomData
            },
            epsilon = 0.01
        );

        assert_cam16_to_rgb!(cam16, Srgb::from(0xff0000).into_format(), epsilon = 0.00001);
    }

    #[test]
    fn green() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let mut cam16: Cam16<_, f64> = Srgb::from(0x00ff00).into_linear().into_color();
        cam16.hue = cam16.hue.into_positive_degrees().into();

        assert_relative_eq!(
            cam16,
            Cam16 {
                lightness: 79.23121430933533,
                chroma: 107.77869525794452,
                hue: 141.93451307926003.into(),
                brightness: 175.38164288466993,
                colorfulness: 94.25088262080988,
                saturation: 73.30787758114869,
                white_point: core::marker::PhantomData
            },
            epsilon = 0.01
        );

        assert_cam16_to_rgb!(
            cam16,
            Srgb::from(0x00ff00).into_format(),
            epsilon = 0.000001
        );
    }

    #[test]
    fn blue() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let mut cam16: Cam16<_, f64> = Srgb::from(0x0000ff).into_linear().into_color();
        cam16.hue = cam16.hue.into_positive_degrees().into();

        assert_relative_eq!(
            cam16,
            Cam16 {
                lightness: 25.22701796474445,
                chroma: 86.59618504567312,
                hue: 282.81848901862566.into(),
                brightness: 98.96210767195342,
                colorfulness: 75.72708922311855,
                saturation: 87.47645277637828,
                white_point: core::marker::PhantomData
            },
            epsilon = 0.01
        );

        assert_cam16_to_rgb!(
            cam16,
            Srgb::from(0x0000ff).into_format(),
            epsilon = 0.000001
        );
    }
}
