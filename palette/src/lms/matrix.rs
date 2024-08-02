//! Matrix types and traits for [`Lms`][super::Lms].

use core::marker::PhantomData;

use crate::{num::Real, white_point::Any, xyz::meta::HasXyzMeta, Mat3};

/// Implemented by meta types that contain an LMS matrix.
pub trait HasLmsMatrix {
    /// The LMS matrix meta type.
    type LmsMatrix;
}

/// Provides a matrix for converting from [`Xyz`][crate::Xyz] to
/// [`Lms`][super::Lms].
pub trait XyzToLms<T> {
    /// Get an [`Xyz`][crate::Xyz] to [`Lms`][super::Lms] conversion matrix with
    /// elements of type `T`.
    fn xyz_to_lms_matrix() -> Mat3<T>;
}

/// Provides a matrix for converting from [`Lms`][super::Lms] to
/// [`Xyz`][crate::Xyz].
pub trait LmsToXyz<T> {
    /// Get an [`Lms`][super::Lms] to [`Xyz`][crate::Xyz] conversion matrix with
    /// elements of type `T`.
    fn lms_to_xyz_matrix() -> Mat3<T>;
}

/// Adds an LMS matrix `Matrix` to another meta type `T`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WithLmsMatrix<M, Matrix>(pub PhantomData<(M, Matrix)>);

impl<M, Matrix> HasXyzMeta for WithLmsMatrix<M, Matrix>
where
    M: HasXyzMeta,
{
    type XyzMeta = M::XyzMeta;
}

impl<M, Matrix> HasLmsMatrix for WithLmsMatrix<M, Matrix> {
    type LmsMatrix = Matrix;
}

/// Represents the matrix used with the von Kries transform method
/// (M<sub>vonKries</sub>).
///
/// It's also known as the Hunt-Pointer-Estevez matrix (M<sub>HPE</sub>) and was
/// originally used in conjunction with the von Kries method for chromatic
/// adaptation. It's also used in the Hunt and RLAB color appearance models.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VonKries;

impl<T> XyzToLms<T> for VonKries
where
    T: Real,
{
    #[rustfmt::skip]
    #[inline]
    fn xyz_to_lms_matrix() -> Mat3<T> {
        [
            T::from_f64( 0.4002400), T::from_f64(0.7076000), T::from_f64(-0.0808100),
            T::from_f64(-0.2263000), T::from_f64(1.1653200), T::from_f64( 0.0457000),
            T::from_f64( 0.0000000), T::from_f64(0.0000000), T::from_f64( 0.9182200),
        ]
    }
}

impl<T> LmsToXyz<T> for VonKries
where
    T: Real,
{
    #[rustfmt::skip]
    #[inline]
    fn lms_to_xyz_matrix() -> Mat3<T> {
        [
            T::from_f64(1.8599364), T::from_f64(-1.1293816), T::from_f64( 0.2198974),
            T::from_f64(0.3611914), T::from_f64( 0.6388125), T::from_f64(-0.0000064),
            T::from_f64(0.0000000), T::from_f64( 0.0000000), T::from_f64( 1.0890636),
        ]
    }
}

impl HasXyzMeta for VonKries {
    type XyzMeta = Any;
}

impl HasLmsMatrix for VonKries {
    type LmsMatrix = Self;
}

/// Represents Bradford's spectrally sharpening matrix (M<sub>BFD</sub>).
///
/// The "spectral sharpening" effect of the Bradford matrix is believed to
/// improve chromatic adaptation, by narrowing the response curves and making L
/// and M more distinct. *It does however not really reflect cone cells*.
///
/// The Bradford matrix is also used in CIECAM97 and LLAB.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bradford;

impl<T> XyzToLms<T> for Bradford
where
    T: Real,
{
    #[rustfmt::skip]
    #[inline]
    fn xyz_to_lms_matrix() -> Mat3<T> {
        [
            T::from_f64( 0.8951000), T::from_f64( 0.2664000), T::from_f64(-0.1614000),
            T::from_f64(-0.7502000), T::from_f64( 1.7135000), T::from_f64( 0.0367000),
            T::from_f64( 0.0389000), T::from_f64(-0.0685000), T::from_f64( 1.0296000),
        ]
    }
}

impl<T> LmsToXyz<T> for Bradford
where
    T: Real,
{
    #[rustfmt::skip]
    #[inline]
    fn lms_to_xyz_matrix() -> Mat3<T> {
        [
            T::from_f64( 0.9869929), T::from_f64(-0.1470543), T::from_f64(0.1599627),
            T::from_f64( 0.4323053), T::from_f64( 0.5183603), T::from_f64(0.0492912),
            T::from_f64(-0.0085287), T::from_f64( 0.0400428), T::from_f64(0.9684867),
        ]
    }
}

impl HasXyzMeta for Bradford {
    type XyzMeta = Any;
}

impl HasLmsMatrix for Bradford {
    type LmsMatrix = Self;
}

/// Represents a unit matrix, for a 1:1 conversion between XYZ to LMS.
///
/// This matrix may be useful in chromatic adaptation, but does otherwise not
/// represent an actual conversion to and from cone cell responses.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UnitMatrix;

impl<T> XyzToLms<T> for UnitMatrix
where
    T: Real,
{
    #[rustfmt::skip]
    #[inline]
    fn xyz_to_lms_matrix() -> Mat3<T> {
        [
            T::from_f64(1.0000000), T::from_f64(0.0000000), T::from_f64(0.0000000),
            T::from_f64(0.0000000), T::from_f64(1.0000000), T::from_f64(0.0000000),
            T::from_f64(0.0000000), T::from_f64(0.0000000), T::from_f64(1.0000000),
        ]
    }
}

impl<T> LmsToXyz<T> for UnitMatrix
where
    T: Real,
{
    #[rustfmt::skip]
    #[inline]
    fn lms_to_xyz_matrix() -> Mat3<T> {
        [
            T::from_f64(1.0000000), T::from_f64(0.0000000), T::from_f64(0.0000000),
            T::from_f64(0.0000000), T::from_f64(1.0000000), T::from_f64(0.0000000),
            T::from_f64(0.0000000), T::from_f64(0.0000000), T::from_f64(1.0000000),
        ]
    }
}

impl HasXyzMeta for UnitMatrix {
    type XyzMeta = Any;
}

impl HasLmsMatrix for UnitMatrix {
    type LmsMatrix = Self;
}
