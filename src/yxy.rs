use num::traits::Float;

use std::ops::{Add, Sub, Mul, Div};

use {Color, Alpha, Rgb, Luma, Lab, Lch, Hsv, Hsl, Limited, Mix, Shade, clamp, Xyz};

const D65_X: f64 = 0.312727;
const D65_Y: f64 = 0.329023;

///CIE 1931 Yxy (xyY) with an alpha component. See the [`Yxya` implementation in `Alpha`](struct.Alpha.html#Yxya).
pub type Yxya<T = f32> = Alpha<Yxy<T>, T>;

///The CIE 1931 Yxy (xyY)  color space.
///
///Yxy is a luminance-chromaticity color space derived from the CIE XYZ
///color space. It is widely used to define colors. The chromacity diagrams
///for the color spaces are a plot of this color space's x and y coordiantes.
///
///Conversions and operations on this color space assumes the CIE Standard
///Illuminant D65 as the white point, and the 2° standard colorimetric
///observer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Yxy<T: Float = f32> {

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
}

impl<T: Float> Yxy<T> {
    ///CIE Yxy.
    pub fn new(x: T, y: T, luma: T,) -> Yxy<T> {
        Yxy {
            x: x,
            y: y,
            luma: luma,
        }
    }
}

///<span id="Yxya"></span>[`Yxya`](type.Yxya.html) implementations.
impl<T: Float> Alpha<Yxy<T>, T> {
    ///CIE Yxy and transparency.
    pub fn new(x: T, y: T, luma: T, alpha: T) -> Yxya<T> {
        Alpha {
            color: Yxy::new(x, y, luma),
            alpha: alpha,
        }
    }
}

impl<T: Float> Limited for Yxy<T> {
    fn is_valid(&self) -> bool {
        self.x >= T::zero() && self.x <= T::one() &&
        self.y >= T::zero() && self.y <= T::one() &&
        self.luma >= T::zero() && self.luma <= T::one()
    }

