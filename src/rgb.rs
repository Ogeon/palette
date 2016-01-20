use std::ops::{Add, Sub, Mul, Div};

use {Color, Luma, Xyz, Lab, Lch, Hsv, Hsl, ColorSpace, Mix, Shade, GetHue, RgbHue, clamp};

///Linear RGB with an alpha component.
///
///RGB is probably the most common color space, when it comes to computer
///graphics, and it's defined as an additive mixture of red, green and blue
///light, where gray scale colors are created when these three channels are
///equal in strength. This version of RGB is based on sRGB, which is pretty
///much the standard RGB model today.
///
///Conversions and operations on this color space assumes that it's linear,
///meaning that gamma correction is required when converting to and from
///a displayable RGB, such as sRGB.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb<T: Float> {
    ///The amount of red light, where T::Zero() is no red light and T::One() is the
    ///highest displayable amount.
    pub red: T,

    ///The amount of green light, where T::Zero() is no green light and T::One() is the
    ///highest displayable amount.
    pub green: T,

    ///The amount of blue light, where T::Zero() is no blue light and T::One() is the
    ///highest displayable amount.
    pub blue: T,

    ///The transparency of the color. T::Zero() is completely transparent and T::One() is
    ///completely opaque.
    pub alpha: T,
}

///Creation from linear RGB.
impl<T: Float> Rgb<T> {
    ///Linear RGB.
    pub fn linear_rgb(red: T, green: T, blue: T) -> Rgb<T> {
        Rgb {
            red: red,
            green: green,
            blue: blue,
            alpha: T::One(),
        }
    }

    ///Linear RGB with transparency.
    pub fn linear_rgba(red: T, green: T, blue: T, alpha: T) -> Rgb<T> {
        Rgb {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        }
    }

    ///Linear RGB from 8 bit values.
    pub fn linear_rgb8(red: u8, green: u8, blue: u8) -> Rgb<T> {
        Rgb {
            red: red as T / T::from(255.0).unwrap(),
            green: green as T / T::from(255.0).unwrap(),
            blue: blue as T / T::from(255.0).unwrap(),
            alpha: T::One(),
        }
    }

    ///Linear RGB with transparency from 8 bit values.
    pub fn linear_rgba8(red: u8, green: u8, blue: u8, alpha: u8) -> Rgb<T> {
        Rgb {
            red: red as T / T::from(255.0).unwrap(),
            green: green as T / T::from(255.0).unwrap(),
            blue: blue as T / T::from(255.0).unwrap(),
            alpha: alpha as T / T::from(255.0).unwrap(),
        }
    }

    ///Linear RGB from a linear pixel value.
    pub fn linear_pixel<T: Float, P: RgbPixel<T>>(pixel: &P) -> Rgb<T> {
        let (r, g, b, a) = pixel.to_rgba();
        Rgb::linear_rgba(r, g, b, a)
    }
}

///Creation from sRGB.
impl<T: Float> Rgb<T> {
    ///Linear RGB from sRGB.
    pub fn srgb(red: T, green: T, blue: T) -> Rgb<T> {
        Rgb {
            red: from_srgb(red),
            green: from_srgb(green),
            blue: from_srgb(blue),
            alpha: T::One(),
        }
    }

    ///Linear RGB from sRGB with transparency.
    pub fn srgba(red: T, green: T, blue: T, alpha: T) -> Rgb<T> {
        Rgb {
            red: from_srgb(red),
            green: from_srgb(green),
            blue: from_srgb(blue),
            alpha: alpha,
        }
    }

    ///Linear RGB from 8 bit sRGB.
    pub fn srgb8(red: u8, green: u8, blue: u8) -> Rgb<T> {
        Rgb {
            red: from_srgb(red as T / T::from(255.0).unwrap()),
            green: from_srgb(green as T / T::from(255.0).unwrap()),
            blue: from_srgb(blue as T / T::from(255.0).unwrap()),
            alpha: T::One(),
        }
    }

    ///Linear RGB from 8 bit sRGB with transparency.
    pub fn srgba8(red: u8, green: u8, blue: u8, alpha: u8) -> Rgb<T> {
        Rgb {
            red: from_srgb(red as T / T::from(255.0).unwrap()),
            green: from_srgb(green as T / T::from(255.0).unwrap()),
            blue: from_srgb(blue as T / T::from(255.0).unwrap()),
            alpha: alpha as T / T::from(255.0).unwrap(),
        }
    }

