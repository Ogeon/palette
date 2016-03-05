use num::Float;

use std::ops::{Add, Sub, Mul, Div};
use std::marker::PhantomData;

use {Alpha, Xyz, Lch, LabHue};
use {Limited, Mix, Shade, GetHue, FromColor, ComponentWise};
use {clamp, flt};
use white_point::{WhitePoint, D65};

///CIE L*a*b* (CIELAB) with an alpha component. See the [`Laba` implementation in `Alpha`](struct.Alpha.html#Laba).
pub type Laba<Wp, T = f32> = Alpha<Lab<Wp, T>, T>;

///The CIE L*a*b* (CIELAB) color space.
///
///CIE L*a*b* is a device independent color space which includes all
///perceivable colors. It's sometimes used to convert between other color
///spaces, because of its ability to represent all of their colors, and
///sometimes in color manipulation, because of its perceptual uniformity. This
///means that the perceptual difference between two colors is equal to their
///numerical difference.
///
///The parameters of L*a*b* are quite different, compared to many other color
///spaces, so manipulating them manually can be unintuitive.
#[derive(Debug, PartialEq)]
pub struct Lab<Wp = D65, T = f32>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///L* is the lightness of the color. 0.0 gives absolute black and 1.0
    ///give the brightest white.
    pub l: T,

    ///a* goes from red at -1.0 to green at 1.0.
    pub a: T,

    ///b* goes from yellow at -1.0 to blue at 1.0.
    pub b: T,

    ///The white point associated with the color's illuminant and observer.
    ///D65 for 2 degree observer is used by default.
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{}

impl<Wp, T> Clone for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn clone(&self) -> Lab<Wp, T> { *self }
}

impl<T> Lab<D65, T>
    where T: Float,
{
    ///CIE L*a*b* with white point D65.
    pub fn new(l: T, a: T, b: T) -> Lab<D65, T> {
        Lab {
            l: l,
            a: a,
            b: b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///CIE L*a*b*.
    pub fn with_wp(l: T, a: T, b: T) -> Lab<Wp, T> {
        Lab {
            l: l,
            a: a,
            b: b,
            white_point: PhantomData,
        }
    }
}

///<span id="Laba"></span>[`Laba`](type.Laba.html) implementations.
impl<T> Alpha<Lab<D65, T>, T>
    where T: Float,
{
    ///CIE L*a*b* and transparency and white point D65.
    pub fn new(l: T, a: T, b: T, alpha: T) -> Laba<D65, T> {
        Alpha {
            color: Lab::new(l, a, b),
            alpha: alpha,
        }
    }
}

///<span id="Laba"></span>[`Laba`](type.Laba.html) implementations.
impl<Wp, T> Alpha<Lab<Wp, T>, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///CIE L*a*b* and transparency.
    pub fn with_wp(l: T, a: T, b: T, alpha: T) -> Laba<Wp, T> {
        Alpha {
            color: Lab::with_wp(l, a, b),
            alpha: alpha,
        }
    }
}

impl<Wp, T> FromColor<Wp, T> for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        let xyz_ref = Wp::get_xyz();
        let mut x: T = xyz.x / xyz_ref.x;
        let mut y: T = xyz.y / xyz_ref.y;
        let mut z: T = xyz.z / xyz_ref.z;

        fn convert<T: Float>(c: T) -> T {
            let epsilon: T = (flt::<T,_>(6.0 / 29.0)).powi(3);
            let kappa: T = flt(841.0 / 108.0);
            let delta: T = flt(4.0 / 29.0);
            if c > epsilon {
                c.powf(T::one() / flt(3.0))
            } else {
                (kappa * c ) + delta
            }
        }

        x = convert(x);
        y = convert(y);
        z = convert(z);

        Lab {
            l: ( (y * flt(116.0)) - flt(16.0) ) / flt(100.0),
            a: ( (x - y) * flt(500.0) ) / flt(128.0),
            b: ( (y - z) * flt(200.0) ) / flt(128.0),
            white_point: PhantomData,
        }
    }

    fn from_lab(lab: Lab<Wp, T>) -> Self {
        lab
    }

    fn from_lch(lch: Lch<Wp, T>) -> Self {
        Lab {
            l: lch.l,
            a: lch.chroma.max(T::zero()) * lch.hue.to_radians().cos(),
            b: lch.chroma.max(T::zero()) * lch.hue.to_radians().sin(),
            white_point: PhantomData,
        }
    }


}

