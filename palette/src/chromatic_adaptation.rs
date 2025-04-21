//! Convert colors from one reference white point to another
//!
//! Chromatic adaptation is the ability to adjust the appearance of colors to
//! changes in illumination. This happens naturally in our body's visual system,
//! and can be emulated with a "chromatic adaptation transform" (CAT).
//!
//! This library implements a one-step adaptation transform, known as the von
//! Kries method. It's provided as [`AdaptFromUnclamped`] or
//! [`AdaptIntoUnclamped`] for convenience, or [`adaptation_matrix`] for control
//! and reusability. All of them can be customized with different LMS matrices.
//!
//! The provided LMS matrices are:
//!
//! - [`Bradford`] - A "spectrally sharpened" matrix, which may improve
//!   chromatic adaptation. This is the default for [`AdaptFromUnclamped`] and
//!   [`AdaptIntoUnclamped`].
//! - [`VonKries`][lms::matrix::VonKries] - Produces cone-describing LMS values,
//!   as opposed to many other matrices, but may perform worse than other
//!   matrices.
//! - [`UnitMatrix`][lms::matrix::UnitMatrix] - Included for completeness, but
//!   generally considered a bad option. Also called "XYZ scaling" or "wrong von
//!   Kries".
//!
//! ```
//! use palette::{
//!     Xyz, white_point::{A, C},
//!     chromatic_adaptation::AdaptIntoUnclamped,
//! };
//! use approx::assert_relative_eq;
//!
//! let input = Xyz::<A, f32>::new(0.315756, 0.162732, 0.015905);
//!
//! //Will convert Xyz<A, f32> to Xyz<C, f32> using Bradford chromatic adaptation;
//! let output: Xyz<C, f32> = input.adapt_into_unclamped();
//!
//! let expected = Xyz::new(0.257963, 0.139776, 0.058825);
//! assert_relative_eq!(output, expected, epsilon = 0.0001);
//! ```

use core::ops::Div;

use crate::{
    convert::{FromColorUnclamped, IntoColorUnclamped, Matrix3},
    lms::{
        self,
        matrix::{Bradford, LmsToXyz, WithLmsMatrix, XyzToLms},
        Lms,
    },
    matrix::{multiply_3x3, multiply_3x3_and_vec3, Mat3},
    num::{Arithmetics, Real, Zero},
    white_point::{Any, WhitePoint},
    xyz::meta::HasXyzMeta,
    Xyz,
};

