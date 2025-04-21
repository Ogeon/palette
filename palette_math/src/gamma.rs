//! Gamma, OETF, and EOTF transfer functions and lookup tables.
//!
//! RGB and some other color spaces tend to be stored and communicated in a
//! non-linear format. This module collects transfer functions for conversion
//! between linear and non-linear values under the "gamma" umbrella term.

use crate::num::Powf;

use self::lut::GammaLutBuilder;

pub mod lut;
mod model;

/// Create a lookup table builder for the transfer functions used in sRGB.
pub fn srgb_lut_builder() -> GammaLutBuilder
where
    f64: Powf,
{
    GammaLutBuilder::new_piecewise_fn(12.92, 0.0031308, 2.4)
}

/// Create a lookup table builder for the transfer functions used in Rec. 709 and Rec. 2020.
pub fn rec_oetf_builder() -> GammaLutBuilder
where
    f64: Powf,
{
    GammaLutBuilder::new_piecewise_fn(4.5, 0.018053968510807, 1.0 / 0.45)
}

/// Create a lookup table builder for the transfer functions used in Adobe RGB.
pub fn adobe_rgb_builder() -> GammaLutBuilder
where
    f64: Powf,
{
    GammaLutBuilder::new_power_fn(563.0 / 256.0)
}

/// Create a lookup table builder for the transfer functions used in DCI-P3.
pub fn p3_builder() -> GammaLutBuilder
where
    f64: Powf,
{
    GammaLutBuilder::new_power_fn(2.6)
}

/// Create a lookup table builder for the transfer functions used in ProPhoto RGB.
pub fn prophoto_rgb_builder() -> GammaLutBuilder
where
    f64: Powf,
{
    GammaLutBuilder::new_piecewise_fn(16.0, 0.001953125, 1.8)
}
