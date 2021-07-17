//! Various encoding traits, types and standards.

pub use self::gamma::{F2p2, Gamma};
pub use self::linear::Linear;
pub use self::srgb::Srgb;

pub mod gamma;
pub mod linear;
pub mod pixel;
pub mod srgb;

/// A transfer function to and from linear space.
pub trait TransferFn<T>: 'static {
    /// Convert the color component `x` from linear space.
    #[must_use]
    fn from_linear(x: T) -> T;

    /// Convert the color component `x` into linear space.
    #[must_use]
    fn into_linear(x: T) -> T;
}
