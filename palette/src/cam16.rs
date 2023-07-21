//! Types for the CIE CAM16 color appearance model.

use core::{fmt::Debug, marker::PhantomData};

use crate::{
    angle::{RealAngle, SignedAngle},
    bool_mask::{HasBoolMask, LazySelect},
    convert::{FromColorUnclamped, IntoColorUnclamped},
    hues::Cam16Hue,
    num::{
        Abs, Arithmetics, Clamp, ClampAssign, Exp, One, PartialCmp, Powf, Real, Signum, Sqrt,
        Trigonometry, Zero,
    },
    white_point::{self, WhitePoint, D65},
    WithAlpha, Xyz,
};

#[cfg(feature = "approx")]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

mod math;

/// The CIE CAM16 color appearance model.
///
/// It's a set of six technically defined attributes that describe the
/// appearance of a color in an environment, and it's a successor of
/// [CIECAM02](https://en.wikipedia.org/wiki/CIECAM02). Not all attributes are
/// needed to be known to convert _from_ CAM16, since they are correlated and
/// derived from each other. This library provides a separate [`PartialCam16`]
/// to make it easier to specify a minimum attribute set.
#[derive(Debug, WithAlpha, FromColorUnclamped)]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Xyz, Cam16)
)]
pub struct Cam16<Wp, T> {
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

