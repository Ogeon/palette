//!This module defines the red, blue and green primaries for the common Rgb color spaces
use num::Float;


use {Yxy, Xyz, Rgb, IntoColor};
use white_point::WhitePoint;
use matrix::{Mat3, matrix_inverse, multiply_xyz_to_rgb};
use flt;

///Represents the tristimulus values for the Rgb primaries
pub trait RgbVariant<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
{
    ///Tristimulus values for red
    fn red() -> Yxy<Wp, T>;
    ///Tristimulus values for green
    fn green() -> Yxy<Wp, T>;
    ///Tristimulus values for blue
    fn blue() -> Yxy<Wp, T>;
    ///The default gamma value for the rgb color space
    fn gamma() -> T;
    ///Converts linear rgb into the gamma encoded rgb
    fn into_gamma(inp: Rgb<Wp, T>) -> Rgb<Wp, T>{
        let gamma = Self::gamma();
        //TO-DO is this check necessary?
        let inv_gamma = if gamma.is_normal() { T::one() / gamma } else { T::one() };
        Rgb::with_wp(inp.red.powf(inv_gamma), inp.green.powf(inv_gamma), inp.blue.powf(inv_gamma) )
    }
    ///Converts gamma enocded rgb into the linear rgb
    fn into_linear(inp: Rgb<Wp, T>) -> Rgb<Wp, T> {
        let gamma = Self::gamma();
        Rgb::with_wp(inp.red.powf(gamma), inp.green.powf(gamma), inp.blue.powf(gamma) )
    }

    fn mat3_from_primaries() -> Mat3<T> {
        let r: Xyz<Wp, T> = SrgbSpace::red().into_xyz();
        let g: Xyz<Wp, T> = SrgbSpace::green().into_xyz();
        let b: Xyz<Wp, T> = SrgbSpace::blue().into_xyz();

        [
            r.x, g.x, b.x,
            r.y, g.y, b.y,
            r.z, g.z, b.z,
        ]
    }

    ///Geneartes to Rgb to Xyz transformation matrix for the given white point
    fn rgb_to_xyz_matrix() -> Mat3<T> {

        let mut transform_matrix = Self::mat3_from_primaries();

        let s_matrix: Rgb<Wp, T> = multiply_xyz_to_rgb(&matrix_inverse(&transform_matrix), &Wp::get_xyz());

        transform_matrix[0] = transform_matrix[0] * s_matrix.red;
        transform_matrix[1] = transform_matrix[1] * s_matrix.green;
        transform_matrix[2] = transform_matrix[2] * s_matrix.blue;
        transform_matrix[3] = transform_matrix[3] * s_matrix.red;
        transform_matrix[4] = transform_matrix[4] * s_matrix.green;
        transform_matrix[5] = transform_matrix[5] * s_matrix.blue;
        transform_matrix[6] = transform_matrix[6] * s_matrix.red;
        transform_matrix[7] = transform_matrix[7] * s_matrix.green;
        transform_matrix[8] = transform_matrix[8] * s_matrix.blue;

        transform_matrix

    }

    ///Geneartes the Xyz to Rgb transformation matrix for the given white point
    fn xyz_to_rgb_matrix() -> Mat3<T> {
        matrix_inverse(&Self::rgb_to_xyz_matrix())
    }

}

///Srgb color space with default D65 white point
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SrgbSpace;

impl<Wp, T> RgbVariant<Wp, T> for SrgbSpace
    where T: Float,
        Wp: WhitePoint<T>,
{
    fn red() -> Yxy<Wp, T> {
        Yxy::with_wp(flt(0.6400), flt(0.3300), flt(0.212656))
    }
    fn green() -> Yxy<Wp, T> {
        Yxy::with_wp(flt(0.3000), flt(0.6000), flt(0.715158))
    }
    fn blue() -> Yxy<Wp, T> {
        Yxy::with_wp(flt(0.1500), flt(0.0600), flt(0.072186))
    }

    fn gamma() -> T {
        flt(2.2)
    }

}




#[cfg(test)]
mod test {
    use Rgb;
    use chromatic_adaptation::AdaptInto;
    use white_point::{D65,D50};
    use super::rgb_to_xyz_matrix;

    #[test]
    fn d65_rgb_conversion_matrix() {
        let expected = [
            0.4124564, 0.3575761, 0.1804375,
            0.2126729, 0.7151522, 0.0721750,
            0.0193339, 0.1191920, 0.9503041
        ];
        let computed = rgb_to_xyz_matrix::<D65, f64>();
        for (e, c) in expected.iter().zip(computed.iter()) {
            assert_relative_eq!(e, c, epsilon = 0.000001)
        }
    }

    #[test]
    fn d65_to_d50() {
        let input: Rgb<D65> = Rgb::new(1.0, 1.0, 1.0);
        let expected: Rgb<D50> = Rgb::with_wp(1.0, 1.0, 1.0);

        let computed: Rgb<D50> = input.adapt_into();
        assert_relative_eq!(expected, computed, epsilon = 0.000001);
    }

}
