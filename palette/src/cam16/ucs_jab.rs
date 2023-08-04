use crate::{angle::RealAngle, convert::FromColorUnclamped, num::Trigonometry};

use super::Cam16UcsJmh;

/// The Cartesian form of CAM16-UCS, or J'a'b'.
#[derive(Clone, Copy, Debug, WithAlpha, FromColorUnclamped)]
#[palette(
    palette_internal,
    component = "T",
    skip_derives(Cam16UcsJmh, Cam16UcsJab)
)]
pub struct Cam16UcsJab<T> {
    /// The [lightness](https://en.wikipedia.org/wiki/Lightness) (J') of the color.
    pub lightness: T,

    /// The redness/greenness (a') of the color.
    pub a: T,

    /// The yellowness/blueness (b') of the color.
    pub b: T,
}

impl<T> FromColorUnclamped<Cam16UcsJab<T>> for Cam16UcsJab<T> {
    fn from_color_unclamped(val: Cam16UcsJab<T>) -> Self {
        val
    }
}

impl<T> FromColorUnclamped<Cam16UcsJmh<T>> for Cam16UcsJab<T>
where
    T: RealAngle + Trigonometry,
{
    fn from_color_unclamped(val: Cam16UcsJmh<T>) -> Self {
        let (a, b) = val.hue.into_cartesian();

        Self {
            lightness: val.lightness,
            a,
            b,
        }
    }
}
