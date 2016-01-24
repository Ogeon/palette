use num::traits::Float;

use std::ops::{Add, Sub};

use {Color, Rgb, Luma, Xyz, Lab, Lch, Hsv, ColorSpace, Mix, Shade, GetHue, Hue, Saturate, RgbHue, clamp};

///Linear HSL color space with an alpha component.
///
///The HSL color space can be seen as a cylindrical version of
///[RGB](struct.Rgb.html), where the `hue` is the angle around the color
///cylinder, the `saturation` is the distance from the center, and the
///`lightness` is the height from the bottom. Its composition makes it
///especially good for operations like changing green to red, making a color
///more gray, or making it darker.
///
///See [HSV](struct.Hsv.html) for a very similar color space, with brightness instead of lightness.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsl<T: Float = f32> {
    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc.
    pub hue: RgbHue<T>,

    ///The colorfulness of the color. 0.0 gives gray scale colors and 1.0 will
    ///give absolutely clear colors.
    pub saturation: T,

    ///Decides how light the color will look. 0.0 will be black, 0.5 will give
    ///a clear color, and 1.0 will give white.
    pub lightness: T,

    ///The transparency of the color. 0.0 is completely transparent and 1.0 is
    ///completely opaque.
    pub alpha: T,
}

impl<T: Float> Hsl<T> {
    ///Linear HSL.
    pub fn hsl(hue: RgbHue<T>, saturation: T, lightness: T) -> Hsl<T> {
        Hsl {
            hue: hue,
            saturation: saturation,
            lightness: lightness,
            alpha: T::one(),
        }
    }

    ///Linear HSL and transparency.
    pub fn hsla(hue: RgbHue<T>, saturation: T, lightness: T, alpha: T) -> Hsl<T> {
        Hsl {
            hue: hue,
            saturation: saturation,
            lightness: lightness,
            alpha: alpha,
        }
    }
}

impl<T: Float> ColorSpace for Hsl<T> {
    fn is_valid(&self) -> bool {
        self.saturation >= T::zero() && self.saturation <= T::one() &&
        self.lightness >= T::zero() && self.lightness <= T::one() &&
        self.alpha >= T::zero() && self.alpha <= T::one()
    }

    fn clamp(&self) -> Hsl<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.saturation = clamp(self.saturation, T::zero(), T::one());
        self.lightness = clamp(self.lightness, T::zero(), T::one());
        self.alpha = clamp(self.alpha, T::zero(), T::one());
    }
}

impl<T: Float> Mix for Hsl<T> {
    type Scalar = T;

    fn mix(&self, other: &Hsl<T>, factor: T) -> Hsl<T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_float();

        Hsl {
            hue: self.hue + factor * hue_diff,
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            lightness: self.lightness + factor * (other.lightness - self.lightness),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl<T: Float> Shade for Hsl<T> {
    type Scalar = T;

    fn lighten(&self, amount: T) -> Hsl<T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation,
            lightness: self.lightness + amount,
            alpha: self.alpha,
        }
    }
}

impl<T: Float> GetHue for Hsl<T> {
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        if self.saturation <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<T: Float> Hue for Hsl<T> {
    fn with_hue(&self, hue: RgbHue<T>) -> Hsl<T> {
        Hsl {
            hue: hue,
            saturation: self.saturation,
            lightness: self.lightness,
            alpha: self.alpha,
        }
    }

    fn shift_hue(&self, amount: RgbHue<T>) -> Hsl<T> {
        Hsl {
            hue: self.hue + amount,
            saturation: self.saturation,
            lightness: self.lightness,
            alpha: self.alpha,
        }
    }
}

impl<T: Float> Saturate for Hsl<T> {
    type Scalar = T;

    fn saturate(&self, factor: T) -> Hsl<T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation * (T::one() + factor),
            lightness: self.lightness,
            alpha: self.alpha,
        }
    }
}

impl<T: Float> Default for Hsl<T> {
    fn default() -> Hsl<T> {
        Hsl::hsl(RgbHue::from(T::zero()), T::zero(), T::zero())
    }
}

impl<T: Float> Add<Hsl<T>> for Hsl<T> {
    type Output = Hsl<T>;