    /// The reference white point, usually inherited from the source/target
    /// color space.
    ///
    /// See also [`Parameters::white_point`] for how it's used in conversion.
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Cam16<Wp, T> {
    fn with_white_point<Wp2>(self) -> Cam16<Wp2, T> {
        let Cam16 {
            lightness,
            chroma,
            hue,
            brightness,
            colorfulness,
            saturation,
            white_point: _,
        } = self;

        Cam16 {
            lightness,
            chroma,
            hue,
            brightness,
            colorfulness,
            saturation,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Clone for Cam16<Wp, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            lightness: self.lightness.clone(),
            chroma: self.chroma.clone(),
            hue: self.hue.clone(),
            brightness: self.brightness.clone(),
            colorfulness: self.colorfulness.clone(),
            saturation: self.saturation.clone(),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Copy for Cam16<Wp, T> where T: Copy {}

impl<Wp, T> crate::Clamp for Cam16<Wp, T>
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
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> crate::ClampAssign for Cam16<Wp, T>
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
    Cam16<Wp>,
    Cam16Hue,
    [lightness, chroma, brightness, colorfulness, saturation]
);

impl<Wp, T> FromColorUnclamped<Cam16<Wp, T>> for Cam16<Wp, T> {
    fn from_color_unclamped(val: Cam16<Wp, T>) -> Self {
        val
    }
}

impl<Wp, T> FromColorUnclamped<Xyz<Wp, T>> for Cam16<Wp, T>
where
    Xyz<Wp, T>: IntoCam16<Wp, T>,
    T: Real,
{
    fn from_color_unclamped(val: Xyz<Wp, T>) -> Self {
        val.into_cam16(Parameters::default())
    }
}

/// A partial version of [`Cam16`] with only one of each kind of parameter.
///
/// This is enough information for converting CAM16 to other color spaces.
#[derive(Debug)]
pub struct PartialCam16<Wp, T> {
    /// The [hue](https://en.wikipedia.org/wiki/Hue) (h) of the color.
    pub hue: Cam16Hue<T>,

    /// The [chroma](https://en.wikipedia.org/wiki/Colorfulness#Chroma),
    /// [colorfulness](https://en.wikipedia.org/wiki/Colorfulness) (M), or
    /// [saturation](https://en.wikipedia.org/wiki/Colorfulness#Saturation) (s)
    /// of the color.
    pub chromaticity: ChromaticityType<T>,

    /// The [lightness](https://en.wikipedia.org/wiki/Lightness) (J) or
    /// [brightness](https://en.wikipedia.org/wiki/Brightness) (Q) of the color.
    pub luminance: LuminanceType<T>,

    /// The reference white point, usually inherited from the source/target
    /// color space.
    ///
    /// See also [`Parameters::white_point`] for how it's used in conversion.
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> PartialCam16<Wp, T> {
    fn with_white_point<Wp2>(self) -> PartialCam16<Wp2, T> {
        let PartialCam16 {
            hue,
            chromaticity,
            luminance,
            white_point: _,
        } = self;

        PartialCam16 {
            hue,
            chromaticity,
            luminance,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Clone for PartialCam16<Wp, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            hue: self.hue.clone(),
            chromaticity: self.chromaticity.clone(),
            luminance: self.luminance.clone(),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Copy for PartialCam16<Wp, T> where T: Copy {}

impl<Wp, T> crate::Clamp for PartialCam16<Wp, T>
where
    T: Clamp + Zero,
{
    fn clamp(self) -> Self {
        Self {
            hue: self.hue,
            chromaticity: match self.chromaticity {
                ChromaticityType::Chroma(chroma) => {
                    ChromaticityType::Chroma(chroma.clamp_min(T::zero()))
                }
                ChromaticityType::Colorfulness(colorfulness) => {
                    ChromaticityType::Colorfulness(colorfulness.clamp_min(T::zero()))
                }
                ChromaticityType::Saturation(saturation) => {
                    ChromaticityType::Saturation(saturation.clamp_min(T::zero()))
                }
            },
            luminance: match self.luminance {
                LuminanceType::Lightness(lightness) => {
                    LuminanceType::Lightness(lightness.clamp_min(T::zero()))
                }
                LuminanceType::Brightness(brightness) => {
                    LuminanceType::Brightness(brightness.clamp_min(T::zero()))
                }
            },
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> crate::ClampAssign for PartialCam16<Wp, T>
where
    T: ClampAssign + Zero,
{
    fn clamp_assign(&mut self) {
        match &mut self.chromaticity {
            ChromaticityType::Chroma(chroma) => chroma.clamp_min_assign(T::zero()),
            ChromaticityType::Colorfulness(colorfulness) => {
                colorfulness.clamp_min_assign(T::zero())
            }
            ChromaticityType::Saturation(saturation) => saturation.clamp_min_assign(T::zero()),
        }
        match &mut self.luminance {
            LuminanceType::Lightness(lightness) => lightness.clamp_min_assign(T::zero()),
            LuminanceType::Brightness(brightness) => brightness.clamp_min_assign(T::zero()),
        }
    }
}

impl<Wp, T> From<Cam16<Wp, T>> for PartialCam16<Wp, T> {
    fn from(value: Cam16<Wp, T>) -> Self {
        let Cam16 {
            lightness,
            chroma,
            hue,
            white_point,
            ..
        } = value;

        PartialCam16 {
            hue,
            chromaticity: ChromaticityType::Chroma(chroma),
            luminance: LuminanceType::Lightness(lightness),
            white_point,
        }
    }
}

/// One of the apparent chromatic intensity metrics of CAM16.
///
/// Combined with the hue and one of [`LuminanceType`], it can describe a
/// complete color as [`PartialCam16`].
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum ChromaticityType<T> {
    /// The [chroma](https://en.wikipedia.org/wiki/Colorfulness#Chroma) of the
    /// color.
    #[doc(alias = "C")]
    Chroma(T),

    /// The [colorfulness](https://en.wikipedia.org/wiki/Colorfulness) (M) of
    /// the color.
    #[doc(alias = "M")]
    Colorfulness(T),

    /// The [saturation](https://en.wikipedia.org/wiki/Colorfulness#Saturation)
    /// (s) of the color.
    #[doc(alias = "s")]
    Saturation(T),
}

/// One of the apparent luminance metrics of CAM16.
///
/// Combined with the hue and one of [`ChromaticityType`], it can describe a
/// complete color as [`PartialCam16`].
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum LuminanceType<T> {
    /// The [lightness](https://en.wikipedia.org/wiki/Lightness) (J) of the
    /// color.
    #[doc(alias = "J")]
    Lightness(T),

    /// The [brightness](https://en.wikipedia.org/wiki/Brightness) (Q) of the
    /// color.
    #[doc(alias = "Q")]
    Brightness(T),
}

/// Converts a color to CAM16, using a set of parameters.
pub trait IntoCam16<Wp, T> {
    /// Convert `self` into CAM16, with `parameters` that describe the viewing
    /// conditions.
    fn into_cam16(self, parameters: Parameters<Wp, T>) -> Cam16<Wp, T>;
}

impl<C, Wp, T> IntoCam16<Wp, T> for C
where
    C: IntoColorUnclamped<Xyz<Wp, T>>,
    T: Real
        + One
        + Zero
        + Clamp
        + PartialCmp
        + Arithmetics
        + Powf
        + Sqrt
        + Exp
        + Abs
        + Signum
        + Trigonometry
        + RealAngle
        + HasBoolMask
        + Clone,
    T::Mask: LazySelect<T>,
    Wp: WhitePoint<T>,
{
    fn into_cam16(self, parameters: Parameters<Wp, T>) -> Cam16<Wp, T> {
        math::xyz_to_cam16(
            self.into_color_unclamped().with_white_point(),
            parameters.into_any_white_point(),
        )
        .with_white_point()
    }
}

/// Converts CAM16 to a color, using a set of parameters.
pub trait FromCam16<Wp, T> {
    /// Convert `cam16` into `Self`, with `parameters` that describe the viewing
    /// conditions.
    fn from_cam16(cam16: PartialCam16<Wp, T>, parameters: Parameters<Wp, T>) -> Self;
}

impl<C, Wp, T> FromCam16<Wp, T> for C
where
    T: Real
        + One
        + Zero
        + Clamp
        + Sqrt
        + Powf
        + Exp
        + Abs
        + Signum
        + Arithmetics
        + Trigonometry
        + RealAngle
        + SignedAngle
        + PartialCmp
        + Clone,
    T::Mask: LazySelect<T> + LazySelect<Xyz<white_point::Any, T>>,
    Xyz<Wp, T>: IntoColorUnclamped<C>,
    Wp: WhitePoint<T>,
{
    fn from_cam16(cam16: PartialCam16<Wp, T>, parameters: Parameters<Wp, T>) -> Self {
        math::cam16_to_xyz(cam16.with_white_point(), parameters.into_any_white_point())
            .with_white_point()
            .into_color_unclamped()
    }
}

/// Parameters for CAM16.
///
/// These parameters describe the viewing conditions for a more accurate color
/// appearance metric. The default values are used in [`FromColor`],
/// [`IntoColor`][crate::IntoColor], etc.
///
/// See also Moroney (2000) [Usage Guidelines for CIECAM97s][moroney_2000] for more
/// information and advice on how to customize these parameters.
///
/// [moroney_2000]: https://www.imaging.org/common/uploaded%20files/pdfs/Papers/2000/PICS-0-81/1611.pdf
#[non_exhaustive]
pub struct Parameters<Wp, T> {
    /// The reference white point. Defaults to `Wp` when it implements
    /// [`WhitePoint`], or [`D65`] when `Wp` is [`white_point::Any`]. It can
    /// also be set to a custom value if `Wp` results in the wrong white point.
    pub white_point: WhitePointParameter<Wp, T>,

    /// The average luminance of the environment (*L<sub>A</sub>*) in
    /// *cd/m<sup>2</sup>* (nits). Under a “gray world” assumption this is 20%
    /// of the luminance of a white reference. Defaults to `40`.
    pub adapting_luminance: T,

    /// The relative luminance of the nearby background (*Y<sub>b</sub>*), out
    /// to 10°, on a scale of 0 to 100. Defaults to `20` (medium gray).
    pub background_luminance: T,

    /// A description of the peripheral area, with a value from `0` to `2`. Any
    /// value outside that range will be clamped to `0` or `2`. It has presets
    /// for "dark", "dim" and "average". Defaults to "average" (`2`).
    pub surround: Surround<T>,

    /// Set to `true` to assume that the observer's eyes have fully adapted to
    /// the illuminant. The degree of discounting will be set based on the other
    /// parameters. Defaults to `false`.
    pub discounting: bool,
}

impl<Wp, T> Parameters<Wp, T>
where
    Wp: WhitePoint<T>,
{
    fn into_any_white_point(self) -> Parameters<white_point::Any, T> {
        Parameters {
            white_point: WhitePointParameter::Custom(self.white_point.into_xyz()),
            adapting_luminance: self.adapting_luminance,
            background_luminance: self.background_luminance,
            surround: self.surround,
            discounting: self.discounting,
        }
    }
}

impl<Wp, T> Parameters<Wp, T>
where
    T: Real,
{
    /// Create a new set of default parameters.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Wp, T> Default for Parameters<Wp, T>
where
    T: Real,
{
    fn default() -> Self {
        Self {
            white_point: WhitePointParameter::Default,
            adapting_luminance: T::from_f64(40.0),
            background_luminance: T::from_f64(20.0),
            surround: Surround::Average,
            discounting: false,
        }
    }
}

/// A description of the peripheral area.
#[non_exhaustive]
pub enum Surround<T> {
    /// Represents a dark room, such as a movie theatre. Corresponds to a
    /// surround value of `0`.
    Dark,

    /// Represents a dimly lit room with a bright TV or monitor. Corresponds to
    /// a surround value of `1`.
    Dim,

    /// Represents a surface color. Corresponds to a surround value of `2`.
    Average,

    /// Any custom value from `0` to `2`. Any value outside that range will be
    /// clamped to either `0` or `2`.
    Custom(T),
}

impl<T> Surround<T> {
    fn into_value(self) -> T
    where
        T: Real + Clamp,
    {
        match self {
            Surround::Dark => T::from_f64(0.0),
            Surround::Dim => T::from_f64(1.0),
            Surround::Average => T::from_f64(2.0),
            Surround::Custom(value) => value.clamp(T::from_f64(0.0), T::from_f64(2.0)),
        }
    }
}

/// A parameter value for the reference white point.
#[non_exhaustive]
pub enum WhitePointParameter<Wp, T> {
    /// Represents the value of `Wp`.
    Default,

    /// Represents any custom white point. The `Wp` parameter isn't used in this
    /// case, but still included for Rust to accept an empty `Default`. See
    /// [`Xyz::with_white_point`] for how to change the reference white point of
    /// an `Xyz` value without changing its numerical value.
    Custom(Xyz<Wp, T>),
}

impl<Wp, T> WhitePointParameter<Wp, T>
where
    Wp: WhitePoint<T>,
{
    fn into_xyz(self) -> Xyz<white_point::Any, T> {
        match self {
            WhitePointParameter::Default => Wp::get_xyz(),
            WhitePointParameter::Custom(xyz) => xyz.with_white_point(),
        }
    }
}

impl<T> WhitePointParameter<white_point::Any, T>
where
    T: Real,
{
    fn any_into_xyz(self) -> Xyz<white_point::Any, T> {
        match self {
            WhitePointParameter::Default => D65::get_xyz(),
            WhitePointParameter::Custom(xyz) => xyz,
        }
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