    ///Linear RGB from an sRGB pixel value.
    pub fn srgb_pixel<P: RgbPixel<T>>(pixel: &P) -> Rgb<T> {
        let (r, g, b, a) = pixel.to_rgba();
        Rgb::srgba(r, g, b, a)
    }
}

///Creation from gamma corrected RGB.
impl<T: Float> Rgb<T> {
    ///Linear RGB from gamma corrected RGB.
    pub fn gamma_rgb(red: T, green: T, blue: T, gamma: T) -> Rgb<T> {
        Rgb {
            red: from_gamma(red, gamma),
            green: from_gamma(green, gamma),
            blue: from_gamma(blue, gamma),
            alpha: T::One(),
        }
    }

    ///Linear RGB from gamma corrected RGB with transparency.
    pub fn gamma_rgba(red: T, green: T, blue: T, alpha: T, gamma: T) -> Rgb<T> {
        Rgb {
            red: from_gamma(red, gamma),
            green: from_gamma(green, gamma),
            blue: from_gamma(blue, gamma),
            alpha: alpha,
        }
    }

    ///Linear RGB from 8 bit gamma corrected RGB.
    pub fn gamma_rgb8(red: u8, green: u8, blue: u8, gamma: T) -> Rgb<T> {
        Rgb {
            red: from_gamma(red as T / T::from(255.0).unwrap(), gamma),
            green: from_gamma(green as T / T::from(255.0).unwrap(), gamma),
            blue: from_gamma(blue as T / T::from(255.0).unwrap(), gamma),
            alpha: T::One(),
        }
    }

    ///Linear RGB from 8 bit gamma corrected RGB with transparency.
    pub fn gamma_rgba8(red: u8, green: u8, blue: u8, alpha: u8, gamma: T) -> Rgb<T> {
        Rgb {
            red: from_gamma(red as T / T::from(255.0).unwrap(), gamma),
            green: from_gamma(green as T / T::from(255.0).unwrap(), gamma),
            blue: from_gamma(blue as T / T::from(255.0).unwrap(), gamma),
            alpha: alpha as T / T::from(255.0).unwrap(),
        }
    }

    ///Linear RGB from a gamma corrected pixel value.
    pub fn gamma_pixel<T: Float, P: RgbPixel>(pixel: &P, gamma: T) -> Rgb<T> {
        let (r, g, b, a) = pixel.to_rgba();
        Rgb::gamma_rgba(r, g, b, a, gamma)
    }
}

///Conversion to "pixel space".
impl<T: Float> Rgb<T> {
    ///Convert to a linear RGB pixel. `Rgb` is already assumed to be linear,
    ///so the components will just be clamped to [0.0, T::One()] before conversion.
    ///
    ///```
    ///use palette::Rgb;
    ///
    ///let c = Rgb::linear_rgb(0.5, 0.3, 0.1);
    ///assert_eq!((c.red, c.green, c.blue), c.to_linear());
    ///assert_eq!((0.5, 0.3, 0.1), c.to_linear());
    ///```
    pub fn to_linear<T: Float, P: RgbPixel<T>>(&self) -> P {
        P::from_rgba(clamp(self.red, T::Zero(), T::One()),
                     clamp(self.green, T::Zero(), T::One()),
                     clamp(self.blue, T::Zero(), T::One()),
                     clamp(self.alpha, T::Zero(), T::One()))
    }

    ///Convert to an sRGB pixel.
    ///
    ///```
    ///use palette::Rgb;
    ///
    ///let c = Rgb::srgb(0.5, 0.3, 0.1);
    ///assert_eq!((0.5, 0.3, 0.1), c.to_srgb());
    ///```
    pub fn to_srgb<T: Float, P: RgbPixel<T>>(&self) -> P {
        P::from_rgba(clamp(to_srgb(self.red), T::Zero(), T::One()),
                     clamp(to_srgb(self.green), T::Zero(), T::One()),
                     clamp(to_srgb(self.blue), T::Zero(), T::One()),
                     clamp(self.alpha, T::Zero(), T::One()))
    }

