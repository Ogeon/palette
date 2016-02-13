use num::Float;

use std::ops::{Add, Sub, Mul, Div};

use {Alpha, Xyz, Luma, Hsl, Hsv, Limited, Mix, Shade, GetHue, RgbHue, FromColor, clamp, flt};
use pixel::{RgbPixel, Srgb, GammaRgb};

///Linear RGB with an alpha component. See the [`Rgba` implementation in `Alpha`](struct.Alpha.html#Rgba).
pub type Rgba<T = f32> = Alpha<Rgb<T>, T>;

///Linear RGB.
///
///RGB is probably the most common color space, when it comes to computer
///graphics, and it's defined as an additive mixture of red, green and blue
///light, where gray scale colors are created when these three channels are
///equal in strength. This particular RGB type is based on the ITU-R BT.709
///primaries, which makes it a linear version of sRGB.
///
///Conversions and operations on this color space assumes that it's linear,
///meaning that gamma correction is required when converting to and from a
///displayable RGB, such as sRGB. See the [`pixel`](pixel/index.html) module
///for encoding types.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb<T: Float = f32> {
    ///The amount of red light, where 0.0 is no red light and 1.0 is the
    ///highest displayable amount.
    pub red: T,

    ///The amount of green light, where 0.0 is no green light and 1.0 is the
    ///highest displayable amount.
    pub green: T,

    ///The amount of blue light, where 0.0 is no blue light and 1.0 is the
    ///highest displayable amount.
    pub blue: T,
}

impl<T: Float> Rgb<T> {
    ///Linear RGB.
    pub fn new(red: T, green: T, blue: T) -> Rgb<T> {
        Rgb {
            red: red,
            green: green,
            blue: blue,
        }
    }

    ///Linear RGB from 8 bit values.
    pub fn new_u8(red: u8, green: u8, blue: u8) -> Rgb<T> {
        Rgb {
            red: flt::<T,_>(red) / flt(255.0),
            green: flt::<T,_>(green) / flt(255.0),
            blue: flt::<T,_>(blue) / flt(255.0),
        }
    }

    ///Linear RGB from a linear pixel value.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P) -> Rgb<T> {
        let (r, g, b, _) = pixel.to_rgba();
        Rgb::new(r, g, b)
    }

    ///Convert to a linear RGB pixel. `Rgb` is already assumed to be linear,
    ///so the components will just be clamped to [0.0, 1.0] before conversion.
    ///
    ///```
    ///use palette::Rgb;
    ///
    ///let c = Rgb::new(0.5, 0.3, 0.1);
    ///assert_eq!((c.red, c.green, c.blue), c.to_pixel());
    ///assert_eq!((0.5, 0.3, 0.1), c.to_pixel());
    ///```
    pub fn to_pixel<P: RgbPixel<T>>(&self) -> P {
        P::from_rgba(
            clamp(self.red, T::zero(), T::one()),
            clamp(self.green, T::zero(), T::one()),
            clamp(self.blue, T::zero(), T::one()),
            T::one(),
        )
    }
}

///<span id="Rgba"></span>[`Rgba`](type.Rgba.html) implementations.
impl<T: Float> Alpha<Rgb<T>, T> {
    ///Linear RGB with transparency.
    pub fn new(red: T, green: T, blue: T, alpha: T) -> Rgba<T> {
        Alpha {
            color: Rgb::new(red, green, blue),
            alpha: alpha,
        }
    }

    ///Linear RGB with transparency from 8 bit values.
    pub fn new_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Rgba<T> {
        Alpha {
            color: Rgb::new_u8(red, green, blue),
            alpha: flt::<T,_>(alpha) / flt(255.0),
        }
    }

