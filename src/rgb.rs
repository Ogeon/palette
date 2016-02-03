use num::Float;

use std::ops::{Add, Sub, Mul, Div};
use std::marker::PhantomData;

use {Alpha, Luma, Xyz, Yxy, Lab, Lch, Hsv, Hsl, Limited, Mix, Shade, GetHue, RgbHue, clamp, flt};
use pixel::RgbPixel;
use white_point::{WhitePoint, D65};
use rgb_variant::{RgbVariant, SrgbSpace};

pub type Rgb<T> = RgbColor<SrgbSpace, D65, T>;
pub type Rgba<T> = AlphaRgbColor<SrgbSpace, D65, T>;

///Linear RGB with an alpha component. See the [`Rgba` implementation in `Alpha`](struct.Alpha.html#Rgba).
pub type AlphaRgbColor<CS, WP, T = f32> = Alpha<RgbColor<CS, WP, T>, T>;

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
#[derive(Debug, PartialEq)]
pub struct RgbColor<CS, WP, T = f32>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    ///The amount of red light, where 0.0 is no red light and 1.0 is the
    ///highest displayable amount.
    pub red: T,

    ///The amount of green light, where 0.0 is no green light and 1.0 is the
    ///highest displayable amount.
    pub green: T,

    ///The amount of blue light, where 0.0 is no blue light and 1.0 is the
    ///highest displayable amount.
    pub blue: T,

    _wp: PhantomData<WP>,

    _cs: PhantomData<CS>,
}

impl<CS, WP, T> Copy for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{}

impl<CS, WP, T> Clone for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    fn clone(&self) -> RgbColor<CS, WP, T> { *self }
}

impl<CS, WP, T> RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    ///Linear RGB.
    pub fn new(red: T, green: T, blue: T) -> RgbColor<CS, WP, T> {
        RgbColor {
            red: red,
            green: green,
            blue: blue,
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }

    ///Linear RGB from 8 bit values.
    pub fn new_u8(red: u8, green: u8, blue: u8) -> RgbColor<CS, WP, T> {
        RgbColor {
            red: flt::<T,_>(red) / flt(255.0),
            green: flt::<T,_>(green) / flt(255.0),
            blue: flt::<T,_>(blue) / flt(255.0),
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }

    ///Linear RGB from a linear pixel value.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P) -> RgbColor<CS, WP, T> {
        let (r, g, b, _) = pixel.to_rgba();
        RgbColor::new(r, g, b)
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
impl<CS, WP, T> Alpha<RgbColor<CS, WP, T>, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    ///Linear RGB with transparency.
    pub fn new(red: T, green: T, blue: T, alpha: T) -> AlphaRgbColor<CS, WP, T> {
        AlphaRgbColor {
            color: RgbColor::new(red, green, blue),
            alpha: alpha,
        }
    }

    ///Linear RGB with transparency from 8 bit values.
    pub fn new_u8(red: u8, green: u8, blue: u8, alpha: u8) -> AlphaRgbColor<CS, WP, T> {
        Alpha {
            color: RgbColor::new_u8(red, green, blue),
            alpha: flt::<T,_>(alpha) / flt(255.0),
        }
    }

    ///Linear RGB from a linear pixel value.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P) -> AlphaRgbColor<CS, WP, T> {
        let (r, g, b, a) = pixel.to_rgba();
        AlphaRgbColor::new(r, g, b, a)
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

impl<CS, WP, T> Limited for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    fn is_valid(&self) -> bool {
        self.red >= T::zero() && self.red <= T::one() &&
        self.green >= T::zero() && self.green <= T::one() &&
        self.blue >= T::zero() && self.blue <= T::one()
    }

    fn clamp(&self) -> RgbColor<CS, WP, T> {
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

impl<CS, WP, T> Mix for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    type Scalar = T;

    fn mix(&self, other: &RgbColor<CS, WP, T>, factor: T) -> RgbColor<CS, WP, T> {
        let factor = clamp(factor, T::zero(), T::one());

        RgbColor {
            red: self.red + factor * (other.red - self.red),
            green: self.green + factor * (other.green - self.green),
            blue: self.blue + factor * (other.blue - self.blue),
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }
}

impl<CS, WP, T> Shade for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> RgbColor<CS, WP, T> {
        RgbColor {
            red: self.red + amount,
            green: self.green + amount,
            blue: self.blue + amount,
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }
}

impl<CS, WP, T> GetHue for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
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

impl<CS, WP, T> Default for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    fn default() -> RgbColor<CS, WP, T> {
        RgbColor::new(T::zero(), T::zero(), T::zero())
    }
}

impl<CS, WP, T> Add<RgbColor<CS, WP, T>> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    type Output = RgbColor<CS, WP, T>;

    fn add(self, other: RgbColor<CS, WP, T>) -> RgbColor<CS, WP, T> {
        RgbColor {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }
}

impl<CS, WP, T> Add<T> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    type Output = RgbColor<CS, WP, T>;

    fn add(self, c: T) -> RgbColor<CS, WP, T> {
        RgbColor {
            red: self.red + c,
            green: self.green + c,
            blue: self.blue + c,
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }
}

impl<CS, WP, T> Sub<RgbColor<CS, WP, T>> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    type Output = RgbColor<CS, WP, T>;

    fn sub(self, other: RgbColor<CS, WP, T>) -> RgbColor<CS, WP, T> {
        RgbColor {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }
}

impl<CS, WP, T> Sub<T> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    type Output = RgbColor<CS, WP, T>;

    fn sub(self, c: T) -> RgbColor<CS, WP, T> {
        RgbColor {
            red: self.red - c,
            green: self.green - c,
            blue: self.blue - c,
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }
}

impl<CS, WP, T> Mul<RgbColor<CS, WP, T>> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    type Output = RgbColor<CS, WP, T>;

    fn mul(self, other: RgbColor<CS, WP, T>) -> RgbColor<CS, WP, T> {
        RgbColor {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }
}

impl<CS, WP, T> Mul<T> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    type Output = RgbColor<CS, WP, T>;

    fn mul(self, c: T) -> RgbColor<CS, WP, T> {
        RgbColor {
            red: self.red * c,
            green: self.green * c,
            blue: self.blue * c,
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }
}

impl<CS, WP, T> Div<RgbColor<CS, WP, T>> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    type Output = RgbColor<CS, WP, T>;

    fn div(self, other: RgbColor<CS, WP, T>) -> RgbColor<CS, WP, T> {
        RgbColor {
            red: self.red / other.red,
            green: self.green / other.green,
            blue: self.blue / other.blue,
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }
}

impl<CS, WP, T> Div<T> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    type Output = RgbColor<CS, WP, T>;

    fn div(self, c: T) -> RgbColor<CS, WP, T> {
        RgbColor {
            red: self.red / c,
            green: self.green / c,
            blue: self.blue / c,
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }
}

// from_color!(to Rgb from Xyz, Yxy, Luma, Lab, Lch, Hsv, Hsl);

// alpha_from!(Rgb {Xyz, Yxy, Luma, Lab, Lch, Hsv, Hsl, Color});

impl<CS, WP, T> From<Luma<T>> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    fn from(luma: Luma<T>) -> RgbColor<CS, WP, T> {
        RgbColor {
            red: luma.luma,
            green: luma.luma,
            blue: luma.luma,
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }
}


impl<CS, WP, T> From<Xyz<T>> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    fn from(xyz: Xyz<T>) -> RgbColor<CS, WP, T> {
        RgbColor {
            red: xyz.x * flt(3.2406) + xyz.y * flt(-1.5372) + xyz.z * flt(-0.4986),
            green: xyz.x * flt(-0.9689) + xyz.y * flt(1.8758) + xyz.z * flt(0.0415),
            blue: xyz.x * flt(0.0557) + xyz.y * flt(-0.2040) + xyz.z * flt(1.0570),
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }
}

impl<CS, WP, T> From<Yxy<T>> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    fn from(yxy: Yxy<T>) -> RgbColor<CS, WP, T> {
        Xyz::from(yxy).into()
    }
}

impl<CS, WP, T> From<Lab<T>> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    fn from(lab: Lab<T>) -> RgbColor<CS, WP, T> {
        Xyz::from(lab).into()
    }
}

