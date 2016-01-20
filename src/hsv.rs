use std::ops::{Add, Sub};

use {Color, Rgb, Luma, Xyz, Lab, Lch, Hsl, ColorSpace, Mix, Shade, GetHue, Hue, Saturate, RgbHue,
     clamp};

///Linear HSV color space with an alpha component.
///
///HSV is a cylindrical version of [RGB](struct.Rgb.html) and it's very
///similar to [HSL](struct.Hsl.html). The difference is that the `value`
///component in HSV determines the _brightness_ of the color, and not the
///_lightness_. The difference is that, for example, red (100% R, 0% G, 0% B)
///and white (100% R, 100% G, 100% B) has the same brightness (or value), but
///not the same lightness.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsv<T: Float> {
    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc.
    pub hue: RgbHue<T>,

    ///The colorfulness of the color. T::zero() gives gray scale colors and T::one() will
    ///give absolutely clear colors.
    pub saturation: T,

    ///Decides how bright the color will look. T::zero() will be black, and T::one() will
    ///give a bright an clear color that goes towards white when `saturation`
    ///goes towards 0.0.
    pub value: T,

    ///The transparency of the color. T::zero() is completely transparent and T::one() is
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
    fn mix(&self, other: &Hsv<T>, factor: T) -> Hsv<T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).into();

        Hsv {
            hue: self.hue + factor * hue_diff,
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            value: self.value + factor * (other.value - self.value),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl<T: Float> Shade for Hsv<T> {
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
        Hsv::hsv(T::zero(), T::zero(), T::zero())
    }
}

impl Add<Hsv> for Hsv {
    type Output = Hsv;

    fn add(self, other: Hsv) -> Hsv {
        Hsv {
            hue: self.hue + other.hue,
            saturation: self.saturation + other.saturation,
            value: self.value + other.value,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl Add<f32> for Hsv {
    type Output = Hsv;

    fn add(self, c: f32) -> Hsv {
        Hsv {
            hue: self.hue + c,
            saturation: self.saturation + c,
            value: self.value + c,
            alpha: self.alpha + c,
        }
    }
}

impl Sub<Hsv> for Hsv {
    type Output = Hsv;

    fn sub(self, other: Hsv) -> Hsv {
        Hsv {
            hue: self.hue - other.hue,
            saturation: self.saturation - other.saturation,
            value: self.value - other.value,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl Sub<f32> for Hsv {
    type Output = Hsv;

    fn sub(self, c: f32) -> Hsv {
        Hsv {
            hue: self.hue - c,
            saturation: self.saturation - c,
            value: self.value - c,
            alpha: self.alpha - c,
        }
    }
}

from_color!(to Hsv from Rgb, Luma, Xyz, Lab, Lch, Hsl);

impl<T: Float> From<Rgb> for Hsv<T> {
    fn from(rgb: Rgb) -> Hsv<T> {
        enum Channel {
            Red,
            Green,
            Blue,
        };

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
            T::from(60.0).unwrap() *
            match chan_max {
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

impl<T: Float> From<Luma> for Hsv<T> {
    fn from(luma: Luma) -> Hsv<T> {
        Rgb::from(luma).into()
    }
}

impl<T: Float> From<Xyz> for Hsv<T> {
    fn from(xyz: Xyz) -> Hsv<T> {
        Rgb::from(xyz).into()
    }
}

impl<T: Float> From<Lab> for Hsv<T> {
    fn from(lab: Lab) -> Hsv<T> {
        Rgb::from(lab).into()
    }
}

impl<T: Float> From<Lch> for Hsv<T> {
    fn from(lch: Lch) -> Hsv<T> {
        Rgb::from(lch).into()
    }
}

impl<T: Float> From<Hsl> for Hsv<T> {
    fn from(hsl: Hsl) -> Hsv<T> {
        let x = hsl.saturation *
                if hsl.lightness < T::from(0.5).unwrap() {
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
    use {Rgb, Hsl};

    #[test]
    fn red() {
        let a = Hsv::from(Rgb::linear_rgb(T::one(), T::zero(), T::zero()));
        let b = Hsv::hsv(0.0.into(), T::one(), T::one());
        let c = Hsv::from(Hsl::hsl(0.0.into(), T::one(), T::from(0.5).unwrap()));

        assert_approx_eq!(a, b, [hue, saturation, value]);
        assert_approx_eq!(a, c, [hue, saturation, value]);
    }

    #[test]
    fn orange() {
        let a = Hsv::from(Rgb::linear_rgb(T::one(), T::from(0.5).unwrap(), T::zero()));
        let b = Hsv::hsv(30.0.into(), T::one(), T::one());
        let c = Hsv::from(Hsl::hsl(30.0.into(), T::one(), T::from(0.5).unwrap()));

        assert_approx_eq!(a, b, [hue, saturation, value]);
        assert_approx_eq!(a, c, [hue, saturation, value]);
    }

    #[test]
    fn green() {
        let a = Hsv::from(Rgb::linear_rgb(T::zero(), T::one(), T::zero()));
        let b = Hsv::hsv(120.0.into(), T::one(), T::one());
        let c = Hsv::from(Hsl::hsl(120.0.into(), T::one(), T::from(0.5).unwrap()));

        assert_approx_eq!(a, b, [hue, saturation, value]);
        assert_approx_eq!(a, c, [hue, saturation, value]);
    }

    #[test]
    fn blue() {
        let a = Hsv::from(Rgb::linear_rgb(T::zero(), T::zero(), T::one()));
        let b = Hsv::hsv(240.0.into(), T::one(), T::one());
        let c = Hsv::from(Hsl::hsl(240.0.into(), T::one(), T::from(0.5).unwrap()));

        assert_approx_eq!(a, b, [hue, saturation, value]);
        assert_approx_eq!(a, c, [hue, saturation, value]);
    }

    #[test]
    fn purple() {
        let a = Hsv::from(Rgb::linear_rgb(T::from(0.5).unwrap(), T::zero(), T::one()));
        let b = Hsv::hsv(270.0.into(), T::one(), T::one());
        let c = Hsv::from(Hsl::hsl(270.0.into(), T::one(), T::from(0.5).unwrap()));

        assert_approx_eq!(a, b, [hue, saturation, value]);
        assert_approx_eq!(a, c, [hue, saturation, value]);
    }
}
