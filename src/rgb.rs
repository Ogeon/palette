use {Color, Luma, Xyz, Lab, Lch, Hsv, Hsl, ColorSpace, Mix, Shade, GetHue, RgbHue, clamp};

///Linear RGB with an alpha component.
///
///Conversions and operations on this color space assumes that it's linear,
///meaning that gamma correction is required when converting to and from
///a displayable RGB, such as sRGB.
#[derive(Clone, Debug, PartialEq)]
pub struct Rgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

///Creation from linear RGB.
impl Rgb {
    ///Linear RGB.
    pub fn rgb(red: f32, green: f32, blue: f32) -> Rgb {
        Rgb {
            red: red,
            green: green,
            blue: blue,
            alpha: 1.0,
        }
    }

    ///Linear RGB with transparency.
    pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Rgb {
        Rgb {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        }
    }

    ///Linear RGB from 8 bit values.
    pub fn rgb8(red: u8, green: u8, blue: u8) -> Rgb {
        Rgb {
            red: red as f32 / 255.0,
            green: green as f32 / 255.0,
            blue: blue as f32 / 255.0,
            alpha: 1.0,
        }
    }

    ///Linear RGB with transparency from 8 bit values.
    pub fn rgba8(red: u8, green: u8, blue: u8, alpha: u8) -> Rgb {
        Rgb {
            red: red as f32 / 255.0,
            green: green as f32 / 255.0,
            blue: blue as f32 / 255.0,
            alpha: alpha as f32 / 255.0,
        }
    }
}

///Creation from sRGB.
impl Rgb {
    ///Linear RGB from sRGB.
    pub fn srgb(red: f32, green: f32, blue: f32) -> Rgb {
        Rgb {
            red: from_srgb(red),
            green: from_srgb(green),
            blue: from_srgb(blue),
            alpha: 1.0,
        }
    }

    ///Linear RGB from sRGB with transparency.
    pub fn srgba(red: f32, green: f32, blue: f32, alpha: f32) -> Rgb {
        Rgb {
            red: from_srgb(red),
            green: from_srgb(green),
            blue: from_srgb(blue),
            alpha: alpha,
        }
    }

    ///Linear RGB from 8 bit sRGB.
    pub fn srgb8(red: u8, green: u8, blue: u8) -> Rgb {
        Rgb {
            red: from_srgb(red as f32 / 255.0),
            green: from_srgb(green as f32 / 255.0),
            blue: from_srgb(blue as f32 / 255.0),
            alpha: 1.0,
        }
    }

    ///Linear RGB from 8 bit sRGB with transparency.
    pub fn srgba8(red: u8, green: u8, blue: u8, alpha: u8) -> Rgb {
        Rgb {
            red: from_srgb(red as f32 / 255.0),
            green: from_srgb(green as f32 / 255.0),
            blue: from_srgb(blue as f32 / 255.0),
            alpha: alpha as f32 / 255.0,
        }
    }
}

///Creation from gamma corrected RGB.
impl Rgb {
    ///Linear RGB from gamma corrected RGB.
    pub fn rgb_gamma(red: f32, green: f32, blue: f32, gamma: f32) -> Rgb {
        Rgb {
            red: from_gamma(red, gamma),
            green: from_gamma(green, gamma),
            blue: from_gamma(blue, gamma),
            alpha: 1.0,
        }
    }

    ///Linear RGB from gamma corrected RGB with transparency.
    pub fn rgba_gamma(red: f32, green: f32, blue: f32, alpha: f32, gamma: f32) -> Rgb {
        Rgb {
            red: from_gamma(red, gamma),
            green: from_gamma(green, gamma),
            blue: from_gamma(blue, gamma),
            alpha: alpha,
        }
    }

    ///Linear RGB from 8 bit gamma corrected RGB.
    pub fn rgb8_gamma(red: u8, green: u8, blue: u8, gamma: f32) -> Rgb {
        Rgb {
            red: from_gamma(red as f32 / 255.0, gamma),
            green: from_gamma(green as f32 / 255.0, gamma),
            blue: from_gamma(blue as f32 / 255.0, gamma),
            alpha: 1.0,
        }
    }

    ///Linear RGB from 8 bit gamma corrected RGB with transparency.
    pub fn rgba8_gamma(red: u8, green: u8, blue: u8, alpha: u8, gamma: f32) -> Rgb {
        Rgb {
            red: from_gamma(red as f32 / 255.0, gamma),
            green: from_gamma(green as f32 / 255.0, gamma),
            blue: from_gamma(blue as f32 / 255.0, gamma),
            alpha: alpha as f32 / 255.0,
        }
    }
}

