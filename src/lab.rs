use num::traits::Float;

use std::ops::{Add, Sub, Mul, Div};

use {Color, Alpha, Rgb, Luma, Xyz, Lch, Hsv, Hsl, ColorSpace, Mix, Shade, GetHue, LabHue, clamp};

use tristimulus::{X_N, Y_N, Z_N};

///CIE L*a*b* (CIELAB) with an alpha component. See the [`Laba` implementation in `Alpha`](struct.Alpha.html#Laba).
pub type Laba<T = f32> = Alpha<Lab<T>, T>;

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
pub struct Lab<T: Float = f32> {
    ///L* is the lightness of the color. 0.0 gives absolute black and 1.0
    ///give the brightest white.
    pub l: T,

    ///a* goes from red at -1.0 to green at 1.0.
    pub a: T,

    ///b* goes from yellow at -1.0 to blue at 1.0.
    pub b: T,
}

impl<T: Float> Lab<T> {
    ///CIE L*a*b*.
    pub fn new(l: T, a: T, b: T) -> Lab<T> {
        Lab {
            l: l,
            a: a,
            b: b,
        }
    }
}

///<span id="Laba"></span>[`Laba`](type.Laba.html) implementations.
impl<T: Float> Alpha<Lab<T>, T> {
    ///CIE L*a*b* and transparency.
    pub fn new(l: T, a: T, b: T, alpha: T) -> Laba<T> {
        Alpha {
            color: Lab::new(l, a, b),
            alpha: alpha,
        }
    }
}

impl<T: Float> ColorSpace for Lab<T> {
    fn is_valid(&self) -> bool {
        self.l >= T::zero() && self.l <= T::one() &&
        self.a >= -T::one() && self.a <= T::one() &&
        self.b >= -T::one() && self.b <= T::one()
    }

    fn clamp(&self) -> Lab<T> {
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

impl<T: Float> Mix for Lab<T> {
    type Scalar = T;
    
    fn mix(&self, other: &Lab<T>, factor: T) -> Lab<T> {
        let factor = clamp(factor, T::zero(), T::one());

        Lab {
            l: self.l + factor * (other.l - self.l),
            a: self.a + factor * (other.a - self.a),
            b: self.b + factor * (other.b - self.b),
        }
    }
}

impl<T: Float> Shade for Lab<T> {
    type Scalar = T;
    
    fn lighten(&self, amount: T) -> Lab<T> {
        Lab {
            l: self.l + amount,
            a: self.a,
            b: self.b,
        }
    }
}

impl<T: Float> GetHue for Lab<T> {
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

impl<T: Float> Add<Lab<T>> for Lab<T> {
    type Output = Lab<T>;

    fn add(self, other: Lab<T>) -> Lab<T> {
        Lab {
            l: self.l + other.l,
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

impl<T: Float> Add<T> for Lab<T> {
    type Output = Lab<T>;

    fn add(self, c: T) -> Lab<T> {
        Lab {
            l: self.l + c,
            a: self.a + c,
            b: self.b + c,
        }
    }
}

impl<T: Float> Sub<Lab<T>> for Lab<T> {
    type Output = Lab<T>;

    fn sub(self, other: Lab<T>) -> Lab<T> {
        Lab {
            l: self.l - other.l,
            a: self.a - other.a,
            b: self.b - other.b,
        }
    }
}

impl<T: Float> Sub<T> for Lab<T> {
    type Output = Lab<T>;

    fn sub(self, c: T) -> Lab<T> {
        Lab {
            l: self.l - c,
            a: self.a - c,
            b: self.b - c,
        }
    }
}

impl<T: Float> Mul<Lab<T>> for Lab<T> {
    type Output = Lab<T>;

    fn mul(self, other: Lab<T>) -> Lab<T> {
        Lab {
            l: self.l * other.l,
            a: self.a * other.a,
            b: self.b * other.b,
        }
    }
}

impl<T: Float> Mul<T> for Lab<T> {
    type Output = Lab<T>;

    fn mul(self, c: T) -> Lab<T> {
        Lab {
            l: self.l * c,
            a: self.a * c,
            b: self.b * c,
        }
    }
}

impl<T: Float> Div<Lab<T>> for Lab<T> {
    type Output = Lab<T>;

    fn div(self, other: Lab<T>) -> Lab<T> {
        Lab {
            l: self.l / other.l,
            a: self.a / other.a,
            b: self.b / other.b,
        }
    }
}

impl<T: Float> Div<T> for Lab<T> {
    type Output = Lab<T>;

    fn div(self, c: T) -> Lab<T> {
        Lab {
            l: self.l / c,
            a: self.a / c,
            b: self.b / c,
        }
    }
}

from_color!(to Lab from Rgb, Luma, Xyz, Lch, Hsv, Hsl);

alpha_from!(Lab {Rgb, Xyz, Luma, Lch, Hsv, Hsl, Color});

impl<T: Float> From<Xyz<T>> for Lab<T> {
    fn from(xyz: Xyz<T>) -> Lab<T> {
        Lab {
            l: (T::from(116.0).unwrap() * f(xyz.y / T::from(Y_N).unwrap()) -
                T::from(16.0).unwrap()) / T::from(100.0).unwrap(),
            a: (T::from(500.0).unwrap() *
                (f(xyz.x / T::from(X_N).unwrap()) - f(xyz.y / T::from(Y_N).unwrap()))) /
               T::from(128.0).unwrap(),
            b: (T::from(200.0).unwrap() *
                (f(xyz.y / T::from(Y_N).unwrap()) - f(xyz.z / T::from(Z_N).unwrap()))) /
               T::from(128.0).unwrap(),
        }
    }
}

impl<T: Float> From<Rgb<T>> for Lab<T> {
    fn from(rgb: Rgb<T>) -> Lab<T> {
        Xyz::from(rgb).into()
    }
}

impl<T: Float> From<Luma<T>> for Lab<T> {
    fn from(luma: Luma<T>) -> Lab<T> {
        Xyz::from(luma).into()
    }
}

impl<T: Float> From<Lch<T>> for Lab<T> {
    fn from(lch: Lch<T>) -> Lab<T> {
        Lab {
            l: lch.l,
            a: lch.chroma.max(T::zero()) * lch.hue.to_radians().cos(),
            b: lch.chroma.max(T::zero()) * lch.hue.to_radians().sin(),
        }
    }
}

impl<T: Float> From<Hsv<T>> for Lab<T> {
    fn from(hsv: Hsv<T>) -> Lab<T> {
        Xyz::from(hsv).into()
    }
}

impl<T: Float> From<Hsl<T>> for Lab<T> {
    fn from(hsl: Hsl<T>) -> Lab<T> {
        Xyz::from(hsl).into()
    }
}

fn f<T: Float>(t: T) -> T {
    //(6/29)^3
    let c_6_o_29_p_3: T = T::from(0.00885645167).unwrap();
    //(29/6)^2
    let c_29_o_6_p_2: T = T::from(23.3611111111).unwrap();

    if t > c_6_o_29_p_3 {
        t.powf(T::one() / T::from(3.0).unwrap())
    } else {
        (T::one() / T::from(3.0).unwrap()) * c_29_o_6_p_2 * t +
        (T::from(4.0).unwrap() / T::from(29.0).unwrap())
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
}
