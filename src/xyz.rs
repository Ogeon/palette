use num::Float;

use std::ops::{Add, Sub, Mul, Div};
use std::marker::PhantomData;

use {Alpha, Yxy, RgbLinear, Luma, Lab};
use {Limited, Mix, Shade, FromColor, ComponentWise};
use white_point::{WhitePoint, D65};
use profile::Primaries;
use matrix::multiply_rgb_to_xyz;
use {clamp, flt};

///CIE 1931 XYZ with an alpha component. See the [`Xyza` implementation in `Alpha`](struct.Alpha.html#Xyza).
pub type Xyza<Wp = D65, T = f32> = Alpha<Xyz<Wp, T>, T>;

///The CIE 1931 XYZ color space.
///
///XYZ links the perceived colors to their wavelengths and simply makes it
///possible to describe the way we see colors as numbers. It's often used when
///converting from one color space to an other, and requires a standard
///illuminant and a standard observer to be defined.
///
///Conversions and operations on this color space depend on the defined white point
#[derive(Debug, PartialEq)]
pub struct Xyz<Wp = D65, T = f32>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///X is the scale of what can be seen as a response curve for the cone
    ///cells in the human eye. Its range depends
    ///on the white point and goes from 0.0 to 0.95047 for the default D65.
    pub x: T,

    ///Y is the luminance of the color, where 0.0 is black and 1.0 is white.
    pub y: T,

    ///Z is the scale of what can be seen as the blue stimulation. Its range depends
    ///on the white point and goes from 0.0 to 1.08883 for the defautl D65.
    pub z: T,

    ///The white point associated with the color's illuminant and observer.
    ///D65 for 2 degree observer is used by default.
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{}

impl<Wp, T> Clone for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn clone(&self) -> Xyz<Wp, T> { *self }
}

impl<T> Xyz<D65, T>
    where T: Float,
{
    ///CIE XYZ with whtie point D65.
    pub fn new(x: T, y: T, z: T) -> Xyz<D65, T> {
        Xyz {
            x: x,
            y: y,
            z: z,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///CIE XYZ.
    pub fn with_wp(x: T, y: T, z: T) -> Xyz<Wp, T> {
        Xyz {
            x: x,
            y: y,
            z: z,
            white_point: PhantomData,
        }
    }
}

///<span id="Xyza"></span>[`Xyza`](type.Xyza.html) implementations.
impl<T> Alpha<Xyz<D65, T>, T>
    where T: Float,
{
    ///CIE Yxy and transparency with white point D65.
    pub fn new(x: T, y: T, luma: T, alpha: T) -> Xyza<D65, T> {
        Alpha {
            color: Xyz::new(x, y, luma),
            alpha: alpha,
        }
    }
}

///<span id="Xyza"></span>[`Xyza`](type.Xyza.html) implementations.
impl<Wp, T> Alpha<Xyz<Wp, T>, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///CIE XYZ and transparency.
    pub fn with_wp(x: T, y: T, z: T, alpha: T) -> Xyza<Wp, T> {
        Alpha {
            color: Xyz::with_wp(x, y, z),
            alpha: alpha,
        }
    }
}

impl<Wp, T> FromColor<Wp, T> for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        xyz
    }

    fn from_rgb<P: Primaries<Wp, T>>(rgb: RgbLinear<P, Wp, T>) -> Self {
        let transform_matrix = P::rgb_to_xyz_matrix();
        multiply_rgb_to_xyz::<P, Wp, Wp, T>(&transform_matrix, &rgb)
    }

    fn from_yxy(yxy: Yxy<Wp, T>) -> Self {
        let mut xyz = Xyz { y: yxy.luma, ..Default::default() };
        // If denominator is zero, NAN or INFINITE leave x and z at the default 0
        if yxy.y.is_normal() {
            xyz.x = yxy.luma * yxy.x / yxy.y;
            xyz.z = yxy.luma * ( T::one() - yxy.x - yxy.y ) / yxy.y;
        }
        xyz
    }

    fn from_lab(input_lab: Lab<Wp, T>) -> Self {
        let mut lab = input_lab.clone();
        lab.l = lab.l * flt(100.0);
        lab.a = lab.a * flt(128.0);
        lab.b = lab.b * flt(128.0);
        let y = (lab.l + flt(16.0)) / flt(116.0);
        let x = y + (lab.a / flt(500.0));
        let z = y - (lab.b / flt(200.0));


        fn convert<T: Float>(c: T) -> T {
            let epsilon: T = flt(6.0 / 29.0);
            let kappa: T = flt(108.0 / 841.0);
            let delta: T = flt(4.0 / 29.0);

            if c > epsilon {
                c.powi(3)
            } else {
                (c - delta) * kappa
            }
        }
        let xyz_ref = Wp::get_xyz();
        Xyz {
            x: convert(x) * xyz_ref.x,
            y: convert(y) * xyz_ref.y,
            z: convert(z) * xyz_ref.z,
            white_point: PhantomData,
        }
    }
    fn from_luma(luma: Luma<Wp, T>) -> Self {
        let xyz_ref = Wp::get_xyz();
        Xyz {
            x: luma.luma * xyz_ref.x,
            y: luma.luma,
            z: luma.luma * xyz_ref.z,
            white_point: PhantomData,
        }
    }

}

