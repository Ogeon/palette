use num::traits::Float;

use std::ops::{Add, Sub, Mul, Div};

use {Color, Rgb, Xyz, Lab, Lch, Hsv, Hsl, ColorSpace, Mix, Shade, clamp};

///Linear luminance with an alpha component.
///
///Luma is a purely gray scale color space, which is included more for
///completeness than anything else, and represents how bright a color is
///perceived to be. It's basically the `Y` component of [CIE
///XYZ](struct.Xyz.html). The lack of any form of hue representation limits
///the set of operations that can be performed on it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Luma<T: Float = f32> {
    ///The lightness of the color. 0.0 is black and 1.0 is white.
    pub luma: T,

    ///The transparency of the color. 0.0 is completely transparent and 1.0 is
    ///completely opaque.
    pub alpha: T,
}

impl<T: Float> Luma<T> {
    ///Linear luminance.
    pub fn y(luma: T) -> Luma<T> {
        Luma {
            luma: luma,
            alpha: T::zero(),
        }
    }

    ///Linear luminance with transparency.
    pub fn ya(luma: T, alpha: T) -> Luma<T> {
        Luma {
            luma: luma,
            alpha: alpha,
        }
    }

    ///Linear luminance from an 8 bit value.
    pub fn y8(luma: u8) -> Luma<T> {
        Luma {
            luma: T::from(luma).unwrap() / T::from(255.0).unwrap(),
            alpha: T::zero(),
        }
    }

    ///Linear luminance and transparency from 8 bit values.
    pub fn ya8(luma: u8, alpha: u8) -> Luma<T> {
        Luma {
            luma: T::from(luma).unwrap() / T::from(255.0).unwrap(),
            alpha: T::from(alpha).unwrap() / T::from(255.0).unwrap(),
        }
    }
}

impl<T: Float> ColorSpace for Luma<T> {
    fn is_valid(&self) -> bool {
        self.luma >= T::zero() && self.luma <= T::one() && self.alpha >= T::zero() &&
        self.alpha <= T::one()
    }

    fn clamp(&self) -> Luma<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.luma = clamp(self.luma, T::zero(), T::one());
        self.alpha = clamp(self.alpha, T::zero(), T::one());
    }
}

impl<T: Float> Mix for Luma<T> {
    type Scalar = T;

    fn mix(&self, other: &Luma<T>, factor: T) -> Luma<T> {
        let factor = clamp(factor, T::zero(), T::one());

        Luma {
            luma: self.luma + factor * (other.luma - self.luma),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl<T: Float> Shade for Luma<T> {
    type Scalar = T;

    fn lighten(&self, amount: T) -> Luma<T> {
        Luma {
            luma: (self.luma + amount).max(T::zero()),
            alpha: self.alpha,
        }
    }
}

impl<T: Float> Default for Luma<T> {
    fn default() -> Luma<T> {
        Luma::y(T::zero())
    }
}

impl<T: Float> Add<Luma<T>> for Luma<T> {
    type Output = Luma<T>;

    fn add(self, other: Luma<T>) -> Luma<T> {
        Luma {
            luma: self.luma + other.luma,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl<T: Float> Add<T> for Luma<T> {
    type Output = Luma<T>;

    fn add(self, c: T) -> Luma<T> {
        Luma {
            luma: self.luma + c,
            alpha: self.alpha + c,
        }
    }
}

impl<T: Float> Sub<Luma<T>> for Luma<T> {
    type Output = Luma<T>;

    fn sub(self, other: Luma<T>) -> Luma<T> {
        Luma {
            luma: self.luma - other.luma,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl<T: Float> Sub<T> for Luma<T> {
    type Output = Luma<T>;

    fn sub(self, c: T) -> Luma<T> {
        Luma {
            luma: self.luma - c,
            alpha: self.alpha - c,
        }
    }
}

impl<T: Float> Mul<Luma<T>> for Luma<T> {
    type Output = Luma<T>;

    fn mul(self, other: Luma<T>) -> Luma<T> {
        Luma {
            luma: self.luma * other.luma,
            alpha: self.alpha * other.alpha,
        }
    }
}

impl<T: Float> Mul<T> for Luma<T> {
    type Output = Luma<T>;

    fn mul(self, c: T) -> Luma<T> {
        Luma {
            luma: self.luma * c,
            alpha: self.alpha * c,
        }
    }
}

impl<T: Float> Div<Luma<T>> for Luma<T> {
    type Output = Luma<T>;

    fn div(self, other: Luma<T>) -> Luma<T> {
        Luma {
            luma: self.luma / other.luma,
            alpha: self.alpha / other.alpha,
        }
    }
}

impl<T: Float> Div<T> for Luma<T> {
    type Output = Luma<T>;

    fn div(self, c: T) -> Luma<T> {
        Luma {
            luma: self.luma / c,
            alpha: self.alpha / c,
        }
    }
}

from_color!(to Luma from Rgb, Xyz, Lab, Lch, Hsv, Hsl);

impl<T: Float> From<Rgb<T>> for Luma<T> {
    fn from(rgb: Rgb<T>) -> Luma<T> {
        Luma {
            luma: rgb.red * T::from(0.2126).unwrap() + rgb.green * T::from(0.7152).unwrap() + rgb.blue * T::from(0.0722).unwrap(),
            alpha: rgb.alpha
        }
    }
}

impl<T: Float> From<Xyz<T>> for Luma<T> {
    fn from(xyz: Xyz<T>) -> Luma<T> {
        Luma {
            luma: xyz.y,
            alpha: xyz.alpha,
        }
    }
}

impl<T: Float> From<Lab<T>> for Luma<T> {
    fn from(lab: Lab<T>) -> Luma<T> {
        Xyz::from(lab).into()
    }
}

impl<T: Float> From<Lch<T>> for Luma<T> {
    fn from(lch: Lch<T>) -> Luma<T> {
        Xyz::from(lch).into()
    }
}

impl<T: Float> From<Hsv<T>> for Luma<T> {
    fn from(hsv: Hsv<T>) -> Luma<T> {
        Rgb::from(hsv).into()
    }
}

impl<T: Float> From<Hsl<T>> for Luma<T> {
    fn from(hsl: Hsl<T>) -> Luma<T> {
        Rgb::from(hsl).into()
    }
}
