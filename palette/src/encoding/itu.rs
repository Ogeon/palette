//! Encodings by the International Telecommunication Union–Radiocommunications, aka. ITU-R.
use crate::float::Float;

use crate::rgb::{Primaries, RgbSpace, RgbStandard};
use crate::luma::LumaStandard;
use crate::encoding::TransferFn;
use crate::white_point::{D65, WhitePoint};
use crate::yuv::{DifferenceFn, YuvStandard};
use crate::{FloatComponent, FromF64, Yxy, from_f64};

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

/// The color space of ITU-R BT2020.
///
/// See [ITU-R Rec.2020].
///
/// [ITU-R Rec.2020]: https://www.itu.int/rec/R-REC-BT.2020/
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BT2020;

/// The color space of ITU-R BT2100 with Hybrid Log-Gamma.
///
/// See [ITU-R Rec.2100].
///
/// [ITU-R Rec.2100]: https://www.itu.int/rec/R-REC-BT.2100/
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BT2100Hlg;

/// This transfer function is shared between `BT601` and `BT709`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Transfer601And709;

/// The transfer function of `BT2020`.
///
/// This is technically very similar to the one of BT.601 and BT.709 but the constants are defined
/// with increased accuracy due to supporting up to 12-bit quantized numbers.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Transfer2020;

/// The electro optical transfer function of `BT2100` with HLG transfer.
///
/// This transforms _scene_ linear light to a non-linear electrical signal and back. It assumes
/// that the opto-optical transfer (OOTF), which adjusts the scene color to the intended viewing
/// environment, is not part of the camera system but the viewing display.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Transfer2100Hlg;

/// The Yuv encoding difference functions for BT601.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DifferenceFn601;

/// The Yuv encoding difference functions for BT709.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DifferenceFn709;

/// The Yuv encoding difference functions for BT2020.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DifferenceFn2020;

// See 2.5.1 (page 2). RGB primary luminances.
const BT601_LUMINANCE: (f64, f64, f64) = (0.2990, 0.5870, 0.1140);
// Divisor to renormalize the blue difference signal.
const BT601_BLUE_NORM: f64 = 1.772;
// Divisor to renormalize the red difference signal.
const BT601_RED_NORM: f64 = 1.402;

// Exact primary luminances derived from the color space primaries.
const BT709_LUMINANCE: (f64, f64, f64) = (0.212656, 0.715158, 0.072186);
// Luminances for the sake of exact specification compliance for YUV luminance.
// See 3.2 (page 4)
const BT709_WEIGHTS: (f64, f64, f64) = (0.2126, 0.7152, 0.07212);
// Divisor to renormalize the blue difference signal.
const BT709_BLUE_NORM: f64 = 1.8556;
// Divisor to renormalize the red difference signal.
const BT709_RED_NORM: f64 = 1.5748;

// See Table 4, Derivation of Luminance.
// BT2100 uses the same as luminance as BT2020, see Table 6.
const BT2020_WEIGHTS: (f64, f64, f64) = (0.2627, 0.6780, 0.0593);
// Divisor to renormalize the blue difference.
const BT2020_BLUE_NORM: f64 = 1.8814;
// Divisor to renormalize the red difference.
const BT2020_RED_NORM: f64 = 1.4746;

impl Primaries for BT601_525 {
    fn red<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.6300), from_f64(0.3400), from_f64(BT601_LUMINANCE.0))
    }
    fn green<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.3100), from_f64(0.5950), from_f64(BT601_LUMINANCE.1))
    }
    fn blue<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.1550), from_f64(0.0700), from_f64(BT601_LUMINANCE.2))
    }
}

impl Primaries for BT601_625 {
    fn red<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.6400), from_f64(0.3300), from_f64(BT601_LUMINANCE.0))
    }
    fn green<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.2900), from_f64(0.6000), from_f64(BT601_LUMINANCE.1))
    }
    fn blue<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.1500), from_f64(0.0600), from_f64(BT601_LUMINANCE.2))
    }
}

impl Primaries for BT709 {
    fn red<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.6400), from_f64(0.3300), from_f64(BT709_LUMINANCE.0))
    }
    fn green<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.3000), from_f64(0.6000), from_f64(BT709_LUMINANCE.1))
    }
    fn blue<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.1500), from_f64(0.0600), from_f64(BT709_LUMINANCE.2))
    }
}

impl Primaries for BT2020 {
    fn red<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.708), from_f64(0.292), from_f64(BT2020_WEIGHTS.0))
    }
    fn green<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.170), from_f64(0.797), from_f64(BT2020_WEIGHTS.1))
    }
    fn blue<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.131), from_f64(0.046), from_f64(BT2020_WEIGHTS.2))
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

impl RgbSpace for BT2020 {
    type Primaries = BT2020;
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

impl RgbStandard for BT2020 {
    type Space = BT2020;
    type TransferFn = Transfer2020;
}

impl RgbStandard for BT2100Hlg {
    /// This uses the same colorimetry as Rec. BT.2020.
    type Space = BT2020;
    type TransferFn = Transfer2100Hlg;
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

impl LumaStandard for BT2020 {
    type WhitePoint = D65;
    type TransferFn = Transfer2020;
}

impl LumaStandard for BT2100Hlg {
    type WhitePoint = D65;
    type TransferFn = Transfer2100Hlg;
}

impl YuvStandard for BT601_525 {
    type RgbSpace = Self;
    type TransferFn = Transfer601And709;
    type DifferenceFn = DifferenceFn601;
}

impl YuvStandard for BT601_625 {
    type RgbSpace = Self;
    type TransferFn = Transfer601And709;
    type DifferenceFn = DifferenceFn601;
}

impl YuvStandard for BT709 {
    type RgbSpace = Self;
    type TransferFn = Transfer601And709;
    type DifferenceFn = DifferenceFn709;
}

impl YuvStandard for BT2020 {
    type RgbSpace = Self;
    type TransferFn = Transfer2020;
    type DifferenceFn = DifferenceFn2020;
}

impl YuvStandard for BT2100Hlg {
    type RgbSpace = BT2020;
    type TransferFn = Transfer2100Hlg;
    /// It uses the same scaling and for Yuv.
    type DifferenceFn = DifferenceFn2020;
}

impl TransferFn for Transfer601And709 {
    fn into_linear<T: Float + FromF64>(x: T) -> T {
        if x <= from_f64(0.0091) {
            x / from_f64(4.500)
        } else {
            ((x + from_f64(0.099)) / from_f64(1.099)).powf(T::one() / from_f64(0.45))
        }
    }

