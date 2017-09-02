use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};
use std::any::TypeId;

use num_traits::Float;
use approx::ApproxEq;

use rgb::{RgbSpace, RgbStandard};
use rgb::standards::{Srgb, Linear, Lin};
use alpha::Alpha;
use pixel::{TransferFn, RgbPixel, GammaRgb};
use convert::{FromColor, IntoColor};
use white_point::WhitePoint;
use blend::PreAlpha;
use matrix::{matrix_inverse, multiply_xyz_to_rgb, rgb_to_xyz_matrix};
use {Xyz, Yxy, Luma, Hsl, Hsv, Hwb, Lab, Lch, RgbHue};
use {Limited, Mix, Shade, GetHue, Blend, ComponentWise};
use {flt, clamp};

/// Generic RGB with an alpha component. See the [`Rgba` implementation in `Alpha`](../struct.Alpha.html#Rgba).
pub type Rgba<S = Srgb, T = f32> = Alpha<Rgb<S, T>, T>;

/// Generic RGB.
///
/// RGB is probably the most common color space, when it comes to computer
/// graphics, and it's defined as an additive mixture of red, green and blue
/// light, where gray scale colors are created when these three channels are
/// equal in strength.
///
/// Many conversions and operations on this color space requires that it's linear,
/// meaning that gamma correction is required when converting to and from a
/// displayable RGB, such as sRGB. See the [`pixel`](pixel/index.html) module
/// for encoding formats.
#[derive(Debug, PartialEq)]
pub struct Rgb<S: RgbStandard = Srgb, T: Float = f32> {
    /// The amount of red light, where 0.0 is no red light and 1.0 is the
    /// highest displayable amount.
    pub red: T,

    /// The amount of green light, where 0.0 is no green light and 1.0 is the
    /// highest displayable amount.
    pub green: T,

    /// The amount of blue light, where 0.0 is no blue light and 1.0 is the
    /// highest displayable amount.
    pub blue: T,

    /// The kind of RGB standard. sRGB is the default.
    pub standard: PhantomData<S>,
}

impl<S: RgbStandard, T: Float> Copy for Rgb<S, T> {}

impl<S: RgbStandard, T: Float> Clone for Rgb<S, T> {
    fn clone(&self) -> Rgb<S, T> { *self }
}

impl<S: RgbStandard, T: Float> Rgb<S, T> {
    ///Create an RGB color.
    pub fn new(red: T, green: T, blue: T) -> Rgb<S, T> {
        Rgb {
            red: red,
            green: green,
            blue: blue,
            standard: PhantomData,
        }
    }

    ///Create an RGB color from 8 bit values.
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
    pub fn into_linear(self) -> Rgb<Lin<S::Space>, T> {
        Rgb::new(
            S::TransferFn::into_linear(self.red),
            S::TransferFn::into_linear(self.green),
            S::TransferFn::into_linear(self.blue),
        )
    }

    ///Convert linear RGB to nonlinear RGB.
    pub fn from_linear(color: Rgb<Lin<S::Space>, T>) -> Rgb<S, T> {
        Rgb::new(
            S::TransferFn::from_linear(color.red),
            S::TransferFn::from_linear(color.green),
            S::TransferFn::from_linear(color.blue),
        )
    }

    ///Convert a linear color to an RGB pixel.
    pub fn linear_to_pixel<C: Into<Rgb<Lin<S::Space>, T>>, P: RgbPixel<T>>(color: C) -> P {
        Rgb::<S, T>::from_linear(color.into()).into_pixel()
    }

    ///Convert an RGB pixel to a linear color.
    pub fn pixel_to_linear<C: From<Rgb<Lin<S::Space>, T>>, P: RgbPixel<T>>(pixel: &P) -> C {
        Rgb::<S, T>::from_pixel(pixel).into_linear().into()
    }
}