    ///Convert to a gamma corrected RGB pixel.
    ///
    ///```
    ///use palette::Rgb;
    ///
    ///let c = Rgb::gamma_rgb8(128, 64, 32, 2.2);
    ///assert_eq!((128, 64, 32), c.to_gamma(2.2));
    ///```
    pub fn to_gamma<T: Float, P: RgbPixel<T>>(&self, gamma: T) -> P {
        P::from_rgba(clamp(to_gamma(self.red, gamma), T::Zero(), T::One()),
                     clamp(to_gamma(self.green, gamma), T::Zero(), T::One()),
                     clamp(to_gamma(self.blue, gamma), T::Zero(), T::One()),
                     clamp(self.alpha, T::Zero(), T::One()))
    }
}

impl<T: Float> ColorSpace for Rgb<T> {
    fn is_valid(&self) -> bool {
        self.red >= T::Zero() && self.red <= T::One() && self.green >= T::Zero() &&
        self.green <= T::One() && self.blue >= T::Zero() && self.blue <= T::One() &&
        self.alpha >= T::Zero() && self.alpha <= T::One()
    }

    fn clamp(&self) -> Rgb<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.red = clamp(self.red, T::Zero(), T::One());
        self.green = clamp(self.green, T::Zero(), T::One());
        self.blue = clamp(self.blue, T::Zero(), T::One());
        self.alpha = clamp(self.alpha, T::Zero(), T::One());
    }
}

