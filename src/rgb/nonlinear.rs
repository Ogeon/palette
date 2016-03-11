use std::marker::PhantomData;

use num_traits::Float;
use approx::ApproxEq;

use rgb::{LinRgb, LinRgba, RgbSpace, RgbStandard};
use rgb::standards::Srgb;
use alpha::Alpha;
use pixel::{TransferFn, RgbPixel};
use convert::{FromColor, IntoColor};
use white_point::WhitePoint;
use {Xyz, Yxy, Luma, Hsl, Hsv, Hwb, Lab, Lch};
use {flt, clamp};

///Nonlinear RGB with an alpha component. See the [`Rgba` implementation in `Alpha`](../struct.Alpha.html#Rgba).
pub type Rgba<S = Srgb, T = f32> = Alpha<Rgb<S, T>, T>;

///Nonlinear RGB.
#[derive(Debug, PartialEq)]
pub struct Rgb<S: RgbStandard = Srgb, T: Float = f32> {
    ///The amount of red light, where 0.0 is no red light and 1.0 is the
    ///highest displayable amount.
    pub red: T,

    ///The amount of green light, where 0.0 is no green light and 1.0 is the
    ///highest displayable amount.
    pub green: T,

    ///The amount of blue light, where 0.0 is no blue light and 1.0 is the
    ///highest displayable amount.
    pub blue: T,

    ///The kind of RGB standard. sRGB is the default.
    pub standard: PhantomData<S>,
}

impl<S: RgbStandard, T: Float> Copy for Rgb<S, T> {}

impl<S: RgbStandard, T: Float> Clone for Rgb<S, T> {
    fn clone(&self) -> Rgb<S, T> { *self }
}

impl<S: RgbStandard, T: Float> Rgb<S, T> {
    ///Nonlinear RGB.
    pub fn new(red: T, green: T, blue: T) -> Rgb<S, T> {
        Rgb {
            red: red,
            green: green,
            blue: blue,
            standard: PhantomData,
        }
    }

    ///Nonlinear RGB with transparency from 8 bit values.
    pub fn new_u8(red: u8, green: u8, blue: u8) -> Rgb<S, T> {
        Rgb {
            red: flt::<T, _>(red) / flt(255.0),
            green: flt::<T, _>(green) / flt(255.0),
            blue: flt::<T, _>(blue) / flt(255.0),
            standard: PhantomData,
        }
    }

    ///Create an RGB color from a pixel.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P) -> Rgb<S, T> {
        let (red, green, blue, _) = pixel.to_rgba();
        Rgb::new(red, green, blue)
    }

    ///Convert the color into a pixel representation.
    pub fn into_pixel<P: RgbPixel<T>>(self) -> P {
        P::from_rgba(
            clamp(self.red, T::zero(), T::one()),
            clamp(self.green, T::zero(), T::one()),
            clamp(self.blue, T::zero(), T::one()),
            T::one(),
        )
    }

    ///Convert the color to linear RGB.
    pub fn into_linear(self) -> LinRgb<S::Space, T> {
        LinRgb::with_wp(
            S::TransferFn::into_linear(self.red),
            S::TransferFn::into_linear(self.green),
            S::TransferFn::into_linear(self.blue),
        )
    }

    ///Convert linear RGB to nonlinear RGB.
    pub fn from_linear(color: LinRgb<S::Space, T>) -> Rgb<S, T> {
        Rgb::new(
            S::TransferFn::from_linear(color.red),
            S::TransferFn::from_linear(color.green),
            S::TransferFn::from_linear(color.blue),
        )
    }

    ///Convert a linear color to an RGB pixel.
    pub fn linear_to_pixel<C: Into<LinRgb<S::Space, T>>, P: RgbPixel<T>>(color: C) -> P {
        Rgb::<S, T>::from_linear(color.into()).into_pixel()
    }

    ///Convert an RGB pixel to a linear color.
    pub fn pixel_to_linear<C: From<LinRgb<S::Space, T>>, P: RgbPixel<T>>(pixel: &P) -> C {
        Rgb::<S, T>::from_pixel(pixel).into_linear().into()
    }
}

///<span id="Rgba"></span>[`Rgba`](rgb/type.Rgba.html) implementations.
impl<S: RgbStandard, T: Float> Alpha<Rgb<S, T>, T> {
    ///Nonlinear RGB.
    pub fn new(red: T, green: T, blue: T, alpha: T) -> Rgba<S, T> {
        Alpha {
            color: Rgb::new(red, green, blue),
            alpha: alpha,
        }
    }

    ///Nonlinear RGB with transparency from 8 bit values.
    pub fn new_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Rgba<S, T> {
        Alpha {
            color: Rgb::new_u8(red, green, blue),
            alpha: flt::<T, _>(alpha) / flt(255.0),
        }
    }

