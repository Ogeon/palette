use crate::{
    convert::FromColorUnclamped,
    hues::Cam16Hue,
    num::{Clamp, ClampAssign, Zero},
    white_point::WhitePoint,
    Xyz,
};

use super::{BakedParameters, IntoCam16, StaticWp};

#[cfg(feature = "approx")]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

/// The CIE CAM16 color appearance model.
///
/// It's a set of six technically defined attributes that describe the
/// appearance of a color under certain viewing conditions, and it's a successor
/// of [CIECAM02](https://en.wikipedia.org/wiki/CIECAM02). The viewing
/// conditions are defined using [`Parameters`][super::Parameters] and two set
/// of `Cam16` attributes are only really comparable if they were calculated
/// from the same set of viewing condition parameters. The implementations of
/// [`FromColor`][crate::FromColor], [`IntoColor`][crate::IntoColor], etc. use
/// `Parameters::default()` as their viewing conditions. See
/// [`FromCam16`][super::FromCam16] and [`IntoCam16`] for options with more
/// control over the parameters.
///
/// Not all attributes are needed to be known to convert _from_ CAM16, since
/// they are correlated and derived from each other. This library provides a
/// separate [`PartialCam16`][super::PartialCam16] to make it easier to specify
/// a minimum attribute set.
#[derive(Clone, Copy, Debug, WithAlpha, FromColorUnclamped)]
#[palette(palette_internal, component = "T", skip_derives(Xyz, Cam16))]
pub struct Cam16<T> {
    /// The [lightness](https://en.wikipedia.org/wiki/Lightness) (J) of the color.
    #[doc(alias = "J")]
    pub lightness: T,

    /// The [chroma](https://en.wikipedia.org/wiki/Colorfulness#Chroma) (C) of the color.
    #[doc(alias = "C")]
    pub chroma: T,

    /// The [hue](https://en.wikipedia.org/wiki/Hue) (h) of the color.
    #[doc(alias = "h")]
    pub hue: Cam16Hue<T>,

    /// The [brightness](https://en.wikipedia.org/wiki/Brightness) (Q) of the color.
    #[doc(alias = "Q")]
    pub brightness: T,

    /// The [colorfulness](https://en.wikipedia.org/wiki/Colorfulness) (M) of the color.
    #[doc(alias = "M")]
    pub colorfulness: T,

    /// The [saturation](https://en.wikipedia.org/wiki/Colorfulness#Saturation) (s) of the color.
    #[doc(alias = "s")]
    pub saturation: T,
}

impl<T> crate::Clamp for Cam16<T>
where
    T: Clamp + Zero,
{
    fn clamp(self) -> Self {
        Self {
            lightness: self.lightness.clamp_min(T::zero()),
            chroma: self.chroma.clamp_min(T::zero()),
            hue: self.hue,
            brightness: self.brightness.clamp_min(T::zero()),
            colorfulness: self.colorfulness.clamp_min(T::zero()),
            saturation: self.saturation.clamp_min(T::zero()),
        }
    }
}

impl<T> crate::ClampAssign for Cam16<T>
where
    T: ClampAssign + Zero,
{
    fn clamp_assign(&mut self) {
        self.lightness.clamp_min_assign(T::zero());
        self.chroma.clamp_min_assign(T::zero());
        self.brightness.clamp_min_assign(T::zero());
        self.colorfulness.clamp_min_assign(T::zero());
        self.saturation.clamp_min_assign(T::zero());
    }
}

impl_eq_hue!(
    Cam16,
    Cam16Hue,
    [lightness, chroma, brightness, colorfulness, saturation]
);

impl<T> FromColorUnclamped<Cam16<T>> for Cam16<T> {
    fn from_color_unclamped(val: Cam16<T>) -> Self {
        val
    }
}

impl<Wp, T> FromColorUnclamped<Xyz<Wp, T>> for Cam16<T>
where
    Wp: WhitePoint<T>,
    Xyz<Wp, T>: IntoCam16<StaticWp<Wp>, T>,
    BakedParameters<StaticWp<Wp>, T>: Default,
{
    fn from_color_unclamped(val: Xyz<Wp, T>) -> Self {
        val.into_cam16(BakedParameters::default())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        cam16::{ChromaticityType, LuminanceType, PartialCam16},
        convert::{FromColorUnclamped, IntoColorUnclamped},
        Srgb,
    };

    use super::Cam16;

    macro_rules! assert_cam16_to_rgb {
        ($cam16:expr, $rgb:expr, $($params:tt)*) => {
            let cam16 = $cam16;

            let rgb: Srgb<f64> = cam16.into_color_unclamped();
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

            for luminance in luminances {
                for chromaticity in chromaticities {
                    let partial = PartialCam16 {
                        hue: cam16.hue,
                        chromaticity,
                        luminance,
                    };
                    assert_relative_eq!(
                        Srgb::<f64>::from_color_unclamped(dbg!(partial)),
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
        let mut cam16: Cam16<f64> = Srgb::from(0x5588cc).into_linear().into_color_unclamped();
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
        let mut cam16: Cam16<f64> = Srgb::from(0x000000).into_linear().into_color_unclamped();
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
        let mut cam16: Cam16<f64> = Srgb::from(0xffffff).into_linear().into_color_unclamped();
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
        let mut cam16: Cam16<f64> = Srgb::from(0xff0000).into_linear().into_color_unclamped();
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
            },
            epsilon = 0.01
        );

        assert_cam16_to_rgb!(cam16, Srgb::from(0xff0000).into_format(), epsilon = 0.00001);
    }

    #[test]
    fn green() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let mut cam16: Cam16<f64> = Srgb::from(0x00ff00).into_linear().into_color_unclamped();
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
        let mut cam16: Cam16<f64> = Srgb::from(0x0000ff).into_linear().into_color_unclamped();
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
