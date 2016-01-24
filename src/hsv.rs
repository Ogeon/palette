use num::traits::Float;

use std::ops::{Add, Sub};

use {Color, Rgb, Luma, Xyz, Lab, Lch, Hsl, ColorSpace, Mix, Shade, GetHue, Hue, Saturate, RgbHue, clamp};

///Linear HSV color space with an alpha component.
///
///HSV is a cylindrical version of [RGB](struct.Rgb.html) and it's very
///similar to [HSL](struct.Hsl.html). The difference is that the `value`
///component in HSV determines the _brightness_ of the color, and not the
///_lightness_. The difference is that, for example, red (100% R, 0% G, 0% B)
///and white (100% R, 100% G, 100% B) has the same brightness (or value), but
///not the same lightness.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsv<T: Float = f32> {
    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc.
    pub hue: RgbHue<T>,

    ///The colorfulness of the color. 0.0 gives gray scale colors and 1.0 will
    ///give absolutely clear colors.
    pub saturation: T,

    ///Decides how bright the color will look. 0.0 will be black, and 1.0 will
    ///give a bright an clear color that goes towards white when `saturation`
    ///goes towards 0.0.
    pub value: T,

    ///The transparency of the color. 0.0 is completely transparent and 1.0 is
    ///completely opaque.
    pub alpha: T,
}

impl<T: Float> Hsv<T> {
    ///Linear HSV.
    pub fn hsv(hue: RgbHue<T>, saturation: T, value: T) -> Hsv<T> {
        Hsv {
            hue: hue,
            saturation: saturation,
            value: value,
            alpha: T::one(),
        }
    }

    ///Linear HSV and transparency.
    pub fn hsva(hue: RgbHue<T>, saturation: T, value: T, alpha: T) -> Hsv<T> {
        Hsv {
            hue: hue,
            saturation: saturation,
            value: value,
            alpha: alpha,
        }
    }
}

impl<T: Float> ColorSpace for Hsv<T> {
    fn is_valid(&self) -> bool {
        self.saturation >= T::zero() && self.saturation <= T::one() &&
        self.value >= T::zero() && self.value <= T::one() && self.alpha >= T::zero() &&
        self.alpha <= T::one()
    }

    fn clamp(&self) -> Hsv<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.saturation = clamp(self.saturation, T::zero(), T::one());
        self.value = clamp(self.value, T::zero(), T::one());
        self.alpha = clamp(self.alpha, T::zero(), T::one());
    }
}

impl<T: Float> Mix for Hsv<T> {
    type Scalar = T;
    
    fn mix(&self, other: &Hsv<T>, factor: T) -> Hsv<T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_float();

