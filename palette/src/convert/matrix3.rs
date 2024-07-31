use core::marker::PhantomData;

use crate::{
    cast::{self, ArrayCast},
    matrix::{matrix_inverse, multiply_3x3, multiply_3x3_and_vec3},
    num::{Arithmetics, IsValidDivisor, One, Recip, Zero},
    ArrayExt, Mat3,
};

use super::{Convert, ConvertOnce};

/// A statically typed 3x3 conversion matrix.
///
/// It's applied via [`Convert`] or [`ConvertOnce`] and helps making some
/// conversions more efficient by only needing to be constructed once.
///
/// ```
/// use palette::{
///     Xyz, Srgb,
///     convert::{Convert, Matrix3},
/// };
///
/// // Multiple matrices can be combined into one:
/// let matrix = Xyz::matrix_from_rgb()
///     .then(Matrix3::scale(0.5, 0.5, 0.5));
///
/// let rgb = Srgb::new(0.8f32, 0.3, 0.3).into_linear();
/// let scaled_xyz = matrix.convert(rgb);
/// ```
pub struct Matrix3<I, O>
where
    I: ArrayCast,
{
    matrix: Mat3<<I::Array as ArrayExt>::Item>,
    transform: PhantomData<fn(I) -> O>,
}

impl<I, O> Convert<I, O> for Matrix3<I, O>
where
    Self: ConvertOnce<I, O> + Copy,
    I: ArrayCast,
{
    #[inline]
    fn convert(&self, input: I) -> O {
        Self::convert_once(*self, input)
    }
}

impl<T, I, O> ConvertOnce<I, O> for Matrix3<I, O>
where
    T: Arithmetics,
    I: ArrayCast<Array = [T; 3]>,
    O: ArrayCast<Array = I::Array>,
{
    #[inline]
    fn convert_once(self, input: I) -> O {
        cast::from_array(multiply_3x3_and_vec3(self.matrix, cast::into_array(input)))
    }
}

impl<T, C> Matrix3<C, C>
where
    C: ArrayCast<Array = [T; 3]>,
{
    /// Produce an identity matrix, which leaves the components unchanged.
    ///
    /// ```
    /// use palette::{Srgb, convert::{Matrix3, Convert}};
    ///
    /// let matrix = Matrix3::identity();
    ///
    /// let input = Srgb::new(0.1, 0.2, 0.3);
    /// let output = matrix.convert(input);
    ///
    /// assert_eq!(input, output);
    /// ```
    #[rustfmt::skip]
    #[inline]
    pub fn identity() -> Self where T: One + Zero {
        Self::from_array([
            T::one(), T::zero(), T::zero(),
            T::zero(), T::one(), T::zero(),
            T::zero(), T::zero(), T::one(),
        ])
    }

    /// Produce a scale matrix, which scales each component separately.
    ///
    /// ```
    /// use palette::{Srgb, convert::{Matrix3, Convert}};
    ///
    /// let matrix = Matrix3::scale(0.1, 0.2, 0.3);
    ///
    /// let input = Srgb::new(1.0, 1.0, 1.0);
    /// let output = matrix.convert(input);
    ///
    /// assert_eq!(Srgb::new(0.1, 0.2, 0.3), output);
    /// ```
    #[rustfmt::skip]
    #[inline]
    pub fn scale(s1: T, s2: T, s3: T) -> Self where T: Zero {
        Self::from_array([
            s1, T::zero(), T::zero(),
            T::zero(), s2, T::zero(),
            T::zero(), T::zero(), s3,
        ])
    }
}

