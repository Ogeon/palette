//! Convert colors from one reference white point to another
//!
//! Chromatic adaptation is the human visual system’s ability to adjust to
//! changes in illumination in order to preserve the appearance of object
//! colors. It is responsible for the stable appearance of object colours
//! despite the wide variation of light which might be reflected from an object
//! and observed by our eyes.
//!
//! This library provides three methods for chromatic adaptation Bradford (which
//! is the default), VonKries and XyzScaling
//!
//! ```
//! use palette::Xyz;
//! use palette::white_point::{A, C};
//! use palette::chromatic_adaptation::AdaptInto;
//!
//!
//! let a = Xyz::<A, f32>::with_wp(0.315756, 0.162732, 0.015905);
//!
//! //Will convert Xyz<A, f32> to Xyz<C, f32> using Bradford chromatic adaptation
//! let c: Xyz<C, f32> = a.adapt_into();
//!
//! //Should print {x: 0.257963, y: 0.139776,z: 0.058825}
//! println!("{:?}", c)
//! ```
use crate::convert::{FromColorUnclamped, IntoColorUnclamped};
use crate::float::Float;
use crate::from_f64;
use crate::matrix::{multiply_3x3, multiply_xyz, Mat3};
use crate::white_point::WhitePoint;
use crate::{FloatComponent, Xyz};

/// Chromatic adaptation methods implemented in the library
pub enum Method {
    /// Bradford chromatic adaptation method
    Bradford,
    /// VonKries chromatic adaptation method
    VonKries,
    /// XyzScaling chromatic adaptation method
    XyzScaling,
}

/// Holds the matrix coefficients for the chromatic adaptation methods
pub struct ConeResponseMatrices<T: Float> {
    ///3x3 matrix for the cone response domains
    pub ma: Mat3<T>,
    ///3x3 matrix for the inverse of the cone response domains
    pub inv_ma: Mat3<T>,
}

/// Generates a conversion matrix to convert the Xyz tristimulus values from
/// one illuminant to another (Swp -> Dwp)
pub trait TransformMatrix<Swp, Dwp, T>
where
    T: FloatComponent,
    Swp: WhitePoint,
    Dwp: WhitePoint,
{
    /// Get the cone response functions for the chromatic adaptation method
    fn get_cone_response(&self) -> ConeResponseMatrices<T>;

    /// Generates a 3x3 transformation matrix to convert color from one
    /// reference white point to another with the given cone_response
    fn generate_transform_matrix(&self) -> Mat3<T> {
        let s_wp: Xyz<Swp, T> = Swp::get_xyz();
        let t_wp: Xyz<Dwp, T> = Dwp::get_xyz();
        let adapt = self.get_cone_response();

        let resp_src: Xyz<Swp, _> = multiply_xyz(&adapt.ma, &s_wp);
        let resp_dst: Xyz<Dwp, _> = multiply_xyz(&adapt.ma, &t_wp);
        let z = T::zero();
        let resp = [
            resp_dst.x / resp_src.x,
            z,
            z,
            z,
            resp_dst.y / resp_src.y,
            z,
            z,
            z,
            resp_dst.z / resp_src.z,
        ];

        let tmp = multiply_3x3(&resp, &adapt.ma);
        multiply_3x3(&adapt.inv_ma, &tmp)
    }
}

