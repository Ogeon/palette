//! Various encoding traits, types and standards.

use crate::float::Float;
use crate::FromF64;

pub use self::gamma::{F2p2, Gamma};
pub use self::linear::Linear;
pub use self::srgb::Srgb;

pub mod gamma;
pub mod linear;
pub mod pixel;
pub mod srgb;

/// A transfer function to and from linear space.
pub trait TransferFn: 'static {
    /// Convert the color component `x` from linear space.
    fn from_linear<T: Float + FromF64>(x: T) -> T;

    /// Convert the color component `x` into linear space.
    fn into_linear<T: Float + FromF64>(x: T) -> T;
}