impl<Wp, T> Limited for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn is_valid(&self) -> bool {
        self.l >= T::zero() && self.l <= T::one() &&
        self.a >= -T::one() && self.a <= T::one() &&
        self.b >= -T::one() && self.b <= T::one()
    }

    fn clamp(&self) -> Lab<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, T::zero(), T::one());
        self.a = clamp(self.a, -T::one(), T::one());
        self.b = clamp(self.b, -T::one(), T::one());
    }
}

impl<Wp, T> Mix for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn mix(&self, other: &Lab<Wp, T>, factor: T) -> Lab<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());

        Lab {
            l: self.l + factor * (other.l - self.l),
            a: self.a + factor * (other.a - self.a),
            b: self.b + factor * (other.b - self.b),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Lab<Wp, T> {
        Lab {
            l: self.l + amount,
            a: self.a,
            b: self.b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> GetHue for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Hue = LabHue<T>;

    fn get_hue(&self) -> Option<LabHue<T>> {
        if self.a == T::zero() && self.b == T::zero() {
            None
        } else {
            Some(LabHue::from_radians(self.b.atan2(self.a)))
        }
    }
}

impl<Wp, T> ComponentWise for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Lab<Wp, T>, mut f: F) -> Lab<Wp, T> {
        Lab {
            l: f(self.l, other.l),
            a: f(self.a, other.a),
            b: f(self.b, other.b),
            white_point: PhantomData,
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Lab<Wp, T> {
        Lab {
            l: f(self.l),
            a: f(self.a),
            b: f(self.b),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn default() -> Lab<Wp, T> {
        Lab::with_wp(T::zero(), T::zero(), T::zero())
    }
}

impl<Wp, T> Add<Lab<Wp, T>> for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Lab<Wp, T>;

    fn add(self, other: Lab<Wp, T>) -> Lab<Wp, T> {
        Lab {
            l: self.l + other.l,
            a: self.a + other.a,
            b: self.b + other.b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Add<T> for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Lab<Wp, T>;

    fn add(self, c: T) -> Lab<Wp, T> {
        Lab {
            l: self.l + c,
            a: self.a + c,
            b: self.b + c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<Lab<Wp, T>> for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Lab<Wp, T>;

    fn sub(self, other: Lab<Wp, T>) -> Lab<Wp, T> {
        Lab {
            l: self.l - other.l,
            a: self.a - other.a,
            b: self.b - other.b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<T> for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Lab<Wp, T>;

    fn sub(self, c: T) -> Lab<Wp, T> {
        Lab {
            l: self.l - c,
            a: self.a - c,
            b: self.b - c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Mul<Lab<Wp, T>> for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Lab<Wp, T>;

    fn mul(self, other: Lab<Wp, T>) -> Lab<Wp, T> {
        Lab {
            l: self.l * other.l,
            a: self.a * other.a,
            b: self.b * other.b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Mul<T> for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Lab<Wp, T>;

    fn mul(self, c: T) -> Lab<Wp, T> {
        Lab {
            l: self.l * c,
            a: self.a * c,
            b: self.b * c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Div<Lab<Wp, T>> for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Lab<Wp, T>;

    fn div(self, other: Lab<Wp, T>) -> Lab<Wp, T> {
        Lab {
            l: self.l / other.l,
            a: self.a / other.a,
            b: self.b / other.b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Div<T> for Lab<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Lab<Wp, T>;

    fn div(self, c: T) -> Lab<Wp, T> {
        Lab {
            l: self.l / c,
            a: self.a / c,
            b: self.b / c,
            white_point: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Lab;
    use ::Rgb;

    #[test]
    fn red() {
        let a = Lab::from(Rgb::new(1.0, 0.0, 0.0));
        let b = Lab::new(53.23288 / 100.0, 80.09246 / 128.0, 67.2031 / 128.0);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn green() {
        let a = Lab::from(Rgb::new(0.0, 1.0, 0.0));
        let b = Lab::new(87.73704 / 100.0, -86.184654 / 128.0, 83.18117 / 128.0);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn blue() {
        let a = Lab::from(Rgb::new(0.0, 0.0, 1.0));
        let b = Lab::new(32.302586 / 100.0, 79.19668 / 128.0, -107.863686 / 128.0);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn ranges() {
        assert_ranges!{
            Lab;
            limited {
                l: 0.0 => 1.0,
                a: -1.0 => 1.0,
                b: -1.0 => 1.0
            }
            limited_min {}
            unlimited {}
        }
    }
}