///Conversion to "pixel space".
impl Rgb {
    ///Convert to a linear RGB pixel. `Rgb` is already assumed to be linear,
    ///so the components will just be clamped to [0.0, 1.0] before conversion.
    ///
    ///```
    ///use palette::Rgb;
    ///
    ///let c = Rgb::rgb(0.5, 0.3, 0.1);
    ///assert_eq!((c.red, c.green, c.blue), c.to_linear());
    ///assert_eq!((0.5, 0.3, 0.1), c.to_linear());
    ///```
    pub fn to_linear<P: RgbPixel>(&self) -> P {
        P::from_rgba(
            clamp(self.red, 0.0, 1.0),
            clamp(self.green, 0.0, 1.0),
            clamp(self.blue, 0.0, 1.0),
            clamp(self.alpha, 0.0, 1.0),
        )
    }

    ///Convert to an sRGB pixel.
    ///
    ///```
    ///use palette::Rgb;
    ///
    ///let c = Rgb::srgb(0.5, 0.3, 0.1);
    ///assert_eq!((0.5, 0.3, 0.1), c.to_srgb());
    ///```
    pub fn to_srgb<P: RgbPixel>(&self) -> P {
        P::from_rgba(
            clamp(to_srgb(self.red), 0.0, 1.0),
            clamp(to_srgb(self.green), 0.0, 1.0),
            clamp(to_srgb(self.blue), 0.0, 1.0),
            clamp(self.alpha, 0.0, 1.0),
        )
    }

    ///Convert to a gamma corrected RGB pixel.
    ///
    ///```
    ///use palette::Rgb;
    ///
    ///let c = Rgb::rgb8_gamma(128, 64, 32, 2.2);
    ///assert_eq!((128, 64, 32), c.to_gamma(2.2));
    ///```
    pub fn to_gamma<P: RgbPixel>(&self, gamma: f32) -> P {
        P::from_rgba(
            clamp(to_gamma(self.red, gamma), 0.0, 1.0),
            clamp(to_gamma(self.green, gamma), 0.0, 1.0),
            clamp(to_gamma(self.blue, gamma), 0.0, 1.0),
            clamp(self.alpha, 0.0, 1.0),
        )
    }
}

impl ColorSpace for Rgb {
    fn is_valid(&self) -> bool {
        self.red >= 0.0 && self.red <= 1.0 &&
        self.green >= 0.0 && self.green <= 1.0 &&
        self.blue >= 0.0 && self.blue <= 1.0 &&
        self.alpha >= 0.0 && self.alpha <= 1.0
    }

    fn clamp(&self) -> Rgb {
        let mut c = self.clone();
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.red = clamp(self.red, 0.0, 1.0);
        self.green = clamp(self.green, 0.0, 1.0);
        self.blue = clamp(self.blue, 0.0, 1.0);
        self.alpha = clamp(self.alpha, 0.0, 1.0);
    }
}