impl<Swp, Dwp, T> TransformMatrix<Swp, Dwp, T> for Method
where
    T: FloatComponent,
    Swp: WhitePoint,
    Dwp: WhitePoint,
{
    #[rustfmt::skip]
    fn get_cone_response(&self) -> ConeResponseMatrices<T> {
        match *self {
             Method::Bradford => {
                ConeResponseMatrices::<T> {
                    ma: [
                        from_f64(0.8951000), from_f64(0.2664000), from_f64(-0.1614000),
                        from_f64(-0.7502000), from_f64(1.7135000), from_f64(0.0367000),
                        from_f64(0.0389000), from_f64(-0.0685000), from_f64(1.0296000)
                    ],
                    inv_ma: [
                        from_f64(0.9869929), from_f64(-0.1470543), from_f64(0.1599627),
                        from_f64(0.4323053), from_f64(0.5183603), from_f64(0.0492912),
                        from_f64(-0.0085287), from_f64(0.0400428), from_f64(0.9684867)
                    ],
                }
            }
             Method::VonKries => {
                ConeResponseMatrices::<T> {
                    ma: [
                        from_f64(0.4002400), from_f64(0.7076000), from_f64(-0.0808100),
                        from_f64(-0.2263000), from_f64(1.1653200), from_f64(0.0457000),
                        from_f64(0.0000000), from_f64(0.0000000), from_f64(0.9182200)
                    ],
                    inv_ma: [
                        from_f64(1.8599364), from_f64(-1.1293816), from_f64(0.2198974),
                        from_f64(0.3611914), from_f64(0.6388125), from_f64(-0.0000064),
                        from_f64(0.0000000), from_f64(0.0000000), from_f64(1.0890636)
                    ],
                }
            }
             Method::XyzScaling => {
                ConeResponseMatrices::<T> {
                    ma: [
                        from_f64(1.0000000), from_f64(0.0000000), from_f64(0.0000000),
                        from_f64(0.0000000), from_f64(1.0000000), from_f64(0.0000000),
                        from_f64(0.0000000), from_f64(0.0000000), from_f64(1.0000000)
                    ],
                    inv_ma: [
                        from_f64(1.0000000), from_f64(0.0000000), from_f64(0.0000000),
                        from_f64(0.0000000), from_f64(1.0000000), from_f64(0.0000000),
                        from_f64(0.0000000), from_f64(0.0000000), from_f64(1.0000000)
                    ],
                }
            }
        }
    }
}

/// Trait to convert color from one reference white point to another
///
/// Converts a color from the source white point (Swp) to the destination white
/// point (Dwp). Uses the bradford method for conversion by default.
pub trait AdaptFrom<S, Swp, Dwp, T>: Sized
where
    T: FloatComponent,
    Swp: WhitePoint,
    Dwp: WhitePoint,
{
    /// Convert the source color to the destination color using the bradford
    /// method by default
    fn adapt_from(color: S) -> Self {
        Self::adapt_from_using(color, Method::Bradford)
    }
    /// Convert the source color to the destination color using the specified
    /// method
    fn adapt_from_using<M: TransformMatrix<Swp, Dwp, T>>(color: S, method: M) -> Self;
}

impl<S, D, Swp, Dwp, T> AdaptFrom<S, Swp, Dwp, T> for D
where
    T: FloatComponent,
    Swp: WhitePoint,
    Dwp: WhitePoint,
    S: IntoColorUnclamped<Xyz<Swp, T>>,
    D: FromColorUnclamped<Xyz<Dwp, T>>,
{
    fn adapt_from_using<M: TransformMatrix<Swp, Dwp, T>>(color: S, method: M) -> D {
        let src_xyz: Xyz<Swp, T> = color.into_color_unclamped();
        let transform_matrix = method.generate_transform_matrix();
        let dst_xyz: Xyz<Dwp, T> = multiply_xyz(&transform_matrix, &src_xyz);
        D::from_color_unclamped(dst_xyz)
    }
}

/// Trait to convert color with one reference white point into another
///
/// Converts a color with the source white point (Swp) into the destination
/// white point (Dwp). Uses the bradford method for conversion by default.
pub trait AdaptInto<D, Swp, Dwp, T>: Sized
where
    T: FloatComponent,
    Swp: WhitePoint,
    Dwp: WhitePoint,
{
    /// Convert the source color to the destination color using the bradford
    /// method by default
    fn adapt_into(self) -> D {
        self.adapt_into_using(Method::Bradford)
    }
    /// Convert the source color to the destination color using the specified
    /// method
    fn adapt_into_using<M: TransformMatrix<Swp, Dwp, T>>(self, method: M) -> D;
}