impl<Wp, T> Limited for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn is_valid(&self) -> bool {
        let xyz_ref = Wp::get_xyz();
        self.x >= T::zero() && self.x <= xyz_ref.x &&
        self.y >= T::zero() && self.y <= xyz_ref.y &&
        self.z >= T::zero() && self.z <= xyz_ref.z
    }

    fn clamp(&self) -> Xyz<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        let xyz_ref = Wp::get_xyz();
        self.x = clamp(self.x, T::zero(), xyz_ref.x);
        self.y = clamp(self.y, T::zero(), xyz_ref.y);
        self.z = clamp(self.z, T::zero(), xyz_ref.z);
    }
}

impl<Wp, T> Mix for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn mix(&self, other: &Xyz<Wp, T>, factor: T) -> Xyz<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());

        Xyz {
            x: self.x + factor * (other.x - self.x),
            y: self.y + factor * (other.y - self.y),
            z: self.z + factor * (other.z - self.z),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Xyz<Wp, T> {
        Xyz {
            x: self.x,
            y: self.y + amount,
            z: self.z,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> ComponentWise for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Xyz<Wp, T>, mut f: F) -> Xyz<Wp, T> {
        Xyz {
            x: f(self.x, other.x),
            y: f(self.y, other.y),
            z: f(self.z, other.z),
            white_point: PhantomData,
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Xyz<Wp, T> {
        Xyz {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn default() -> Xyz<Wp, T> {
        Xyz::with_wp(T::zero(), T::zero(), T::zero())
    }
}

impl<Wp, T> Add<Xyz<Wp, T>> for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Xyz<Wp, T>;

    fn add(self, other: Xyz<Wp, T>) -> Xyz<Wp, T> {
        Xyz {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Add<T> for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Xyz<Wp, T>;

    fn add(self, c: T) -> Xyz<Wp, T> {
        Xyz {
            x: self.x + c,
            y: self.y + c,
            z: self.z + c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<Xyz<Wp, T>> for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Xyz<Wp, T>;

    fn sub(self, other: Xyz<Wp, T>) -> Xyz<Wp, T> {
        Xyz {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<T> for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Xyz<Wp, T>;

    fn sub(self, c: T) -> Xyz<Wp, T> {
        Xyz {
            x: self.x - c,
            y: self.y - c,
            z: self.z - c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Mul<Xyz<Wp, T>> for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Xyz<Wp, T>;

    fn mul(self, other: Xyz<Wp, T>) -> Xyz<Wp, T> {
        Xyz {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Mul<T> for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Xyz<Wp, T>;

    fn mul(self, c: T) -> Xyz<Wp, T> {
        Xyz {
            x: self.x * c,
            y: self.y * c,
            z: self.z * c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Div<Xyz<Wp, T>> for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Xyz<Wp, T>;

    fn div(self, other: Xyz<Wp, T>) -> Xyz<Wp, T> {
        Xyz {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Div<T> for Xyz<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Xyz<Wp, T>;

    fn div(self, c: T) -> Xyz<Wp, T> {
        Xyz {
            x: self.x / c,
            y: self.y / c,
            z: self.z / c,
            white_point: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Xyz;
    use RgbLinear;
    use Luma;
    const X_N: f64 = 0.95047;
    const Y_N: f64 = 1.0;
    const Z_N: f64 = 1.08883;

    #[test]
    fn luma() {
        let a = Xyz::from(Luma::new(0.5));
        let b = Xyz::new(0.475235, 0.5, 0.544415);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn red() {
        let a = Xyz::from(RgbLinear::new(1.0, 0.0, 0.0));
        let b = Xyz::new(0.41240, 0.21260, 0.01930);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn green() {
        let a = Xyz::from(RgbLinear::new(0.0, 1.0, 0.0));
        let b = Xyz::new(0.35760, 0.71520, 0.11920);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn blue() {
        let a = Xyz::from(RgbLinear::new(0.0, 0.0, 1.0));
        let b = Xyz::new(0.18050, 0.07220, 0.95030);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn ranges() {
        assert_ranges!{
            Xyz;
            limited {
                x: 0.0 => X_N,
                y: 0.0 => Y_N,
                z: 0.0 => Z_N
            }
            limited_min {}
            unlimited {}
        }
    }
}
