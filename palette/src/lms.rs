//! Types for the LMS color space.

#[allow(clippy::module_inception)]
mod lms;

pub mod meta;

use crate::Alpha;

pub use self::lms::*;

/// LMS that uses the von Kries matrix.
pub type VonKriesLms<M, T> = Lms<meta::WithLmsMatrix<M, meta::VonKries>, T>;

/// LMSA that uses the von Kries matrix.
pub type VonKriesLmsa<M, T> = Alpha<Lms<meta::WithLmsMatrix<M, meta::VonKries>, T>, T>;

/// LMS that uses the Bradford matrix.
pub type BradfordLms<M, T> = Lms<meta::WithLmsMatrix<M, meta::Bradford>, T>;

/// LMSA that uses the Bradford matrix.
pub type BradfordLmsa<M, T> = Alpha<Lms<meta::WithLmsMatrix<M, meta::Bradford>, T>, T>;
