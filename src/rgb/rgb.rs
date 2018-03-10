use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};
use std::any::TypeId;

use num_traits::Float;
use approx::ApproxEq;

use rgb::{RgbSpace, RgbStandard, TransferFn};
use encoding::{Linear, Srgb};
use encoding::linear::LinearFn;
use encoding::pixel::RawPixel;
use alpha::Alpha;
use convert::{FromColor, IntoColor};
use white_point::WhitePoint;
use blend::PreAlpha;
use matrix::{matrix_inverse, multiply_xyz_to_rgb, rgb_to_xyz_matrix};
use {Hsl, Hsv, Hwb, Lab, Lch, Luma, RgbHue, Xyz, Yxy};
use {Blend, Component, ComponentWise, GetHue, Limited, Mix, Pixel, Shade};
use {cast, clamp};

/// Generic RGB with an alpha component. See the [`Rgba` implementation in
/// `Alpha`](../struct.Alpha.html#Rgba).
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
#[repr(C)]
pub struct Rgb<S: RgbStandard = Srgb, T: Component = f32> {
    /// The amount of red light, where 0.0 is no red light and 1.0f (or 255u8) is the
    /// highest displayable amount.
    pub red: T,

    /// The amount of green light, where 0.0 is no green light and 1.0f (or 255u8) is the
    /// highest displayable amount.
    pub green: T,

    /// The amount of blue light, where 0.0 is no blue light and 1.0f (or 255u8) is the
    /// highest displayable amount.
    pub blue: T,

    /// The kind of RGB standard. sRGB is the default.
    pub standard: PhantomData<S>,
}

impl<S: RgbStandard, T: Component> Copy for Rgb<S, T> {}

impl<S: RgbStandard, T: Component> Clone for Rgb<S, T> {
    fn clone(&self) -> Rgb<S, T> {
        *self
    }
}