    fn add(self, other: Hsl<T>) -> Hsl<T> {
        Hsl {
            hue: self.hue + other.hue,
            saturation: self.saturation + other.saturation,
            lightness: self.lightness + other.lightness,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl<T: Float> Add<T> for Hsl<T> {
    type Output = Hsl<T>;

    fn add(self, c: T) -> Hsl<T> {
        Hsl {
            hue: self.hue + c,
            saturation: self.saturation + c,
            lightness: self.lightness + c,
            alpha: self.alpha + c,
        }
    }
}

impl<T: Float> Sub<Hsl<T>> for Hsl<T> {
    type Output = Hsl<T>;

    fn sub(self, other: Hsl<T>) -> Hsl<T> {
        Hsl {
            hue: self.hue - other.hue,
            saturation: self.saturation - other.saturation,
            lightness: self.lightness - other.lightness,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl<T: Float> Sub<T> for Hsl<T> {
    type Output = Hsl<T>;

    fn sub(self, c: T) -> Hsl<T> {
        Hsl {
            hue: self.hue - c,
            saturation: self.saturation - c,
            lightness: self.lightness - c,
            alpha: self.alpha - c,
        }
    }
}

from_color!(to Hsl from Rgb, Luma, Xyz, Lab, Lch, Hsv);

impl<T: Float> From<Rgb<T>> for Hsl<T> {
    fn from(rgb: Rgb<T>) -> Hsl<T> {
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
        let lightness = (val_min + val_max) / T::from(2.0).unwrap();

        let hue = if diff == T::zero() {
            T::zero()
        } else {
            T::from(60.0).unwrap() * match chan_max {
                Channel::Red => ((rgb.green - rgb.blue) / diff) % T::from(6.0).unwrap(),
                Channel::Green => ((rgb.blue - rgb.red) / diff + T::from(2.0).unwrap()),
                Channel::Blue => ((rgb.red - rgb.green) / diff + T::from(4.0).unwrap()),
            }
        };

        let saturation = if diff == T::zero() {
            T::zero()
        } else {
            diff / (T::one() - (T::from(2.0).unwrap() * lightness - T::one()).abs())
        };

        Hsl {
            hue: hue.into(),
            saturation: saturation,
            lightness: lightness,
            alpha: rgb.alpha,
        }
    }
}

impl<T: Float> From<Luma<T>> for Hsl<T> {
    fn from(luma: Luma<T>) -> Hsl<T> {
        Rgb::from(luma).into()
    }
}

impl<T: Float> From<Xyz<T>> for Hsl<T> {
    fn from(xyz: Xyz<T>) -> Hsl<T> {
        Rgb::from(xyz).into()
    }
}

impl<T: Float> From<Lab<T>> for Hsl<T> {
    fn from(lab: Lab<T>) -> Hsl<T> {
        Rgb::from(lab).into()
    }
}

impl<T: Float> From<Lch<T>> for Hsl<T> {
    fn from(lch: Lch<T>) -> Hsl<T> {
        Rgb::from(lch).into()
    }
}

impl<T: Float> From<Hsv<T>> for Hsl<T> {
    fn from(hsv: Hsv<T>) -> Hsl<T> {
        let x = (T::from(2.0).unwrap() - hsv.saturation) * hsv.value;
        let saturation = if hsv.value == T::zero() {
            T::zero()
        } else if x < T::one() {
            hsv.saturation * hsv.value / x
        } else {
            hsv.saturation * hsv.value / (T::from(2.0).unwrap() - x)
        };

        Hsl {
            hue: hsv.hue,
            saturation: saturation,
            lightness: x / T::from(2.0).unwrap(),
            alpha: hsv.alpha,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Hsl;
    use ::{Rgb, Hsv};

    #[test]
    fn red() {
        let a = Hsl::from(Rgb::rgb(1.0, 0.0, 0.0));
        let b = Hsl::hsl(0.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::hsv(0.0.into(), 1.0, 1.0));

        assert_approx_eq!(a, b, [hue, saturation, lightness]);
        assert_approx_eq!(a, c, [hue, saturation, lightness]);
    }

    #[test]
    fn orange() {
        let a = Hsl::from(Rgb::rgb(1.0, 0.5, 0.0));
        let b = Hsl::hsl(30.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::hsv(30.0.into(), 1.0, 1.0));

        assert_approx_eq!(a, b, [hue, saturation, lightness]);
        assert_approx_eq!(a, c, [hue, saturation, lightness]);
    }

    #[test]
    fn green() {
        let a = Hsl::from(Rgb::rgb(0.0, 1.0, 0.0));
        let b = Hsl::hsl(120.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::hsv(120.0.into(), 1.0, 1.0));

        assert_approx_eq!(a, b, [hue, saturation, lightness]);
        assert_approx_eq!(a, c, [hue, saturation, lightness]);
    }

    #[test]
    fn blue() {
        let a = Hsl::from(Rgb::rgb(0.0, 0.0, 1.0));
        let b = Hsl::hsl(240.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::hsv(240.0.into(), 1.0, 1.0));

        assert_approx_eq!(a, b, [hue, saturation, lightness]);
        assert_approx_eq!(a, c, [hue, saturation, lightness]);
    }

    #[test]
    fn purple() {
        let a = Hsl::from(Rgb::rgb(0.5, 0.0, 1.0));
        let b = Hsl::hsl(270.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::hsv(270.0.into(), 1.0, 1.0));

        assert_approx_eq!(a, b, [hue, saturation, lightness]);
        assert_approx_eq!(a, c, [hue, saturation, lightness]);
    }
}