impl<T, I, O> Matrix3<I, O>
where
    I: ArrayCast<Array = [T; 3]>,
    O: ArrayCast<Array = I::Array>,
{
    /// Chain another matrix after this one.
    ///
    /// The combined matrix will result in the same values as if the two
    /// matrices were applied separately, at the cost of only applying a single
    /// matrix. This may speed up computations if the matrix can be constructed
    /// once, and applied multiple times.
    ///
    /// ```
    /// use palette::{
    ///     lms::VonKriesLms, Xyz, Srgb,
    ///     convert::Convert,
    ///     white_point::D65,
    /// };
    /// use approx::assert_relative_eq;
    ///
    /// let rgb_to_xyz = Xyz::matrix_from_rgb();
    /// let xyz_to_lms = VonKriesLms::<D65, _>::matrix_from_xyz();
    ///
    /// let rgb_to_lms = rgb_to_xyz.then(xyz_to_lms);
    ///
    /// let rgb = Srgb::new(0.8f32, 0.3, 0.3).into_linear();
    /// let lms = rgb_to_lms.convert(rgb);
    ///
    /// // Applying the matrices separately for comparison:
    /// let xyz = rgb_to_xyz.convert(rgb);
    /// let lms2 = xyz_to_lms.convert(xyz);
    /// assert_relative_eq!(lms, lms2);
    /// ```
    #[inline]
    pub fn then<U>(self, next: Matrix3<O, U>) -> Matrix3<I, U>
    where
        U: ArrayCast<Array = I::Array>,
        T: Arithmetics + Clone,
    {
        Matrix3 {
            matrix: multiply_3x3(next.matrix, self.matrix),
            transform: PhantomData,
        }
    }

    /// Invert the matrix to create a reversed conversion.
    ///
    /// ## Panics
    ///
    /// A matrix that cannot be inverted will result in a panic.
    ///
    /// ## Examples
    ///
    /// ```
    /// use palette::{
    ///     Xyz, Srgb,
    ///     convert::Convert,
    /// };
    /// use approx::assert_relative_eq;
    ///
    /// let rgb_to_xyz = Xyz::matrix_from_rgb();
    /// let xyz_to_rgb = rgb_to_xyz.invert();
    ///
    /// let rgb = Srgb::new(0.8f32, 0.3, 0.3).into_linear();
    /// let xyz = rgb_to_xyz.convert(rgb);
    /// let rgb2 = xyz_to_rgb.convert(xyz);
    ///
    /// assert_relative_eq!(rgb, rgb2);
    /// ```
    #[inline]
    pub fn invert(self) -> Matrix3<O, I>
    where
        T: Recip + IsValidDivisor<Mask = bool> + Arithmetics + Clone,
    {
        Matrix3 {
            matrix: matrix_inverse(self.matrix),
            transform: PhantomData,
        }
    }

    /// Create a conversion matrix from a plain array.
    ///
    /// The matrix elements are expected to be in row-major order.
    ///
    /// <p class="warning">
    /// This doesn't verify that the conversion results in correct or expected values.
    /// </p>
    ///
    /// ```
    /// use palette::{Srgb, convert::{Matrix3, Convert}};
    ///
    /// let matrix = Matrix3::from_array([
    ///     1.0, 0.0, 0.0,
    ///     0.0, 1.0, 0.0,
    ///     0.0, 0.0, 1.0,
    /// ]);
    ///
    /// let input = Srgb::new(0.1, 0.2, 0.3);
    /// let output = matrix.convert(input);
    ///
    /// assert_eq!(input, output);
    /// ```
    #[inline]
    pub const fn from_array(matrix: Mat3<T>) -> Self {
        Self {
            matrix,
            transform: PhantomData,
        }
    }

    /// Extract the inner array.
    ///
    /// The matrix elements are stored in row-major order.
    #[inline]
    pub fn into_array(self) -> Mat3<T> {
        self.matrix
    }
}

impl<I, O> Clone for Matrix3<I, O>
where
    I: ArrayCast,
    <I::Array as ArrayExt>::Item: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            matrix: self.matrix.clone(),
            transform: self.transform,
        }
    }
}

impl<I, O> Copy for Matrix3<I, O>
where
    I: ArrayCast,
    <I::Array as ArrayExt>::Item: Copy,
{
}