impl Mix for Rgb {
    fn mix(&self, other: &Rgb, factor: f32) -> Rgb {
        let factor = clamp(factor, 0.0, 1.0);

        Rgb {
            red: self.red + factor * (other.red - self.red),
            green: self.green + factor * (other.green - self.green),
            blue: self.blue + factor * (other.blue - self.blue),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl Shade for Rgb {
    fn lighten(&self, amount: f32) -> Rgb {
        Rgb {
            red: self.red + amount,
            green: self.green + amount,
            blue: self.blue + amount,
            alpha: self.alpha,
        }
    }
}

impl GetHue for Rgb {
    type Hue = RgbHue;

    fn get_hue(&self) -> Option<RgbHue> {
        const SQRT_3: f32 = 1.73205081;

        if self.red == self.green && self.red == self.blue {
            None
        } else {
            Some(RgbHue::from_radians((SQRT_3 * (self.green - self.blue)).atan2(2.0 * self.red - self.green - self.blue)))
        }
    }
}

impl Default for Rgb {
    fn default() -> Rgb {
        Rgb::rgb(0.0, 0.0, 0.0)
    }
}

from_color!(to Rgb from Xyz, Luma, Lab, Lch, Hsv, Hsl);

impl From<Luma> for Rgb {
    fn from(luma: Luma) -> Rgb {
        Rgb {
            red: luma.luma,
            green: luma.luma,
            blue: luma.luma,
            alpha: luma.alpha,
        }
    }
}

impl From<Xyz> for Rgb {
    fn from(xyz: Xyz) -> Rgb {
        Rgb {
            red: xyz.x * 3.2406 + xyz.y * -1.5372 + xyz.z * -0.4986,
            green: xyz.x * -0.9689 + xyz.y * 1.8758 + xyz.z * 0.0415,
            blue: xyz.x * 0.0557 + xyz.y * -0.2040 + xyz.z * 1.0570,
            alpha: xyz.alpha,
        }
    }
}

impl From<Lab> for Rgb {
    fn from(lab: Lab) -> Rgb {
        Xyz::from(lab).into()
    }
}

impl From<Lch> for Rgb {
    fn from(lch: Lch) -> Rgb {
        Lab::from(lch).into()
    }
}

impl From<Hsv> for Rgb {
    fn from(hsv: Hsv) -> Rgb {
        let c = hsv.value * hsv.saturation;
        let h = ((Into::<f32>::into(hsv.hue) + 360.0) % 360.0) / 60.0;
        let x = c * (1.0 - (h % 2.0 - 1.0).abs());
        let m = hsv.value - c;

        let (red, green, blue) = if h >= 0.0 && h < 1.0 {
            (c, x, 0.0)
        } else if h >= 1.0 && h < 2.0 {
            (x, c, 0.0)
        } else if h >= 2.0 && h < 3.0 {
            (0.0, c, x)
        } else if h >= 3.0 && h < 4.0 {
            (0.0, x, c)
        } else if h >= 4.0 && h < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };


        Rgb {
            red: red + m,
            green: green + m,
            blue: blue + m,
            alpha: hsv.alpha,
        }
    }
}

impl From<Hsl> for Rgb {
    fn from(hsl: Hsl) -> Rgb {
        let c = (1.0 - (2.0 * hsl.lightness - 1.0).abs()) * hsl.saturation;
        let h = ((Into::<f32>::into(hsl.hue) + 360.0) % 360.0) / 60.0;
        let x = c * (1.0 - (h % 2.0 - 1.0).abs());
        let m = hsl.lightness - 0.5 * c;

        let (red, green, blue) = if h >= 0.0 && h < 1.0 {
            (c, x, 0.0)
        } else if h >= 1.0 && h < 2.0 {
            (x, c, 0.0)
        } else if h >= 2.0 && h < 3.0 {
            (0.0, c, x)
        } else if h >= 3.0 && h < 4.0 {
            (0.0, x, c)
        } else if h >= 4.0 && h < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };


        Rgb {
            red: red + m,
            green: green + m,
            blue: blue + m,
            alpha: hsl.alpha,
        }
    }
}

fn from_srgb(x: f32) -> f32 {
    if x <= 0.04045 {
        x / 12.92
    } else {
        ((x + 0.055) / 1.055).powf(2.4)
    }
}

fn to_srgb(x: f32) -> f32 {
    if x <= 0.0031308 {
        12.92 * x
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    }
}

fn from_gamma(x: f32, gamma: f32) -> f32 {
    x.powf(1.0 / gamma)
}

fn to_gamma(x: f32, gamma: f32) -> f32 {
    x.powf(gamma)
}

///A conversion trait for RGB pixel types.
///
///It makes conversion from `Rgb` to various pixel representations easy and
///extensible.
pub trait RgbPixel {
    ///Create an instance of self from red, green, blue and alpha values.
    ///These can be assumed to already be gamma corrected and belongs to the
    ///range [0.0, 1.0].
    fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self;
}

impl RgbPixel for (f32, f32, f32, f32) {
    fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> (f32, f32, f32, f32) {
        (red, green, blue, alpha)
    }
}

impl RgbPixel for (f32, f32, f32) {
    fn from_rgba(red: f32, green: f32, blue: f32, _alpha: f32) -> (f32, f32, f32) {
        (red, green, blue)
    }
}

impl RgbPixel for (u8, u8, u8, u8) {
    fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> (u8, u8, u8, u8) {
        ((red * 255.0) as u8, (green * 255.0) as u8, (blue * 255.0) as u8, (alpha * 255.0) as u8)
    }
}

impl RgbPixel for (u8, u8, u8) {
    fn from_rgba(red: f32, green: f32, blue: f32, _alpha: f32) -> (u8, u8, u8) {
        ((red * 255.0) as u8, (green * 255.0) as u8, (blue * 255.0) as u8)
    }
}

impl RgbPixel for [f32; 4] {
    fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> [f32; 4] {
        [red, green, blue, alpha]
    }
}

impl RgbPixel for [f32; 3] {
    fn from_rgba(red: f32, green: f32, blue: f32, _alpha: f32) -> [f32; 3] {
        [red, green, blue]
    }
}

impl RgbPixel for [u8; 4] {
    fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> [u8; 4] {
        [(red * 255.0) as u8, (green * 255.0) as u8, (blue * 255.0) as u8, (alpha * 255.0) as u8]
    }
}

impl RgbPixel for [u8; 3] {
    fn from_rgba(red: f32, green: f32, blue: f32, _alpha: f32) -> [u8; 3] {
        [(red * 255.0) as u8, (green * 255.0) as u8, (blue * 255.0) as u8]
    }
}
