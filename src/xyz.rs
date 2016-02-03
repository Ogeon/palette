use num::Float;

use std::ops::{Add, Sub, Mul, Div};

use {Alpha, Yxy, Rgb, Luma, Lab};
use {Limited, Mix, Shade, FromColor, ComponentWise};
use {clamp, flt};

use tristimulus::{X_N, Y_N, Z_N};

///CIE 1931 XYZ with an alpha component. See the [`Xyza` implementation in `Alpha`](struct.Alpha.html#Xyza).
pub type Xyza<T = f32> = Alpha<Xyz<T>, T>;

///The CIE 1931 XYZ color space.
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
pub struct Xyz<T: Float = f32> {
    ///X is the scale of what can be seen as a response curve for the cone
    ///cells in the human eye. It goes from 0.0 to 0.95047.
    pub x: T,

    ///Y is the luminance of the color, where 0.0 is black and 1.0 is white.
    pub y: T,

    ///Z is the scale of what can be seen as the blue stimulation. It goes
    ///from 0.0 to 1.08883.
    pub z: T,
}

impl<T: Float> Xyz<T> {
    ///CIE XYZ.
    pub fn new(x: T, y: T, z: T) -> Xyz<T> {
        Xyz {
            x: x,
            y: y,
            z: z,
        }
    }
}

///<span id="Xyza"></span>[`Xyza`](type.Xyza.html) implementations.
impl<T: Float> Alpha<Xyz<T>, T> {
    ///CIE XYZ and transparency.
    pub fn new(x: T, y: T, z: T, alpha: T) -> Xyza<T> {
        Alpha {
            color: Xyz::new(x, y, z),
            alpha: alpha,
        }
    }
}

impl<T: Float> FromColor<T> for Xyz<T> {
    fn from_xyz(xyz: Xyz<T>) -> Self {
        xyz
    }

    fn from_rgb(rgb: Rgb<T>) -> Self {
        Xyz {
            x: rgb.red * flt(0.4124564) + rgb.green * flt(0.3575761) + rgb.blue * flt(0.1804375),
            y: rgb.red * flt(0.2126729) + rgb.green * flt(0.7151522) + rgb.blue * flt(0.0721750),
            z: rgb.red * flt(0.0193339) + rgb.green * flt(0.1191920) + rgb.blue * flt(0.9503041),
        }
    }

    fn from_yxy(yxy: Yxy<T>) -> Self {
        let mut xyz = Xyz { y: yxy.luma, ..Default::default() };
        // If denominator is zero, NAN or INFINITE leave x and z at the default 0
        if yxy.y.is_normal() {
            xyz.x = yxy.luma * yxy.x / yxy.y;
            xyz.z = yxy.luma * ( T::one() - yxy.x - yxy.y ) / yxy.y;
        }
        xyz
    }

    fn from_lab(input_lab: Lab<T>) -> Self {
        let mut lab: Lab<T> = input_lab.clone();
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

        Xyz {
            x: convert(x) * flt(X_N),
            y: convert(y) * flt(Y_N),
            z: convert(z) * flt(Z_N),
        }
    }
    fn from_luma(luma: Luma<T>) -> Self {
        Xyz {
            x: luma.luma * flt(X_N),
            y: luma.luma,
            z: luma.luma * flt(Z_N),
        }
    }

}

impl<T: Float> Limited for Xyz<T> {
    fn is_valid(&self) -> bool {
        self.x >= T::zero() && self.x <= flt(X_N) &&
        self.y >= T::zero() && self.y <= flt(Y_N) &&
        self.z >= T::zero() && self.z <= flt(Z_N)
    }

    fn clamp(&self) -> Xyz<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.x = clamp(self.x, T::zero(), flt(X_N));
        self.y = clamp(self.y, T::zero(), flt(Y_N));
        self.z = clamp(self.z, T::zero(), flt(Z_N));
    }
}

impl<T: Float> Mix for Xyz<T> {
    type Scalar = T;

    fn mix(&self, other: &Xyz<T>, factor: T) -> Xyz<T> {
        let factor = clamp(factor, T::zero(), T::one());

        Xyz {
            x: self.x + factor * (other.x - self.x),
            y: self.y + factor * (other.y - self.y),
            z: self.z + factor * (other.z - self.z),
        }
    }
}

impl<T: Float> Shade for Xyz<T> {
    type Scalar = T;

    fn lighten(&self, amount: T) -> Xyz<T> {
        Xyz {
            x: self.x,
            y: self.y + amount,
            z: self.z,
        }
    }
}

impl<T: Float> ComponentWise for Xyz<T> {
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Xyz<T>, mut f: F) -> Xyz<T> {
        Xyz {
            x: f(self.x, other.x),
            y: f(self.y, other.y),
            z: f(self.z, other.z),
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Xyz<T> {
        Xyz {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }
}

impl<T: Float> Default for Xyz<T> {
    fn default() -> Xyz<T> {
        Xyz::new(T::zero(), T::zero(), T::zero())
    }
}

impl<T: Float> Add<Xyz<T>> for Xyz<T> {
    type Output = Xyz<T>;

    fn add(self, other: Xyz<T>) -> Xyz<T> {
        Xyz {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Float> Add<T> for Xyz<T> {
    type Output = Xyz<T>;

    fn add(self, c: T) -> Xyz<T> {
        Xyz {
            x: self.x + c,
            y: self.y + c,
            z: self.z + c,
        }
    }
}

impl<T: Float> Sub<Xyz<T>> for Xyz<T> {
    type Output = Xyz<T>;

    fn sub(self, other: Xyz<T>) -> Xyz<T> {
        Xyz {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Float> Sub<T> for Xyz<T> {
    type Output = Xyz<T>;

    fn sub(self, c: T) -> Xyz<T> {
        Xyz {
            x: self.x - c,
            y: self.y - c,
            z: self.z - c,
        }
    }
}

impl<T: Float> Mul<Xyz<T>> for Xyz<T> {
    type Output = Xyz<T>;

    fn mul(self, other: Xyz<T>) -> Xyz<T> {
        Xyz {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T: Float> Mul<T> for Xyz<T> {
    type Output = Xyz<T>;

    fn mul(self, c: T) -> Xyz<T> {
        Xyz {
            x: self.x * c,
            y: self.y * c,
            z: self.z * c,
        }
    }
}

impl<T: Float> Div<Xyz<T>> for Xyz<T> {
    type Output = Xyz<T>;

    fn div(self, other: Xyz<T>) -> Xyz<T> {
        Xyz {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T: Float> Div<T> for Xyz<T> {
    type Output = Xyz<T>;

    fn div(self, c: T) -> Xyz<T> {
        Xyz {
            x: self.x / c,
            y: self.y / c,
            z: self.z / c,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Xyz;
    use Rgb;
    use Luma;
    use tristimulus::{X_N, Y_N, Z_N};

    #[test]
    fn luma() {
        let a = Xyz::from(Luma::new(0.5));
        let b = Xyz::new(0.475235, 0.5, 0.544415);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn red() {
        let a = Xyz::from(Rgb::new(1.0, 0.0, 0.0));
        let b = Xyz::new(0.41240, 0.21260, 0.01930);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn green() {
        let a = Xyz::from(Rgb::new(0.0, 1.0, 0.0));
        let b = Xyz::new(0.35760, 0.71520, 0.11920);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn blue() {
        let a = Xyz::from(Rgb::new(0.0, 0.0, 1.0));
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
