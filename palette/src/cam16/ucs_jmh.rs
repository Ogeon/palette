use core::marker::PhantomData;

use crate::{
    angle::RealAngle,
    convert::FromColorUnclamped,
    hues::Cam16Hue,
    num::{Arithmetics, Hypot, Ln, One, Real, Trigonometry},
};

use super::{Cam16UcsJab, Colorfulness, Lightness, PartialCam16Jmh};

/// The polar form of CAM16-UCS, or J'M'h'.
#[derive(Debug, WithAlpha, FromColorUnclamped)]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    cam16_chromaticity = "Colorfulness<T>",
    cam16_luminance = "Lightness<T>",
    skip_derives(PartialCam16, Cam16UcsJmh, Cam16UcsJab)
)]
pub struct Cam16UcsJmh<Wp, T> {
    /// The [lightness](https://en.wikipedia.org/wiki/Lightness) (J') of the color.
    pub lightness: T,

    /// The [colorfulness](https://en.wikipedia.org/wiki/Colorfulness) (M') of the color.
    pub colorfulness: T,

    /// The [hue](https://en.wikipedia.org/wiki/Hue) (h') of the color.
    pub hue: Cam16Hue<T>,

    /// The reference white point, usually inherited from the source/target
    /// color space.
    ///
    /// See also [`Parameters::white_point`][super::Parameters::white_point] for
    /// how it's used in conversion.
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> FromColorUnclamped<Cam16UcsJmh<Wp, T>> for Cam16UcsJmh<Wp, T> {
    fn from_color_unclamped(val: Cam16UcsJmh<Wp, T>) -> Self {
        val
    }
}

impl<Wp, T> FromColorUnclamped<PartialCam16Jmh<Wp, T>> for Cam16UcsJmh<Wp, T>
where
    T: Real + One + Ln + Arithmetics,
{
    fn from_color_unclamped(val: PartialCam16Jmh<Wp, T>) -> Self {
        let colorfulness =
            (T::one() + T::from_f64(0.0228) * val.chromaticity.0).ln() / T::from_f64(0.0228);
        let lightness =
            T::from_f64(1.7) * &val.luminance.0 / (T::one() + T::from_f64(0.007) * val.luminance.0);

        Cam16UcsJmh {
            lightness,
            colorfulness,
            hue: val.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> FromColorUnclamped<Cam16UcsJab<Wp, T>> for Cam16UcsJmh<Wp, T>
where
    T: RealAngle + Hypot + Trigonometry + Arithmetics + Clone,
{
    fn from_color_unclamped(val: Cam16UcsJab<Wp, T>) -> Self {
        Self {
            lightness: val.lightness,
            colorfulness: val.a.clone().hypot(val.b.clone()),
            hue: Cam16Hue::from_cartesian(val.a, val.b),
            white_point: PhantomData,
        }
    }
}
