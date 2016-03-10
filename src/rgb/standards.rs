//!Various RGB standards.

use num_traits::Float;
use Yxy;
use flt;
use white_point::{D65, WhitePoint};
use rgb::{Primaries, RgbSpace, RgbStandard};
use pixel::TransferFn;

///The sRGB color space.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Srgb;

impl Primaries for Srgb {
    fn red<Wp: WhitePoint, T: Float>() -> Yxy<Wp, T> {
        Yxy::with_wp(flt(0.6400), flt(0.3300), flt(0.212656))
    }
    fn green<Wp: WhitePoint, T: Float>() -> Yxy<Wp, T> {
        Yxy::with_wp(flt(0.3000), flt(0.6000), flt(0.715158))
    }
    fn blue<Wp: WhitePoint, T: Float>() -> Yxy<Wp, T> {
        Yxy::with_wp(flt(0.1500), flt(0.0600), flt(0.072186))
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

impl TransferFn for Srgb {
    fn into_linear<T: Float>(x: T) -> T {
        if x <= flt(0.04045) {
            x / flt(12.92)
        } else {
            ((x + flt(0.055)) / flt(1.055)).powf(flt(2.4))
        }
    }

    fn from_linear<T: Float>(x: T) -> T {
        if x <= flt(0.0031308) {
            x * flt(12.92)
        } else {
            x.powf(T::one() / flt(2.4)) * flt(1.055) - flt(0.055)
        }
    }
}
