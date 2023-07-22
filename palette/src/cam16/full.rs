use core::marker::PhantomData;

use crate::{
    convert::FromColorUnclamped,
    hues::Cam16Hue,
    num::{Clamp, ClampAssign, Zero},
    Xyz,
};

use super::{BakedParameters, IntoCam16};

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
    /// See also [`Parameters::white_point`][super::Parameters::white_point] for
    /// how it's used in conversion.
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Cam16<Wp, T> {
    pub(super) fn with_white_point<Wp2>(self) -> Cam16<Wp2, T> {
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
    BakedParameters<Wp, T>: Default,
{
    fn from_color_unclamped(val: Xyz<Wp, T>) -> Self {
        val.into_cam16(BakedParameters::default())
    }
}