/// Construct a one-step chromatic adaptation matrix.
///
/// The matrix uses the von Kries method to fully adapt a color from an input
/// white point to an output white point, using a provided LMS matrix. See the
/// [`chromatic_adaptation`][self] module for more details.
///
/// ## Static White Points
///
/// The `input_wp` and `output_wp` parameters represent the color "white" for
/// the input and output colors, respectively. Passing `None` will make it use
/// `I` and `O` to calculate the white points:
///
/// ```
/// use palette::{
///     chromatic_adaptation::adaptation_matrix,
///     lms::matrix::Bradford,
///     convert::Convert,
///     white_point::{A, C},
///     Xyz,
/// };
/// use approx::assert_relative_eq;
///
/// // Adapts from white point A to white point C:
/// let matrix = adaptation_matrix::<f32, A, C, Bradford>(None, None);
///
/// // Explicit types added for illustration.
/// let input: Xyz<A> = Xyz::new(0.315756, 0.162732, 0.015905);
/// let output: Xyz<C> = matrix.convert(input);
///
/// let expected = Xyz::new(0.257963, 0.139776, 0.058825);
/// assert_relative_eq!(output, expected, epsilon = 0.0001);
/// ```
///
/// ## Dynamic White Points
///
/// It's also possible to use arbitrary colors as white points, as long as they
/// are brighter than black. This can be useful for white balancing a photo,
/// where we may want to use the same static white point for both the input and
/// the output:
///
/// ```
/// use palette::{
///     chromatic_adaptation::adaptation_matrix,
///     lms::matrix::Bradford,
///     convert::{FromColorUnclampedMut, Convert},
///     Srgb, Xyz,
/// };
/// use approx::assert_relative_eq;
///
/// fn simple_white_balance(image: &mut [Srgb<f32>]) {
///     // Temporarily convert to Xyz:
///     let mut image = <[Xyz<_, f32>]>::from_color_unclamped_mut(image);
///
///     // Find the average Xyz color:
///     let sum = image.iter().fold(Xyz::new(0.0, 0.0, 0.0), |sum, &c| sum + c);
///     let average = sum / image.len() as f32;
///
///     // Considering the average color to be "white", this matrix adapts from the
///     // average to default sRGB white, D65:
///     let matrix = adaptation_matrix::<_, _, _, Bradford>(Some(average), None);
///
///     for pixel in &mut *image {
///         *pixel = matrix.convert(*pixel);
///     }
/// }
///
/// // Minimal test case. This one pixel becomes gray after white balancing:
/// let mut image = [Srgb::new(0.8, 0.3, 0.9)];
/// simple_white_balance(&mut image);
///
/// let expected = Srgb::new(0.524706, 0.524706, 0.524706);
/// assert_relative_eq!(image[0], expected, epsilon = 0.00001);
/// ```
///
/// See also [Wikipedia - Von Kries transform][wikipedia].
///
/// [wikipedia]:
///     https://en.wikipedia.org/wiki/Chromatic_adaptation#Von_Kries_transform
pub fn adaptation_matrix<T, I, O, M>(
    input_wp: Option<Xyz<I, T>>,
    output_wp: Option<Xyz<O, T>>,
) -> Matrix3<Xyz<I, T>, Xyz<O, T>>
where
    T: Zero + Arithmetics + Clone,
    I: WhitePoint<T> + HasXyzMeta<XyzMeta = I>,
    O: WhitePoint<T> + HasXyzMeta<XyzMeta = O>,
    M: XyzToLms<T> + LmsToXyz<T>,
    Xyz<I, T>: IntoColorUnclamped<Lms<WithLmsMatrix<I, M>, T>>,
    Xyz<O, T>: IntoColorUnclamped<Lms<WithLmsMatrix<O, M>, T>>,
{
    let input_to_lms = Lms::<WithLmsMatrix<I, M>, T>::matrix_from_xyz();
    let lms_to_output = Xyz::<O, T>::matrix_from_lms::<WithLmsMatrix<O, M>>();

    let input_wp = input_wp
        .unwrap_or_else(|| I::get_xyz().with_white_point())
        .normalize()
        .into_color_unclamped();

    let output_wp = output_wp
        .unwrap_or_else(|| O::get_xyz().with_white_point())
        .normalize()
        .into_color_unclamped();

    input_to_lms
        .then(diagonal_matrix(input_wp, output_wp))
        .then(lms_to_output)
}

/// Construct a diagonal matrix for full adaptation of [`Lms`] colors.
///
/// This is the core matrix in the von Kries adaptation method and is a central
/// part of the matrix from [`adaptation_matrix`]. It's offered separately, as
/// an option for building more advanced adaptation matrices.
///
/// The produced matrix is a diagonal matrix, containing the output white point
/// divided by the input white point:
///
/// ```text
/// [out.l / in.l,            0,            0]
/// [           0, out.m / in.m,            0]
/// [           0,            0, out.s / in.s]
/// ```
///
/// See also [Wikipedia - Von Kries transform][wikipedia].
///
/// [wikipedia]:
///     https://en.wikipedia.org/wiki/Chromatic_adaptation#Von_Kries_transform
#[inline]
pub fn diagonal_matrix<T, I, O>(
    input_wp: Lms<I, T>,
    output_wp: Lms<O, T>,
) -> Matrix3<Lms<I, T>, Lms<O, T>>
where
    T: Zero + Div<Output = T>,
{
    let gain = output_wp / input_wp.with_meta();

    #[rustfmt::skip]
    let matrix = [
        gain.long, T::zero(),   T::zero(),
        T::zero(), gain.medium, T::zero(),
        T::zero(), T::zero(),   gain.short,
    ];

    Matrix3::from_array(matrix)
}

