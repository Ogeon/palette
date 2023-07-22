use core::marker::PhantomData;

use crate::{
    hues::Cam16Hue,
    num::{Clamp, ClampAssign, Zero},
    Cam16,
};

/// A partial version of [`Cam16`] with only one of each kind of attribute.
///
/// This is enough information for converting CAM16 to other color spaces.
#[derive(Debug)]
pub struct PartialCam16<Wp, T> {
    /// The [hue](https://en.wikipedia.org/wiki/Hue) (h) of the color.
    pub hue: Cam16Hue<T>,

    /// The [chroma](https://en.wikipedia.org/wiki/Colorfulness#Chroma) (C),
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
    /// See also [`Parameters::white_point`][super::Parameters::white_point] for how it's used in conversion.
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> PartialCam16<Wp, T> {
    pub(super) fn with_white_point<Wp2>(self) -> PartialCam16<Wp2, T> {
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
    /// The [chroma](https://en.wikipedia.org/wiki/Colorfulness#Chroma) (C) of the
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