    ///Create an RGB color with transparency from a pixel.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P) -> Rgba<S, T> {
        let (red, green, blue, alpha) = pixel.to_rgba();
        Rgba::new(red, green, blue, alpha)
    }

    ///Convert the color into a pixel representation.
    pub fn into_pixel<P: RgbPixel<T>>(self) -> P {
        P::from_rgba(
            clamp(self.red, T::zero(), T::one()),
            clamp(self.green, T::zero(), T::one()),
            clamp(self.blue, T::zero(), T::one()),
            clamp(self.alpha, T::zero(), T::one()),
        )
    }

    ///Convert the color to linear RGB with transparency.
    pub fn into_linear(self) -> LinRgba<S::Space, T> {
        LinRgba::with_wp(
            S::TransferFn::into_linear(self.red),
            S::TransferFn::into_linear(self.green),
            S::TransferFn::into_linear(self.blue),
            self.alpha,
        )
    }

    ///Convert linear RGB to nonlinear RGB with transparency.
    pub fn from_linear(color: LinRgba<S::Space, T>) -> Rgba<S, T> {
        Rgba::new(
            S::TransferFn::from_linear(color.red),
            S::TransferFn::from_linear(color.green),
            S::TransferFn::from_linear(color.blue),
            color.alpha,
        )
    }

    ///Convert a linear color to an RGB pixel.
    pub fn linear_to_pixel<C: Into<LinRgba<S::Space, T>>, P: RgbPixel<T>>(color: C) -> P {
        Rgba::<S, T>::from_linear(color.into()).into_pixel()
    }

    ///Convert an RGB pixel to a linear color.
    pub fn pixel_to_linear<C: From<LinRgba<S::Space, T>>, P: RgbPixel<T>>(pixel: &P) -> C {
        Rgba::<S, T>::from_pixel(pixel).into_linear().into()
    }
}

impl<S, T, Wp> FromColor<Wp, T> for Rgb<S, T> where
    S: RgbStandard,
    T: Float,
    Wp: WhitePoint,
    S::Space: RgbSpace<WhitePoint=Wp>,
{
    fn from_xyz(inp: Xyz<Wp, T>) -> Self {
        Self::from_linear(LinRgb::from_xyz(inp))
    }

    fn from_yxy(inp: Yxy<Wp, T>) -> Self {
        Self::from_linear(LinRgb::from_yxy(inp))
    }

    fn from_lab(inp: Lab<Wp, T>) -> Self {
        Self::from_linear(LinRgb::from_lab(inp))
    }

    fn from_lch(inp: Lch<Wp, T>) -> Self {
        Self::from_linear(LinRgb::from_lch(inp))
    }

    fn from_rgb<Sp: RgbSpace<WhitePoint=Wp>>(inp: LinRgb<Sp, T>) -> Self {
        Self::from_linear(LinRgb::from_rgb(inp))
    }

    fn from_hsl(inp: Hsl<Wp, T>) -> Self {
        Self::from_linear(LinRgb::from_hsl(inp))
    }

    fn from_hsv(inp: Hsv<Wp, T>) -> Self {
        Self::from_linear(LinRgb::from_hsv(inp))
    }

    fn from_hwb(inp: Hwb<Wp, T>) -> Self {
        Self::from_linear(LinRgb::from_hwb(inp))
    }

    fn from_luma(inp: Luma<Wp, T>) -> Self {
        Self::from_linear(LinRgb::from_luma(inp))
    }
}

impl<S, T, Wp> IntoColor<Wp, T> for Rgb<S, T> where
    S: RgbStandard,
    T: Float,
    Wp: WhitePoint,
    S::Space: RgbSpace<WhitePoint=Wp>,
{
    fn into_xyz(self) -> Xyz<Wp, T> {
        self.into_linear().into_xyz()
    }

    fn into_yxy(self) -> Yxy<Wp, T> {
        self.into_linear().into_yxy()
    }

    fn into_lab(self) -> Lab<Wp, T> {
        self.into_linear().into_lab()
    }

    fn into_lch(self) -> Lch<Wp, T> {
        self.into_linear().into_lch()
    }

    fn into_rgb<Sp: RgbSpace<WhitePoint=Wp>>(self) -> LinRgb<Sp, T> {
        self.into_linear().into_rgb()
    }

    fn into_hsl(self) -> Hsl<Wp, T> {
        self.into_linear().into_hsl()
    }

    fn into_hsv(self) -> Hsv<Wp, T> {
        self.into_linear().into_hsv()
    }

    fn into_hwb(self) -> Hwb<Wp, T> {
        self.into_linear().into_hwb()
    }

    fn into_luma(self) -> Luma<Wp, T> {
        self.into_linear().into_luma()
    }
}

impl<S, T> ApproxEq for Rgb<S, T>
    where T: Float + ApproxEq,
        T::Epsilon: Copy + Float,
        S: RgbStandard,
{
    type Epsilon = <T as ApproxEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }
    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        self.red.relative_eq(&other.red, epsilon, max_relative) &&
        self.green.relative_eq(&other.green, epsilon, max_relative) &&
        self.blue.relative_eq(&other.blue, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool{
        self.red.ulps_eq(&other.red, epsilon, max_ulps) &&
        self.green.ulps_eq(&other.green, epsilon, max_ulps) &&
        self.blue.ulps_eq(&other.blue, epsilon, max_ulps)
    }
}
