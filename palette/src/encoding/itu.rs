use float::Float;

use rgb::{Primaries, RgbSpace, RgbStandard};
use luma::LumaStandard;
use encoding::TransferFn;
use white_point::{D65, WhitePoint};
use {cast, Component, Yxy};

///The color space of ITU-R BT601 for 525-line.
///
/// See [ITU-R Rec.601].
///
/// [ITU-R Rec.601]: https://www.itu.int/rec/R-REC-BT.601/
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BT601_525;

/// The color space of ITU-R BT601 for 625-line.
///
/// See [ITU-R Rec.601].
///
/// [ITU-R Rec.601]: https://www.itu.int/rec/R-REC-BT.601/
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BT601_625;

/// The color space of ITU-R BT709.
///
/// See [ITU-R Rec.709].
///
/// [ITU-R Rec.709]: https://www.itu.int/rec/R-REC-BT.709/
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BT709;

/// This transfer function is shared between `BT601` and `BT709`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Transfer601And709;

impl Primaries for BT601_525 {
    fn red<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.6300), cast(0.3400), cast(0.2990))
    }
    fn green<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.3100), cast(0.5950), cast(0.5870))
    }
    fn blue<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.1550), cast(0.0700), cast(0.1140))
    }
}

impl Primaries for BT601_625 {
    fn red<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.6400), cast(0.3300), cast(0.2990))
    }
    fn green<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.2900), cast(0.6000), cast(0.5870))
    }
    fn blue<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T> {
        Yxy::with_wp(cast(0.1500), cast(0.0600), cast(0.1140))
    }
}

impl Primaries for BT709 {
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

impl RgbSpace for BT601_525 {
    type Primaries = BT601_525;
    type WhitePoint = D65;
}

impl RgbSpace for BT601_625 {
    type Primaries = BT601_625;
    type WhitePoint = D65;
}

impl RgbSpace for BT709 {
    type Primaries = BT709;
    type WhitePoint = D65;
}

impl RgbStandard for BT601_525 {
    type Space = BT601_525;
    type TransferFn = Transfer601And709;
}

impl RgbStandard for BT601_625 {
    type Space = BT601_625;
    type TransferFn = Transfer601And709;
}

impl RgbStandard for BT709 {
    type Space = BT709;
    type TransferFn = Transfer601And709;
}

impl LumaStandard for BT601_525 {
    type WhitePoint = D65;
    type TransferFn = Transfer601And709;
}

impl LumaStandard for BT601_625 {
    type WhitePoint = D65;
    type TransferFn = Transfer601And709;
}

impl LumaStandard for BT709 {
    type WhitePoint = D65;
    type TransferFn = Transfer601And709;
}

impl TransferFn for Transfer601And709 {
    fn into_linear<T: Float>(x: T) -> T {
        if x <= cast(0.0091) {
            x / cast(4.500)
        } else {
            ((x + cast(0.099)) / cast(1.099)).powf(T::one() / cast(0.45))
        }
    }

    fn from_linear<T: Float>(x: T) -> T {
        if x <= cast(0.0018) {
            x * cast(4.500)
        } else {
            x.powf(cast(0.45)) * cast(1.099) - cast(0.099)
        }
    }
}
