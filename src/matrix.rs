//!This module provides simple matrix operations on 3x3 matrix to aid in chromatic adaptation and
//!conversion calculations.

use num::Float;

use std::marker::PhantomData;

use {Xyz, RgbLinear};
use white_point::WhitePoint;
use profile::Primaries;

///A 9 element array representing a 3x3 matrix
pub type Mat3<T> = [T;9];

///Multiply the 3x3 matrix with the XYZ color
pub fn multiply_xyz<Swp, Dwp, T>(c: &Mat3<T>, f: &Xyz<Swp, T>) -> Xyz<Dwp, T>
    where T: Float,
        Swp: WhitePoint<T>,
        Dwp: WhitePoint<T>,
{
    Xyz {
        x: (c[0] * f.x) + (c[1] * f.y) + (c[2] * f.z),
        y: (c[3] * f.x) + (c[4] * f.y) + (c[5] * f.z),
        z: (c[6] * f.x) + (c[7] * f.y) + (c[8] * f.z),
        white_point: PhantomData,
    }
}
///Multiply the 3x3 matrix with the XYZ color into RGB color
pub fn multiply_xyz_to_rgb<C, Swp, Dwp, T>(c: &Mat3<T>, f: &Xyz<Swp, T>) -> RgbLinear<C, Dwp, T>
    where T: Float,
        Swp: WhitePoint<T>,
        Dwp: WhitePoint<T>,
        C: Primaries<Dwp, T>
{
    RgbLinear {
        red: (c[0] * f.x) + (c[1] * f.y) + (c[2] * f.z),
        green: (c[3] * f.x) + (c[4] * f.y) + (c[5] * f.z),
        blue: (c[6] * f.x) + (c[7] * f.y) + (c[8] * f.z),
        white_point: PhantomData,
        primaries: PhantomData,
    }
}
///Multiply the 3x3 matrix with the  RGB into XYZ color
pub fn multiply_rgb_to_xyz<P, Swp, Dwp, T>(c: &Mat3<T>, f: &RgbLinear<P, Swp, T>) -> Xyz<Dwp, T>
    where T: Float,
        Swp: WhitePoint<T>,
        Dwp: WhitePoint<T>,
        P: Primaries<Swp, T>
{
    Xyz {
        x: (c[0] * f.red) + (c[1] * f.green) + (c[2] * f.blue),
        y: (c[3] * f.red) + (c[4] * f.green) + (c[5] * f.blue),
        z: (c[6] * f.red) + (c[7] * f.green) + (c[8] * f.blue),
        white_point: PhantomData,
    }
}

///Multiply a 3x3 matrix with another 3x3 matrix
pub fn multiply_3x3<T: Float>(c: &Mat3<T>, f: &Mat3<T>) -> Mat3<T> {
    let mut out = [T::zero();9];
    out[0] = c[0] * f[0] + c[1] * f[3] + c[2] * f[6];
    out[1] = c[0] * f[1] + c[1] * f[4] + c[2] * f[7];
    out[2] = c[0] * f[2] + c[1] * f[5] + c[2] * f[8];

    out[3] = c[3] * f[0] + c[4] * f[3] + c[5] * f[6];
    out[4] = c[3] * f[1] + c[4] * f[4] + c[5] * f[7];
    out[5] = c[3] * f[2] + c[4] * f[5] + c[5] * f[8];

    out[6] = c[6] * f[0] + c[7] * f[3] + c[8] * f[6];
    out[7] = c[6] * f[1] + c[7] * f[4] + c[8] * f[7];
    out[8] = c[6] * f[2] + c[7] * f[5] + c[8] * f[8];

    out
}

///Invert a 3x3 matrix and panic if matrix is not invertable.
pub fn matrix_inverse<T: Float>(a: &Mat3<T>) -> Mat3<T> {
    let d0 = a[4] * a[8] - a[5] * a[7];
    let d1 = a[3] * a[8] - a[5] * a[6];
    let d2 = a[3] * a[7] - a[4] * a[6];
    let det =  a[0] * d0 - a[1] * d1 + a[2] * d2;
    if !det.is_normal() {
        panic!("The given matrix is not invertable")
    }
    let d3 = a[1] * a[8] - a[2] * a[7];
    let d4 = a[0] * a[8] - a[2] * a[6];
    let d5 = a[0] * a[7] - a[1] * a[6];
    let d6 = a[1] * a[5] - a[2] * a[4];
    let d7 = a[0] * a[5] - a[2] * a[3];
    let d8 = a[0] * a[4] - a[1] * a[3];

    [d0/det, -d3/det, d6/det, -d1/det, d4/det, -d7/det, d2/det, -d5/det, d8/det]
}


#[cfg(test)]
mod test {
    use Xyz;
    use super::{matrix_inverse, multiply_3x3, multiply_xyz};
    #[test]
    fn matrix_multiply_3x3() {
        let inp1 = [1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 1.0, 3.0];
        let inp2 = [4.0, 5.0, 6.0, 6.0, 5.0, 4.0, 4.0, 6.0, 5.0];
        let expected = [28.0, 33.0, 29.0, 28.0, 31.0, 31.0, 26.0, 33.0, 31.0];

        let computed = multiply_3x3(&inp1, &inp2);
        assert_eq!(expected, computed)
    }

    #[test]
    fn matrix_multiply_xyz() {
        let inp1 = [0.1, 0.2, 0.3, 0.3, 0.2, 0.1, 0.2, 0.1, 0.3];
        let inp2 = Xyz::new(0.4, 0.6, 0.8);

        let expected = Xyz::new(0.4, 0.32, 0.38);

        let computed = multiply_xyz(&inp1, &inp2);
        assert_relative_eq!(expected, computed)
    }

    #[test]
    fn matrix_inverse_check_1() {
        let input: [f64; 9] = [3.0, 0.0, 2.0, 2.0, 0.0, -2.0, 0.0, 1.0, 1.0];

        let expected: [f64; 9] = [0.2, 0.2, 0.0, -0.2, 0.3, 1.0, 0.2, -0.3, 0.0];
        let computed = matrix_inverse(&input);
        assert_eq!(expected, computed);
    }
    #[test]
    fn matrix_inverse_check_2() {
        let input: [f64; 9] = [1.0, 0.0, 1.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0];

        let expected: [f64; 9] = [-1.0, -1.0, 2.0, -1.0, 0.0, 1.0, 2.0, 1.0, -2.0];
        let computed = matrix_inverse(&input);
        assert_eq!(expected, computed);
    }
    #[test]
    #[should_panic]
    fn matrix_inverse_panic() {
        let input: [f64; 9] = [1.0, 0.0, 0.0, 2.0, 0.0, 0.0, -4.0, 6.0, 1.0];
        matrix_inverse(&input);
    }
}