    fn clamp(&self) -> Yxy<T> {
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

impl<T: Float> Mix for Yxy<T> {
    type Scalar = T;

    fn mix(&self, other: &Yxy<T>, factor: T) -> Yxy<T> {
        let factor = clamp(factor, T::zero(), T::one());

        Yxy {
            x: self.x + factor * (other.x - self.x),
            y: self.y + factor * (other.y - self.y),
            luma: self.luma + factor * (other.luma - self.luma),
        }
    }
}

impl<T: Float> Shade for Yxy<T> {
    type Scalar = T;

    fn lighten(&self, amount: T) -> Yxy<T> {
        Yxy {
            x: self.x,
            y: self.y,
            luma: self.luma + amount,
        }
    }
}

impl<T: Float> Default for Yxy<T> {
    fn default() -> Yxy<T> {
        // The default for x and y are the white point x and y ( from the default D65).
        // Since Y (luma) is 0.0, this makes the default color black just like for other colors.
        // The reason for not using 0 for x and y is that this outside the usual color gamut and might
        // cause scaling issues.
        Yxy::new(T::from(D65_X).unwrap(), T::from(D65_Y).unwrap(), T::zero())
    }
}

impl<T: Float> Add<Yxy<T>> for Yxy<T> {
    type Output = Yxy<T>;

    fn add(self, other: Yxy<T>) -> Yxy<T> {
        Yxy {
            x: self.x + other.x,
            y: self.y + other.y,
            luma: self.luma + other.luma,
        }
    }
}

impl<T: Float> Add<T> for Yxy<T> {
    type Output = Yxy<T>;

    fn add(self, c: T) -> Yxy<T> {
        Yxy {
            x: self.x + c,
            y: self.y + c,
            luma: self.luma + c,
        }
    }
}

impl<T: Float> Sub<Yxy<T>> for Yxy<T> {
    type Output = Yxy<T>;

    fn sub(self, other: Yxy<T>) -> Yxy<T> {
        Yxy {
            x: self.x - other.x,
            y: self.y - other.y,
            luma: self.luma - other.luma,
        }
    }
}

impl<T: Float> Sub<T> for Yxy<T> {
    type Output = Yxy<T>;

    fn sub(self, c: T) -> Yxy<T> {
        Yxy {
            x: self.x - c,
            y: self.y - c,
            luma: self.luma - c,
        }
    }
}

impl<T: Float> Mul<Yxy<T>> for Yxy<T> {
    type Output = Yxy<T>;

    fn mul(self, other: Yxy<T>) -> Yxy<T> {
        Yxy {
            x: self.x * other.x,
            y: self.y * other.y,
            luma: self.luma * other.luma,
        }
    }
}

impl<T: Float> Mul<T> for Yxy<T> {
    type Output = Yxy<T>;

    fn mul(self, c: T) -> Yxy<T> {
        Yxy {
            x: self.x * c,
            y: self.y * c,
            luma: self.luma * c,
        }
    }
}

impl<T: Float> Div<Yxy<T>> for Yxy<T> {
    type Output = Yxy<T>;

    fn div(self, other: Yxy<T>) -> Yxy<T> {
        Yxy {
            x: self.x / other.x,
            y: self.y / other.y,
            luma: self.luma / other.luma,
        }
    }
}

impl<T: Float> Div<T> for Yxy<T> {
    type Output = Yxy<T>;

    fn div(self, c: T) -> Yxy<T> {
        Yxy {
            x: self.x / c,
            y: self.y / c,
            luma: self.luma / c,
        }
    }
}

from_color!(to Yxy from Xyz, Rgb, Luma, Lab, Lch, Hsv, Hsl);

alpha_from!(Yxy {Xyz, Rgb, Luma, Lab, Lch, Hsv, Hsl, Color});

impl<T: Float> From<Xyz<T>> for Yxy<T> {
    fn from(xyz: Xyz<T>) -> Yxy<T> {
        let mut yxy = Yxy::new(T::zero(), T::zero(), xyz.y);
        let sum = xyz.x + xyz.y + xyz.z;
        // If denominator is zero, NAN or INFINITE leave x and y at the default 0
        if sum.is_normal() {
            yxy.x = xyz.x / sum;
            yxy.y = xyz.y / sum;
        }
        yxy
    }
}

impl<T: Float> From<Luma<T>> for Yxy<T> {
    fn from(luma: Luma<T>) -> Yxy<T> {
        // Use the D65 white point Yxy values for x and y as D65 is used as the default
        Yxy {
            x: T::from(D65_X).unwrap(),
            y: T::from(D65_Y).unwrap(),
            luma: luma.luma,
        }
    }
}

impl<T: Float> From<Lab<T>> for Yxy<T> {
    fn from(lab: Lab<T>) -> Yxy<T> {
        Xyz::from(lab).into()
    }
}

impl<T: Float> From<Lch<T>> for Yxy<T> {
    fn from(lch: Lch<T>) -> Yxy<T> {
        Lab::from(lch).into()
    }
}

impl<T: Float> From<Rgb<T>> for Yxy<T> {
    fn from(rgb: Rgb<T>) -> Yxy<T> {
        Xyz::from(rgb).into()
    }
}


impl<T: Float> From<Hsv<T>> for Yxy<T> {
    fn from(hsv: Hsv<T>) -> Yxy<T> {
        Rgb::from(hsv).into()
    }
}

impl<T: Float> From<Hsl<T>> for Yxy<T> {
    fn from(hsl: Hsl<T>) -> Yxy<T> {
        Rgb::from(hsl).into()
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
        assert_approx_eq!(a, b, [x, y, luma]);
    }

    #[test]
    fn red() {
        let a = Yxy::from(Rgb::new(1.0, 0.0, 0.0));
        let b = Yxy::new(0.64, 0.33, 0.212673);
        assert_approx_eq!(a, b, [x, y, luma]);
    }

    #[test]
    fn green() {
        let a = Yxy::from(Rgb::new(0.0, 1.0, 0.0));
        let b = Yxy::new(0.3, 0.6, 0.715152);
        assert_approx_eq!(a, b, [x, y, luma]);
    }

    #[test]
    fn blue() {
        let a = Yxy::from(Rgb::new(0.0, 0.0, 1.0));
        let b = Yxy::new(0.15, 0.06, 0.072175);
        assert_approx_eq!(a, b, [x, y, luma]);
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
