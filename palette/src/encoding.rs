//! Number and color encoding traits, types and standards.
//!
//! Some color spaces, particularly RGB, may be encoded in more than one way and
//! may have more than one standard. These encodings and standards are
//! represented as type parameters in Palette, as a form of type branding, to
//! prevent accidental mixups.

use palette_math::gamma::lut::GammaLutBuilder;

pub use self::adobe::AdobeRgb;
#[allow(deprecated)]
pub use self::gamma::{F2p2, Gamma};
pub use self::linear::Linear;
pub use self::lut::{FromLinearLut, IntoLinearLut};
pub use self::p3::{DciP3, DciP3Plus, DisplayP3, P3Gamma};
pub use self::prophoto::ProPhotoRgb;
pub use self::rec_standards::{Rec2020, Rec709, RecOetf};
pub use self::srgb::Srgb;

pub mod adobe;
#[deprecated(
    since = "0.7.7",
    note = "`Gamma`, `GammaFn` and `F2p2` are error prone and incorrectly implemented. See `palette::encoding` for possible alternatives or implement `palette::encoding::FromLinear` and `palette::encoding::IntoLinear` for a custom type."
)]
pub mod gamma;
pub mod linear;
pub mod p3;
pub mod prophoto;
pub mod rec_standards;
pub mod srgb;

mod lut;

/// A transfer function from linear space.
pub trait FromLinear<L, E> {
    /// Convert the color component `linear` from linear space.
    #[must_use]
    fn from_linear(linear: L) -> E;
}

/// A transfer function to linear space.
pub trait IntoLinear<L, E> {
    /// Convert the color component `encoded` into linear space.
    #[must_use]
    fn into_linear(encoded: E) -> L;
}

/// A transfer function that can produce a gamma lookup table.
///
/// See [`FromLinearLut`] and [`IntoLinearLut`] for how to use a type that
/// implements this trait to make a lookup table.
pub trait GetLutBuilder {
    /// Get a builder for gamma lookup tables for this transfer function.
    ///
    /// This function is called by [`FromLinearLut`] and [`IntoLinearLut`] to
    /// generate their table entries.
    fn get_lut_builder() -> GammaLutBuilder;
}