impl<S: RgbStandard, T: Component> Rgb<S, T> {
    /// Create an RGB color.
    pub fn new(red: T, green: T, blue: T) -> Rgb<S, T> {
        Rgb {
            red: red,
            green: green,
            blue: blue,
            standard: PhantomData,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U: Component>(self) -> Rgb<S, U> {
        Rgb {
            red: self.red.convert(),
            green: self.green.convert(),
            blue: self.blue.convert(),
            standard: PhantomData,
        }
    }

    /// Convert from another component type.
    pub fn from_format<U: Component>(color: Rgb<S, U>) -> Self {
        color.into_format()
    }
}

unsafe impl<S: RgbStandard, T: Component> Pixel<T> for Rgb<S, T> {
    const CHANNELS: usize = 3;
}

impl<S: RgbStandard, T: Component + Float> Rgb<S, T> {
    /// Convert the color to linear RGB.
    pub fn into_linear(self) -> Rgb<Linear<S::Space>, T> {
        Rgb::new(
            S::TransferFn::into_linear(self.red),
            S::TransferFn::into_linear(self.green),
            S::TransferFn::into_linear(self.blue),
        )
    }

    /// Convert linear RGB to nonlinear RGB.
    pub fn from_linear(color: Rgb<Linear<S::Space>, T>) -> Rgb<S, T> {
        Rgb::new(
            S::TransferFn::from_linear(color.red),
            S::TransferFn::from_linear(color.green),
            S::TransferFn::from_linear(color.blue),
        )
    }
}

impl<S: RgbStandard<TransferFn = LinearFn>, T: Component> Rgb<S, T> {
    #[inline]
    fn reinterpret_as<St: RgbStandard<TransferFn = LinearFn>>(self) -> Rgb<St, T>
    where
        S::Space: RgbSpace<WhitePoint = <St::Space as RgbSpace>::WhitePoint>,
    {
        Rgb {
            red: self.red,
            green: self.green,
            blue: self.blue,
            standard: PhantomData,
        }
    }
}

/// <span id="Rgba"></span>[`Rgba`](rgb/type.Rgba.html) implementations.
impl<S: RgbStandard, T: Component> Alpha<Rgb<S, T>, T> {
    /// Nonlinear RGB.
    pub fn new(red: T, green: T, blue: T, alpha: T) -> Rgba<S, T> {
        Alpha {
            color: Rgb::new(red, green, blue),
            alpha: alpha,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U: Component>(self) -> Alpha<Rgb<S, U>, U> {
        Rgba::new(
            self.red.convert(),
            self.green.convert(),
            self.blue.convert(),
            self.alpha.convert(),
        )
    }

    /// Convert from another component type.
    pub fn from_format<U: Component>(color: Alpha<Rgb<S, U>, U>) -> Self {
        color.into_format()
    }
}

/// <span id="Rgba"></span>[`Rgba`](rgb/type.Rgba.html) implementations.
impl<S: RgbStandard, T: Component + Float> Alpha<Rgb<S, T>, T> {
    /// Convert the color to linear RGB with transparency.
    pub fn into_linear(self) -> Rgba<Linear<S::Space>, T> {
        Rgba::new(
            S::TransferFn::into_linear(self.red),
            S::TransferFn::into_linear(self.green),
            S::TransferFn::into_linear(self.blue),
            self.alpha,
        )
    }

    /// Convert linear RGB to nonlinear RGB with transparency.
    pub fn from_linear(color: Rgba<Linear<S::Space>, T>) -> Rgba<S, T> {
        Rgba::new(
            S::TransferFn::from_linear(color.red),
            S::TransferFn::from_linear(color.green),
            S::TransferFn::from_linear(color.blue),
            color.alpha,
        )
    }
}

impl<S, T> Limited for Rgb<S, T>
where
    S: RgbStandard,
    T: Component,
{
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn is_valid(&self) -> bool {
        self.red >= T::zero() && self.red <= T::max_intensity() &&
        self.green >= T::zero() && self.green <= T::max_intensity() &&
        self.blue >= T::zero() && self.blue <= T::max_intensity()
    }

    fn clamp(&self) -> Rgb<S, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.red = clamp(self.red, T::zero(), T::max_intensity());
        self.green = clamp(self.green, T::zero(), T::max_intensity());
        self.blue = clamp(self.blue, T::zero(), T::max_intensity());
    }
}

impl<S, T> Mix for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + Float,
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

impl<S, T> Shade for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + Float,
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

impl<S, T> GetHue for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + Float,
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        let sqrt_3: T = cast(1.73205081);

        if self.red == self.green && self.red == self.blue {
            None
        } else {
            Some(RgbHue::from_radians(
                (sqrt_3 * (self.green - self.blue))
                    .atan2(self.red * cast(2.0) - self.green - self.blue),
            ))
        }
    }
}

impl<S, T> Blend for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + Float,
{
    type Color = Rgb<S, T>;

    fn into_premultiplied(self) -> PreAlpha<Rgb<S, T>, T> {
        Rgba::from(self).into()
    }

