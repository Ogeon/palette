//! Gamma encoding.

use core::marker::PhantomData;

use float::Float;

use cast;
use rgb::{RgbSpace, RgbStandard};
use luma::LumaStandard;
use encoding::TransferFn;
use white_point::WhitePoint;

/// Gamma encoding.
///
/// Gamma encoding or gamma correction is used to transform the intensity
/// values to either match a non-linear display, like CRT, or to prevent
/// banding among the darker colors. `GammaRgb` represents a gamma corrected
/// RGB color, where the intensities are encoded using the following power-law
/// expression: _V<sup> γ</sup>_ (where _V_ is the intensity value an _γ_ is the encoding
/// gamma).
///
/// The gamma value is stored as a simple type that represents an `f32` constant.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Gamma<S, N: Number = F2p2>(PhantomData<(S, N)>);

impl<S: RgbSpace, N: Number> RgbStandard for Gamma<S, N> {
    type Space = S;
    type TransferFn = GammaFn<N>;
}

impl<Wp: WhitePoint, N: Number> LumaStandard for Gamma<Wp, N> {
    type WhitePoint = Wp;
    type TransferFn = GammaFn<N>;
}

/// The transfer function for gamma encoded colors.
///
/// The gamma value is stored as a simple type that represents an `f32` constant.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GammaFn<N: Number = F2p2>(PhantomData<N>);

impl<N: Number> TransferFn for GammaFn<N> {
    fn into_linear<T: Float>(x: T) -> T {
        x.powf(T::one() / cast(N::VALUE))
    }

    fn from_linear<T: Float>(x: T) -> T {
        x.powf(cast(N::VALUE))
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
