//! Various encoding traits, types and standards.

pub use self::gamma::{F2p2, Gamma};
pub use self::linear::Linear;
pub use self::srgb::Srgb;

pub mod gamma;
pub mod linear;
pub mod srgb;

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