    ///Linear RGB from a linear pixel value.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P) -> Rgba<T> {
        let (r, g, b, a) = pixel.to_rgba();
        Rgba::new(r, g, b, a)
    }

    ///Convert to a linear RGB pixel. `Rgb` is already assumed to be linear,
    ///so the components will just be clamped to [0.0, 1.0] before conversion.
    ///
    ///```
    ///use palette::Rgba;
    ///
    ///let c = Rgba::new(0.5, 0.3, 0.1, 0.5);
    ///assert_eq!((c.red, c.green, c.blue, c.alpha), c.to_pixel());
    ///assert_eq!((0.5, 0.3, 0.1, 0.5), c.to_pixel());
    ///```
    pub fn to_pixel<P: RgbPixel<T>>(&self) -> P {
        P::from_rgba(
            clamp(self.red, T::zero(), T::one()),
            clamp(self.green, T::zero(), T::one()),
            clamp(self.blue, T::zero(), T::one()),
            clamp(self.alpha, T::zero(), T::one())
        )
    }
}

impl<T: Float> FromColor<T> for Rgb<T> {
    fn from_xyz(xyz: Xyz<T>) -> Self {
        Rgb {
            red: xyz.x * flt(3.2404542) + xyz.y * flt(-1.5371385) + xyz.z * flt(-0.4985314),
            green: xyz.x * flt(-0.9692660) + xyz.y * flt(1.8760108) + xyz.z * flt(0.0415560),
            blue: xyz.x * flt(0.0556434) + xyz.y * flt(-0.2040259) + xyz.z * flt(1.0572252),
        }
    }


    fn from_rgb(rgb: Rgb<T>) -> Self {
        rgb
    }

    fn from_hsl(hsl: Hsl<T>) -> Self {
        let c = (T::one() - ( hsl.lightness * flt(2.0) - T::one()).abs()) * hsl.saturation;
        let h = hsl.hue.to_positive_degrees() / flt(60.0);
        let x = c * (T::one() - (h % flt(2.0) - T::one()).abs());
        let m = hsl.lightness - c * flt(0.5);

        let (red, green, blue) = if h >= T::zero() && h < T::one() {
            (c, x, T::zero())
        } else if h >= T::one() && h < flt(2.0) {
            (x, c, T::zero())
        } else if h >= flt(2.0) && h < flt(3.0) {
            (T::zero(), c, x)
        } else if h >= flt(3.0) && h < flt(4.0) {
            (T::zero(), x, c)
        } else if h >= flt(4.0) && h < flt(5.0) {
            (x, T::zero(), c)
        } else {
            (c, T::zero(), x)
        };


        Rgb {
            red: red + m,
            green: green + m,
            blue: blue + m,
        }
    }

    fn from_hsv(hsv: Hsv<T>) -> Self {
        let c = hsv.value * hsv.saturation;
        let h = hsv.hue.to_positive_degrees() / flt(60.0);
        let x = c * (T::one() - (h % flt(2.0) - T::one()).abs());
        let m = hsv.value - c;

        let (red, green, blue) = if h >= T::zero() && h < T::one() {
            (c, x, T::zero())
        } else if h >= T::one() && h < flt(2.0) {
            (x, c, T::zero())
        } else if h >= flt(2.0) && h < flt(3.0) {
            (T::zero(), c, x)
        } else if h >= flt(3.0) && h < flt(4.0) {
            (T::zero(), x, c)
        } else if h >= flt(4.0) && h < flt(5.0) {
            (x, T::zero(), c)
        } else {
            (c, T::zero(), x)
        };


        Rgb {
            red: red + m,
            green: green + m,
            blue: blue + m,
        }

    }

    fn from_luma(luma: Luma<T>) -> Self {
        Rgb {
            red: luma.luma,
            green: luma.luma,
            blue: luma.luma,
        }
    }

}

impl<T: Float> Limited for Rgb<T> {
    fn is_valid(&self) -> bool {
        self.red >= T::zero() && self.red <= T::one() &&
        self.green >= T::zero() && self.green <= T::one() &&
        self.blue >= T::zero() && self.blue <= T::one()
    }

    fn clamp(&self) -> Rgb<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.red = clamp(self.red, T::zero(), T::one());
        self.green = clamp(self.green, T::zero(), T::one());
        self.blue = clamp(self.blue, T::zero(), T::one());
    }
}

impl<T: Float> Mix for Rgb<T> {
    type Scalar = T;

    fn mix(&self, other: &Rgb<T>, factor: T) -> Rgb<T> {
        let factor = clamp(factor, T::zero(), T::one());

        Rgb {
            red: self.red + factor * (other.red - self.red),
            green: self.green + factor * (other.green - self.green),
            blue: self.blue + factor * (other.blue - self.blue),
        }
    }
}

