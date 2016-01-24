use num::Float;

use {Rgb, clamp};

use pixel::RgbPixel;

///A normalized gamma encoded color.
///
///```
///use palette::Rgb;
///use palette::pixel::GammaRgb;
///
///let c: Rgb = GammaRgb::new_u8(128, 64, 32, 2.2).into();
///assert_eq!((128, 64, 32), GammaRgb::from_linear(c, 2.2).to_pixel());
///```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GammaRgb<T: Float = f32> {
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

    ///The decoding gamma value. Commonly 2.2.
    pub gamma: T,
}

impl<T: Float> GammaRgb<T> {
    ///Create a new opaque gamma encoded color.
    pub fn new(red: T, green: T, blue: T, gamma: T) -> GammaRgb<T> {
        GammaRgb::with_alpha(red, green, blue, T::one(), gamma)
    }

    ///Create a new gamma encoded color with transparency.
    pub fn with_alpha(red: T, green: T, blue: T, alpha: T, gamma: T) -> GammaRgb<T> {
        GammaRgb {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
            gamma: gamma,
        }
    }

    ///Create a new opaque gamma encoded color from `u8` values.
    pub fn new_u8(red: u8, green: u8, blue: u8, gamma: T) -> GammaRgb<T> {
        GammaRgb::with_alpha_u8(red, green, blue, 255, gamma)
    }

    ///Create a new gamma encoded color, with transparency, from `u8` values.
    pub fn with_alpha_u8(red: u8, green: u8, blue: u8, alpha: u8, gamma: T) -> GammaRgb<T> {
        GammaRgb {
            red: T::from(red).unwrap() / T::from(255.0).unwrap(),
            green: T::from(green).unwrap() / T::from(255.0).unwrap(),
            blue: T::from(blue).unwrap() / T::from(255.0).unwrap(),
            alpha: T::from(alpha).unwrap() / T::from(255.0).unwrap(),
            gamma: gamma,
        }
    }

    ///Create a new gamma encoded color from a pixel value.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P, gamma: T) -> GammaRgb<T> {
        let (r, g, b, a) = pixel.to_rgba();
        GammaRgb::with_alpha(r, g, b, a, gamma)
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

    ///Convert linear color components to gamma encoding.
    pub fn from_linear<C: Into<Rgb<T>>>(color: C, gamma: T) -> GammaRgb<T> {
        let rgb = color.into();
        GammaRgb {
            red: to_gamma(rgb.red, gamma),
            green: to_gamma(rgb.green, gamma),
            blue: to_gamma(rgb.blue, gamma),
            alpha: rgb.alpha,
            gamma: gamma,
        }
    }

    ///Decode this color to a linear representation.
    pub fn to_linear(&self) -> (T, T, T, T) {
        (
            from_gamma(self.red, self.gamma),
            from_gamma(self.green, self.gamma),
            from_gamma(self.blue, self.gamma),
            self.alpha
        )
    }
}

fn from_gamma<T: Float>(x: T, gamma: T) -> T {
    x.powf(T::one() / gamma)
}

fn to_gamma<T: Float>(x: T, gamma: T) -> T {
    x.powf(gamma)
}