/// A trait for unchecked conversion of one color from another via chromatic
/// adaptation.
///
/// See [`FromColor`][crate::convert::FromColor],
/// [`TryFromColor`][crate::convert::TryFromColor] and [`FromColorUnclamped`]
/// for when there's no need for chromatic adaptation.
///
/// Some conversions require the reference white point to be changed, while
/// maintaining the appearance of the color. This is called "chromatic
/// adaptation" or "white balancing", and typically involves converting the
/// color to the [`Lms`] color space. This trait defaults to using the
/// [`Bradford`] matrix as part of the process, but other options are available
/// in [`lms::matrix`].
///
/// The [`adaptation_matrix`] function offers more options and control. This
/// trait can be a convenient alternative when the source and destination white
/// points are statically known.
pub trait AdaptFromUnclamped<T>: Sized {
    /// The number type that's used as the color's components.
    type Scalar;

    /// Adapt a color of type `T` into a color of type `Self`, using the
    /// [`Bradford`] matrix.
    ///
    /// ```
    /// use palette::{
    ///     Xyz, white_point::{A, C},
    ///     chromatic_adaptation::AdaptFromUnclamped,
    /// };
    ///
    /// let input = Xyz::<A, f32>::new(0.315756, 0.162732, 0.015905);
    ///
    /// //Will convert Xyz<A, f32> to Xyz<C, f32> using Bradford chromatic adaptation:
    /// let output = Xyz::<C, f32>::adapt_from_unclamped(input);
    /// ```
    #[must_use]
    #[inline]
    fn adapt_from_unclamped(input: T) -> Self
    where
        Bradford: LmsToXyz<Self::Scalar> + XyzToLms<Self::Scalar>,
    {
        Self::adapt_from_unclamped_with::<Bradford>(input)
    }

    /// Adapt a color of type `T` into a color of type `Self`, using the custom
    /// matrix `M`.
    ///
    /// ```
    /// use palette::{
    ///     Xyz, white_point::{A, C}, lms::matrix::VonKries,
    ///     chromatic_adaptation::AdaptFromUnclamped,
    /// };
    ///
    /// let input = Xyz::<A, f32>::new(0.315756, 0.162732, 0.015905);
    ///
    /// //Will convert Xyz<A, f32> to Xyz<C, f32> using von Kries chromatic adaptation:
    /// let output = Xyz::<C, f32>::adapt_from_unclamped_with::<VonKries>(input);
    /// ```
    #[must_use]
    fn adapt_from_unclamped_with<M>(input: T) -> Self
    where
        M: LmsToXyz<Self::Scalar> + XyzToLms<Self::Scalar>;
}

/// A trait for unchecked conversion of one color into another via chromatic
/// adaptation.
///
/// See [`IntoColor`][crate::convert::IntoColor],
/// [`TryIntoColor`][crate::convert::TryIntoColor] and [`IntoColorUnclamped`]
/// for when there's no need for chromatic adaptation.
///
/// Some conversions require the reference white point to be changed, while
/// maintaining the appearance of the color. This is called "chromatic
/// adaptation" or "white balancing", and typically involves converting the
/// color to the [`Lms`] color space. This trait defaults to using the
/// [`Bradford`] matrix as part of the process, but other options are available
/// in [`lms::matrix`].
///
/// The [`adaptation_matrix`] function offers more options and control. This
/// trait can be a convenient alternative when the source and destination white
/// points are statically known.
pub trait AdaptIntoUnclamped<T>: Sized {
    /// The number type that's used as the color's components.
    type Scalar;

    /// Adapt a color of type `Self` into a color of type `T`, using the
    /// [`Bradford`] matrix.
    ///
    /// ```
    /// use palette::{
    ///     Xyz, white_point::{A, C},
    ///     chromatic_adaptation::AdaptIntoUnclamped,
    /// };
    ///
    /// let input = Xyz::<A, f32>::new(0.315756, 0.162732, 0.015905);
    ///
    /// //Will convert Xyz<A, f32> to Xyz<C, f32> using Bradford chromatic adaptation:
    /// let output: Xyz<C, f32> = input.adapt_into_unclamped();
    /// ```
    #[must_use]
    #[inline]
    fn adapt_into_unclamped(self) -> T
    where
        Bradford: LmsToXyz<Self::Scalar> + XyzToLms<Self::Scalar>,
    {
        self.adapt_into_unclamped_with::<Bradford>()
    }

