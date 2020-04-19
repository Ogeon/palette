//! This module provides simple matrix operations on 3x3 matrices to aid in
//! chromatic adaptation and conversion calculations.

use core::marker::PhantomData;

use crate::convert::IntoColorUnclamped;
use crate::encoding::Linear;
use crate::float::Float;
use crate::rgb::{Primaries, Rgb, RgbSpace};
use crate::white_point::WhitePoint;
use crate::{FloatComponent, Xyz};

/// A 9 element array representing a 3x3 matrix.
pub type Mat3<T> = [T; 9];

/// Multiply the 3x3 matrix with an XYZ color.
#[inline]
pub fn multiply_xyz<Swp: WhitePoint, Dwp: WhitePoint, T: FloatComponent>(
    c: &Mat3<T>,
    f: &Xyz<Swp, T>,
) -> Xyz<Dwp, T> {
    // Input Mat3 is destructured to avoid panic paths
    let [c0, c1, c2, c3, c4, c5, c6, c7, c8] = *c;

    let x1 = c0 * f.x;
    let y1 = c3 * f.x;
    let z1 = c6 * f.x;
    let x2 = c1 * f.y;
    let y2 = c4 * f.y;
    let z2 = c7 * f.y;
    let x3 = c2 * f.z;
    let y3 = c5 * f.z;
    let z3 = c8 * f.z;

    Xyz {
        x: x1 + x2 + x3,
        y: y1 + y2 + y3,
        z: z1 + z2 + z3,
        white_point: PhantomData,
    }
}
/// Multiply the 3x3 matrix with an XYZ color to return an RGB color.
#[inline]
pub fn multiply_xyz_to_rgb<S: RgbSpace, T: FloatComponent>(
    c: &Mat3<T>,
    f: &Xyz<S::WhitePoint, T>,
) -> Rgb<Linear<S>, T> {
    // Input Mat3 is destructured to avoid panic paths. red, green, and blue
    // can't be extracted like in `multiply_xyz` to get a performance increase
    let [c0, c1, c2, c3, c4, c5, c6, c7, c8] = *c;

    Rgb {
        red: (c0 * f.x) + (c1 * f.y) + (c2 * f.z),
        green: (c3 * f.x) + (c4 * f.y) + (c5 * f.z),
        blue: (c6 * f.x) + (c7 * f.y) + (c8 * f.z),
        standard: PhantomData,
    }
}
/// Multiply the 3x3 matrix with an RGB color to return an XYZ color.
#[inline]
pub fn multiply_rgb_to_xyz<S: RgbSpace, T: FloatComponent>(
    c: &Mat3<T>,
    f: &Rgb<Linear<S>, T>,
) -> Xyz<S::WhitePoint, T> {
    // Input Mat3 is destructured to avoid panic paths. Same problem as
    // `multiply_xyz_to_rgb` for extracting x, y, z
    let [c0, c1, c2, c3, c4, c5, c6, c7, c8] = *c;

    Xyz {
        x: (c0 * f.red) + (c1 * f.green) + (c2 * f.blue),
        y: (c3 * f.red) + (c4 * f.green) + (c5 * f.blue),
        z: (c6 * f.red) + (c7 * f.green) + (c8 * f.blue),
        white_point: PhantomData,
    }
}

/// Multiply two 3x3 matrices.
#[inline]
pub fn multiply_3x3<T: Float>(c: &Mat3<T>, f: &Mat3<T>) -> Mat3<T> {
    // Input Mat3 are destructured to avoid panic paths
    let [c0, c1, c2, c3, c4, c5, c6, c7, c8] = *c;
    let [f0, f1, f2, f3, f4, f5, f6, f7, f8] = *f;

    let o0 = c0 * f0 + c1 * f3 + c2 * f6;
    let o1 = c0 * f1 + c1 * f4 + c2 * f7;
    let o2 = c0 * f2 + c1 * f5 + c2 * f8;

    let o3 = c3 * f0 + c4 * f3 + c5 * f6;
    let o4 = c3 * f1 + c4 * f4 + c5 * f7;
    let o5 = c3 * f2 + c4 * f5 + c5 * f8;

    let o6 = c6 * f0 + c7 * f3 + c8 * f6;
    let o7 = c6 * f1 + c7 * f4 + c8 * f7;
    let o8 = c6 * f2 + c7 * f5 + c8 * f8;

    [o0, o1, o2, o3, o4, o5, o6, o7, o8]
}