impl<CS, WP, T> From<Lch<T>> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    fn from(lch: Lch<T>) -> RgbColor<CS, WP, T> {
        Lab::from(lch).into()
    }
}

impl<CS, WP, T> From<Hsv<T>> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    fn from(hsv: Hsv<T>) -> RgbColor<CS, WP, T> {
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


        RgbColor {
            red: red + m,
            green: green + m,
            blue: blue + m,
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }
}

impl<CS, WP, T> From<Hsl<T>> for RgbColor<CS, WP, T>
    where T: Float,
        WP: WhitePoint<T>,
        CS: RgbVariant<T>
{
    fn from(hsl: Hsl<T>) -> RgbColor<CS, WP, T> {
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


        RgbColor {
            red: red + m,
            green: green + m,
            blue: blue + m,
            _wp: PhantomData,
            _cs: PhantomData,
        }
    }
}

// impl<CS, WP, T> From<Srgb<T>> for RgbColor<CS, WP, T>
//     fn from(srgb: Srgb<T>) -> RgbColor<CS, WP, T> {
//         srgb.to_linear().into()
//     }
// }
//
// impl<CS, WP, T> From<GammaRgbColor<CS, WP, T>> for RgbColor<CS, WP, T>
//     fn from(gamma_rgb: GammaRgbColor<CS, WP, T>) -> RgbColor<CS, WP, T> {
//         gamma_rgb.to_linear().into()
//     }
// }

// impl<CS, WP, T> From<Srgb<T>> for Alpha<RgbColor<CS, WP, T>, T> {
//     fn from(srgb: Srgb<T>) -> Alpha<RgbColor<CS, WP, T>, T> {
//         srgb.to_linear()
//     }
// }
//
// impl<CS, WP, T> From<GammaRgbColor<CS, WP, T>> for Alpha<RgbColor<CS, WP, T>, T> {
//     fn from(gamma_rgb: GammaRgbColor<CS, WP, T>) -> Alpha<RgbColor<CS, WP, T>, T> {
//         gamma_rgb.to_linear()
//     }
// }

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
