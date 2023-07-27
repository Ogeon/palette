use core::marker::PhantomData;

use crate::{angle::RealAngle, convert::FromColorUnclamped, num::Trigonometry};

use super::Cam16UcsJmh;

/// The Cartesian form of CAM16-UCS, or J'a'b'.
#[derive(Debug, WithAlpha, FromColorUnclamped)]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Cam16UcsJmh, Cam16UcsJab)
)]
pub struct Cam16UcsJab<Wp, T> {
    /// The [lightness](https://en.wikipedia.org/wiki/Lightness) (J') of the color.
    pub lightness: T,

    /// The redness/greenness (a') of the color.
    pub a: T,

    /// The yellowness/blueness (b') of the color.
    pub b: T,

    /// The reference white point, usually inherited from the source/target
    /// color space.
    ///
    /// See also [`Parameters::white_point`][super::Parameters::white_point] for
    /// how it's used in conversion.
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> FromColorUnclamped<Cam16UcsJmh<Wp, T>> for Cam16UcsJab<Wp, T>
where
    T: RealAngle + Trigonometry,
{
    fn from_color_unclamped(val: Cam16UcsJmh<Wp, T>) -> Self {
        let (a, b) = val.hue.into_cartesian();

        Self {
            lightness: val.lightness,
            a,
            b,
            white_point: PhantomData,
        }
    }
}