        Hsv {
            hue: self.hue + factor * hue_diff,
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            value: self.value + factor * (other.value - self.value),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl<T: Float> Shade for Hsv<T> {
    type Scalar = T;
    
    fn lighten(&self, amount: T) -> Hsv<T> {
        Hsv {
            hue: self.hue,
            saturation: self.saturation,
            value: self.value + amount,
            alpha: self.alpha,
        }
    }
}

impl<T: Float> GetHue for Hsv<T> {
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        if self.saturation <= T::zero() || self.value <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<T: Float> Hue for Hsv<T> {
    fn with_hue(&self, hue: RgbHue<T>) -> Hsv<T> {
        Hsv {
            hue: hue,
            saturation: self.saturation,
            value: self.value,
            alpha: self.alpha,
        }
    }

    fn shift_hue(&self, amount: RgbHue<T>) -> Hsv<T> {
        Hsv {
            hue: self.hue + amount,
            saturation: self.saturation,
            value: self.value,
            alpha: self.alpha,
        }
    }
}

impl<T: Float> Saturate for Hsv<T> {
    type Scalar = T;

    fn saturate(&self, factor: T) -> Hsv<T> {
        Hsv {
            hue: self.hue,
            saturation: self.saturation * (T::one() + factor),
            value: self.value,
            alpha: self.alpha,
        }
    }
}

impl<T: Float> Default for Hsv<T> {
    fn default() -> Hsv<T> {
        Hsv::hsv(RgbHue::from(T::zero()), T::zero(), T::zero())
    }
}

impl<T: Float> Add<Hsv<T>> for Hsv<T> {
    type Output = Hsv<T>;

    fn add(self, other: Hsv<T>) -> Hsv<T> {
        Hsv {
            hue: self.hue + other.hue,
            saturation: self.saturation + other.saturation,
            value: self.value + other.value,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl<T: Float> Add<T> for Hsv<T> {
    type Output = Hsv<T>;

    fn add(self, c: T) -> Hsv<T> {
        Hsv {
            hue: self.hue + c,
            saturation: self.saturation + c,
            value: self.value + c,
            alpha: self.alpha + c,
        }
    }
}

impl<T: Float> Sub<Hsv<T>> for Hsv<T> {
    type Output = Hsv<T>;

    fn sub(self, other: Hsv<T>) -> Hsv<T> {
        Hsv {
            hue: self.hue - other.hue,
            saturation: self.saturation - other.saturation,
            value: self.value - other.value,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl<T: Float> Sub<T> for Hsv<T> {
    type Output = Hsv<T>;

    fn sub(self, c: T) -> Hsv<T> {
        Hsv {
            hue: self.hue - c,
            saturation: self.saturation - c,
            value: self.value - c,
            alpha: self.alpha - c,
        }
    }
}

from_color!(to Hsv from Rgb, Luma, Xyz, Lab, Lch, Hsl);

impl<T: Float> From<Rgb<T>> for Hsv<T> {
    fn from(rgb: Rgb<T>) -> Hsv<T> {
        enum Channel { Red, Green, Blue };

        let val_min = rgb.red.min(rgb.green).min(rgb.blue);
        let mut val_max = rgb.red;
        let mut chan_max = Channel::Red;

        if rgb.green > val_max {
            chan_max = Channel::Green;
            val_max = rgb.green;
        }

        if rgb.blue > val_max {
            chan_max = Channel::Blue;
            val_max = rgb.blue;
        }

        let diff = val_max - val_min;

        let hue = if diff == T::zero() {
            T::zero()
        } else {
            T::from(60.0).unwrap() * match chan_max {
                Channel::Red => ((rgb.green - rgb.blue) / diff) % T::from(6.0).unwrap(),
                Channel::Green => ((rgb.blue - rgb.red) / diff + T::from(2.0).unwrap()),
                Channel::Blue => ((rgb.red - rgb.green) / diff + T::from(4.0).unwrap()),
            }
        };

        let saturation = if val_max == T::zero() {
            T::zero()
        } else {
            diff / val_max
        };

        Hsv {
            hue: hue.into(),
            saturation: saturation,
            value: val_max,
            alpha: rgb.alpha,
        }
    }
}

impl<T: Float> From<Luma<T>> for Hsv<T> {
    fn from(luma: Luma<T>) -> Hsv<T> {
        Rgb::from(luma).into()
    }
}

impl<T: Float> From<Xyz<T>> for Hsv<T> {
    fn from(xyz: Xyz<T>) -> Hsv<T> {
        Rgb::from(xyz).into()
    }
}

impl<T: Float> From<Lab<T>> for Hsv<T> {
    fn from(lab: Lab<T>) -> Hsv<T> {
        Rgb::from(lab).into()
    }
}

impl<T: Float> From<Lch<T>> for Hsv<T> {
    fn from(lch: Lch<T>) -> Hsv<T> {
        Rgb::from(lch).into()
    }
}

impl<T: Float> From<Hsl<T>> for Hsv<T> {
    fn from(hsl: Hsl<T>) -> Hsv<T> {
        let x = hsl.saturation * if hsl.lightness < T::from(0.5).unwrap() {
            hsl.lightness
        } else {
            T::one() - hsl.lightness
        };

        Hsv {
            hue: hsl.hue,
            saturation: T::from(2.0).unwrap() * x / (hsl.lightness + x),
            value: hsl.lightness + x,
            alpha: hsl.alpha,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Hsv;
    use ::{Rgb, Hsl};

    #[test]
    fn red() {
        let a = Hsv::from(Rgb::rgb(1.0, 0.0, 0.0));
        let b = Hsv::hsv(0.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::hsl(0.0.into(), 1.0, 0.5));

        assert_approx_eq!(a, b, [hue, saturation, value]);
        assert_approx_eq!(a, c, [hue, saturation, value]);
    }

    #[test]
    fn orange() {
        let a = Hsv::from(Rgb::rgb(1.0, 0.5, 0.0));
        let b = Hsv::hsv(30.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::hsl(30.0.into(), 1.0, 0.5));

        assert_approx_eq!(a, b, [hue, saturation, value]);
        assert_approx_eq!(a, c, [hue, saturation, value]);
    }

    #[test]
    fn green() {
        let a = Hsv::from(Rgb::rgb(0.0, 1.0, 0.0));
        let b = Hsv::hsv(120.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::hsl(120.0.into(), 1.0, 0.5));

        assert_approx_eq!(a, b, [hue, saturation, value]);
        assert_approx_eq!(a, c, [hue, saturation, value]);
    }

    #[test]
    fn blue() {
        let a = Hsv::from(Rgb::rgb(0.0, 0.0, 1.0));
        let b = Hsv::hsv(240.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::hsl(240.0.into(), 1.0, 0.5));

        assert_approx_eq!(a, b, [hue, saturation, value]);
        assert_approx_eq!(a, c, [hue, saturation, value]);
    }

    #[test]
    fn purple() {
        let a = Hsv::from(Rgb::rgb(0.5, 0.0, 1.0));
        let b = Hsv::hsv(270.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::hsl(270.0.into(), 1.0, 0.5));

        assert_approx_eq!(a, b, [hue, saturation, value]);
        assert_approx_eq!(a, c, [hue, saturation, value]);
    }
}