impl<S, D, Swp, Dwp, T> AdaptInto<D, Swp, Dwp, T> for S
where
    T: FloatComponent,
    Swp: WhitePoint,
    Dwp: WhitePoint,
    D: AdaptFrom<S, Swp, Dwp, T>,
{
    fn adapt_into_using<M: TransformMatrix<Swp, Dwp, T>>(self, method: M) -> D {
        D::adapt_from_using(self, method)
    }
}

#[cfg(test)]
mod test {
    use super::{AdaptFrom, AdaptInto, Method, TransformMatrix};
    use crate::white_point::{A, C, D50, D65};
    use crate::Xyz;

    #[test]
    fn d65_to_d50_matrix_xyz_scaling() {
        let expected = [
            1.0144665, 0.0000000, 0.0000000, 0.0000000, 1.0000000, 0.0000000, 0.0000000, 0.0000000,
            0.7578869,
        ];
        let xyz_scaling = Method::XyzScaling;
        let computed = <dyn TransformMatrix<D65, D50, _>>::generate_transform_matrix(&xyz_scaling);
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
        let computed = <dyn TransformMatrix<D65, D50, _>>::generate_transform_matrix(&von_kries);
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
        let computed = <dyn TransformMatrix<D65, D50, _>>::generate_transform_matrix(&bradford);
        for (e, c) in expected.iter().zip(computed.iter()) {
            assert_relative_eq!(e, c, epsilon = 0.0001)
        }
    }

    #[test]
    fn chromatic_adaptation_from_a_to_c() {
        let input_a = Xyz::<A, f32>::with_wp(0.315756, 0.162732, 0.015905);

        let expected_bradford = Xyz::<C, f32>::with_wp(0.257963, 0.139776, 0.058825);
        let expected_vonkries = Xyz::<C, f32>::with_wp(0.268446, 0.159139, 0.052843);
        let expected_xyz_scaling = Xyz::<C, f32>::with_wp(0.281868, 0.162732, 0.052844);

        let computed_bradford: Xyz<C, f32> = Xyz::adapt_from(input_a);
        assert_relative_eq!(expected_bradford, computed_bradford, epsilon = 0.0001);

        let computed_vonkries: Xyz<C, f32> = Xyz::adapt_from_using(input_a, Method::VonKries);
        assert_relative_eq!(expected_vonkries, computed_vonkries, epsilon = 0.0001);

        let computed_xyz_scaling: Xyz<C, _> = Xyz::adapt_from_using(input_a, Method::XyzScaling);
        assert_relative_eq!(expected_xyz_scaling, computed_xyz_scaling, epsilon = 0.0001);
    }

    #[test]
    fn chromatic_adaptation_into_a_to_c() {
        let input_a = Xyz::<A, f32>::with_wp(0.315756, 0.162732, 0.015905);

        let expected_bradford = Xyz::<C, f32>::with_wp(0.257963, 0.139776, 0.058825);
        let expected_vonkries = Xyz::<C, f32>::with_wp(0.268446, 0.159139, 0.052843);
        let expected_xyz_scaling = Xyz::<C, f32>::with_wp(0.281868, 0.162732, 0.052844);

        let computed_bradford: Xyz<C, f32> = input_a.adapt_into();
        assert_relative_eq!(expected_bradford, computed_bradford, epsilon = 0.0001);

        let computed_vonkries: Xyz<C, f32> = input_a.adapt_into_using(Method::VonKries);
        assert_relative_eq!(expected_vonkries, computed_vonkries, epsilon = 0.0001);

        let computed_xyz_scaling: Xyz<C, _> = input_a.adapt_into_using(Method::XyzScaling);
        assert_relative_eq!(expected_xyz_scaling, computed_xyz_scaling, epsilon = 0.0001);
    }
}