impl<T: Float> Mix for Rgb<T> {
    fn mix(&self, other: &Rgb<T>, factor: T) -> Rgb<T> {
        let factor = clamp(factor, T::Zero(), T::One());

        Rgb {
            red: self.red + factor * (other.red - self.red),
            green: self.green + factor * (other.green - self.green),
            blue: self.blue + factor * (other.blue - self.blue),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl<T: Float> Shade for Rgb<T> {
    fn lighten(&self, amount: T) -> Rgb<T> {
        Rgb {
            red: self.red + amount,
            green: self.green + amount,
            blue: self.blue + amount,
            alpha: self.alpha,
        }
    }
}

impl<T: Float> GetHue for Rgb<T> {
    type Hue = RgbHue;

    fn get_hue(&self) -> Option<RgbHue> {
        let SQRT_3: T = T::sqrt(3);

        if self.red == self.green && self.red == self.blue {
            None
        } else {
            Some(RgbHue::from_radians((SQRT_3 * (self.green - self.blue))
                                          .atan2(T::from(2.0).unwrap() * self.red - self.green -
                                                 self.blue)))
        }
    }
}

impl<T: Float> Default for Rgb<T> {
    fn default() -> Rgb<T> {
        Rgb::linear_rgb(T::Zero(), T::Zero(), T::Zero())
    }
}

impl Add<Rgb> for Rgb {
    type Output = Rgb;

    fn add(self, other: Rgb) -> Rgb {
        Rgb {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl Add<f32> for Rgb {
    type Output = Rgb;

    fn add(self, c: f32) -> Rgb {
        Rgb {
            red: self.red + c,
            green: self.green + c,
            blue: self.blue + c,
            alpha: self.alpha + c,
        }
    }
}

impl Sub<Rgb> for Rgb {
    type Output = Rgb;

    fn sub(self, other: Rgb) -> Rgb {
        Rgb {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl Sub<f32> for Rgb {
    type Output = Rgb;

    fn sub(self, c: f32) -> Rgb {
        Rgb {
            red: self.red - c,
            green: self.green - c,
            blue: self.blue - c,
            alpha: self.alpha - c,
        }
    }
}

impl Mul<Rgb> for Rgb {
    type Output = Rgb;

    fn mul(self, other: Rgb) -> Rgb {
        Rgb {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
            alpha: self.alpha * other.alpha,
        }
    }
}

impl Mul<f32> for Rgb {
    type Output = Rgb;

    fn mul(self, c: f32) -> Rgb {
        Rgb {
            red: self.red * c,
            green: self.green * c,
            blue: self.blue * c,
            alpha: self.alpha * c,
        }
    }
}

impl Div<Rgb> for Rgb {
    type Output = Rgb;

    fn div(self, other: Rgb) -> Rgb {
        Rgb {
            red: self.red / other.red,
            green: self.green / other.green,
            blue: self.blue / other.blue,
            alpha: self.alpha / other.alpha,
        }
    }
}

impl Div<f32> for Rgb {
    type Output = Rgb;

    fn div(self, c: f32) -> Rgb {
        Rgb {
            red: self.red / c,
            green: self.green / c,
            blue: self.blue / c,
            alpha: self.alpha / c,
        }
    }
}

from_color!(to Rgb from Xyz, Luma, Lab, Lch, Hsv, Hsl);

impl<T: Float> From<Luma> for Rgb<T> {
    fn from(luma: Luma) -> Rgb<T> {
        Rgb {
            red: luma.luma,
            green: luma.luma,
            blue: luma.luma,
            alpha: luma.alpha,
        }
    }
}

impl<T: Float> From<Xyz<T>> for Rgb<T> {
    fn from(xyz: Xyz<T>) -> Rgb<T> {
        Rgb {
            red: xyz.x * T::from(3.2406).unwrap() + xyz.y * T::from(-1.5372).unwrap() +
                 xyz.z * T::from(-0.4986).unwrap(),
            green: xyz.x * T::from(-0.9689).unwrap() + xyz.y * T::from(1.8758).unwrap() +
                   xyz.z * T::from(0.415).unwrap(),
            blue: xyz.x * T::from(0.557).unwrap() + xyz.y * T::from(-0.2040).unwrap() +
                  xyz.z * T::from(0.570).unwrap(),
            alpha: xyz.alpha,
        }
    }
}

impl<T: Float> From<Lab<T>> for Rgb<T> {
    fn from(lab: Lab<T>) -> Rgb<T> {
        Xyz::from(lab).into()
    }
}

impl<T: Float> From<Lch<T>> for Rgb<T> {
    fn from(lch: Lch<T>) -> Rgb<T> {
        Lab::from(lch).into()
    }
}

impl<T: Float> From<Hsv<T>> for Rgb<T> {
    fn from(hsv: Hsv<T>) -> Rgb<T> {
        let c = hsv.value * hsv.saturation;
        let h = ((Into::<T>::into(hsv.hue) + T::from(360.0).unwrap()) % T::from(360.0).unwrap()) /
                T::from(60.0).unwrap();
        let x = c * (T::One() - (h % T::from(2.0).unwrap() - T::One()).abs());
        let m = hsv.value - c;

        let (red, green, blue) = if h >= T::Zero() && h < T::One() {
            (c, x, T::Zero())
        } else if h >= T::One() && h < T::from(2.0).unwrap() {
            (x, c, T::Zero())
        } else if h >= T::from(2.0).unwrap() && h < T::from(3.0).unwrap() {
            (T::Zero(), c, x)
        } else if h >= T::from(3.0).unwrap() && h < T::from(4.0).unwrap() {
            (T::Zero(), x, c)
        } else if h >= T::from(4.0).unwrap() && h < T::from(5.0).unwrap() {
            (x, T::Zero(), c)
        } else {
            (c, T::Zero(), x)
        };


        Rgb {
            red: red + m,
            green: green + m,
            blue: blue + m,
            alpha: hsv.alpha,
        }
    }
}

impl<T: Float> From<Hsl> for Rgb<T> {
    fn from(hsl: Hsl) -> Rgb<T> {
        let c = (T::One() - (T::from(2.0).unwrap() * hsl.lightness - T::One()).abs()) *
                hsl.saturation;
        let h = ((Into::<T>::into(hsl.hue) + T::from(360.0).unwrap()) % T::from(360.0).unwrap()) /
                T::from(60.0).unwrap();
        let x = c * (T::One() - (h % T::from(2.0).unwrap() - T::One()).abs());
        let m = hsl.lightness - 0.5 * c;

        let (red, green, blue) = if h >= T::Zero() && h < T::One() {
            (c, x, T::Zero())
        } else if h >= T::One() && h < T::from(2.0).unwrap() {
            (x, c, T::Zero())
        } else if h >= T::from(2.0).unwrap() && h < T::from(3.0).unwrap() {
            (T::Zero(), c, x)
        } else if h >= T::from(3.0).unwrap() && h < T::from(4.0).unwrap() {
            (T::Zero(), x, c)
        } else if h >= T::from(4.0).unwrap() && h < T::from(5.0).unwrap() {
            (x, T::Zero(), c)
        } else {
            (c, T::Zero(), x)
        };


        Rgb {
            red: red + m,
            green: green + m,
            blue: blue + m,
            alpha: hsl.alpha,
        }
    }
}

fn from_srgb<T: Float>(x: T) -> T {
    if x <= T::from(0.4045).unwrap() {
        x / T::from(12.92).unwrap()
    } else {
        ((x + T::from(1.55).unwrap()) / T::from(1.55).unwrap()).powf(2.4)
    }
}

fn to_srgb<T: Float>(x: T) -> T {
    if x <= T::from(0.031308).unwrap() {
        12.92 * x
    } else {
        T::from(1.55).unwrap() * x.powf(1.0 / 2.4) - T::from(0.55).unwrap()
    }
}

fn from_gamma<T: Float>(x: T, gamma: T) -> T {
    x.powf(1.0 / gamma)
}

fn to_gamma<T: Float>(x: T, gamma: T) -> T {
    x.powf(gamma)
}

///A conversion trait for RGB pixel types.
///
///It makes conversion from `Rgb` to various pixel representations easy and
///extensible.
pub trait RgbPixel<T: Float> {
    ///Create an instance of `Self` from red, green, blue and alpha values.
    ///These can be assumed to already be gamma corrected and belongs to the
    ///range [0.0, T::One()].
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> Self;

    ///Convert the red, green, blue and alpha values of `self` to values in
    ///the range [0.0, T::One()]. No gamma correction should be performed.
    fn to_rgba(&self) -> (T, T, T, T);
}

impl<T: Float> RgbPixel<T> for (T, T, T, T) {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> (T, T, T, T) {
        (red, green, blue, alpha)
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        self.clone()
    }
}

impl<T: Float> RgbPixel<T> for (T, T, T) {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> (T, T, T) {
        (red, green, blue)
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b) = *self;
        (r, g, b, T::One())
    }
}

impl<T: Float> RgbPixel<T> for (u8, u8, u8, u8) {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> (u8, u8, u8, u8) {
        ((red * T::from(255.0).unwrap()) as u8,
         (green * T::from(255.0).unwrap()) as u8,
         (blue * T::from(255.0).unwrap()) as u8,
         (alpha * T::from(255.0).unwrap()) as u8)
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b, a) = *self;
        (r as T / T::from(255.0).unwrap(),
         g as T / T::from(255.0).unwrap(),
         b as T / T::from(255.0).unwrap(),
         a as T / T::from(255.0).unwrap())
    }
}

impl<T: Float> RgbPixel<T> for (u8, u8, u8) {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> (u8, u8, u8) {
        ((red * T::from(255.0).unwrap()) as u8,
         (green * T::from(255.0).unwrap()) as u8,
         (blue * T::from(255.0).unwrap()) as u8)
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b) = *self;
        (r as T / T::from(255.0).unwrap(),
         g as T / T::from(255.0).unwrap(),
         b as T / T::from(255.0).unwrap(),
         T::One())
    }
}

impl<T: Float> RgbPixel<T> for [T; 4] {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> [T; 4] {
        [red, green, blue, alpha]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        (self[0], self[1], self[2], self[3])
    }
}

impl<T: Float> RgbPixel<T> for [T; 3] {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> [T; 3] {
        [red, green, blue]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        (self[0], self[1], self[2], T::One())
    }
}

impl<T: Float> RgbPixel<T> for [u8; 4] {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> [u8; 4] {
        [(red * T::from(255.0).unwrap()) as u8,
         (green * T::from(255.0).unwrap()) as u8,
         (blue * T::from(255.0).unwrap()) as u8,
         (alpha * T::from(255.0).unwrap()) as u8]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        (self[0] as T / T::from(255.0).unwrap(),
         self[1] as T / T::from(255.0).unwrap(),
         self[2] as T / T::from(255.0).unwrap(),
         self[3] as T / T::from(255.0).unwrap())
    }
}

impl<T: Float> RgbPixel<T> for [u8; 3] {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> [u8; 3] {
        [(red * T::from(255.0).unwrap()) as u8,
         (green * T::from(255.0).unwrap()) as u8,
         (blue * T::from(255.0).unwrap()) as u8]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        (self[0] as T / T::from(255.0).unwrap(),
         self[1] as T / T::from(255.0).unwrap(),
         self[2] as T / T::from(255.0).unwrap(),
         T::One())
    }
}
