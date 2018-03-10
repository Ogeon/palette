//!RGB types, spaces and standards.

use num_traits::Float;

use {Component, Yxy};
use white_point::WhitePoint;
use std::any::Any;

pub use self::rgb::{Rgb, Rgba};
pub use self::standards::Linear;

pub mod standards;
//mod linear;
mod rgb;

///Nonlinear sRGB.
pub type Srgb<T = f32> = Rgb<standards::Srgb, T>;
///Nonlinear sRGB with an alpha component.
pub type Srgba<T = f32> = Rgba<standards::Srgb, T>;

///Linear sRGB.
pub type LinSrgb<T = f32> = Rgb<Linear<standards::Srgb>, T>;
///Linear sRGB with an alpha component.
pub type LinSrgba<T = f32> = Rgba<Linear<standards::Srgb>, T>;

/// Gamma 2.2 encoded sRGB.
pub type GammaSrgb<T = f32> = Rgb<standards::Gamma<standards::Srgb>, T>;
/// Gamma 2.2 encoded sRGB with an alpha component.
pub type GammaSrgba<T = f32> = Rgba<standards::Gamma<standards::Srgb>, T>;

///An RGB space and a transfer function.
pub trait RgbStandard {
    ///The RGB color space.
    type Space: RgbSpace;

    ///The transfer function for the color components.
    type TransferFn: TransferFn;
}

impl<S: RgbSpace, T: TransferFn> RgbStandard for (S, T) {
    type Space = S;
    type TransferFn = T;
}

impl<P: Primaries, W: WhitePoint, T: TransferFn> RgbStandard for (P, W, T) {
    type Space = (P, W);
    type TransferFn = T;
}

///A set of primaries and a white point.
pub trait RgbSpace {
    ///The primaries of the RGB color space.
    type Primaries: Primaries;

    ///The white point of the RGB color space.
    type WhitePoint: WhitePoint;
}

impl<P: Primaries, W: WhitePoint> RgbSpace for (P, W) {
    type Primaries = P;
    type WhitePoint = W;
}

///Represents the red, green and blue primaries of an RGB space.
pub trait Primaries: Any {
    ///Primary red.
    fn red<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T>;
    ///Primary green.
    fn green<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T>;
    ///Primary blue.
    fn blue<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T>;
}

///A transfer function to and from linear space.
pub trait TransferFn {
    ///Convert the color component `x` from linear space.
    fn from_linear<T: Float>(x: T) -> T;

    ///Convert the color component `x` into linear space.
    fn into_linear<T: Float>(x: T) -> T;
}
