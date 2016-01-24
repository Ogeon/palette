use num::Float;

use {Color, Rgb, clamp};

use pixel::RgbPixel;

///A normalized sRGB encoded color.
///
///```
///use palette::Rgb;
///use palette::pixel::Srgb;
///
///let c: Rgb = Srgb::new(0.5, 0.3, 0.1).into();
///assert_eq!((0.5f32, 0.3, 0.1), Srgb::from(c).to_pixel());
///```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Srgb<T: Float = f32> {
    ///The red component, where 0.0 is no red light and 1.0 is the
    ///highest displayable amount.
    pub red: T,

    ///The green component, where 0.0 is no red light and 1.0 is the
    ///highest displayable amount.
    pub green: T,

    ///The blue component, where 0.0 is no red light and 1.0 is the
    ///highest displayable amount.
    pub blue: T,

    ///The transparency of the color. 0.0 is completely transparent and 1.0 is
    ///completely opaque.
    pub alpha: T,
}

impl<T: Float> Srgb<T> {
    ///Create a new opaque sRGB encoded color.
    pub fn new(red: T, green: T, blue: T) -> Srgb<T> {
        Srgb::with_alpha(red, green, blue, T::one())
    }

    ///Create a new sRGB encoded color with transparency.
    pub fn with_alpha(red: T, green: T, blue: T, alpha: T) -> Srgb<T> {
        Srgb {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        }
    }

    ///Create a new opaque sRGB encoded color from `u8` values.
    pub fn new_u8(red: u8, green: u8, blue: u8) -> Srgb<T> {
        Srgb::with_alpha_u8(red, green, blue, 255)
    }

    ///Create a new sRGB encoded color, with transparency, from `u8` values.
    pub fn with_alpha_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Srgb<T> {
        Srgb {
            red: T::from(red).unwrap() / T::from(255.0).unwrap(),
            green: T::from(green).unwrap() / T::from(255.0).unwrap(),
            blue: T::from(blue).unwrap() / T::from(255.0).unwrap(),
            alpha: T::from(alpha).unwrap() / T::from(255.0).unwrap(),
        }
    }

    ///Create a new sRGB encoded color from a pixel value.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P) -> Srgb<T> {
        let (r, g, b, a) = pixel.to_rgba();
        Srgb::with_alpha(r, g, b, a)
    }

    ///Transform this color into a pixel representation.
    pub fn to_pixel<P: RgbPixel<T>>(&self) -> P {
        P::from_rgba(
            clamp(self.red, T::zero(), T::one()),
            clamp(self.green, T::zero(), T::one()),
            clamp(self.blue, T::zero(), T::one()),
            clamp(self.alpha, T::zero(), T::one()),
        )
    }

    ///Convert linear color components to sRGB encoding.
    pub fn from_linear<C: Into<Rgb<T>>>(color: C) -> Srgb<T> {
        let rgb = color.into();
        Srgb {
            red: to_srgb(rgb.red),
            green: to_srgb(rgb.green),
            blue: to_srgb(rgb.blue),
            alpha: rgb.alpha,
        }
    }

    ///Decode this color to a linear representation.
    pub fn to_linear(&self) -> Rgb<T> {
        Rgb {
            red: from_srgb(self.red),
            green: from_srgb(self.green),
            blue: from_srgb(self.blue),
            alpha: self.alpha,
        }
    }

    ///Shortcut to convert a linear color to an sRGB encoded pixel.
    pub fn linear_to_pixel<C: Into<Rgb<T>>, P: RgbPixel<T>>(color: C) -> P {
        Srgb::from_linear(color).to_pixel()
    }
}

impl<T: Float> From<Rgb<T>> for Srgb<T> {
    fn from(rgb: Rgb<T>) -> Srgb<T> {
        Srgb::from_linear(rgb)
    }
}

impl<T: Float> From<Color<T>> for Srgb<T> {
    fn from(color: Color<T>) -> Srgb<T> {
        Rgb::from(color).into()
    }
}

fn from_srgb<T: Float>(x: T) -> T {
    if x <= T::from(0.04045).unwrap() {
        x / T::from(12.92).unwrap()
    } else {
        ((x + T::from(0.055).unwrap()) / T::from(1.055).unwrap()).powf(T::from(2.4).unwrap())
    }
}

fn to_srgb<T: Float>(x: T) -> T {
    if x <= T::from(0.0031308).unwrap() {
        T::from(12.92).unwrap() * x
    } else {
        T::from(1.055).unwrap() * x.powf(T::from(1.0 / 2.4).unwrap()) - T::from(0.055).unwrap()
    }
}