    fn from_linear<T: Float + FromF64>(x: T) -> T {
        if x <= from_f64(0.0018) {
            x * from_f64(4.500)
        } else {
            x.powf(from_f64(0.45)) * from_f64(1.099) - from_f64(0.099)
        }
    }
}

impl TransferFn for Transfer2020 {
    fn into_linear<T: Float + FromF64>(x: T) -> T {
        let alpha: T = from_f64(1.09929682680944);
        let beta: T = from_f64(0.018053968510807);

        if x < beta * from_f64(4.500) {
            x / from_f64(4.500)
        } else {
            ((x + alpha - T::one()) / alpha).powf(T::one() / from_f64(0.45))
        }
    }

    fn from_linear<T: Float + FromF64>(x: T) -> T {
        let alpha: T = from_f64(1.09929682680944);
        let beta: T = from_f64(0.018053968510807);

        if x < beta {
            x * from_f64(4.5)
        } else {
            x.powf(from_f64(0.45)) * alpha - (alpha - T::one())
        }
    }
}

impl Transfer2100Hlg {
    const A: f64 = 0.17883277;
    const B: f64 = 1.0 - 4.0*Self::A;
}

impl TransferFn for Transfer2100Hlg {
    fn into_linear<T: Float + FromF64>(x: T) -> T {
        // ln is not yet const.
        let c: f64 = 0.5 - Self::A*(4.0*Self::A).ln();

        if x <= from_f64(0.5) {
            x.sqrt() / from_f64(3.0)
        } else {
            let i: T = (x - from_f64(c)) / from_f64(Self::A);
            (i.exp() + from_f64(Self::B)) / from_f64(12.0)
        }
    }

    fn from_linear<T: Float + FromF64>(x: T) -> T {
        // ln is not yet const.
        let c: f64 = 0.5 - Self::A*(4.0*Self::A).ln();

        if x <= from_f64(1.0 / 12.0) {
            (x * from_f64(3.0)).sqrt()
        } else {
            let i: T = x * from_f64(12.0) - from_f64(Self::B);
            i.ln() * from_f64(Self::A) + from_f64(c)
        }
    }
}

impl DifferenceFn for DifferenceFn601 {
    fn luminance<T: FloatComponent>() -> [T; 3] {
        // Full intensity matches whitepoint, these are exactly the Y component of primares.
        let (r, g, b) = BT601_LUMINANCE;
        [from_f64(r), from_f64(g), from_f64(b)]
    }

    fn normalize_blue<T: FloatComponent>(denorm: T) -> T {
        denorm / from_f64(BT601_BLUE_NORM)
    }

    fn denormalize_blue<T: FloatComponent>(norm: T) -> T {
        norm * from_f64(BT601_BLUE_NORM)
    }

    fn normalize_red<T: FloatComponent>(denorm: T) -> T {
        denorm / from_f64(BT601_RED_NORM)
    }

    fn denormalize_red<T: FloatComponent>(norm: T) -> T {
        norm * from_f64(BT601_RED_NORM)
    }
}

impl DifferenceFn for DifferenceFn709 {
    fn luminance<T: FloatComponent>() -> [T; 3] {
        // Full intensity matches whitepoint, these are exactly the Y component of primares.
        let (r, g, b) = BT709_WEIGHTS;
        [from_f64(r), from_f64(g), from_f64(b)]
    }

    fn normalize_blue<T: FloatComponent>(denorm: T) -> T {
        denorm / from_f64(BT709_BLUE_NORM)
    }

    fn denormalize_blue<T: FloatComponent>(norm: T) -> T {
        norm * from_f64(BT709_BLUE_NORM)
    }

    fn normalize_red<T: FloatComponent>(denorm: T) -> T {
        denorm / from_f64(BT709_RED_NORM)
    }

    fn denormalize_red<T: FloatComponent>(norm: T) -> T {
        norm * from_f64(BT709_RED_NORM)
    }
}

impl DifferenceFn for DifferenceFn2020 {
    fn luminance<T: FloatComponent>() -> [T; 3] {
        // Full intensity matches whitepoint, these are exactly the Y component of primares.
        let (r, g, b) = BT2020_WEIGHTS;
        [from_f64(r), from_f64(g), from_f64(b)]
    }

    fn normalize_blue<T: FloatComponent>(denorm: T) -> T {
        denorm / from_f64(BT2020_BLUE_NORM)
    }

    fn denormalize_blue<T: FloatComponent>(norm: T) -> T {
        norm * from_f64(BT2020_BLUE_NORM)
    }

    fn normalize_red<T: FloatComponent>(denorm: T) -> T {
        denorm / from_f64(BT2020_RED_NORM)
    }

    fn denormalize_red<T: FloatComponent>(norm: T) -> T {
        norm * from_f64(BT2020_RED_NORM)
    }
}
