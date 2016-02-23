use num::Float;

use std::ops::{Add, Sub};

use {Alpha, Rgb, Xyz, Hsv, Limited, Mix, Shade, GetHue, Hue, Saturate, RgbHue, FromColor, IntoColor, clamp, flt};
use ColorType;

///Linear HSL with an alpha component. See the [`Hsla` implementation in `Alpha`](struct.Alpha.html#Hsla).
pub type Hsla<T = f32> = Alpha<Hsl<T>>;

///Linear HSL color space.
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
}

impl<T: Float> Hsl<T> {
    ///Linear HSL.
    pub fn new(hue: RgbHue<T>, saturation: T, lightness: T) -> Hsl<T> {
        Hsl {
            hue: hue,
            saturation: saturation,
            lightness: lightness,
        }
    }
}

///<span id="Hsla"></span>[`Hsla`](type.Hsla.html) implementations.
impl<T: Float> Alpha<Hsl<T>> {
    ///Linear HSL and transparency.
    pub fn new(hue: RgbHue<T>, saturation: T, lightness: T, alpha: T) -> Hsla<T> {
        Alpha {
            color: Hsl::new(hue, saturation, lightness),
            alpha: alpha,
        }
    }
}

impl<T: Float> ColorType for Hsl<T> {
    type Scalar = T;
}

impl<T: Float> FromColor<T> for Hsl<T> {
    fn from_xyz(xyz: Xyz<T>) -> Self {
        let rgb: Rgb<T> = xyz.into_rgb();
        Self::from_rgb(rgb)
    }

    fn from_rgb(rgb: Rgb<T>) -> Self {
        let ( max, min, sep , coeff) = {
            let (max, min , sep, coeff) = if rgb.red > rgb.green {
                (rgb.red, rgb.green, rgb.green - rgb.blue, T::zero() )
            } else {
                (rgb.green, rgb.red, rgb.blue - rgb.red , flt(2.0))
            };
            if rgb.blue > max {
                ( rgb.blue, min , rgb.red - rgb.green , flt(4.0))
            } else {
                let min_val = if rgb.blue < min { rgb.blue } else { min };
                (max , min_val , sep, coeff)
            }
        };

        let mut h = T::zero();
        let mut s = T::zero();

        let sum = max + min;
        let l = sum / flt(2.0);
        if max != min {
            let d = max - min;
            s = if sum > T::one() { d /( flt::<T,_>(2.0) - sum) } else { d / sum };
            h = (( sep / d ) + coeff) *  flt(60.0);
        };

        Hsl {
            hue: h.into(),
            saturation: s,
            lightness: l,
        }
    }

    fn from_hsl(hsl: Hsl<T>) -> Self {
        hsl
    }

    fn from_hsv(hsv: Hsv<T>) -> Self {
        let x = (flt::<T,_>(2.0) - hsv.saturation) * hsv.value;
        let saturation = if !hsv.value.is_normal() {
            T::zero()
        } else if x < T::one() {
            if x.is_normal() { hsv.saturation * hsv.value / x } else { T::zero() }
        } else {
            let denom = flt::<T,_>(2.0) - x;
            if denom.is_normal() { hsv.saturation * hsv.value / denom } else { T::zero() }
        };

        Hsl {
            hue: hsv.hue,
            saturation: saturation,
            lightness: x / flt(2.0),
        }
    }

}

impl<T: Float> Limited for Hsl<T> {
    fn is_valid(&self) -> bool {
        self.saturation >= T::zero() && self.saturation <= T::one() &&
        self.lightness >= T::zero() && self.lightness <= T::one()
    }

    fn clamp(&self) -> Hsl<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.saturation = clamp(self.saturation, T::zero(), T::one());
        self.lightness = clamp(self.lightness, T::zero(), T::one());
    }
}

impl<T: Float> Mix for Hsl<T> {
    fn mix(&self, other: &Hsl<T>, factor: T) -> Hsl<T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();

        Hsl {
            hue: self.hue + factor * hue_diff,
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            lightness: self.lightness + factor * (other.lightness - self.lightness),
        }
    }
}

impl<T: Float> Shade for Hsl<T> {
    fn lighten(&self, amount: T) -> Hsl<T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation,
            lightness: self.lightness + amount,
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
        }
    }

    fn shift_hue(&self, amount: RgbHue<T>) -> Hsl<T> {
        Hsl {
            hue: self.hue + amount,
            saturation: self.saturation,
            lightness: self.lightness,
        }
    }
}

impl<T: Float> Saturate for Hsl<T> {
    fn saturate(&self, factor: T) -> Hsl<T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation * (T::one() + factor),
            lightness: self.lightness,
        }
    }
}

impl<T: Float> Default for Hsl<T> {
    fn default() -> Hsl<T> {
        Hsl::new(RgbHue::from(T::zero()), T::zero(), T::zero())
    }
}

impl<T: Float> Add<Hsl<T>> for Hsl<T> {
    type Output = Hsl<T>;

    fn add(self, other: Hsl<T>) -> Hsl<T> {
        Hsl {
            hue: self.hue + other.hue,
            saturation: self.saturation + other.saturation,
            lightness: self.lightness + other.lightness,
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
        }
    }
}

#[cfg(test)]
mod test {
    use super::Hsl;
    use {Rgb, Hsv};

    #[test]
    fn red() {
        let a = Hsl::from(Rgb::new(1.0, 0.0, 0.0));
        let b = Hsl::new(0.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::new(0.0.into(), 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);

    }

    #[test]
    fn orange() {
        let a = Hsl::from(Rgb::new(1.0, 0.5, 0.0));
        let b = Hsl::new(30.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::new(30.0.into(), 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn green() {
        let a = Hsl::from(Rgb::new(0.0, 1.0, 0.0));
        let b = Hsl::new(120.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::new(120.0.into(), 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn blue() {
        let a = Hsl::from(Rgb::new(0.0, 0.0, 1.0));
        let b = Hsl::new(240.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::new(240.0.into(), 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn purple() {
        let a = Hsl::from(Rgb::new(0.5, 0.0, 1.0));
        let b = Hsl::new(270.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::new(270.0.into(), 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn ranges() {
        assert_ranges!{
            Hsl;
            limited {
                saturation: 0.0 => 1.0,
                lightness: 0.0 => 1.0
            }
            limited_min {}
            unlimited {
                hue: -360.0 => 360.0
            }
        }
    }
}
