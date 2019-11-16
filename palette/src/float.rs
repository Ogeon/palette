//! Floating point trait
//!
//! This module will just re-export the currently used floating point trait.
//! Both for use in derive macros and for anyone who don't want to add it as an
//! additional dependency.

#[cfg(any(feature = "std", feature = "libm"))]
pub use num_traits::Float;

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!(
    "The palette crate needs a float library. Please enable the \"std\" or \"libm\" feature."
);
