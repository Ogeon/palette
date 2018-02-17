//!Various RGB standards.
pub use self::srgb::Srgb;
pub use self::gamma::{Gamma, F2p2};
pub use self::linear::Linear;

pub mod srgb;
pub mod gamma;
pub mod linear;
