//! The sRGB standard.

use float::Float;

use rgb::{Primaries, RgbSpace, RgbStandard};
use luma::LumaStandard;
use encoding::TransferFn;
use white_point::{D65, WhitePoint};
use {cast, Component, Yxy};

///The sRGB color space.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Srgb;

impl Primaries for Srgb {
    fn red<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.6400), cast(0.3300), cast(0.212656))
    }
    fn green<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.3000), cast(0.6000), cast(0.715158))
    }
    fn blue<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.1500), cast(0.0600), cast(0.072186))
    }
}

impl RgbSpace for Srgb {
    type Primaries = Srgb;
    type WhitePoint = D65;
}

impl RgbStandard for Srgb {
    type Space = Srgb;
    type TransferFn = Srgb;
}

impl LumaStandard for Srgb {
    type WhitePoint = D65;
    type TransferFn = Srgb;
}

impl TransferFn for Srgb {
    fn into_linear<T: Float>(x: T) -> T {
        if x <= cast(0.04045) {
            x / cast(12.92)
        } else {
            ((x + cast(0.055)) / cast(1.055)).powf(cast(2.4))
        }
    }

    fn from_linear<T: Float>(x: T) -> T {
        if x <= cast(0.0031308) {
            x * cast(12.92)
        } else {
            x.powf(T::one() / cast(2.4)) * cast(1.055) - cast(0.055)
        }
    }
}
