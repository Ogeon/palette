use num::Float;

use std::ops::{Add, Sub, Mul, Div};
use std::marker::PhantomData;

use { Alpha, Rgb, Luma, Xyz, Yxy, Hsv, Hsl, Limited, Mix, Shade, GetHue, LabHue, WhitePoint, D65, clamp, flt};
use lch::LchSpace;

pub type Lab<T> = LabSpace<D65, T>;
pub type Laba<T> = AlphaLabSpace<D65, T>;

///CIE L*a*b* (CIELAB) with an alpha component. See the [`Laba` implementation in `Alpha`](struct.Alpha.html#Laba).
pub type AlphaLabSpace<WP, T = f32> = Alpha<LabSpace<WP,T>, T>;

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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LabSpace<WP = D65, T = f32>
    where T: Float,
        WP: WhitePoint<T>
{
    ///L* is the lightness of the color. 0.0 gives absolute black and 1.0
    ///give the brightest white.
    pub l: T,

    ///a* goes from red at -1.0 to green at 1.0.
    pub a: T,

    ///b* goes from yellow at -1.0 to blue at 1.0.
    pub b: T,

    _wp: PhantomData<WP>,
}

impl<WP, T> LabSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    ///CIE L*a*b*.
    pub fn new(l: T, a: T, b: T) -> LabSpace<WP, T> {
        LabSpace {
            l: l,
            a: a,
            b: b,
            _wp: PhantomData,
        }
    }
}

///<span id="Laba"></span>[`Laba`](type.Laba.html) implementations.
impl<WP, T> Alpha<LabSpace<WP, T>, T>
    where T: Float,
        WP: WhitePoint<T>
{
    ///CIE L*a*b* and transparency.
    pub fn new(l: T, a: T, b: T, alpha: T) -> AlphaLabSpace<WP, T> {
        Alpha {
            color: LabSpace::new(l, a, b),
            alpha: alpha,
        }
    }
}

// Rendered error http://is.gd/tHCm5C
// impl<WP, T> Limited for LabSpace<WP, T>
// where T: Float,
//     WP: WhitePoint<T>
// {
//     fn is_valid(&self) -> bool {
//         self.l >= T::zero() && self.l <= T::one() &&
//         self.a >= -T::one() && self.a <= T::one() &&
//         self.b >= -T::one() && self.b <= T::one()
//     }
//
//     fn clamp(&self) -> LabSpace<WP, T> {
//         let mut c = *self;
//         c.clamp_self();
//         c
//     }
//
//     fn clamp_self(&mut self) {
//         self.l = clamp(self.l, T::zero(), T::one());
//         self.a = clamp(self.a, -T::one(), T::one());
//         self.b = clamp(self.b, -T::one(), T::one());
//     }
// }

impl<WP, T> Mix for LabSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Scalar = T;

    fn mix(&self, other: &LabSpace<WP, T>, factor: T) -> LabSpace<WP, T> {
        let factor = clamp(factor, T::zero(), T::one());

        LabSpace {
            l: self.l + factor * (other.l - self.l),
            a: self.a + factor * (other.a - self.a),
            b: self.b + factor * (other.b - self.b),
            _wp: PhantomData,
        }
    }
}

impl<WP, T> Shade for LabSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> LabSpace<WP, T> {
        LabSpace {
            l: self.l + amount,
            a: self.a,
            b: self.b,
            _wp: PhantomData,
        }
    }
}

impl<WP, T> GetHue for LabSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
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

impl<T: Float> Default for Lab<T> {
    fn default() -> Lab<T> {
        Lab::new(T::zero(), T::zero(), T::zero())
    }
}


impl<WP, T> Add<LabSpace<WP, T>> for LabSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Output = LabSpace<WP, T>;

    fn add(self, other: LabSpace<WP, T>) -> LabSpace<WP, T> {
        LabSpace {
            l: self.l + other.l,
            a: self.a + other.a,
            b: self.b + other.b,
            _wp: PhantomData,
        }
    }
}

impl<WP, T> Add<T> for LabSpace<WP, T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Output = LabSpace<WP, T>;

    fn add(self, c: T) -> LabSpace<WP, T> {
        LabSpace {
            l: self.l + c,
            a: self.a + c,
            b: self.b + c,
            _wp: PhantomData
        }
    }
}

impl<WP, T> Sub<LabSpace<WP,T>> for LabSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Output = LabSpace<WP,T>;

    fn sub(self, other: LabSpace<WP,T>) -> LabSpace<WP,T> {
        LabSpace {
            l: self.l - other.l,
            a: self.a - other.a,
            b: self.b - other.b,
            _wp: PhantomData
        }
    }
}

impl<WP, T> Sub<T> for LabSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Output = LabSpace<WP,T>;

    fn sub(self, c: T) -> LabSpace<WP,T> {
        LabSpace {
            l: self.l - c,
            a: self.a - c,
            b: self.b - c,
            _wp: PhantomData
        }
    }
}