impl<T: Float> Shade for Rgb<T> {
    type Scalar = T;

    fn lighten(&self, amount: T) -> Rgb<T> {
        Rgb {
            red: self.red + amount,
            green: self.green + amount,
            blue: self.blue + amount,
        }
    }
}

impl<T: Float> GetHue for Rgb<T> {
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        let sqrt_3: T = flt(1.73205081);

        if self.red == self.green && self.red == self.blue {
            None
        } else {
            Some(RgbHue::from_radians((sqrt_3 * (self.green - self.blue)).atan2(self.red * flt(2.0) - self.green - self.blue)))
        }
    }
}

impl<T: Float> Default for Rgb<T> {
    fn default() -> Rgb<T> {
        Rgb::new(T::zero(), T::zero(), T::zero())
    }
}

impl<T: Float> Add<Rgb<T>> for Rgb<T> {
    type Output = Rgb<T>;

    fn add(self, other: Rgb<T>) -> Rgb<T> {
        Rgb {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl<T: Float> Add<T> for Rgb<T> {
    type Output = Rgb<T>;

    fn add(self, c: T) -> Rgb<T> {
        Rgb {
            red: self.red + c,
            green: self.green + c,
            blue: self.blue + c,
        }
    }
}

impl<T: Float> Sub<Rgb<T>> for Rgb<T> {
    type Output = Rgb<T>;

    fn sub(self, other: Rgb<T>) -> Rgb<T> {
        Rgb {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl<T: Float> Sub<T> for Rgb<T> {
    type Output = Rgb<T>;

    fn sub(self, c: T) -> Rgb<T> {
        Rgb {
            red: self.red - c,
            green: self.green - c,
            blue: self.blue - c,
        }
    }
}

impl<T: Float> Mul<Rgb<T>> for Rgb<T> {
    type Output = Rgb<T>;

    fn mul(self, other: Rgb<T>) -> Rgb<T> {
        Rgb {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl<T: Float> Mul<T> for Rgb<T> {
    type Output = Rgb<T>;

    fn mul(self, c: T) -> Rgb<T> {
        Rgb {
            red: self.red * c,
            green: self.green * c,
            blue: self.blue * c,
        }
    }
}

impl<T: Float> Div<Rgb<T>> for Rgb<T> {
    type Output = Rgb<T>;

    fn div(self, other: Rgb<T>) -> Rgb<T> {
        Rgb {
            red: self.red / other.red,
            green: self.green / other.green,
            blue: self.blue / other.blue,
        }
    }
}

impl<T: Float> Div<T> for Rgb<T> {
    type Output = Rgb<T>;

    fn div(self, c: T) -> Rgb<T> {
        Rgb {
            red: self.red / c,
            green: self.green / c,
            blue: self.blue / c,
        }
    }
}

impl<T: Float> From<Srgb<T>> for Rgb<T> {
    fn from(srgb: Srgb<T>) -> Rgb<T> {
        srgb.to_linear().into()
    }
}

impl<T: Float> From<GammaRgb<T>> for Rgb<T> {
    fn from(gamma_rgb: GammaRgb<T>) -> Rgb<T> {
        gamma_rgb.to_linear().into()
    }
}

impl<T: Float> From<Srgb<T>> for Alpha<Rgb<T>, T> {
    fn from(srgb: Srgb<T>) -> Alpha<Rgb<T>, T> {
        srgb.to_linear()
    }
}

impl<T: Float> From<GammaRgb<T>> for Alpha<Rgb<T>, T> {
    fn from(gamma_rgb: GammaRgb<T>) -> Alpha<Rgb<T>, T> {
        gamma_rgb.to_linear()
    }
}

#[cfg(test)]
mod test {
    use Rgb;

    #[test]
    fn ranges() {
        assert_ranges!{
            Rgb;
            limited {
                red: 0.0 => 1.0,
                green: 0.0 => 1.0,
                blue: 0.0 => 1.0
            }
            limited_min {}
            unlimited {}
        }
    }
}
