//! Various encoding traits, types and standards.

use float::Float;

pub use self::srgb::Srgb;
pub use self::gamma::{F2p2, Gamma};
pub use self::linear::Linear;

pub mod srgb;
pub mod gamma;
pub mod linear;
pub mod pixel;

/// A transfer function to and from linear space.
pub trait TransferFn {
    /// Convert the color component `x` from linear space.
    fn from_linear<T: Float>(x: T) -> T;

    /// Convert the color component `x` into linear space.
    fn into_linear<T: Float>(x: T) -> T;
}