    /// Adapt a color of type `Self` into a color of type `T`, using the custom
    /// matrix `M`.
    ///
    /// ```
    /// use palette::{
    ///     Xyz, white_point::{A, C}, lms::matrix::VonKries,
    ///     chromatic_adaptation::AdaptIntoUnclamped,
    /// };
    ///
    /// let input = Xyz::<A, f32>::new(0.315756, 0.162732, 0.015905);
    ///
    /// //Will convert Xyz<A, f32> to Xyz<C, f32> using von Kries chromatic adaptation:
    /// let output: Xyz<C, f32> = input.adapt_into_unclamped_with::<VonKries>();
    /// ```
    #[must_use]
    fn adapt_into_unclamped_with<M>(self) -> T
    where
        M: LmsToXyz<Self::Scalar> + XyzToLms<Self::Scalar>;
}

impl<T, C> AdaptIntoUnclamped<T> for C
where
    T: AdaptFromUnclamped<C>,
{
    type Scalar = T::Scalar;

    #[inline]
    fn adapt_into_unclamped_with<M>(self) -> T
    where
        M: LmsToXyz<Self::Scalar> + XyzToLms<Self::Scalar>,
    {
        T::adapt_from_unclamped_with::<M>(self)
    }
}

/// Chromatic adaptation methods implemented in the library
#[deprecated(
    since = "0.7.7",
    note = "use the options from `palette::lms::matrix` or a custom matrix"
)]
pub enum Method {
    /// Bradford chromatic adaptation method
    Bradford,
    /// VonKries chromatic adaptation method
    VonKries,
    /// XyzScaling chromatic adaptation method
    XyzScaling,
}

/// Holds the matrix coefficients for the chromatic adaptation methods
#[deprecated(
    since = "0.7.7",
    note = "use the options from `palette::lms::matrix` or a custom matrix"
)]
pub struct ConeResponseMatrices<T> {
    ///3x3 matrix for the cone response domains
    pub ma: Mat3<T>,
    ///3x3 matrix for the inverse of the cone response domains
    pub inv_ma: Mat3<T>,
}

/// Generates a conversion matrix to convert the Xyz tristimulus values from
/// one illuminant to another (`source_wp` to `destination_wp`)
#[deprecated(
    since = "0.7.7",
    note = "use the options from `palette::lms::matrix` or a custom matrix"
)]
#[allow(deprecated)]
pub trait TransformMatrix<T>
where
    T: Zero + Arithmetics + Clone,
{
    /// Get the cone response functions for the chromatic adaptation method
    #[must_use]
    fn get_cone_response(&self) -> ConeResponseMatrices<T>;

    /// Generates a 3x3 transformation matrix to convert color from one
    /// reference white point to another with the given cone_response
    #[must_use]
    fn generate_transform_matrix(
        &self,
        source_wp: Xyz<Any, T>,
        destination_wp: Xyz<Any, T>,
    ) -> Mat3<T> {
        let adapt = self.get_cone_response();

        let resp_src: Lms<Any, T> =
            multiply_3x3_and_vec3(adapt.ma.clone(), source_wp.into()).into();
        let resp_dst: Lms<Any, T> =
            multiply_3x3_and_vec3(adapt.ma.clone(), destination_wp.into()).into();

        let resp = diagonal_matrix(resp_src, resp_dst).into_array();

        let tmp = multiply_3x3(resp, adapt.ma);
        multiply_3x3(adapt.inv_ma, tmp)
    }
}

#[allow(deprecated)]
impl<T> TransformMatrix<T> for Method
where
    T: Real + Zero + Arithmetics + Clone,
{
    #[rustfmt::skip]
    #[inline]
    fn get_cone_response(&self) -> ConeResponseMatrices<T> {
        match *self {
             Method::Bradford => {
                ConeResponseMatrices::<T> {
                    ma: lms::matrix::Bradford::xyz_to_lms_matrix(),
                    inv_ma: lms::matrix::Bradford::lms_to_xyz_matrix(),
                }
            }
             Method::VonKries => {
                ConeResponseMatrices::<T> {
                    ma: lms::matrix::VonKries::xyz_to_lms_matrix(),
                    inv_ma: lms::matrix::VonKries::lms_to_xyz_matrix(),
                }
            }
             Method::XyzScaling => {
                ConeResponseMatrices::<T> {
                    ma: lms::matrix::UnitMatrix::xyz_to_lms_matrix(),
                    inv_ma: lms::matrix::UnitMatrix::lms_to_xyz_matrix(),
                }
            }
        }
    }
}

