//! Types for the LMS color space.

#[allow(clippy::module_inception)]
mod lms;

pub mod matrix;

use crate::Alpha;

pub use self::lms::*;

/// LMS that uses the von Kries matrix.
pub type VonKriesLms<M, T> = Lms<matrix::WithLmsMatrix<M, matrix::VonKries>, T>;

/// LMSA that uses the von Kries matrix.
pub type VonKriesLmsa<M, T> = Alpha<Lms<matrix::WithLmsMatrix<M, matrix::VonKries>, T>, T>;

/// LMS that uses the Bradford matrix.
pub type BradfordLms<M, T> = Lms<matrix::WithLmsMatrix<M, matrix::Bradford>, T>;

/// LMSA that uses the Bradford matrix.
pub type BradfordLmsa<M, T> = Alpha<Lms<matrix::WithLmsMatrix<M, matrix::Bradford>, T>, T>;