impl<WP, T> Mul<LabSpace<WP,T>> for LabSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Output = LabSpace<WP,T>;

    fn mul(self, other: LabSpace<WP,T>) -> LabSpace<WP,T> {
        LabSpace {
            l: self.l * other.l,
            a: self.a * other.a,
            b: self.b * other.b,
            _wp: PhantomData
        }
    }
}

impl<WP, T> Mul<T> for LabSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Output = LabSpace<WP,T>;

    fn mul(self, c: T) -> LabSpace<WP,T> {
        LabSpace {
            l: self.l * c,
            a: self.a * c,
            b: self.b * c,
            _wp: PhantomData
        }
    }
}

impl<WP, T> Div<LabSpace<WP,T>> for LabSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Output = LabSpace<WP,T>;

    fn div(self, other: LabSpace<WP,T>) -> LabSpace<WP,T> {
        LabSpace {
            l: self.l / other.l,
            a: self.a / other.a,
            b: self.b / other.b,
            _wp: PhantomData
        }
    }
}

impl<WP, T> Div<T> for LabSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    type Output = LabSpace<WP,T>;

    fn div(self, c: T) -> LabSpace<WP,T> {
        LabSpace {
            l: self.l / c,
            a: self.a / c,
            b: self.b / c,
            _wp: PhantomData
        }
    }
}

// from_color!(to Lab from Rgb, Luma, Xyz, Yxy, Lch, Hsv, Hsl);

// alpha_from!(LabSpace {Rgb, Xyz, Yxy, Luma, Lch, Hsv, Hsl, Color});

impl<WP, T> From<Xyz<T>> for LabSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn from(xyz: Xyz<T>) -> LabSpace<WP,T> {
        let wp_xyz: Xyz<T> = WP::get_yxy().into();

        LabSpace {
            l: (f(xyz.y / wp_xyz.y) * flt(116.0) - flt(16.0)) / flt(100.0),
            a: (f(xyz.x / wp_xyz.x) - f(xyz.y / wp_xyz.y)) * flt(500.0) / flt(128.0),
            b: (f(xyz.y / wp_xyz.y) - f(xyz.z / wp_xyz.z)) * flt(200.0) / flt(128.0),
            _wp: PhantomData
        }
    }
}

impl<WP, T> From<Yxy<T>> for LabSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn from(yxy: Yxy<T>) -> LabSpace<WP,T> {
        Xyz::from(yxy).into()
    }
}

impl<WP, T> From<Rgb<T>> for LabSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn from(rgb: Rgb<T>) -> LabSpace<WP,T> {
        Xyz::from(rgb).into()
    }
}

impl<WP, T> From<Luma<T>> for LabSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn from(luma: Luma<T>) -> LabSpace<WP,T> {
        Xyz::from(luma).into()
    }
}

impl<WP, T> From<LchSpace<WP, T>> for LabSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn from(lch: LchSpace<WP, T>) -> LabSpace<WP,T> {
        LabSpace {
            l: lch.l,
            a: lch.chroma.max(T::zero()) * lch.hue.to_radians().cos(),
            b: lch.chroma.max(T::zero()) * lch.hue.to_radians().sin(),
            _wp: PhantomData
        }
    }
}

impl<WP, T> From<Hsv<T>> for LabSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn from(hsv: Hsv<T>) -> LabSpace<WP,T> {
        Xyz::from(hsv).into()
    }
}

impl<WP, T> From<Hsl<T>> for LabSpace<WP,T>
    where T: Float,
        WP: WhitePoint<T>
{
    fn from(hsl: Hsl<T>) -> LabSpace<WP,T> {
        Xyz::from(hsl).into()
    }
}

fn f<T: Float>(t: T) -> T {
    //(6/29)^3
    let c_6_o_29_p_3: T = flt(0.00885645167);
    //(29/6)^2
    let c_29_o_6_p_2: T = flt(23.3611111111);

    if t > c_6_o_29_p_3 {
        t.powf(T::one() / flt(3.0))
    } else {
        (T::one() / flt(3.0)) * c_29_o_6_p_2 * t + (flt::<T,_>(4.0) / flt(29.0))
    }
}

#[cfg(test)]
mod test {
    use super::Lab;
    use ::Rgb;

    #[test]
    fn red() {
        let a = Lab::from(Rgb::new(1.0, 0.0, 0.0));
        let b = Lab::new(53.23288 / 100.0, 80.10933 / 128.0, 67.22006 / 128.0);
        assert_approx_eq!(a, b, [l, a, b]);
    }

    #[test]
    fn green() {
        let a = Lab::from(Rgb::new(0.0, 1.0, 0.0));
        let b = Lab::new(87.73704 / 100.0, -86.184654 / 128.0, 83.18117 / 128.0);
        assert_approx_eq!(a, b, [l, a, b]);
    }

    #[test]
    fn blue() {
        let a = Lab::from(Rgb::new(0.0, 0.0, 1.0));
        let b = Lab::new(32.302586 / 100.0, 79.19668 / 128.0, -107.863686 / 128.0);
        assert_approx_eq!(a, b, [l, a, b]);
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
