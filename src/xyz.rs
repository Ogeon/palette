use std::ops::{Add, Sub, Mul, Div};

use {Color, Rgb, Luma, Lab, Lch, Hsv, Hsl, ColorSpace, Mix, Shade, clamp};

use tristimulus::{X_N, Y_N, Z_N};

///The CIE 1931 XYZ color space with an alpha component.
///
///XYZ links the perceived colors to their wavelengths and simply makes it
///possible to describe the way we see colors as numbers. It's often used when
///converting from one color space to an other, and requires a standard
///illuminant and a standard observer to be defined.
///
///Conversions and operations on this color space assumes the CIE Standard
///Illuminant D65 as the white point, and the 2° standard colorimetric
///observer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Xyz<T: Float> {
    ///X is the scale of what can be seen as a response curve for the cone
    ///cells in the human eye. It goes from 0.0 to 1.0.
    pub x: T,

    ///Y is the luminance of the color, where 0.0 is black and 1.0 is white.
    pub y: T,

    ///Z is the scale of what can be seen as the blue stimulation. It goes
    ///from 0.0 to 1.0.
    pub z: T,

    ///The transparency of the color. 0.0 is completely transparent and 1.0 is
    ///completely opaque.
    pub alpha: T,
}

impl<T: Float> Xyz<T> {
    ///CIE XYZ.
    pub fn xyz(x: T, y: T, z: T) -> Xyz<T> {
        Xyz {
            x: x,
            y: y,
            z: z,
            alpha: T::one(),
        }
    }

    ///CIE XYZ and transparency.
    pub fn xyza(x: T, y: T, z: T, alpha: T) -> Xyz<T> {
        Xyz {
            x: x,
            y: y,
            z: z,
            alpha: alpha,
        }
    }
}

impl<T: Float> ColorSpace for Xyz<T> {
    fn is_valid(&self) -> bool {
        self.x >= T::Zero() && self.x <= T::One() && self.y >= T::Zero() &&
        self.y <= T::One() && self.z >= T::Zero() && self.z <= T::One() &&
        self.alpha >= T::Zero() && self.alpha <= T::One()
    }

    fn clamp(&self) -> Xyz<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.x = clamp(self.x, T::Zero(), T::One());
        self.y = clamp(self.y, T::Zero(), T::One());
        self.z = clamp(self.z, T::Zero(), T::One());
        self.alpha = clamp(self.alpha, T::Zero(), T::One());
    }
}