    fn from_premultiplied(color: PreAlpha<Rgb<S, T>, T>) -> Self {
        Rgba::from(color).into()
    }
}

impl<S, T> ComponentWise for Rgb<S, T>
where
    S: RgbStandard,
    T: Component,
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
where
    T: Component,
    S: RgbStandard,
{
    fn default() -> Rgb<S, T> {
        Rgb::new(T::zero(), T::zero(), T::zero())
    }
}

impl<S, T> Add<Rgb<S, T>> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + Add,
    <T as Add>::Output: Component,
{
    type Output = Rgb<S, <T as Add>::Output>;

    fn add(self, other: Rgb<S, T>) -> Self::Output {
        Rgb {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
            standard: PhantomData,
        }
    }
}

impl<S, T> Add<T> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + Add,
    <T as Add>::Output: Component,
{
    type Output = Rgb<S, <T as Add>::Output>;

    fn add(self, c: T) -> Self::Output {
        Rgb {
            red: self.red + c,
            green: self.green + c,
            blue: self.blue + c,
            standard: PhantomData,
        }
    }
}

impl<S, T> Sub<Rgb<S, T>> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + Sub,
    <T as Sub>::Output: Component,
{
    type Output = Rgb<S, <T as Sub>::Output>;

    fn sub(self, other: Rgb<S, T>) -> Self::Output {
        Rgb {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
            standard: PhantomData,
        }
    }
}

impl<S, T> Sub<T> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + Sub,
    <T as Sub>::Output: Component,
{
    type Output = Rgb<S, <T as Sub>::Output>;

    fn sub(self, c: T) -> Self::Output {
        Rgb {
            red: self.red - c,
            green: self.green - c,
            blue: self.blue - c,
            standard: PhantomData,
        }
    }
}

impl<S, T> Mul<Rgb<S, T>> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + Mul,
    <T as Mul>::Output: Component,
{
    type Output = Rgb<S, <T as Mul>::Output>;

    fn mul(self, other: Rgb<S, T>) -> Self::Output {
        Rgb {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
            standard: PhantomData,
        }
    }
}

impl<S, T> Mul<T> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + Mul,
    <T as Mul>::Output: Component,
{
    type Output = Rgb<S, <T as Mul>::Output>;

    fn mul(self, c: T) -> Self::Output {
        Rgb {
            red: self.red * c,
            green: self.green * c,
            blue: self.blue * c,
            standard: PhantomData,
        }
    }
}

impl<S, T> Div<Rgb<S, T>> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + Div,
    <T as Div>::Output: Component,
{
    type Output = Rgb<S, <T as Div>::Output>;

    fn div(self, other: Rgb<S, T>) -> Self::Output {
        Rgb {
            red: self.red / other.red,
            green: self.green / other.green,
            blue: self.blue / other.blue,
            standard: PhantomData,
        }
    }
}

impl<S, T> Div<T> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + Div,
    <T as Div>::Output: Component,
{
    type Output = Rgb<S, <T as Div>::Output>;

    fn div(self, c: T) -> Self::Output {
        Rgb {
            red: self.red / c,
            green: self.green / c,
            blue: self.blue / c,
            standard: PhantomData,
        }
    }
}

impl<S, Wp, T> FromColor<Wp, T> for Rgb<S, T>
where
    S: RgbStandard,
    T: Component + Float,
    Wp: WhitePoint,
    S::Space: RgbSpace<WhitePoint = Wp>,
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        let transform_matrix = matrix_inverse(&rgb_to_xyz_matrix::<S::Space, T>());
        Self::from_linear(multiply_xyz_to_rgb(&transform_matrix, &xyz))
    }

    fn from_rgb<Sp: RgbSpace<WhitePoint = Wp>>(rgb: Rgb<Linear<Sp>, T>) -> Self {
        if TypeId::of::<Sp::Primaries>() == TypeId::of::<<S::Space as RgbSpace>::Primaries>() {
            Self::from_linear(rgb.reinterpret_as())
        } else {
            Self::from_xyz(Xyz::from_rgb(rgb))
        }
    }

    fn from_hsl<Sp: RgbSpace<WhitePoint = Wp>>(hsl: Hsl<Sp, T>) -> Self {
        let hsl = Hsl::<S::Space, T>::from_hsl(hsl);

        let c = (T::one() - (hsl.lightness * cast(2.0) - T::one()).abs()) * hsl.saturation;
        let h = hsl.hue.to_positive_degrees() / cast(60.0);
        let x = c * (T::one() - (h % cast(2.0) - T::one()).abs());
        let m = hsl.lightness - c * cast(0.5);

        let (red, green, blue) = if h >= T::zero() && h < T::one() {
            (c, x, T::zero())
        } else if h >= T::one() && h < cast(2.0) {
            (x, c, T::zero())
        } else if h >= cast(2.0) && h < cast(3.0) {
            (T::zero(), c, x)
        } else if h >= cast(3.0) && h < cast(4.0) {
            (T::zero(), x, c)
        } else if h >= cast(4.0) && h < cast(5.0) {
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

    fn from_hsv<Sp: RgbSpace<WhitePoint = Wp>>(hsv: Hsv<Sp, T>) -> Self {
        let hsv = Hsv::<S::Space, T>::from_hsv(hsv);

        let c = hsv.value * hsv.saturation;
        let h = hsv.hue.to_positive_degrees() / cast(60.0);
        let x = c * (T::one() - (h % cast(2.0) - T::one()).abs());
        let m = hsv.value - c;

        let (red, green, blue) = if h >= T::zero() && h < T::one() {
            (c, x, T::zero())
        } else if h >= T::one() && h < cast(2.0) {
            (x, c, T::zero())
        } else if h >= cast(2.0) && h < cast(3.0) {
            (T::zero(), c, x)
        } else if h >= cast(3.0) && h < cast(4.0) {
            (T::zero(), x, c)
        } else if h >= cast(4.0) && h < cast(5.0) {
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

    fn from_luma(luma: Luma<Linear<Wp>, T>) -> Self {
        Self::from_linear(Rgb {
            red: luma.luma,
            green: luma.luma,
            blue: luma.luma,
            standard: PhantomData,
        })
    }
}

impl<S, T, Wp> IntoColor<Wp, T> for Rgb<S, T>
where
    S: RgbStandard,
    T: Component + Float,
    Wp: WhitePoint,
    S::Space: RgbSpace<WhitePoint = Wp>,
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
    fn into_rgb<Sp: RgbSpace<WhitePoint = Wp>>(self) -> Rgb<Linear<Sp>, T> {
        Rgb::from_rgb(self.into_linear())
    }

    #[inline(always)]
    fn into_hsl<Sp: RgbSpace<WhitePoint = Wp>>(self) -> Hsl<Sp, T> {
        Hsl::from_rgb(self.into_linear())
    }

    #[inline(always)]
    fn into_hsv<Sp: RgbSpace<WhitePoint = Wp>>(self) -> Hsv<Sp, T> {
        Hsv::from_rgb(self.into_linear())
    }

    #[inline(always)]
    fn into_hwb<Sp: RgbSpace<WhitePoint = Wp>>(self) -> Hwb<Sp, T> {
        Hwb::from_rgb(self.into_linear())
    }

    #[inline(always)]
    fn into_luma(self) -> Luma<Linear<Wp>, T> {
        Luma::from_rgb(self.into_linear())
    }
}

impl<S, T> ApproxEq for Rgb<S, T>
where
    T: Component + ApproxEq,
    T::Epsilon: Copy,
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
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.red.relative_eq(&other.red, epsilon, max_relative) &&
        self.green.relative_eq(&other.green, epsilon, max_relative) &&
        self.blue.relative_eq(&other.blue, epsilon, max_relative)
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.red.ulps_eq(&other.red, epsilon, max_ulps) &&
        self.green.ulps_eq(&other.green, epsilon, max_ulps) &&
        self.blue.ulps_eq(&other.blue, epsilon, max_ulps)
    }
}

