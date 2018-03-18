use num_traits::Float;

use std::ops::{Add, Div, Mul, Sub};
use std::marker::PhantomData;

use {Alpha, LabHue, Lch, Xyz};
use {Component, ComponentWise, GetHue, Limited, Mix, Pixel, Shade};
use {cast, clamp};
use white_point::{D65, WhitePoint};
use encoding::pixel::RawPixel;

/// CIE L\*a\*b\* (CIELAB) with an alpha component. See the [`Laba`
/// implementation in `Alpha`](struct.Alpha.html#Laba).
pub type Laba<Wp, T = f32> = Alpha<Lab<Wp, T>, T>;

///The CIE L\*a\*b\* (CIELAB) color space.
///
///CIE L\*a\*b\* is a device independent color space which includes all
///perceivable colors. It's sometimes used to convert between other color
///spaces, because of its ability to represent all of their colors, and
///sometimes in color manipulation, because of its perceptual uniformity. This
///means that the perceptual difference between two colors is equal to their
///numerical difference.
///
///The parameters of L\*a\*b\* are quite different, compared to many other color
///spaces, so manipulating them manually may be unintuitive.
#[derive(Debug, PartialEq, FromColor)]
#[palette_internal]
#[palette_white_point = "Wp"]
#[palette_component = "T"]
#[palette_manual_from(Xyz, Lab, Lch)]
#[repr(C)]
pub struct Lab<Wp = D65, T = f32>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    ///L\* is the lightness of the color. 0.0 gives absolute black and 100
    ///give the brightest white.
    pub l: T,

    ///a\* goes from red at -128 to green at 127.
    pub a: T,

    ///b\* goes from yellow at -128 to blue at 127.
    pub b: T,

    ///The white point associated with the color's illuminant and observer.
    ///D65 for 2 degree observer is used by default.
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
}

impl<Wp, T> Clone for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    fn clone(&self) -> Lab<Wp, T> {
        *self
    }
}

unsafe impl<Wp: WhitePoint, T: Component + Float> Pixel<T> for Lab<Wp, T> {
    const CHANNELS: usize = 3;
}

impl<T> Lab<D65, T>
where
    T: Component + Float,
{
    ///CIE L\*a\*b\* with white point D65.
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
where
    T: Component + Float,
    Wp: WhitePoint,
{
    ///CIE L\*a\*b\*.
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
where
    T: Component + Float,
{
    ///CIE L\*a\*b\* and transparency and white point D65.
    pub fn new(l: T, a: T, b: T, alpha: T) -> Laba<D65, T> {
        Alpha {
            color: Lab::new(l, a, b),
            alpha: alpha,
        }
    }
}

///<span id="Laba"></span>[`Laba`](type.Laba.html) implementations.
impl<Wp, T> Alpha<Lab<Wp, T>, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    ///CIE L\*a\*b\* and transparency.
    pub fn with_wp(l: T, a: T, b: T, alpha: T) -> Laba<Wp, T> {
        Alpha {
            color: Lab::with_wp(l, a, b),
            alpha: alpha,
        }
    }
}

impl<Wp, T> From<Xyz<Wp, T>> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    fn from(color: Xyz<Wp, T>) -> Self {
        let Xyz {
            mut x,
            mut y,
            mut z,
            ..
        } = color / Wp::get_xyz();

        fn convert<T: Component + Float>(c: T) -> T {
            let epsilon: T = (cast::<T, _>(6.0 / 29.0)).powi(3);
            let kappa: T = cast(841.0 / 108.0);
            let delta: T = cast(4.0 / 29.0);
            if c > epsilon {
                c.powf(T::one() / cast(3.0))
            } else {
                (kappa * c) + delta
            }
        }

        x = convert(x);
        y = convert(y);
        z = convert(z);

        Lab {
            l: ((y * cast(116.0)) - cast(16.0)),
            a: ((x - y) * cast(500.0)),
            b: ((y - z) * cast(200.0)),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> From<Lch<Wp, T>> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    fn from(color: Lch<Wp, T>) -> Self {
        Lab {
            l: color.l,
            a: color.chroma.max(T::zero()) * color.hue.to_radians().cos(),
            b: color.chroma.max(T::zero()) * color.hue.to_radians().sin(),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Limited for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn is_valid(&self) -> bool {
        self.l >= T::zero() && self.l <= cast(100.0) &&
        self.a >= cast(-128) && self.a <= cast(127.0) &&
        self.b >= cast(-128) && self.b <= cast(127.0)
    }

    fn clamp(&self) -> Lab<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, T::zero(), cast(100.0));
        self.a = clamp(self.a, cast(-128.0), cast(127.0));
        self.b = clamp(self.b, cast(-128.0), cast(127.0));
    }
}

impl<Wp, T> Mix for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
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
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Lab<Wp, T> {
        Lab {
            l: self.l + amount * cast(100.0),
            a: self.a,
            b: self.b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> GetHue for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
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
where
    T: Component + Float,
    Wp: WhitePoint,
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
where
    T: Component + Float,
    Wp: WhitePoint,
{
    fn default() -> Lab<Wp, T> {
        Lab::with_wp(T::zero(), T::zero(), T::zero())
    }
}

impl<Wp, T> Add<Lab<Wp, T>> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
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
where
    T: Component + Float,
    Wp: WhitePoint,
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
where
    T: Component + Float,
    Wp: WhitePoint,
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
where
    T: Component + Float,
    Wp: WhitePoint,
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
where
    T: Component + Float,
    Wp: WhitePoint,
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
where
    T: Component + Float,
    Wp: WhitePoint,
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
where
    T: Component + Float,
    Wp: WhitePoint,
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
where
    T: Component + Float,
    Wp: WhitePoint,
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

impl<Wp, T, P> AsRef<P> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<Wp, T, P> AsMut<P> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

#[cfg(test)]
mod test {
    use super::Lab;
    use LinSrgb;
    use white_point::D65;

    #[test]
    fn red() {
        let a = Lab::from(LinSrgb::new(1.0, 0.0, 0.0));
        let b = Lab::new(53.23288, 80.09246, 67.2031);
        assert_relative_eq!(a, b, epsilon = 0.01);
    }

    #[test]
    fn green() {
        let a = Lab::from(LinSrgb::new(0.0, 1.0, 0.0));
        let b = Lab::new(87.73704, -86.184654, 83.18117);
        assert_relative_eq!(a, b, epsilon = 0.01);
    }

    #[test]
    fn blue() {
        let a = Lab::from(LinSrgb::new(0.0, 0.0, 1.0));
        let b = Lab::new(32.302586, 79.19668, -107.863686);
        assert_relative_eq!(a, b, epsilon = 0.01);
    }

    #[test]
    fn ranges() {
        assert_ranges!{
            Lab<D65, f64>;
            limited {
                l: 0.0 => 100.0,
                a: -128.0 => 127.0,
                b: -128.0 => 127.0
            }
            limited_min {}
            unlimited {}
        }
    }

    raw_pixel_conversion_tests!(Lab<D65>: l, a, b);
    raw_pixel_conversion_fail_tests!(Lab<D65>: l, a, b);
}
