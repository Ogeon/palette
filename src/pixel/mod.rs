//!Pixel encodings and pixel format conversion.

use num_traits::Float;

use {clamp, flt};

pub use self::gamma_rgb::GammaRgb;

mod gamma_rgb;

///A transfer function to and from linear space.
pub trait TransferFn {
    ///Convert the color component `x` from linear space.
    fn from_linear<T: Float>(x: T) -> T;

    ///Convert the color component `x` into linear space.
    fn into_linear<T: Float>(x: T) -> T;
}

///A conversion trait for RGB pixel formats.
///
///It provided methods for encoding and decoding RGB colors as pixel storage
///formats, and is intended as a bridge between Palette and image processing
///libraries.
pub trait RgbPixel<T: Float = f32> {
    ///Create an instance of `Self` from red, green, blue and alpha values.
    ///These can be assumed to already be gamma corrected and belongs to the
    ///range [0.0, 1.0].
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> Self;

    ///Convert the red, green, blue and alpha values of `self` to values in
    ///the range [0.0, 1.0]. No gamma correction should be performed.
    fn to_rgba(&self) -> (T, T, T, T);
}

impl<T: Float> RgbPixel<T> for (f32, f32, f32, f32) {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> (f32, f32, f32, f32) {
        ( red.to_f32().unwrap(), green.to_f32().unwrap(), blue.to_f32().unwrap(), alpha.to_f32().unwrap() )
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b, a) = *self;
        ( flt(r), flt(g), flt(b), flt(a) )
    }
}

impl<T: Float> RgbPixel<T> for (f32, f32, f32) {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> (f32, f32, f32) {
        ( red.to_f32().unwrap(), green.to_f32().unwrap(), blue.to_f32().unwrap() )
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b) = *self;
        ( flt(r), flt(g), flt(b), T::one() )
    }
}
impl<T: Float> RgbPixel<T> for (f64, f64, f64, f64) {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> (f64, f64, f64, f64) {
        ( red.to_f64().unwrap(), green.to_f64().unwrap(), blue.to_f64().unwrap(), alpha.to_f64().unwrap() )
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b, a) = *self;
        ( flt(r), flt(g), flt(b), flt(a) )
    }
}

impl<T: Float> RgbPixel<T> for (f64, f64, f64) {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> (f64, f64, f64) {
        (red.to_f64().unwrap(), green.to_f64().unwrap(), blue.to_f64().unwrap())
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b) = *self;
        ( flt(r), flt(g), flt(b), T::one() )
    }
}

impl<T: Float> RgbPixel<T> for (u8, u8, u8, u8) {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> (u8, u8, u8, u8) {
        let c255 = flt(255.0);
        (
            clamp(red * c255, T::zero(), c255).to_u8().unwrap(),
            clamp(green * c255, T::zero(), c255).to_u8().unwrap(),
            clamp(blue * c255, T::zero(), c255).to_u8().unwrap(),
            clamp(alpha * c255, T::zero(), c255).to_u8().unwrap(),
        )
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b, a) = *self;
        let c255: T = flt(255.0);
        (
            flt::<T,_>(r) / c255,
            flt::<T,_>(g) / c255,
            flt::<T,_>(b) / c255,
            flt::<T,_>(a) / c255,
        )
    }
}

impl<T: Float> RgbPixel<T> for (u8, u8, u8) {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> (u8, u8, u8) {
        let c255: T = flt(255.0);
        (
            clamp(red * c255, T::zero(), c255).to_u8().unwrap(),
            clamp(green * c255, T::zero(), c255).to_u8().unwrap(),
            clamp(blue * c255, T::zero(), c255).to_u8().unwrap(),
        )
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let (r, g, b) = *self;
        let c255: T = flt(255.0);
        (
            flt::<T,_>(r) / c255,
            flt::<T,_>(g) / c255,
            flt::<T,_>(b) / c255,
            T::one(),
        )
    }
}

impl<T: Float> RgbPixel<T> for [f32; 4] {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> [f32; 4] {
        [ red.to_f32().unwrap(), green.to_f32().unwrap(), blue.to_f32().unwrap(), alpha.to_f32().unwrap() ]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        ( flt(self[0]), flt(self[1]), flt(self[2]), flt(self[3]) )
    }
}

impl<T: Float> RgbPixel<T> for [f32; 3] {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> [f32; 3] {
        [red.to_f32().unwrap(), green.to_f32().unwrap(), blue.to_f32().unwrap()]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        (flt(self[0]), flt(self[1]), flt(self[2]), T::one())
    }
}
impl<T: Float> RgbPixel<T> for [f64; 4] {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> [f64; 4] {
        [red.to_f64().unwrap(), green.to_f64().unwrap(), blue.to_f64().unwrap(), alpha.to_f64().unwrap()]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        (flt(self[0]), flt(self[1]), flt(self[2]), flt(self[3]))
    }
}

impl<T: Float> RgbPixel<T> for [f64; 3] {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> [f64; 3] {
        [red.to_f64().unwrap(), green.to_f64().unwrap(), blue.to_f64().unwrap()]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        (flt(self[0]), flt(self[1]), flt(self[2]), T::one())
    }
}

impl<T: Float> RgbPixel<T> for [u8; 4] {
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> [u8; 4] {
        let c255 = flt(255.0);
        [
            clamp(red * c255, T::zero(), c255).to_u8().unwrap(),
            clamp(green * c255, T::zero(), c255).to_u8().unwrap(),
            clamp(blue * c255, T::zero(), c255).to_u8().unwrap(),
            clamp(alpha * c255, T::zero(), c255).to_u8().unwrap(),
        ]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let c255: T = flt(255.0);
        (
            flt::<T,_>(self[0]) / c255,
            flt::<T,_>(self[1]) / c255,
            flt::<T,_>(self[2]) / c255,
            flt::<T,_>(self[3]) / c255,
        )
    }
}

impl<T: Float> RgbPixel<T> for [u8; 3] {
    fn from_rgba(red: T, green: T, blue: T, _alpha: T) -> [u8; 3] {
        let c255 = flt(255.0);
        [
            clamp(red * c255, T::zero(), c255).to_u8().unwrap(),
            clamp(green * c255, T::zero(), c255).to_u8().unwrap(),
            clamp(blue * c255, T::zero(), c255).to_u8().unwrap(),
        ]
    }

    fn to_rgba(&self) -> (T, T, T, T) {
        let c255: T = flt(255.0);
        (
            flt::<T,_>(self[0]) / c255,
            flt::<T,_>(self[1]) / c255,
            flt::<T,_>(self[2]) / c255,
            T::one(),
        )
    }
}
