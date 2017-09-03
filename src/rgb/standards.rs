//!Various RGB standards.
use std::marker::PhantomData;

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

/// A generic RGB standard with linear components.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Linear<S: RgbSpace = Srgb>(PhantomData<S>);

impl<S: RgbSpace> RgbStandard for Linear<S> {
    type Space = S;
    type TransferFn = LinearFn;
}

///Linear color component encoding.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LinearFn;

impl TransferFn for LinearFn {
    #[inline(always)]
    fn into_linear<T: Float>(x: T) -> T {
        x
    }

    #[inline(always)]
    fn from_linear<T: Float>(x: T) -> T {
        x
    }
}

/// Gamma encoding.
///
/// Gamma encoding or gamma correction is used to transform the intensity
/// values to either match a non-linear display, like CRT, or to prevent
/// banding among the darker colors. `GammaRgb` represents a gamma corrected
/// RGB color, where the intensities are encoded using the following power-law
/// expression: _V ^γ_ (where _V_ is the intensity value an _γ_ is the encoding
/// gamma).
///
/// The gamma value is stored as a simple type that represents an `f32` constant.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Gamma<S: RgbSpace = Srgb, N: Number = F2p2>(PhantomData<(S, N)>);

impl<S: RgbSpace, N: Number> RgbStandard for Gamma<S, N> {
    type Space = S;
    type TransferFn = GammaFn<N>;
}

/// The transfer function for gamma encoded colors.
///
/// The gamma value is stored as a simple type that represents an `f32` constant.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GammaFn<N: Number = F2p2>(PhantomData<N>);

impl<N: Number> TransferFn for GammaFn<N> {
    fn into_linear<T: Float>(x: T) -> T {
        x.powf(T::one() / flt(N::VALUE))
    }

    fn from_linear<T: Float>(x: T) -> T {
        x.powf(flt(N::VALUE))
    }
}

/// A type level float constant.
pub trait Number {
    /// The represented number.
    const VALUE: f32;
}

/// Represents `2.2f32`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct F2p2;

impl Number for F2p2 {
    const VALUE: f32 = 2.2;
}