/// Invert a 3x3 matrix and panic if matrix is not invertible.
#[inline]
pub fn matrix_inverse<T: Float>(a: &Mat3<T>) -> Mat3<T> {
    // This function runs fastest with assert and no destructuring. The `det`'s
    // location should not be changed until benched that it's faster elsewhere
    assert!(a.len() > 8);

    let d0 = a[4] * a[8] - a[5] * a[7];
    let d1 = a[3] * a[8] - a[5] * a[6];
    let d2 = a[3] * a[7] - a[4] * a[6];
    let mut det = a[0] * d0 - a[1] * d1 + a[2] * d2;
    let d3 = a[1] * a[8] - a[2] * a[7];
    let d4 = a[0] * a[8] - a[2] * a[6];
    let d5 = a[0] * a[7] - a[1] * a[6];
    let d6 = a[1] * a[5] - a[2] * a[4];
    let d7 = a[0] * a[5] - a[2] * a[3];
    let d8 = a[0] * a[4] - a[1] * a[3];

    if !det.is_normal() {
        panic!("The given matrix is not invertible")
    }
    det = det.recip();

    [
        d0 * det,
        -d3 * det,
        d6 * det,
        -d1 * det,
        d4 * det,
        -d7 * det,
        d2 * det,
        -d5 * det,
        d8 * det,
    ]
}

/// Generates the Srgb to Xyz transformation matrix for a given white point.
#[inline]
pub fn rgb_to_xyz_matrix<S: RgbSpace, T: FloatComponent>() -> Mat3<T> {
    let r: Xyz<S::WhitePoint, T> = S::Primaries::red().into_color_unclamped();
    let g: Xyz<S::WhitePoint, T> = S::Primaries::green().into_color_unclamped();
    let b: Xyz<S::WhitePoint, T> = S::Primaries::blue().into_color_unclamped();

    // Destructuring has some performance benefits, don't change unless measured
    let [t0, t1, t2, t3, t4, t5, t6, t7, t8] = mat3_from_primaries(r, g, b);

    let s_matrix: Rgb<Linear<S>, T> = multiply_xyz_to_rgb(
        &matrix_inverse(&[t0, t1, t2, t3, t4, t5, t6, t7, t8]),
        &S::WhitePoint::get_xyz(),
    );

    [
        t0 * s_matrix.red,
        t1 * s_matrix.green,
        t2 * s_matrix.blue,
        t3 * s_matrix.red,
        t4 * s_matrix.green,
        t5 * s_matrix.blue,
        t6 * s_matrix.red,
        t7 * s_matrix.green,
        t8 * s_matrix.blue,
    ]
}

#[rustfmt::skip]
#[inline]
fn mat3_from_primaries<T: FloatComponent, Wp: WhitePoint>(r: Xyz<Wp, T>, g: Xyz<Wp, T>, b: Xyz<Wp, T>) -> Mat3<T> {
    [
        r.x, g.x, b.x,
        r.y, g.y, b.y,
        r.z, g.z, b.z,
    ]
}

#[cfg(test)]
mod test {
    use super::{matrix_inverse, multiply_3x3, multiply_xyz, rgb_to_xyz_matrix};
    use crate::chromatic_adaptation::AdaptInto;
    use crate::encoding::{Linear, Srgb};
    use crate::rgb::Rgb;
    use crate::white_point::D50;
    use crate::Xyz;

    #[test]
    fn matrix_multiply_3x3() {
        let inp1 = [1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 1.0, 3.0];
        let inp2 = [4.0, 5.0, 6.0, 6.0, 5.0, 4.0, 4.0, 6.0, 5.0];
        let expected = [28.0, 33.0, 29.0, 28.0, 31.0, 31.0, 26.0, 33.0, 31.0];

        let computed = multiply_3x3(&inp1, &inp2);
        for (t1, t2) in expected.iter().zip(computed.iter()) {
            assert_relative_eq!(t1, t2);
        }
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
        for (t1, t2) in expected.iter().zip(computed.iter()) {
            assert_relative_eq!(t1, t2);
        }
    }
    #[test]
    fn matrix_inverse_check_2() {
        let input: [f64; 9] = [1.0, 0.0, 1.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0];

        let expected: [f64; 9] = [-1.0, -1.0, 2.0, -1.0, 0.0, 1.0, 2.0, 1.0, -2.0];
        let computed = matrix_inverse(&input);
        for (t1, t2) in expected.iter().zip(computed.iter()) {
            assert_relative_eq!(t1, t2);
        }
    }
    #[test]
    #[should_panic]
    fn matrix_inverse_panic() {
        let input: [f64; 9] = [1.0, 0.0, 0.0, 2.0, 0.0, 0.0, -4.0, 6.0, 1.0];
        matrix_inverse(&input);
    }

    #[rustfmt::skip]
    #[test]
    fn d65_rgb_conversion_matrix() {
        let expected = [
            0.4124564, 0.3575761, 0.1804375,
            0.2126729, 0.7151522, 0.0721750,
            0.0193339, 0.1191920, 0.9503041
        ];
        let computed = rgb_to_xyz_matrix::<Srgb, f64>();
        for (e, c) in expected.iter().zip(computed.iter()) {
            assert_relative_eq!(e, c, epsilon = 0.000001)
        }
    }

    #[test]
    fn d65_to_d50() {
        let input: Rgb<Linear<Srgb>> = Rgb::new(1.0, 1.0, 1.0);
        let expected: Rgb<Linear<(Srgb, D50)>> = Rgb::new(1.0, 1.0, 1.0);

        let computed: Rgb<Linear<(Srgb, D50)>> = input.adapt_into();
        assert_relative_eq!(expected, computed, epsilon = 0.000001);
    }
}
