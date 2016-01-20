use std::ops::{Add, Sub, Mul, Div};

use {Color, Rgb, Luma, Xyz, Lch, Hsv, Hsl, ColorSpace, Mix, Shade, GetHue, LabHue, clamp};

use tristimulus::{X_N, Y_N, Z_N};

///The CIE L*a*b* (CIELAB) color space with an alpha component.
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
pub struct Lab<T: Float> {
    ///L* is the lightness of the color. T::zero() gives absolute black and T::one()
    ///give the brightest white.
    pub l: T,

    ///a* goes from red at -1.0 to green at 1.0.
    pub a: T,

    ///b* goes from yellow at -1.0 to blue at 1.0.
    pub b: T,

    ///The transparency of the color. T::zero() is completely transparent and T::one() is
    ///completely opaque.
    pub alpha: T,
}

impl<T: Float> Lab<T> {
    ///CIE L*a*b*.
    pub fn lab(l: T, a: T, b: T) -> Lab<T> {
        Lab {
            l: l,
            a: a,
            b: b,
            alpha: T::one(),
        }
    }

    ///CIE L*a*b* and transparency.
    pub fn laba(l: T, a: T, b: T, alpha: T) -> Lab<T> {
        Lab {
            l: l,
            a: a,
            b: b,
            alpha: alpha,
        }
    }
}

impl<T: Float> ColorSpace for Lab<T> {
    fn is_valid(&self) -> bool {
        self.l >= T::zero() && self.l <= T::one() && self.a >= -1.0 && self.a <= T::one() &&
        self.b >= -1.0 && self.b <= T::one() &&
        self.alpha >= T::zero() && self.alpha <= T::one()
    }

    fn clamp(&self) -> Lab<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, T::zero(), T::one());
        self.a = clamp(self.a, -1.0, T::one());
        self.b = clamp(self.b, -1.0, T::one());
        self.alpha = clamp(self.alpha, T::zero(), T::one());
    }
}

