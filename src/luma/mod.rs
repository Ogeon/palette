//! Luminance types.

mod luma;

use white_point::{D65, WhitePoint};
use encoding::{Gamma, Linear, Srgb, TransferFn};

pub use self::luma::{Luma, Lumaa};

/// sRGB encoded luminance.
pub type SrgbLuma<T = f32> = Luma<Srgb, T>;
/// sRGB encoded luminance with an alpha component.
pub type SrgbLumaa<T = f32> = Lumaa<Srgb, T>;

/// Linear luminance.
pub type LinLuma<Wp = D65, T = f32> = Luma<Linear<Wp>, T>;
/// Linear luminance with an alpha component.
pub type LinLumaa<Wp = D65, T = f32> = Lumaa<Linear<Wp>, T>;

/// Gamma 2.2 encoded luminance.
pub type GammaLuma<T = f32> = Luma<Gamma<D65>, T>;
/// Gamma 2.2 encoded luminance with an alpha component.
pub type GammaLumaa<T = f32> = Lumaa<Gamma<D65>, T>;

/// A white point and a transfer function.
pub trait LumaStandard {
    /// The white point of the color space.
    type WhitePoint: WhitePoint;

    /// The transfer function for the luminance component.
    type TransferFn: TransferFn;
}

impl<Wp: WhitePoint, T: TransferFn> LumaStandard for (Wp, T) {
    type WhitePoint = Wp;
    type TransferFn = T;
}