impl<S, T, P> AsRef<P> for Rgb<S, T>
where
    T: Component,
    S: RgbStandard,
    P: RawPixel<T> + ?Sized,
{
    /// Convert to a raw pixel format.
    ///
    /// ```rust
    /// use palette::Srgb;
    ///
    /// let mut rgb = Srgb::new(38, 42, 19);
    /// let raw: &[u8] = rgb.as_ref();
    ///
    /// assert_eq!(raw[1], 42);
    /// ```
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<S, T, P> AsMut<P> for Rgb<S, T>
where
    T: Component,
    S: RgbStandard,
    P: RawPixel<T> + ?Sized,
{
    /// Convert to a raw pixel format.
    ///
    /// ```rust
    /// use palette::Srgb;
    ///
    /// let mut rgb = Srgb::new(38, 42, 19);
    /// {
    ///     let raw: &mut [u8] = rgb.as_mut();
    ///     raw[1] = 5;
    /// }
    ///
    /// assert_eq!(rgb.green, 5);
    /// ```
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<S, T> From<Alpha<Rgb<S, T>, T>> for Rgb<S, T>
where
    T: Component,
    S: RgbStandard,
{
    fn from(color: Alpha<Rgb<S, T>, T>) -> Rgb<S, T> {
        color.color
    }
}

#[cfg(test)]
mod test {
    use super::Rgb;
    use encoding::Srgb;

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

    raw_pixel_conversion_tests!(Rgb<Srgb>: red, green, blue);
    raw_pixel_conversion_fail_tests!(Rgb<Srgb>: red, green, blue);
}