/// Trait to convert color from one reference white point to another
///
/// Converts a color from the source white point (Swp) to the destination white
/// point (Dwp). Uses the bradford method for conversion by default.
#[deprecated(
    since = "0.7.7",
    note = "replaced by `palette::chromatic_adaptation::AdaptFromUnclamped`"
)]
#[allow(deprecated)]
pub trait AdaptFrom<S, Swp, Dwp, T>: Sized
where
    T: Real + Zero + Arithmetics + Clone,
    Swp: WhitePoint<T>,
    Dwp: WhitePoint<T>,
{
    /// Convert the source color to the destination color using the bradford
    /// method by default.
    #[must_use]
    #[inline]
    fn adapt_from(color: S) -> Self {
        Self::adapt_from_using(color, Method::Bradford)
    }
    /// Convert the source color to the destination color using the specified
    /// method.
    #[must_use]
    fn adapt_from_using<M: TransformMatrix<T>>(color: S, method: M) -> Self;
}

#[allow(deprecated)]
impl<S, D, Swp, Dwp, T> AdaptFrom<S, Swp, Dwp, T> for D
where
    T: Real + Zero + Arithmetics + Clone,
    Swp: WhitePoint<T>,
    Dwp: WhitePoint<T>,
    S: IntoColorUnclamped<Xyz<Swp, T>>,
    D: FromColorUnclamped<Xyz<Dwp, T>>,
{
    #[inline]
    fn adapt_from_using<M: TransformMatrix<T>>(color: S, method: M) -> D {
        let src_xyz: Xyz<Swp, T> = color.into_color_unclamped();
        let transform_matrix = method.generate_transform_matrix(Swp::get_xyz(), Dwp::get_xyz());
        let dst_xyz: Xyz<Dwp, T> = multiply_3x3_and_vec3(transform_matrix, src_xyz.into()).into();
        D::from_color_unclamped(dst_xyz)
    }
}

/// Trait to convert color with one reference white point into another
///
/// Converts a color with the source white point (Swp) into the destination
/// white point (Dwp). Uses the bradford method for conversion by default.
#[deprecated(
    since = "0.7.7",
    note = "replaced by `palette::chromatic_adaptation::AdaptIntoUnclamped`"
)]
#[allow(deprecated)]
pub trait AdaptInto<D, Swp, Dwp, T>: Sized
where
    T: Real + Zero + Arithmetics + Clone,
    Swp: WhitePoint<T>,
    Dwp: WhitePoint<T>,
{
    /// Convert the source color to the destination color using the bradford
    /// method by default.
    #[must_use]
    #[inline]
    fn adapt_into(self) -> D {
        self.adapt_into_using(Method::Bradford)
    }
    /// Convert the source color to the destination color using the specified
    /// method.
    #[must_use]
    fn adapt_into_using<M: TransformMatrix<T>>(self, method: M) -> D;
}

#[allow(deprecated)]
impl<S, D, Swp, Dwp, T> AdaptInto<D, Swp, Dwp, T> for S
where
    T: Real + Zero + Arithmetics + Clone,
    Swp: WhitePoint<T>,
    Dwp: WhitePoint<T>,
    D: AdaptFrom<S, Swp, Dwp, T>,
{
    #[inline]
    fn adapt_into_using<M: TransformMatrix<T>>(self, method: M) -> D {
        D::adapt_from_using(self, method)
    }
}

#[cfg(feature = "approx")]
#[cfg(test)]
mod test {
    #![allow(deprecated)]

    use super::{AdaptFrom, AdaptInto, Method, TransformMatrix};
    use crate::{
        encoding::{Linear, Srgb},
        Xyz,
    };
    use crate::{
        rgb::Rgb,
        white_point::{WhitePoint, A, C, D50, D65},
    };

