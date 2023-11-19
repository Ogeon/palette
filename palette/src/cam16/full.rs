use crate::{
    angle::{RealAngle, SignedAngle},
    bool_mask::LazySelect,
    hues::Cam16Hue,
    num::{
        Abs, Arithmetics, Clamp, ClampAssign, One, PartialCmp, Powf, Real, Signum, Sqrt,
        Trigonometry, Zero,
    },
    white_point, Xyz,
};

use super::{
    BakedParameters, Cam16Chromaticity, Cam16Luminance, PartialCam16, WhitePointParameter,
};

/// The CIE CAM16 color appearance model.
///
/// It's a set of six technically defined attributes that describe the
/// appearance of a color under certain viewing conditions, and it's a successor
/// of [CIECAM02](https://en.wikipedia.org/wiki/CIECAM02). The viewing
/// conditions are defined using [`Parameters`][super::Parameters], and two sets
/// of parameters can be used to translate the appearance of a color from one
/// set of viewing conditions to another.
///
/// The use of the viewing conditions parameters sets `Cam16` and its derived
/// types apart from most other color types in this library. It's, for example,
/// not possible to use [`FromColor`][crate::FromColor] and friends to convert
/// to and from other types, since that would require default viewing conditions
/// to exist. Instead, the explicit [`Cam16::from_xyz`] and [`Cam16::into_xyz`]
/// are there to bridge the gap.
///
/// Not all attributes are used when converting _from_ CAM16, since they are
/// correlated and derived from each other. This library provides a separate
/// [`PartialCam16`][super::PartialCam16] to make it easier to correctly specify
/// a minimum attribute set.
#[derive(Clone, Copy, Debug, WithAlpha)]
#[palette(palette_internal, component = "T")]
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

impl<T> Cam16<T> {
    /// Derive CIE CAM16 attributes for the provided color, under the provided
    /// viewing conditions.
    #[inline]
    pub fn from_xyz<WpParam>(
        color: Xyz<WpParam::StaticWp, T>,
        parameters: impl Into<BakedParameters<WpParam, T>>,
    ) -> Self
    where
        WpParam: WhitePointParameter<T>,
        T: Real + Arithmetics + Powf + Sqrt + Abs + Signum + Trigonometry + RealAngle + Clone,
    {
        super::math::xyz_to_cam16(color.with_white_point(), parameters.into().inner)
    }

    /// Construct an XYZ color that matches these CIE CAM16 attributes, under
    /// the provided viewing conditions.
    ///
    /// This assumes that all of the correlated attributes are consistent, as
    /// only some of them are actually used. You may want to use
    /// [`PartialCam16`] for more control over which set of attributes that
    /// should be.
    #[inline]
    pub fn into_xyz<WpParam>(
        self,
        parameters: impl Into<BakedParameters<WpParam, T>>,
    ) -> Xyz<WpParam::StaticWp, T>
    where
        WpParam: WhitePointParameter<T>,
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
    {
        super::math::cam16_to_xyz(self.into(), parameters.into().inner).with_white_point()
    }

    /// Reconstruct a full set of CIE CAM16 attributes, using the original viewing conditions.
    #[inline]
    pub fn from_partial<WpParam, L, C>(
        partial: PartialCam16<T, L, C>,
        parameters: impl Into<BakedParameters<WpParam, T>>,
    ) -> Self
    where
        WpParam: WhitePointParameter<T>,
        T: Real + Zero + Arithmetics + Sqrt + PartialCmp + Clone,
        T::Mask: LazySelect<T> + Clone,
        L: Cam16Luminance<T>,
        C: Cam16Chromaticity<T>,
    {
        partial.into_full(parameters)
    }

    /// Create a partial set of CIE CAM16 attributes.
    ///
    /// It's also possible to use `PartialCam16::from` or `Cam16::into`.
    #[inline]
    pub fn into_partial<L, C>(self) -> PartialCam16<T, L, C>
    where
        L: Cam16Luminance<T>,
        C: Cam16Chromaticity<T>,
    {
        PartialCam16::from_full(self)
    }
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

#[cfg(test)]
mod test {
    use crate::{
        cam16::{ChromaticityType, LuminanceType, Parameters, PartialCam16},
        convert::{FromColorUnclamped, IntoColorUnclamped},
        Srgb,
    };

    use super::Cam16;

    macro_rules! assert_cam16_to_rgb {
        ($cam16:expr, $rgb:expr, $($params:tt)*) => {
            let cam16 = $cam16;
            let parameters = Parameters::TEST_DEFAULTS;

            let rgb: Srgb<f64> = cam16.into_xyz(parameters).into_color_unclamped();
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
                        Srgb::<f64>::from_color_unclamped(dbg!(partial).into_xyz(parameters)),
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
        let xyz = Srgb::from(0x5588cc).into_linear().into_color_unclamped();
        let mut cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
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
        let xyz = Srgb::from(0x000000).into_linear().into_color_unclamped();
        let mut cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
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
        let xyz = Srgb::from(0xffffff).into_linear().into_color_unclamped();
        let mut cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
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
        let xyz = Srgb::from(0xff0000).into_linear().into_color_unclamped();
        let mut cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
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
        let xyz = Srgb::from(0x00ff00).into_linear().into_color_unclamped();
        let mut cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
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
        let xyz = Srgb::from(0x0000ff).into_linear().into_color_unclamped();
        let mut cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
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
