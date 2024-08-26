//! Number and color encoding traits, types and standards.
//!
//! Some color spaces, particularly RGB, may be encoded in more than one way and
//! may have more than one standard. These encodings and standards are
//! represented as type parameters in Palette, as a form of type branding, to
//! prevent accidental mixups.

pub use self::adobe::AdobeRgb;
pub use self::gamma::{F2p2, Gamma};
pub use self::linear::Linear;
pub use self::p3::{DciP3, DciP3Plus, DisplayP3, P3Gamma};
pub use self::prophoto::ProPhotoRgb;
pub use self::rec_standards::{Rec2020, Rec709, RecOetf};
pub use self::srgb::Srgb;

pub mod adobe;
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
