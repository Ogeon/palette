use num::Float;

use std::ops::{Add, Sub, Mul, Div};

use {Color, Alpha, Yxy, Rgb, Luma, Lab, Lch, Hsv, Hsl, Limited, Mix, Shade, clamp, flt};

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

from_color!(to Xyz from Yxy, Rgb, Luma, Lab, Lch, Hsv, Hsl);

alpha_from!(Xyz {Yxy, Rgb, Luma, Lab, Lch, Hsv, Hsl, Color});

impl<T: Float> From<Yxy<T>> for Xyz<T> {
    fn from(yxy: Yxy<T>) -> Xyz<T> {
        let mut xyz = Xyz::new(T::zero(), yxy.luma, T::zero());
        // If denominator is zero, NAN or INFINITE leave x and z at the default 0
        if yxy.y.is_normal() {
            xyz.x = yxy.luma * yxy.x / yxy.y;
            xyz.z = yxy.luma * ( T::one() - yxy.x - yxy.y ) / yxy.y;
        }
        xyz
    }
}

impl<T: Float> From<Rgb<T>> for Xyz<T> {
    fn from(rgb: Rgb<T>) -> Xyz<T> {
        Xyz {
            x: rgb.red * flt(0.4124) + rgb.green * flt(0.3576) + rgb.blue * flt(0.1805),
            y: rgb.red * flt(0.2126) + rgb.green * flt(0.7152) + rgb.blue * flt(0.0722),
            z: rgb.red * flt(0.0193) + rgb.green * flt(0.1192) + rgb.blue * flt(0.9505),
        }
    }
}

impl<T: Float> From<Luma<T>> for Xyz<T> {
    fn from(luma: Luma<T>) -> Xyz<T> {
        // Use the D65 white point Xyz values for x and z as D65 is used as the default and scale by luma
        Xyz {
            x: luma.luma * flt(X_N),
            y: luma.luma,
            z: luma.luma * flt(Z_N),
        }
    }
}

impl<T: Float> From<Lab<T>> for Xyz<T> {
    fn from(lab: Lab<T>) -> Xyz<T> {
        Xyz {
            x: flt::<T,_>(X_N) * f_inv((T::one() / flt(116.0)) * (lab.l * flt(100.0) + flt(16.0)) +
                (T::one() / flt(500.0)) * lab.a * flt(128.0)),
            y: flt::<T,_>(Y_N) * f_inv((T::one() / flt(116.0)) * (lab.l * flt(100.0) + flt(16.0))),
            z: flt::<T,_>(Z_N) * f_inv((T::one() / flt(116.0)) * (lab.l * flt(100.0) + flt(16.0)) -
                (T::one() / flt(200.0)) * lab.b * flt(128.0)),
        }
    }
}

impl<T: Float> From<Lch<T>> for Xyz<T> {
    fn from(lch: Lch<T>) -> Xyz<T> {
        Lab::from(lch).into()
    }
}

impl<T: Float> From<Hsv<T>> for Xyz<T> {
    fn from(hsv: Hsv<T>) -> Xyz<T> {
        Rgb::from(hsv).into()
    }
}

impl<T: Float> From<Hsl<T>> for Xyz<T> {
    fn from(hsl: Hsl<T>) -> Xyz<T> {
        Rgb::from(hsl).into()
    }
}


fn f_inv<T: Float>(t: T) -> T {
    //(6/29)^2
    let c_6_o_29_p_2: T = flt(0.04280618311);

    if t > flt::<T,_>(6.0) / flt(29.0) {
        t * t * t
    } else {
         c_6_o_29_p_2 * flt(3.0) * (t - (flt::<T,_>(4.0) / flt(29.0)))
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
        assert_approx_eq!(a, b, [x, y, z]);
    }

    #[test]
    fn red() {
        let a = Xyz::from(Rgb::new(1.0, 0.0, 0.0));
        let b = Xyz::new(0.41240, 0.21260, 0.01930);
        assert_approx_eq!(a, b, [x, y, z]);
    }

    #[test]
    fn green() {
        let a = Xyz::from(Rgb::new(0.0, 1.0, 0.0));
        let b = Xyz::new(0.35760, 0.71520, 0.11920);
        assert_approx_eq!(a, b, [x, y, z]);
    }

    #[test]
    fn blue() {
        let a = Xyz::from(Rgb::new(0.0, 0.0, 1.0));
        let b = Xyz::new(0.18050, 0.07220, 0.95050);
        assert_approx_eq!(a, b, [x, y, z]);
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