impl<T: Float> Mix for Lab<T> {
    fn mix(&self, other: &Lab<T>, factor: T) -> Lab<T> {
        let factor = clamp(factor, T::zero(), T::one());

        Lab {
            l: self.l + factor * (other.l - self.l),
            a: self.a + factor * (other.a - self.a),
            b: self.b + factor * (other.b - self.b),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl<T: Float> Shade for Lab<T> {
    fn lighten(&self, amount: T) -> Lab<T> {
        Lab {
            l: self.l + amount * T::one(),
            a: self.a,
            b: self.b,
            alpha: self.alpha,
        }
    }
}

impl<T: Float> GetHue for Lab<T> {
    type Hue = LabHue;

    fn get_hue(&self) -> Option<LabHue> {
        if self.a == T::zero() && self.b == T::zero() {
            None
        } else {
            Some(LabHue::from_radians(self.b.atan2(self.a)))
        }
    }
}

impl<T: Float> Default for Lab<T> {
    fn default() -> Lab<T> {
        Lab::lab(T::zero(), T::zero(), T::zero())
    }
}

impl Add<Lab> for Lab {
    type Output = Lab;

    fn add(self, other: Lab) -> Lab {
        Lab {
            l: self.l + other.l,
            a: self.a + other.a,
            b: self.b + other.b,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl Add<f32> for Lab {
    type Output = Lab;

    fn add(self, c: f32) -> Lab {
        Lab {
            l: self.l + c,
            a: self.a + c,
            b: self.b + c,
            alpha: self.alpha + c,
        }
    }
}

impl Sub<Lab> for Lab {
    type Output = Lab;

    fn sub(self, other: Lab) -> Lab {
        Lab {
            l: self.l - other.l,
            a: self.a - other.a,
            b: self.b - other.b,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl Sub<f32> for Lab {
    type Output = Lab;

    fn sub(self, c: f32) -> Lab {
        Lab {
            l: self.l - c,
            a: self.a - c,
            b: self.b - c,
            alpha: self.alpha - c,
        }
    }
}

impl Mul<Lab> for Lab {
    type Output = Lab;

    fn mul(self, other: Lab) -> Lab {
        Lab {
            l: self.l * other.l,
            a: self.a * other.a,
            b: self.b * other.b,
            alpha: self.alpha * other.alpha,
        }
    }
}

impl Mul<f32> for Lab {
    type Output = Lab;

    fn mul(self, c: f32) -> Lab {
        Lab {
            l: self.l * c,
            a: self.a * c,
            b: self.b * c,
            alpha: self.alpha * c,
        }
    }
}

impl Div<Lab> for Lab {
    type Output = Lab;

    fn div(self, other: Lab) -> Lab {
        Lab {
            l: self.l / other.l,
            a: self.a / other.a,
            b: self.b / other.b,
            alpha: self.alpha / other.alpha,
        }
    }
}

impl Div<f32> for Lab {
    type Output = Lab;

    fn div(self, c: f32) -> Lab {
        Lab {
            l: self.l / c,
            a: self.a / c,
            b: self.b / c,
            alpha: self.alpha / c,
        }
    }
}

from_color!(to Lab from Rgb, Luma, Xyz, Lch, Hsv, Hsl);

impl<T: Float> From<Xyz> for Lab<T> {
    fn from(xyz: Xyz) -> Lab<T> {
        Lab {
            l: (T::from(116.0).unwrap() * f(xyz.y / Y_N) - T::from(16.0).unwrap()) /
               T::from(100.0).unwrap(),
            a: (T::from(500.0).unwrap() * (f(xyz.x / X_N) - f(xyz.y / Y_N))) /
               T::from(128.0).unwrap(),
            b: (T::from(200.0).unwrap() * (f(xyz.y / Y_N) - f(xyz.z / Z_N))) /
               T::from(128.0).unwrap(),
            alpha: xyz.alpha,
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
            alpha: lch.alpha,
        }
    }
}

impl<T: Float> From<Hsv> for Lab<T> {
    fn from(hsv: Hsv) -> Lab<T> {
        Xyz::from(hsv).into()
    }
}

impl<T: Float> From<Hsl<T>> for Lab<T> {
    fn from(hsl: Hsl<T>) -> Lab<T> {
        Xyz::from(hsl).into()
    }
}

fn f<T: Float>(t: T) -> T {
    // (T::from(6/29).unwrap())^3
    let C_6_O_29_P_3: T = T::from(0.00885645167).unwrap();
    // (T::from(29/6).unwrap())^2
    let C_29_O_6_P_2: T = T::from(23.3611111111).unwrap();

    if t > C_6_O_29_P_3 {
        t.powf(T::one() / T::from(3.0).unwrap())
    } else {
        (T::one() / T::from(3.0).unwrap()) * C_29_O_6_P_2 * t +
        (T::from(4.0).unwrap() / T::from(29.0).unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::Lab;
    use Rgb;

    #[test]
    fn red() {
        let a = Lab::from(Rgb::linear_rgb(T::one(), T::zero(), T::zero()));
        let b = Lab::lab(T::from(53.23288).unwrap() / T::from(100.0).unwrap(),
                         T::from(80.10933).unwrap() / T::from(128.0).unwrap(),
                         T::from(67.22006).unwrap() / T::from(128.0).unwrap());
        assert_approx_eq!(a, b, [l, a, b]);
    }

    #[test]
    fn green() {
        let a = Lab::from(Rgb::linear_rgb(T::zero(), T::one(), T::zero()));
        let b = Lab::lab(T::from(87.73704).unwrap() / T::from(100.0).unwrap(),
                         -86.184654 / T::from(128.0).unwrap(),
                         T::from(83.18117).unwrap() / T::from(128.0).unwrap());
        assert_approx_eq!(a, b, [l, a, b]);
    }

    #[test]
    fn blue() {
        let a = Lab::from(Rgb::linear_rgb(T::zero(), T::zero(), T::one()));
        let b = Lab::lab(T::from(32.302586).unwrap() / T::from(100.0).unwrap(),
                         T::from(79.19668).unwrap() / T::from(128.0).unwrap(),
                         -107.863686 / T::from(128.0).unwrap());
        assert_approx_eq!(a, b, [l, a, b]);
    }
}