    #[test]
    fn d65_to_d50_matrix_xyz_scaling() {
        let expected = [
            1.0144665, 0.0000000, 0.0000000, 0.0000000, 1.0000000, 0.0000000, 0.0000000, 0.0000000,
            0.7578869,
        ];
        let xyz_scaling = Method::XyzScaling;
        let computed = xyz_scaling.generate_transform_matrix(D65::get_xyz(), D50::get_xyz());
        for (e, c) in expected.iter().zip(computed.iter()) {
            assert_relative_eq!(e, c, epsilon = 0.0001)
        }
    }
    #[test]
    fn d65_to_d50_matrix_von_kries() {
        let expected = [
            1.0160803, 0.0552297, -0.0521326, 0.0060666, 0.9955661, -0.0012235, 0.0000000,
            0.0000000, 0.7578869,
        ];
        let von_kries = Method::VonKries;
        let computed = von_kries.generate_transform_matrix(D65::get_xyz(), D50::get_xyz());
        for (e, c) in expected.iter().zip(computed.iter()) {
            assert_relative_eq!(e, c, epsilon = 0.0001)
        }
    }
    #[test]
    fn d65_to_d50_matrix_bradford() {
        let expected = [
            1.0478112, 0.0228866, -0.0501270, 0.0295424, 0.9904844, -0.0170491, -0.0092345,
            0.0150436, 0.7521316,
        ];
        let bradford = Method::Bradford;
        let computed = bradford.generate_transform_matrix(D65::get_xyz(), D50::get_xyz());
        for (e, c) in expected.iter().zip(computed.iter()) {
            assert_relative_eq!(e, c, epsilon = 0.0001)
        }
    }

    #[test]
    fn chromatic_adaptation_from_a_to_c() {
        let input_a = Xyz::<A, f32>::new(0.315756, 0.162732, 0.015905);

        let expected_bradford = Xyz::<C, f32>::new(0.257963, 0.139776, 0.058825);
        let expected_vonkries = Xyz::<C, f32>::new(0.268446, 0.159139, 0.052843);
        let expected_xyz_scaling = Xyz::<C, f32>::new(0.281868, 0.162732, 0.052844);

        let computed_bradford: Xyz<C, f32> = Xyz::adapt_from(input_a);
        assert_relative_eq!(expected_bradford, computed_bradford, epsilon = 0.0001);

        let computed_vonkries: Xyz<C, f32> = Xyz::adapt_from_using(input_a, Method::VonKries);
        assert_relative_eq!(expected_vonkries, computed_vonkries, epsilon = 0.0001);

        let computed_xyz_scaling: Xyz<C, _> = Xyz::adapt_from_using(input_a, Method::XyzScaling);
        assert_relative_eq!(expected_xyz_scaling, computed_xyz_scaling, epsilon = 0.0001);
    }

    #[test]
    fn chromatic_adaptation_into_a_to_c() {
        let input_a = Xyz::<A, f32>::new(0.315756, 0.162732, 0.015905);

        let expected_bradford = Xyz::<C, f32>::new(0.257963, 0.139776, 0.058825);
        let expected_vonkries = Xyz::<C, f32>::new(0.268446, 0.159139, 0.052843);
        let expected_xyz_scaling = Xyz::<C, f32>::new(0.281868, 0.162732, 0.052844);

        let computed_bradford: Xyz<C, f32> = input_a.adapt_into();
        assert_relative_eq!(expected_bradford, computed_bradford, epsilon = 0.0001);

        let computed_vonkries: Xyz<C, f32> = input_a.adapt_into_using(Method::VonKries);
        assert_relative_eq!(expected_vonkries, computed_vonkries, epsilon = 0.0001);

        let computed_xyz_scaling: Xyz<C, _> = input_a.adapt_into_using(Method::XyzScaling);
        assert_relative_eq!(expected_xyz_scaling, computed_xyz_scaling, epsilon = 0.0001);
    }

    #[test]
    fn d65_to_d50() {
        let input: Rgb<Linear<Srgb>> = Rgb::new(1.0, 1.0, 1.0);
        let expected: Rgb<Linear<(Srgb, D50)>> = Rgb::new(1.0, 1.0, 1.0);

        let computed: Rgb<Linear<(Srgb, D50)>> = input.adapt_into();
        assert_relative_eq!(expected, computed, epsilon = 0.000001);
    }
}
