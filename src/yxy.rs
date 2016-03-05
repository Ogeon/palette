use num::Float;

use std::ops::{Add, Sub, Mul, Div};
use std::marker::PhantomData;

use {Alpha, Luma, Xyz};
use {Limited, Mix, Shade, FromColor, IntoColor, ComponentWise};
use white_point::{WhitePoint, D65};
use clamp;

///CIE 1931 Yxy (xyY) with an alpha component. See the [`Yxya` implementation in `Alpha`](struct.Alpha.html#Yxya).
pub type Yxya<Wp = D65, T = f32> = Alpha<Yxy<Wp, T>, T>;

///The CIE 1931 Yxy (xyY)  color space.
///
///Yxy is a luminance-chromaticity color space derived from the CIE XYZ
///color space. It is widely used to define colors. The chromacity diagrams
///for the color spaces are a plot of this color space's x and y coordiantes.
///
///Conversions and operations on this color space depend on the white point.
#[derive(Debug, PartialEq)]
pub struct Yxy<Wp = D65, T = f32>
    where T: Float,
        Wp: WhitePoint<T>
{

    ///x chromacity co-ordinate derived from XYZ color space as X/(X+Y+Z).
    ///Typical range is between 0 and 1
    pub x: T,

    ///y chromacity co-ordinate derived from XYZ color space as Y/(X+Y+Z).
    ///Typical range is between 0 and 1
    pub y: T,

    ///luma (Y) was a measure of the brightness or luminance of a color.
    ///It is the same as the Y from the XYZ color space. Its range is from
    ///0 to 1, where 0 is black and 1 is white.
    pub luma: T,

    ///The white point associated with the color's illuminant and observer.
    ///D65 for 2 degree observer is used by default.
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{}

impl<Wp, T> Clone for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn clone(&self) -> Yxy<Wp, T> { *self }
}

impl<T> Yxy<D65, T>
    where T: Float,
{
    ///CIE Yxy with white point D65.
    pub fn new(x: T, y: T, luma: T,) -> Yxy<D65, T> {
        Yxy {
            x: x,
            y: y,
            luma: luma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///CIE Yxy.
    pub fn with_wp(x: T, y: T, luma: T,) -> Yxy<Wp, T> {
        Yxy {
            x: x,
            y: y,
            luma: luma,
            white_point: PhantomData,
        }
    }
}

///<span id="Yxya"></span>[`Yxya`](type.Yxya.html) implementations.
impl<T> Alpha<Yxy<D65, T>, T>
    where T: Float,
{
    ///CIE Yxy and transparency with white point D65.
    pub fn new(x: T, y: T, luma: T, alpha: T) -> Yxya<D65, T> {
        Alpha {
            color: Yxy::new(x, y, luma),
            alpha: alpha,
        }
    }
}
///<span id="Yxya"></span>[`Yxya`](type.Yxya.html) implementations.
impl<Wp, T> Alpha<Yxy<Wp, T>, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///CIE Yxy and transparency.
    pub fn with_wp(x: T, y: T, luma: T, alpha: T) -> Yxya<Wp, T> {
        Alpha {
            color: Yxy::with_wp(x, y, luma),
            alpha: alpha,
        }
    }
}

impl<Wp, T> FromColor<Wp, T> for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        let mut yxy = Yxy{ x: T::zero(), y: T::zero(), luma: xyz.y, white_point: PhantomData };
        let sum = xyz.x + xyz.y + xyz.z;
        // If denominator is zero, NAN or INFINITE leave x and y at the default 0
        if sum.is_normal() {
            yxy.x = xyz.x / sum;
            yxy.y = xyz.y / sum;
        }
        yxy
    }

    fn from_yxy(yxy: Yxy<Wp, T>) -> Self {
        yxy
    }

    // direct conversion implemented in Luma
    fn from_luma(luma: Luma<Wp, T>) -> Self {
        Yxy { luma: luma.luma, ..Default::default() }
    }

}

impl<Wp, T> Limited for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn is_valid(&self) -> bool {
        self.x >= T::zero() && self.x <= T::one() &&
        self.y >= T::zero() && self.y <= T::one() &&
        self.luma >= T::zero() && self.luma <= T::one()
    }

    fn clamp(&self) -> Yxy<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.x= clamp(self.x, T::zero(), T::one());
        self.y = clamp(self.y, T::zero(), T::one());
        self.luma = clamp(self.luma, T::zero(), T::one());
    }
}