impl<T: Float> Mix for Xyz<T> {
    fn mix(&self, other: &Xyz<T>, factor: T) -> Xyz<T> {
        let factor = clamp(factor, T::Zero(), T::One());

        Xyz {
            x: self.x + factor * (other.x - self.x),
            y: self.y + factor * (other.y - self.y),
            z: self.z + factor * (other.z - self.z),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl<T: Float> Shade for Xyz<T> {
    fn lighten(&self, amount: T) -> Xyz<T> {
        Xyz {
            x: self.x,
            y: self.y + amount,
            z: self.z,
            alpha: self.alpha,
        }
    }
}

impl<T: Float> Default for Xyz<T> {
    fn default() -> Xyz<T> {
        Xyz::xyz(T::Zero(), T::Zero(), T::Zero())
    }
}

impl Add<Xyz> for Xyz {
    type Output = Xyz;

    fn add(self, other: Xyz) -> Xyz {
        Xyz {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl Add<f32> for Xyz {
    type Output = Xyz;

    fn add(self, c: f32) -> Xyz {
        Xyz {
            x: self.x + c,
            y: self.y + c,
            z: self.z + c,
            alpha: self.alpha + c,
        }
    }
}

impl Sub<Xyz> for Xyz {
    type Output = Xyz;

    fn sub(self, other: Xyz) -> Xyz {
        Xyz {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl Sub<f32> for Xyz {
    type Output = Xyz;

    fn sub(self, c: f32) -> Xyz {
        Xyz {
            x: self.x - c,
            y: self.y - c,
            z: self.z - c,
            alpha: self.alpha - c,
        }
    }
}

impl Mul<Xyz> for Xyz {
    type Output = Xyz;

    fn mul(self, other: Xyz) -> Xyz {
        Xyz {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            alpha: self.alpha * other.alpha,
        }
    }
}

impl Mul<f32> for Xyz {
    type Output = Xyz;

    fn mul(self, c: f32) -> Xyz {
        Xyz {
            x: self.x * c,
            y: self.y * c,
            z: self.z * c,
            alpha: self.alpha * c,
        }
    }
}

impl Div<Xyz> for Xyz {
    type Output = Xyz;

    fn div(self, other: Xyz) -> Xyz {
        Xyz {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            alpha: self.alpha / other.alpha,
        }
    }
}

impl Div<f32> for Xyz {
    type Output = Xyz;

    fn div(self, c: f32) -> Xyz {
        Xyz {
            x: self.x / c,
            y: self.y / c,
            z: self.z / c,
            alpha: self.alpha / c,
        }
    }
}

from_color!(to Xyz from Rgb, Luma, Lab, Lch, Hsv, Hsl);

impl<T: Float> From<Rgb<T>> for Xyz<T> {
    fn from(rgb: Rgb<T>) -> Xyz<T> {
        Xyz {
            x: rgb.red * T::from(0.4124).unwrap() + rgb.green * T::from(0.3576).unwrap() +
               rgb.blue * T::from(0.1805).unwrap(),
            y: rgb.red * T::from(0.2126).unwrap() + rgb.green * T::from(0.7152).unwrap() +
               rgb.blue * T::from(0.0722).unwrap(),
            z: rgb.red * T::from(0.0193).unwrap() + rgb.green * T::from(0.1192).unwrap() +
               rgb.blue * T::from(0.9505).unwrap(),
            alpha: rgb.alpha,
        }
    }
}

impl<T: Float> From<Luma<T>> for Xyz<T> {
    fn from(luma: Luma) -> Xyz<T> {
        Xyz {
            x: T::Zero(),
            y: luma.luma,
            z: T::Zero(),
            alpha: luma.alpha,
        }
    }
}

impl<T: Float> From<Lab<T>> for Xyz<T> {
    fn from(lab: Lab) -> Xyz {
        Xyz {
            x: X_N *
               f_inv((T::One() / T::from(116.0).unwrap()) *
                     (lab.l * T::from(100.0) + T::from(16.0)) +
                     (T::One() / T::from(500.0).unwrap()) * lab.a * T::from(128.0).unwrap()),
            y: Y_N *
               f_inv((T::One() / T::from(116.0).unwrap()) *
                     (lab.l * T::from(100.0) + T::from(16.0))),
            z: Z_N *
               f_inv((T::One() / T::from(116.0).unwrap()) *
                     (lab.l * T::from(100.0) + T::from(16.0)) -
                     (T::One() / T::from(200.0).unwrap()) * lab.b * T::from(128.0).unwrap()),
            alpha: lab.alpha,
        }
    }
}

impl<T: Float> From<Lch<T>> for Xyz<T> {
    fn from(lch: Lch<T>) -> Xyz<T> {
        Lab::from(lch).into()
    }
}

impl<T: Float> From<Hsv<T>> for Xyz<T> {
    fn from(hsv: Hsv) -> Xyz<T> {
        Rgb::from(hsv).into()
    }
}

impl<T: Float> From<Hsl<T>> for Xyz<T> {
    fn from(hsl: Hsl<T>) -> Xyz<T> {
        Rgb::from(hsl).into()
    }
}


fn f_inv<T: Float>(t: T) -> T {
    // (6/29)^2
    let C_6_O_29_P_2: T = T::from(0.04280618311).unwrap();

    if t > T::from(6.0 / 29.0).unwrap() {
        t * t * t
    } else {
        T::from(3.0).unwrap() * C_6_O_29_P_2 * (t - T::from(4.0 / 29.0).unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::Xyz;
    use Rgb;

    #[test]
    fn red() {
        let a = Xyz::from(Rgb::linear_rgb(1.0, 0.0, 0.0));
        let b = Xyz::xyz(0.41240, 0.21260, 0.01930);
        assert_approx_eq!(a, b, [x, y, z]);
    }

    #[test]
    fn green() {
        let a = Xyz::from(Rgb::linear_rgb(0.0, 1.0, 0.0));
        let b = Xyz::xyz(0.35760, 0.71520, 0.11920);
        assert_approx_eq!(a, b, [x, y, z]);
    }

    #[test]
    fn blue() {
        let a = Xyz::from(Rgb::linear_rgb(0.0, 0.0, 1.0));
        let b = Xyz::xyz(0.18050, 0.07220, 0.95050);
        assert_approx_eq!(a, b, [x, y, z]);
    }
}