impl<S: RgbStandard<TransferFn=Linear>, T: Float> Rgb<S, T> {
    #[inline]
    fn reinterpret_as<St: RgbStandard<TransferFn=Linear>>(self) -> Rgb<St, T> where S::Space: RgbSpace<WhitePoint=<St::Space as RgbSpace>::WhitePoint> {
        Rgb {
            red: self.red,
            green: self.green,
            blue: self.blue,
            standard: PhantomData,
        }
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
    pub fn into_linear(self) -> Rgba<Lin<S::Space>, T> {
        Rgba::new(
            S::TransferFn::into_linear(self.red),
            S::TransferFn::into_linear(self.green),
            S::TransferFn::into_linear(self.blue),
            self.alpha,
        )
    }

    ///Convert linear RGB to nonlinear RGB with transparency.
    pub fn from_linear(color: Rgba<Lin<S::Space>, T>) -> Rgba<S, T> {
        Rgba::new(
            S::TransferFn::from_linear(color.red),
            S::TransferFn::from_linear(color.green),
            S::TransferFn::from_linear(color.blue),
            color.alpha,
        )
    }

    ///Convert a linear color to an RGB pixel.
    pub fn linear_to_pixel<C: Into<Rgba<Lin<S::Space>, T>>, P: RgbPixel<T>>(color: C) -> P {
        Rgba::<S, T>::from_linear(color.into()).into_pixel()
    }

    ///Convert an RGB pixel to a linear color.
    pub fn pixel_to_linear<C: From<Rgba<Lin<S::Space>, T>>, P: RgbPixel<T>>(pixel: &P) -> C {
        Rgba::<S, T>::from_pixel(pixel).into_linear().into()
    }
}

impl<S, T> Limited for Rgb<S, T> where
    S: RgbStandard,
    T: Float
{
    fn is_valid(&self) -> bool {
        self.red >= T::zero() && self.red <= T::one() &&
        self.green >= T::zero() && self.green <= T::one() &&
        self.blue >= T::zero() && self.blue <= T::one()
    }

    fn clamp(&self) -> Rgb<S, T> {
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

impl<S, T> Mix for Rgb<S, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
{
    type Scalar = T;

    fn mix(&self, other: &Rgb<S, T>, factor: T) -> Rgb<S, T> {
        let factor = clamp(factor, T::zero(), T::one());

        Rgb {
            red: self.red + factor * (other.red - self.red),
            green: self.green + factor * (other.green - self.green),
            blue: self.blue + factor * (other.blue - self.blue),
            standard: PhantomData,
        }
    }
}

impl<S, T> Shade for Rgb<S, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Rgb<S, T> {
        Rgb {
            red: self.red + amount,
            green: self.green + amount,
            blue: self.blue + amount,
            standard: PhantomData,
        }
    }
}

impl<S, T> GetHue for Rgb<S, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
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

impl<S, T> Blend for Rgb<S, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
{
    type Color = Rgb<S, T>;

    fn into_premultiplied(self) -> PreAlpha<Rgb<S, T>, T> {
        Rgba::from(self).into()
    }

    fn from_premultiplied(color: PreAlpha<Rgb<S, T>, T>) -> Self {
        Rgba::from(color).into()
    }
}

impl<S, T> ComponentWise for Rgb<S, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
{
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Rgb<S, T>, mut f: F) -> Rgb<S, T> {
        Rgb {
            red: f(self.red, other.red),
            green: f(self.green, other.green),
            blue: f(self.blue, other.blue),
            standard: PhantomData,
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Rgb<S, T> {
        Rgb {
            red: f(self.red),
            green: f(self.green),
            blue: f(self.blue),
            standard: PhantomData,
        }
    }
}

impl<S, T> Default for Rgb<S, T>
    where T: Float,
        S: RgbStandard,
{
    fn default() -> Rgb<S, T> {
        Rgb::new(T::zero(), T::zero(), T::zero())
    }
}

impl<S, T> Add<Rgb<S, T>> for Rgb<S, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
{
    type Output = Rgb<S, T>;

    fn add(self, other: Rgb<S, T>) -> Rgb<S, T> {
        Rgb {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
            standard: PhantomData,
        }
    }
}

impl<S, T> Add<T> for Rgb<S, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
{
    type Output = Rgb<S, T>;

    fn add(self, c: T) -> Rgb<S, T> {
        Rgb {
            red: self.red + c,
            green: self.green + c,
            blue: self.blue + c,
            standard: PhantomData,
        }
    }
}

impl<S, T> Sub<Rgb<S, T>> for Rgb<S, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
{
    type Output = Rgb<S, T>;

    fn sub(self, other: Rgb<S, T>) -> Rgb<S, T> {
        Rgb {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
            standard: PhantomData,
        }
    }
}

impl<S, T> Sub<T> for Rgb<S, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
{
    type Output = Rgb<S, T>;

    fn sub(self, c: T) -> Rgb<S, T> {
        Rgb {
            red: self.red - c,
            green: self.green - c,
            blue: self.blue - c,
            standard: PhantomData,
        }
    }
}

impl<S, T> Mul<Rgb<S, T>> for Rgb<S, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
{
    type Output = Rgb<S, T>;

    fn mul(self, other: Rgb<S, T>) -> Rgb<S, T> {
        Rgb {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
            standard: PhantomData,
        }
    }
}

impl<S, T> Mul<T> for Rgb<S, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
{
    type Output = Rgb<S, T>;

    fn mul(self, c: T) -> Rgb<S, T> {
        Rgb {
            red: self.red * c,
            green: self.green * c,
            blue: self.blue * c,
            standard: PhantomData,
        }
    }
}

impl<S, T> Div<Rgb<S, T>> for Rgb<S, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
{
    type Output = Rgb<S, T>;

    fn div(self, other: Rgb<S, T>) -> Rgb<S, T> {
        Rgb {
            red: self.red / other.red,
            green: self.green / other.green,
            blue: self.blue / other.blue,
            standard: PhantomData,
        }
    }
}

impl<S, T> Div<T> for Rgb<S, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
{
    type Output = Rgb<S, T>;

    fn div(self, c: T) -> Rgb<S, T> {
        Rgb {
            red: self.red / c,
            green: self.green / c,
            blue: self.blue / c,
            standard: PhantomData,
        }
    }
}

impl<S, Wp, T> FromColor<Wp, T> for Rgb<S, T> where
    S: RgbStandard,
    T: Float,
    Wp: WhitePoint,
    S::Space: RgbSpace<WhitePoint=Wp>,
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        let transform_matrix = matrix_inverse(&rgb_to_xyz_matrix::<S::Space, T>());
        Self::from_linear(multiply_xyz_to_rgb(&transform_matrix, &xyz))
    }

    fn from_rgb<Sp: RgbSpace<WhitePoint=Wp>>(rgb: Rgb<Lin<Sp>, T>) -> Self {
        if TypeId::of::<Sp::Primaries>() == TypeId::of::<<S::Space as RgbSpace>::Primaries>() {
            Self::from_linear(rgb.reinterpret_as())
        } else {
            Self::from_xyz(Xyz::from_rgb(rgb))
        }
    }

    fn from_hsl<Sp: RgbSpace<WhitePoint=Wp>>(hsl: Hsl<Sp, T>) -> Self {
        let hsl = Hsl::<S::Space, T>::from_hsl(hsl);

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


        Self::from_linear(Rgb {
            red: red + m,
            green: green + m,
            blue: blue + m,
            standard: PhantomData,
        })
    }

    fn from_hsv<Sp: RgbSpace<WhitePoint=Wp>>(hsv: Hsv<Sp, T>) -> Self {
        let hsv = Hsv::<S::Space, T>::from_hsv(hsv);

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


        Self::from_linear(Rgb {
            red: red + m,
            green: green + m,
            blue: blue + m,
            standard: PhantomData,
        })
    }

    fn from_luma(luma: Luma<Wp, T>) -> Self {
        Self::from_linear(Rgb {
            red: luma.luma,
            green: luma.luma,
            blue: luma.luma,
            standard: PhantomData,
        })
    }
}

impl<S, T, Wp> IntoColor<Wp, T> for Rgb<S, T> where
    S: RgbStandard,
    T: Float,
    Wp: WhitePoint,
    S::Space: RgbSpace<WhitePoint=Wp>,
{
    #[inline(always)]
    fn into_xyz(self) -> Xyz<Wp, T> {
        Xyz::from_rgb(self.into_linear())
    }

    #[inline(always)]
    fn into_yxy(self) -> Yxy<Wp, T> {
        Yxy::from_rgb(self.into_linear())
    }

    #[inline(always)]
    fn into_lab(self) -> Lab<Wp, T> {
        Lab::from_rgb(self.into_linear())
    }

    #[inline(always)]
    fn into_lch(self) -> Lch<Wp, T> {
        Lch::from_rgb(self.into_linear())
    }

    #[inline(always)]
    fn into_rgb<Sp: RgbSpace<WhitePoint=Wp>>(self) -> Rgb<Lin<Sp>, T> {
        Rgb::from_rgb(self.into_linear())
    }

    #[inline(always)]
    fn into_hsl<Sp: RgbSpace<WhitePoint=Wp>>(self) -> Hsl<Sp, T> {
        Hsl::from_rgb(self.into_linear())
    }

    #[inline(always)]
    fn into_hsv<Sp: RgbSpace<WhitePoint=Wp>>(self) -> Hsv<Sp, T> {
        Hsv::from_rgb(self.into_linear())
    }

    #[inline(always)]
    fn into_hwb<Sp: RgbSpace<WhitePoint=Wp>>(self) -> Hwb<Sp, T> {
        Hwb::from_rgb(self.into_linear())
    }

    #[inline(always)]
    fn into_luma(self) -> Luma<Wp, T> {
        Luma::from_rgb(self.into_linear())
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

impl<S, T> From<Alpha<Rgb<S, T>, T>> for Rgb<S, T>
    where T: Float,
        S: RgbStandard
{
    fn from(color: Alpha<Rgb<S, T>, T>) -> Rgb<S, T> {
        color.color
    }
}

impl<S, T> From<GammaRgb<<S::Space as RgbSpace>::WhitePoint, T>> for Rgb<S, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
{
    fn from(gamma_rgb: GammaRgb<<S::Space as RgbSpace>::WhitePoint, T>) -> Rgb<S, T> {
        gamma_rgb.to_linear().into()
    }
}

impl<S, T> From<GammaRgb<<S::Space as RgbSpace>::WhitePoint, T>> for Alpha<Rgb<S, T>, T> where
    S: RgbStandard<TransferFn=Linear>,
    T: Float
{
    fn from(gamma_rgb: GammaRgb<<S::Space as RgbSpace>::WhitePoint, T>) -> Alpha<Rgb<S, T>, T> {
        gamma_rgb.to_linear()
    }
}

#[cfg(test)]
mod test {
    use super::Rgb;
    use rgb::standards::Srgb;

    #[test]
    fn ranges() {
        assert_ranges!{
            Rgb<Srgb, f64>;
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