impl<Wp, T> Mix for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn mix(&self, other: &Yxy<Wp, T>, factor: T) -> Yxy<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());

        Yxy {
            x: self.x + factor * (other.x - self.x),
            y: self.y + factor * (other.y - self.y),
            luma: self.luma + factor * (other.luma - self.luma),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Yxy<Wp, T> {
        Yxy {
            x: self.x,
            y: self.y,
            luma: self.luma + amount,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> ComponentWise for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Yxy<Wp, T>, mut f: F) -> Yxy<Wp, T> {
        Yxy {
            x: f(self.x, other.x),
            y: f(self.y, other.y),
            luma: f(self.luma, other.luma),
            white_point: PhantomData,
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Yxy<Wp, T> {
        Yxy {
            x: f(self.x),
            y: f(self.y),
            luma: f(self.luma),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn default() -> Yxy<Wp, T> {
        // The default for x and y are the white point x and y ( from the default D65).
        // Since Y (luma) is 0.0, this makes the default color black just like for other colors.
        // The reason for not using 0 for x and y is that this outside the usual color gamut and might
        // cause scaling issues.
        let yxy_wp = Wp::get_xyz().into_yxy();
        Yxy::with_wp(yxy_wp.x, yxy_wp.y, T::zero())
    }
}

impl<Wp, T> Add<Yxy<Wp, T>> for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Yxy<Wp, T>;

    fn add(self, other: Yxy<Wp, T>) -> Yxy<Wp, T> {
        Yxy {
            x: self.x + other.x,
            y: self.y + other.y,
            luma: self.luma + other.luma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Add<T> for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Yxy<Wp, T>;

    fn add(self, c: T) -> Yxy<Wp, T> {
        Yxy {
            x: self.x + c,
            y: self.y + c,
            luma: self.luma + c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<Yxy<Wp, T>> for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Yxy<Wp, T>;

    fn sub(self, other: Yxy<Wp, T>) -> Yxy<Wp, T> {
        Yxy {
            x: self.x - other.x,
            y: self.y - other.y,
            luma: self.luma - other.luma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<T> for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Yxy<Wp, T>;

    fn sub(self, c: T) -> Yxy<Wp, T> {
        Yxy {
            x: self.x - c,
            y: self.y - c,
            luma: self.luma - c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Mul<Yxy<Wp, T>> for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Yxy<Wp, T>;

    fn mul(self, other: Yxy<Wp, T>) -> Yxy<Wp, T> {
        Yxy {
            x: self.x * other.x,
            y: self.y * other.y,
            luma: self.luma * other.luma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Mul<T> for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Yxy<Wp, T>;

    fn mul(self, c: T) -> Yxy<Wp, T> {
        Yxy {
            x: self.x * c,
            y: self.y * c,
            luma: self.luma * c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Div<Yxy<Wp, T>> for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Yxy<Wp, T>;

    fn div(self, other: Yxy<Wp, T>) -> Yxy<Wp, T> {
        Yxy {
            x: self.x / other.x,
            y: self.y / other.y,
            luma: self.luma / other.luma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Div<T> for Yxy<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Yxy<Wp, T>;

    fn div(self, c: T) -> Yxy<Wp, T> {
        Yxy {
            x: self.x / c,
            y: self.y / c,
            luma: self.luma / c,
            white_point: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Yxy;
    use Rgb;
    use Luma;

    #[test]
    fn luma() {
        let a = Yxy::from(Luma::new(0.5));
        let b = Yxy::new(0.312727, 0.329023, 0.5);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn red() {
        let a = Yxy::from(Rgb::new(1.0, 0.0, 0.0));
        let b = Yxy::new(0.64, 0.33, 0.212673);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn green() {
        let a = Yxy::from(Rgb::new(0.0, 1.0, 0.0));
        let b = Yxy::new(0.3, 0.6, 0.715152);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn blue() {
        let a = Yxy::from(Rgb::new(0.0, 0.0, 1.0));
        let b = Yxy::new(0.15, 0.06, 0.072175);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn ranges() {
        assert_ranges!{
            Yxy;
            limited {
                x: 0.0 => 1.0,
                y: 0.0 => 1.0,
                luma: 0.0 => 1.0
            }
            limited_min {}
            unlimited {}
        }
    }
}
