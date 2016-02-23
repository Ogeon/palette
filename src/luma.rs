use num::Float;

use std::ops::{Add, Sub, Mul, Div};

use {Alpha, Rgb, Xyz, Yxy};
use {Limited, Mix, Shade, FromColor, Blend, ComponentWise, ColorType};
use {clamp, flt};
use blend::PreAlpha;

///Linear luminance with an alpha component. See the [`Lumaa` implementation in `Alpha`](struct.Alpha.html#Lumaa).
pub type Lumaa<T = f32> = Alpha<Luma<T>>;

///Linear luminance.
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
}

impl<T: Float> Luma<T> {
    ///Linear luminance.
    pub fn new(luma: T) -> Luma<T> {
        Luma {
            luma: luma,
        }
    }

    ///Linear luminance from an 8 bit value.
    pub fn new_u8(luma: u8) -> Luma<T> {
        Luma {
            luma: flt::<T,_>(luma) / flt(255.0),
        }
    }
}

///<span id="Lumaa"></span>[`Lumaa`](type.Lumaa.html) implementations.
impl<T: Float> Alpha<Luma<T>> {
    ///Linear luminance with transparency.
    pub fn new(luma: T, alpha: T) -> Lumaa<T> {
        Alpha {
            color: Luma::new(luma),
            alpha: alpha,
        }
    }

    ///Linear luminance and transparency from 8 bit values.
    pub fn new_u8(luma: u8, alpha: u8) -> Lumaa<T> {
        Alpha {
            color: Luma::new_u8(luma),
            alpha: flt::<T,_>(alpha) / flt(255.0),
        }
    }
}

impl<T: Float> ColorType for Luma<T> {
    type Scalar = T;
}

impl<T: Float> FromColor<T> for Luma<T> {
    fn from_xyz(xyz: Xyz<T>) -> Self {
        Luma {
            luma: xyz.y,
        }
    }

    fn from_yxy(yxy: Yxy<T>) -> Self {
        Luma {
            luma: yxy.luma,
        }
    }

    fn from_rgb(rgb: Rgb<T>) -> Self {
        Luma {
            luma: rgb.red * flt(0.2126) + rgb.green * flt(0.7152) + rgb.blue * flt(0.0722),
        }
    }

    fn from_luma(luma: Luma<T>) -> Self {
        luma
    }

}

impl<T: Float> Limited for Luma<T> {
    fn is_valid(&self) -> bool {
        self.luma >= T::zero() && self.luma <= T::one()
    }

    fn clamp(&self) -> Luma<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.luma = clamp(self.luma, T::zero(), T::one());
    }
}

impl<T: Float> Mix for Luma<T> {
    fn mix(&self, other: &Luma<T>, factor: T) -> Luma<T> {
        let factor = clamp(factor, T::zero(), T::one());

        Luma {
            luma: self.luma + factor * (other.luma - self.luma),
        }
    }
}

impl<T: Float> Shade for Luma<T> {
    fn lighten(&self, amount: T) -> Luma<T> {
        Luma {
            luma: (self.luma + amount).max(T::zero()),
        }
    }
}

impl<T: Float> Blend for Luma<T> {
    type Color = Luma<T>;

    fn into_premultiplied(self) -> PreAlpha<Luma<T>> {
        Lumaa::from(self).into()
    }

    fn from_premultiplied(color: PreAlpha<Luma<T>>) -> Self {
        Lumaa::from(color).into()
    }
}

impl<T: Float> ComponentWise for Luma<T> {
    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Luma<T>, mut f: F) -> Luma<T> {
        Luma {
            luma: f(self.luma, other.luma),
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Luma<T> {
        Luma {
            luma: f(self.luma),
        }
    }
}

impl<T: Float> Default for Luma<T> {
    fn default() -> Luma<T> {
        Luma::new(T::zero())
    }
}

impl<T: Float> Add<Luma<T>> for Luma<T> {
    type Output = Luma<T>;

    fn add(self, other: Luma<T>) -> Luma<T> {
        Luma {
            luma: self.luma + other.luma,
        }
    }
}

impl<T: Float> Add<T> for Luma<T> {
    type Output = Luma<T>;

    fn add(self, c: T) -> Luma<T> {
        Luma {
            luma: self.luma + c,
        }
    }
}

impl<T: Float> Sub<Luma<T>> for Luma<T> {
    type Output = Luma<T>;

    fn sub(self, other: Luma<T>) -> Luma<T> {
        Luma {
            luma: self.luma - other.luma,
        }
    }
}

impl<T: Float> Sub<T> for Luma<T> {
    type Output = Luma<T>;

    fn sub(self, c: T) -> Luma<T> {
        Luma {
            luma: self.luma - c,
        }
    }
}

impl<T: Float> Mul<Luma<T>> for Luma<T> {
    type Output = Luma<T>;

    fn mul(self, other: Luma<T>) -> Luma<T> {
        Luma {
            luma: self.luma * other.luma,
        }
    }
}

impl<T: Float> Mul<T> for Luma<T> {
    type Output = Luma<T>;

    fn mul(self, c: T) -> Luma<T> {
        Luma {
            luma: self.luma * c,
        }
    }
}

impl<T: Float> Div<Luma<T>> for Luma<T> {
    type Output = Luma<T>;

    fn div(self, other: Luma<T>) -> Luma<T> {
        Luma {
            luma: self.luma / other.luma,
        }
    }
}

impl<T: Float> Div<T> for Luma<T> {
    type Output = Luma<T>;

    fn div(self, c: T) -> Luma<T> {
        Luma {
            luma: self.luma / c,
        }
    }
}

#[cfg(test)]
mod test {
    use Luma;

    #[test]
    fn ranges() {
        assert_ranges!{
            Luma;
            limited {
                luma: 0.0 => 1.0
            }
            limited_min {}
            unlimited {}
        }
    }
}
