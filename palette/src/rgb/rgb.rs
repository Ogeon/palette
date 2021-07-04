use core::any::TypeId;
use core::fmt;
use core::marker::PhantomData;
use core::num::ParseIntError;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use core::str::FromStr;

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::alpha::Alpha;
use crate::blend::PreAlpha;
use crate::convert::FromColorUnclamped;
use crate::encoding::linear::LinearFn;
use crate::encoding::pixel::RawPixel;
use crate::encoding::{Linear, Srgb};
use crate::luma::LumaStandard;
use crate::matrix::{matrix_inverse, multiply_xyz_to_rgb, rgb_to_xyz_matrix};
use crate::rgb::{Packed, RgbChannels, RgbSpace, RgbStandard, TransferFn};
use crate::{
    clamp, contrast_ratio, from_f64, Blend, Clamp, Component, ComponentWise, FloatComponent,
    FromComponent, GetHue, Mix, Pixel, RelativeContrast, Shade,
};
use crate::{Hsl, Hsv, Luma, RgbHue, Xyz};

/// Generic RGB with an alpha component. See the [`Rgba` implementation in
/// `Alpha`](crate::Alpha#Rgba).
pub type Rgba<S = Srgb, T = f32> = Alpha<Rgb<S, T>, T>;

/// Generic RGB.
///
/// RGB is probably the most common color space, when it comes to computer
/// graphics, and it's defined as an additive mixture of red, green and blue
/// light, where gray scale colors are created when these three channels are
/// equal in strength.
///
/// Many conversions and operations on this color space requires that it's
/// linear, meaning that gamma correction is required when converting to and
/// from a displayable RGB, such as sRGB. See the [`pixel`](crate::encoding::pixel)
/// module for encoding formats.
#[derive(Debug, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    rgb_standard = "S",
    white_point = "<S::Space as RgbSpace>::WhitePoint",
    component = "T",
    skip_derives(Xyz, Hsv, Hsl, Luma, Rgb)
)]
#[repr(C)]
pub struct Rgb<S: RgbStandard = Srgb, T: Component = f32> {
    /// The amount of red light, where 0.0 is no red light and 1.0f (or 255u8)
    /// is the highest displayable amount.
    pub red: T,

    /// The amount of green light, where 0.0 is no green light and 1.0f (or
    /// 255u8) is the highest displayable amount.
    pub green: T,

    /// The amount of blue light, where 0.0 is no blue light and 1.0f (or
    /// 255u8) is the highest displayable amount.
    pub blue: T,

    /// The kind of RGB standard. sRGB is the default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
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
            red,
            green,
            blue,
            standard: PhantomData,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U>(self) -> Rgb<S, U>
    where
        U: Component + FromComponent<T>,
    {
        Rgb {
            red: U::from_component(self.red),
            green: U::from_component(self.green),
            blue: U::from_component(self.blue),
            standard: PhantomData,
        }
    }

    /// Convert from another component type.
    pub fn from_format<U>(color: Rgb<S, U>) -> Self
    where
        T: FromComponent<U>,
        U: Component,
    {
        color.into_format()
    }

    /// Convert to a `(red, green, blue)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.red, self.green, self.blue)
    }

    /// Convert from a `(red, green, blue)` tuple.
    pub fn from_components((red, green, blue): (T, T, T)) -> Self {
        Self::new(red, green, blue)
    }

    /// Return the `red` value minimum.
    pub fn min_red() -> T {
        T::zero()
    }

    /// Return the `red` value maximum.
    pub fn max_red() -> T {
        T::max_intensity()
    }

    /// Return the `green` value minimum.
    pub fn min_green() -> T {
        T::zero()
    }

    /// Return the `green` value maximum.
    pub fn max_green() -> T {
        T::max_intensity()
    }

    /// Return the `blue` value minimum.
    pub fn min_blue() -> T {
        T::zero()
    }

    /// Return the `blue` value maximum.
    pub fn max_blue() -> T {
        T::max_intensity()
    }
}

impl<S, T> PartialEq for Rgb<S, T>
where
    T: Component + PartialEq,
    S: RgbStandard,
{
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

impl<S, T> Eq for Rgb<S, T>
where
    T: Component + Eq,
    S: RgbStandard,
{
}

/// Convenience functions to convert between a packed `u32` and `Rgb`.
///
/// ```
/// use palette::Srgb;
///
/// let rgb = Srgb::from(0x607F00);
/// assert_eq!(Srgb::new(96u8, 127, 0), rgb);
///
/// let integer = u32::from(rgb);
/// assert_eq!(0xFF607F00, integer);
/// ```
impl<S: RgbStandard> Rgb<S, u8> {
    /// Convert to a packed `u32` with with specifiable component order.
    /// Defaults to ARGB ordering (0xAARRGGBB).
    ///
    /// See [Packed](crate::Packed) for more details.
    pub fn into_u32<C: RgbChannels>(self) -> u32 {
        Packed::<C>::from(self).color
    }

    /// Convert from a packed `u32` with specifiable component order. Defaults
    /// to ARGB ordering (0xAARRGGBB).
    ///
    /// See [Packed](crate::Packed) for more details.
    pub fn from_u32<C: RgbChannels>(color: u32) -> Self {
        Packed::<C>::from(color).into()
    }
}

impl<S: RgbStandard, T: FloatComponent> Rgb<S, T> {
    /// Convert the color to linear RGB.
    pub fn into_linear(self) -> Rgb<Linear<S::Space>, T> {
        Rgb::new(
            S::TransferFn::into_linear(self.red),
            S::TransferFn::into_linear(self.green),
            S::TransferFn::into_linear(self.blue),
        )
    }

    /// Convert linear RGB to non-linear RGB.
    pub fn from_linear(color: Rgb<Linear<S::Space>, T>) -> Rgb<S, T> {
        Rgb::new(
            S::TransferFn::from_linear(color.red),
            S::TransferFn::from_linear(color.green),
            S::TransferFn::from_linear(color.blue),
        )
    }

    /// Convert the color to a different encoding.
    pub fn into_encoding<St: RgbStandard<Space = S::Space>>(self) -> Rgb<St, T> {
        Rgb::new(
            St::TransferFn::from_linear(S::TransferFn::into_linear(self.red)),
            St::TransferFn::from_linear(S::TransferFn::into_linear(self.green)),
            St::TransferFn::from_linear(S::TransferFn::into_linear(self.blue)),
        )
    }

    /// Convert RGB from a different encoding.
    pub fn from_encoding<St: RgbStandard<Space = S::Space>>(color: Rgb<St, T>) -> Rgb<S, T> {
        Rgb::new(
            S::TransferFn::from_linear(St::TransferFn::into_linear(color.red)),
            S::TransferFn::from_linear(St::TransferFn::into_linear(color.green)),
            S::TransferFn::from_linear(St::TransferFn::into_linear(color.blue)),
        )
    }
}

impl<S: RgbStandard, T: Component> Rgb<S, T> {
    #[inline]
    fn reinterpret_as<St: RgbStandard>(self) -> Rgb<St, T>
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

/// <span id="Rgba"></span>[`Rgba`](crate::rgb::Rgba) implementations.
impl<S: RgbStandard, T: Component, A: Component> Alpha<Rgb<S, T>, A> {
    /// Non-linear RGB.
    pub fn new(red: T, green: T, blue: T, alpha: A) -> Self {
        Alpha {
            color: Rgb::new(red, green, blue),
            alpha,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U, B>(self) -> Alpha<Rgb<S, U>, B>
    where
        U: Component + FromComponent<T>,
        B: Component + FromComponent<A>,
    {
        Alpha::<Rgb<S, U>, B>::new(
            U::from_component(self.red),
            U::from_component(self.green),
            U::from_component(self.blue),
            B::from_component(self.alpha),
        )
    }

    /// Convert from another component type.
    pub fn from_format<U, B>(color: Alpha<Rgb<S, U>, B>) -> Self
    where
        T: FromComponent<U>,
        U: Component,
        A: FromComponent<B>,
        B: Component,
    {
        color.into_format()
    }

    /// Convert to a `(red, green, blue, alpha)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (self.red, self.green, self.blue, self.alpha)
    }

    /// Convert from a `(red, green, blue, alpha)` tuple.
    pub fn from_components((red, green, blue, alpha): (T, T, T, A)) -> Self {
        Self::new(red, green, blue, alpha)
    }
}

/// Convenience functions to convert between a packed `u32` and `Rgba`.
///
/// ```
/// use palette::Srgba;
///
/// let rgba = Srgba::from(0x607F00FF);
/// assert_eq!(Srgba::new(96u8, 127, 0, 255), rgba);
///
/// let integer = u32::from(rgba);
/// assert_eq!(0x607F00FF, integer);
/// ```
impl<S: RgbStandard> Rgba<S, u8> {
    /// Convert to a packed `u32` with with specifiable component order.
    /// Defaults to ARGB ordering (0xAARRGGBB).
    ///
    /// See [Packed](crate::Packed) for more details.
    pub fn into_u32<C: RgbChannels>(self) -> u32 {
        Packed::<C>::from(self).color
    }

    /// Convert from a packed `u32` with specifiable component order. Defaults
    /// to ARGB ordering (0xAARRGGBB).
    ///
    /// See [Packed](crate::Packed) for more details.
    pub fn from_u32<C: RgbChannels>(color: u32) -> Self {
        Packed::<C>::from(color).into()
    }
}

/// <span id="Rgba"></span>[`Rgba`](crate::rgb::Rgba) implementations.
impl<S: RgbStandard, T: FloatComponent, A: Component> Alpha<Rgb<S, T>, A> {
    /// Convert the color to linear RGB with transparency.
    pub fn into_linear(self) -> Alpha<Rgb<Linear<S::Space>, T>, A> {
        Alpha::<Rgb<Linear<S::Space>, T>, A>::new(
            S::TransferFn::into_linear(self.red),
            S::TransferFn::into_linear(self.green),
            S::TransferFn::into_linear(self.blue),
            self.alpha,
        )
    }

    /// Convert linear RGB to non-linear RGB with transparency.
    pub fn from_linear(color: Alpha<Rgb<Linear<S::Space>, T>, A>) -> Self {
        Self::new(
            S::TransferFn::from_linear(color.red),
            S::TransferFn::from_linear(color.green),
            S::TransferFn::from_linear(color.blue),
            color.alpha,
        )
    }

    /// Convert the color to a different encoding with transparency.
    pub fn into_encoding<St: RgbStandard<Space = S::Space>>(self) -> Alpha<Rgb<St, T>, A> {
        Alpha::<Rgb<St, T>, A>::new(
            St::TransferFn::from_linear(S::TransferFn::into_linear(self.red)),
            St::TransferFn::from_linear(S::TransferFn::into_linear(self.green)),
            St::TransferFn::from_linear(S::TransferFn::into_linear(self.blue)),
            self.alpha,
        )
    }

    /// Convert RGB from a different encoding with transparency.
    pub fn from_encoding<St: RgbStandard<Space = S::Space>>(color: Alpha<Rgb<St, T>, A>) -> Self {
        Self::new(
            S::TransferFn::from_linear(St::TransferFn::into_linear(color.red)),
            S::TransferFn::from_linear(St::TransferFn::into_linear(color.green)),
            S::TransferFn::from_linear(St::TransferFn::into_linear(color.blue)),
            color.alpha,
        )
    }
}

impl<S1, S2, T> FromColorUnclamped<Rgb<S2, T>> for Rgb<S1, T>
where
    S1: RgbStandard,
    S2: RgbStandard,
    S2::Space: RgbSpace<WhitePoint = <S1::Space as RgbSpace>::WhitePoint>,
    T: FloatComponent,
{
    fn from_color_unclamped(rgb: Rgb<S2, T>) -> Self {
        if TypeId::of::<S1>() == TypeId::of::<S2>() {
            rgb.reinterpret_as()
        } else if TypeId::of::<<S1::Space as RgbSpace>::Primaries>()
            == TypeId::of::<<S2::Space as RgbSpace>::Primaries>()
        {
            Self::from_linear(rgb.into_linear().reinterpret_as())
        } else {
            Self::from_color_unclamped(Xyz::from_color_unclamped(rgb))
        }
    }
}

impl<S, T> FromColorUnclamped<Xyz<<S::Space as RgbSpace>::WhitePoint, T>> for Rgb<S, T>
where
    S: RgbStandard,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Xyz<<S::Space as RgbSpace>::WhitePoint, T>) -> Self {
        let transform_matrix = matrix_inverse(&rgb_to_xyz_matrix::<S::Space, T>());
        Self::from_linear(multiply_xyz_to_rgb(&transform_matrix, &color))
    }
}

impl<S, T> FromColorUnclamped<Hsl<S, T>> for Rgb<S, T>
where
    S: RgbStandard,
    T: FloatComponent,
{
    fn from_color_unclamped(hsl: Hsl<S, T>) -> Self {
        let c = (T::one() - (hsl.lightness * from_f64(2.0) - T::one()).abs()) * hsl.saturation;
        let h = hsl.hue.to_positive_degrees() / from_f64(60.0);
        let x = c * (T::one() - (h % from_f64(2.0) - T::one()).abs());
        let m = hsl.lightness - c * from_f64(0.5);

        let (red, green, blue) = if h >= T::zero() && h < T::one() {
            (c, x, T::zero())
        } else if h >= T::one() && h < from_f64(2.0) {
            (x, c, T::zero())
        } else if h >= from_f64(2.0) && h < from_f64(3.0) {
            (T::zero(), c, x)
        } else if h >= from_f64(3.0) && h < from_f64(4.0) {
            (T::zero(), x, c)
        } else if h >= from_f64(4.0) && h < from_f64(5.0) {
            (x, T::zero(), c)
        } else {
            (c, T::zero(), x)
        };

        Rgb {
            red: red + m,
            green: green + m,
            blue: blue + m,
            standard: PhantomData,
        }
    }
}

impl<S, T> FromColorUnclamped<Hsv<S, T>> for Rgb<S, T>
where
    S: RgbStandard,
    T: FloatComponent,
{
    fn from_color_unclamped(hsv: Hsv<S, T>) -> Self {
        let c = hsv.value * hsv.saturation;
        let h = hsv.hue.to_positive_degrees() / from_f64(60.0);
        let x = c * (T::one() - (h % from_f64(2.0) - T::one()).abs());
        let m = hsv.value - c;

        let (red, green, blue) = if h >= T::zero() && h < T::one() {
            (c, x, T::zero())
        } else if h >= T::one() && h < from_f64(2.0) {
            (x, c, T::zero())
        } else if h >= from_f64(2.0) && h < from_f64(3.0) {
            (T::zero(), c, x)
        } else if h >= from_f64(3.0) && h < from_f64(4.0) {
            (T::zero(), x, c)
        } else if h >= from_f64(4.0) && h < from_f64(5.0) {
            (x, T::zero(), c)
        } else {
            (c, T::zero(), x)
        };

        Rgb {
            red: red + m,
            green: green + m,
            blue: blue + m,
            standard: PhantomData,
        }
    }
}

impl<S, St, T> FromColorUnclamped<Luma<St, T>> for Rgb<S, T>
where
    S: RgbStandard,
    St: LumaStandard<WhitePoint = <S::Space as RgbSpace>::WhitePoint>,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Luma<St, T>) -> Self {
        let luma = color.into_linear();

        Self::from_linear(Rgb {
            red: luma.luma,
            green: luma.luma,
            blue: luma.luma,
            standard: PhantomData,
        })
    }
}

impl<S, T> Clamp for Rgb<S, T>
where
    S: RgbStandard,
    T: Component,
{
    #[rustfmt::skip]
    fn is_within_bounds(&self) -> bool {
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
    T: FloatComponent,
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
    T: FloatComponent,
{
    type Scalar = T;

    fn lighten(&self, factor: T) -> Rgb<S, T> {
        let difference_red = if factor >= T::zero() {
            T::max_intensity() - self.red
        } else {
            self.red
        };
        let delta_red = difference_red.max(T::zero()) * factor;

        let difference_green = if factor >= T::zero() {
            T::max_intensity() - self.green
        } else {
            self.green
        };
        let delta_green = difference_green.max(T::zero()) * factor;

        let difference_blue = if factor >= T::zero() {
            T::max_intensity() - self.blue
        } else {
            self.blue
        };
        let delta_blue = difference_blue.max(T::zero()) * factor;

        Rgb {
            red: (self.red + delta_red).max(T::zero()),
            green: (self.green + delta_green).max(T::zero()),
            blue: (self.blue + delta_blue).max(T::zero()),
            standard: PhantomData,
        }
    }

    fn lighten_fixed(&self, amount: T) -> Rgb<S, T> {
        Rgb {
            red: (self.red + T::max_intensity() * amount).max(T::zero()),
            green: (self.green + T::max_intensity() * amount).max(T::zero()),
            blue: (self.blue + T::max_intensity() * amount).max(T::zero()),
            standard: PhantomData,
        }
    }
}

impl<S, T> GetHue for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: FloatComponent,
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        let sqrt_3: T = from_f64(1.73205081);

        if self.red == self.green && self.red == self.blue {
            None
        } else {
            Some(RgbHue::from_radians(
                (sqrt_3 * (self.green - self.blue))
                    .atan2(self.red * from_f64(2.0) - self.green - self.blue),
            ))
        }
    }
}

impl<S, T> Blend for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: FloatComponent,
{
    type Color = Rgb<S, T>;

    fn into_premultiplied(self) -> PreAlpha<Rgb<S, T>, T> {
        Rgba {
            color: self,
            alpha: T::one(),
        }
        .into_premultiplied()
    }

    fn from_premultiplied(color: PreAlpha<Rgb<S, T>, T>) -> Self {
        Rgba::from_premultiplied(color).color
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

impl<S, T> AddAssign<Rgb<S, T>> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + AddAssign,
{
    fn add_assign(&mut self, other: Rgb<S, T>) {
        self.red += other.red;
        self.green += other.green;
        self.blue += other.blue;
    }
}

impl<S, T> AddAssign<T> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + AddAssign,
{
    fn add_assign(&mut self, c: T) {
        self.red += c;
        self.green += c;
        self.blue += c;
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

impl<S, T> SubAssign<Rgb<S, T>> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + SubAssign,
{
    fn sub_assign(&mut self, other: Rgb<S, T>) {
        self.red -= other.red;
        self.green -= other.green;
        self.blue -= other.blue;
    }
}

impl<S, T> SubAssign<T> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + SubAssign,
{
    fn sub_assign(&mut self, c: T) {
        self.red -= c;
        self.green -= c;
        self.blue -= c;
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

impl<S, T> MulAssign<Rgb<S, T>> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + MulAssign,
{
    fn mul_assign(&mut self, other: Rgb<S, T>) {
        self.red *= other.red;
        self.green *= other.green;
        self.blue *= other.blue;
    }
}

impl<S, T> MulAssign<T> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + MulAssign,
{
    fn mul_assign(&mut self, c: T) {
        self.red *= c;
        self.green *= c;
        self.blue *= c;
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

impl<S, T> DivAssign<Rgb<S, T>> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + DivAssign,
{
    fn div_assign(&mut self, other: Rgb<S, T>) {
        self.red /= other.red;
        self.green /= other.green;
        self.blue /= other.blue;
    }
}

impl<S, T> DivAssign<T> for Rgb<S, T>
where
    S: RgbStandard<TransferFn = LinearFn>,
    T: Component + DivAssign,
{
    fn div_assign(&mut self, c: T) {
        self.red /= c;
        self.green /= c;
        self.blue /= c;
    }
}

impl<S: RgbStandard, T: Component> From<(T, T, T)> for Rgb<S, T> {
    fn from(components: (T, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<S: RgbStandard, T: Component> Into<(T, T, T)> for Rgb<S, T> {
    fn into(self) -> (T, T, T) {
        self.into_components()
    }
}

impl<S: RgbStandard, T: Component, A: Component> From<(T, T, T, A)> for Alpha<Rgb<S, T>, A> {
    fn from(components: (T, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<S: RgbStandard, T: Component, A: Component> Into<(T, T, T, A)> for Alpha<Rgb<S, T>, A> {
    fn into(self) -> (T, T, T, A) {
        self.into_components()
    }
}

impl<S, T> AbsDiffEq for Rgb<S, T>
where
    T: Component + AbsDiffEq,
    T::Epsilon: Copy,
    S: RgbStandard + PartialEq,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.red.abs_diff_eq(&other.red, epsilon)
            && self.green.abs_diff_eq(&other.green, epsilon)
            && self.blue.abs_diff_eq(&other.blue, epsilon)
    }
}

impl<S, T> RelativeEq for Rgb<S, T>
where
    T: Component + RelativeEq,
    T::Epsilon: Copy,
    S: RgbStandard + PartialEq,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[rustfmt::skip]
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
}

impl<S, T> UlpsEq for Rgb<S, T>
where
    T: Component + UlpsEq,
    T::Epsilon: Copy,
    S: RgbStandard + PartialEq,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[rustfmt::skip]
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

impl<S, T> fmt::LowerHex for Rgb<S, T>
where
    T: Component + fmt::LowerHex,
    S: RgbStandard,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = f.width().unwrap_or(::core::mem::size_of::<T>() * 2);
        write!(
            f,
            "{:0width$x}{:0width$x}{:0width$x}",
            self.red,
            self.green,
            self.blue,
            width = size
        )
    }
}

impl<S, T> fmt::UpperHex for Rgb<S, T>
where
    T: Component + fmt::UpperHex,
    S: RgbStandard,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = f.width().unwrap_or(::core::mem::size_of::<T>() * 2);
        write!(
            f,
            "{:0width$X}{:0width$X}{:0width$X}",
            self.red,
            self.green,
            self.blue,
            width = size
        )
    }
}

/// Error type for parsing a string of hexadecimal characters to an `Rgb` color.
#[derive(Debug)]
pub enum FromHexError {
    /// An error occurred while parsing the string into a valid integer.
    ParseIntError(ParseIntError),
    /// The hex value was not in a valid 3 or 6 character format.
    HexFormatError(&'static str),
}

impl From<ParseIntError> for FromHexError {
    fn from(err: ParseIntError) -> FromHexError {
        FromHexError::ParseIntError(err)
    }
}

impl From<&'static str> for FromHexError {
    fn from(err: &'static str) -> FromHexError {
        FromHexError::HexFormatError(err)
    }
}
impl core::fmt::Display for FromHexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            FromHexError::ParseIntError(e) => write!(f, "{}", e),
            FromHexError::HexFormatError(s) => write!(
                f,
                "{}, please use format '#fff', 'fff', '#ffffff' or 'ffffff'.",
                s
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromHexError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &*self {
            FromHexError::HexFormatError(_s) => None,
            FromHexError::ParseIntError(e) => Some(e),
        }
    }
}

impl<S: RgbStandard> FromStr for Rgb<S, u8> {
    type Err = FromHexError;

    // Parses a color hex code of format '#ff00bb' or '#abc' into a
    // Rgb<S, u8> instance.
    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        let hex_code = hex.strip_prefix('#').map_or(hex, |stripped| stripped);
        match hex_code.len() {
            3 => {
                let red = u8::from_str_radix(&hex_code[..1], 16)?;
                let green = u8::from_str_radix(&hex_code[1..2], 16)?;
                let blue = u8::from_str_radix(&hex_code[2..3], 16)?;
                let col: Rgb<S, u8> = Rgb::new(red * 17, green * 17, blue * 17);
                Ok(col)
            }
            6 => {
                let red = u8::from_str_radix(&hex_code[..2], 16)?;
                let green = u8::from_str_radix(&hex_code[2..4], 16)?;
                let blue = u8::from_str_radix(&hex_code[4..6], 16)?;
                let col: Rgb<S, u8> = Rgb::new(red, green, blue);
                Ok(col)
            }
            _ => Err("invalid hex code format".into()),
        }
    }
}

impl<S, T> RelativeContrast for Rgb<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Scalar = T;

    fn get_contrast_ratio(&self, other: &Self) -> T {
        use crate::FromColor;

        let xyz1 = Xyz::from_color(*self);
        let xyz2 = Xyz::from_color(*other);

        contrast_ratio(xyz1.y, xyz2.y)
    }
}

#[cfg(feature = "random")]
impl<S, T> Distribution<Rgb<S, T>> for Standard
where
    T: Component,
    S: RgbStandard,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rgb<S, T> {
        Rgb {
            red: rng.gen(),
            green: rng.gen(),
            blue: rng.gen(),
            standard: PhantomData,
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformRgb<S, T>
where
    T: Component + SampleUniform,
    S: RgbStandard,
{
    red: Uniform<T>,
    green: Uniform<T>,
    blue: Uniform<T>,
    standard: PhantomData<S>,
}

#[cfg(feature = "random")]
impl<S, T> SampleUniform for Rgb<S, T>
where
    T: Component + SampleUniform,
    S: RgbStandard,
{
    type Sampler = UniformRgb<S, T>;
}

#[cfg(feature = "random")]
impl<S, T> UniformSampler for UniformRgb<S, T>
where
    T: Component + SampleUniform,
    S: RgbStandard,
{
    type X = Rgb<S, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        UniformRgb {
            red: Uniform::new::<_, T>(low.red, high.red),
            green: Uniform::new::<_, T>(low.green, high.green),
            blue: Uniform::new::<_, T>(low.blue, high.blue),
            standard: PhantomData,
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        UniformRgb {
            red: Uniform::new_inclusive::<_, T>(low.red, high.red),
            green: Uniform::new_inclusive::<_, T>(low.green, high.green),
            blue: Uniform::new_inclusive::<_, T>(low.blue, high.blue),
            standard: PhantomData,
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rgb<S, T> {
        Rgb {
            red: self.red.sample(rng),
            green: self.green.sample(rng),
            blue: self.blue.sample(rng),
            standard: PhantomData,
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<S, T> bytemuck::Zeroable for Rgb<S, T>
where
    S: RgbStandard,
    T: Component + bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<S, T> bytemuck::Pod for Rgb<S, T>
where
    S: RgbStandard,
    T: Component + bytemuck::Pod,
{
}

#[cfg(test)]
mod test {
    use core::str::FromStr;

    use super::{Rgb, Rgba};
    use crate::encoding::Srgb;
    use crate::rgb::packed::channels;

    #[test]
    fn ranges() {
        assert_ranges! {
            Rgb<Srgb, f64>;
            clamped {
                red: 0.0 => 1.0,
                green: 0.0 => 1.0,
                blue: 0.0 => 1.0
            }
            clamped_min {}
            unclamped {}
        }
    }

    raw_pixel_conversion_tests!(Rgb<Srgb>: red, green, blue);
    raw_pixel_conversion_fail_tests!(Rgb<Srgb>: red, green, blue);

    #[test]
    fn lower_hex() {
        assert_eq!(
            format!("{:x}", Rgb::<Srgb, u8>::new(171, 193, 35)),
            "abc123"
        );
    }

    #[test]
    fn lower_hex_small_numbers() {
        assert_eq!(format!("{:x}", Rgb::<Srgb, u8>::new(1, 2, 3)), "010203");
        assert_eq!(
            format!("{:x}", Rgb::<Srgb, u16>::new(1, 2, 3)),
            "000100020003"
        );
        assert_eq!(
            format!("{:x}", Rgb::<Srgb, u32>::new(1, 2, 3)),
            "000000010000000200000003"
        );
        assert_eq!(
            format!("{:x}", Rgb::<Srgb, u64>::new(1, 2, 3)),
            "000000000000000100000000000000020000000000000003"
        );
    }

    #[test]
    fn lower_hex_custom_width() {
        assert_eq!(
            format!("{:03x}", Rgb::<Srgb, u8>::new(1, 2, 3)),
            "001002003"
        );
        assert_eq!(
            format!("{:03x}", Rgb::<Srgb, u16>::new(1, 2, 3)),
            "001002003"
        );
        assert_eq!(
            format!("{:03x}", Rgb::<Srgb, u32>::new(1, 2, 3)),
            "001002003"
        );
        assert_eq!(
            format!("{:03x}", Rgb::<Srgb, u64>::new(1, 2, 3)),
            "001002003"
        );
    }

    #[test]
    fn upper_hex() {
        assert_eq!(
            format!("{:X}", Rgb::<Srgb, u8>::new(171, 193, 35)),
            "ABC123"
        );
    }

    #[test]
    fn upper_hex_small_numbers() {
        assert_eq!(format!("{:X}", Rgb::<Srgb, u8>::new(1, 2, 3)), "010203");
        assert_eq!(
            format!("{:X}", Rgb::<Srgb, u16>::new(1, 2, 3)),
            "000100020003"
        );
        assert_eq!(
            format!("{:X}", Rgb::<Srgb, u32>::new(1, 2, 3)),
            "000000010000000200000003"
        );
        assert_eq!(
            format!("{:X}", Rgb::<Srgb, u64>::new(1, 2, 3)),
            "000000000000000100000000000000020000000000000003"
        );
    }

    #[test]
    fn upper_hex_custom_width() {
        assert_eq!(
            format!("{:03X}", Rgb::<Srgb, u8>::new(1, 2, 3)),
            "001002003"
        );
        assert_eq!(
            format!("{:03X}", Rgb::<Srgb, u16>::new(1, 2, 3)),
            "001002003"
        );
        assert_eq!(
            format!("{:03X}", Rgb::<Srgb, u32>::new(1, 2, 3)),
            "001002003"
        );
        assert_eq!(
            format!("{:03X}", Rgb::<Srgb, u64>::new(1, 2, 3)),
            "001002003"
        );
    }

    #[test]
    fn rgb_hex_into_from() {
        let c1 = Rgb::<Srgb, u8>::from_u32::<channels::Argb>(0x1100_7FFF);
        let c2 = Rgb::<Srgb, u8>::new(0u8, 127, 255);
        assert_eq!(c1, c2);
        assert_eq!(Rgb::<Srgb, u8>::into_u32::<channels::Argb>(c1), 0xFF00_7FFF);

        let c1 = Rgba::<Srgb, u8>::from_u32::<channels::Rgba>(0x007F_FF80);
        let c2 = Rgba::<Srgb, u8>::new(0u8, 127, 255, 128);
        assert_eq!(c1, c2);
        assert_eq!(
            Rgba::<Srgb, u8>::into_u32::<channels::Rgba>(c1),
            0x007F_FF80
        );

        assert_eq!(
            Rgb::<Srgb, u8>::from(0x7FFF_FF80),
            Rgb::from((255u8, 255, 128))
        );
        assert_eq!(
            Rgba::<Srgb, u8>::from(0x7FFF_FF80),
            Rgba::from((127u8, 255, 255, 128))
        );
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Rgb::<Srgb>::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"red":0.3,"green":0.8,"blue":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Rgb<Srgb> =
            ::serde_json::from_str(r#"{"red":0.3,"green":0.8,"blue":0.1}"#).unwrap();

        assert_eq!(deserialized, Rgb::<Srgb>::new(0.3, 0.8, 0.1));
    }

    #[test]
    fn from_str() {
        let c = Rgb::<Srgb, u8>::from_str("#ffffff");
        assert!(c.is_ok());
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(255, 255, 255));
        let c = Rgb::<Srgb, u8>::from_str("#gggggg");
        assert!(c.is_err());
        let c = Rgb::<Srgb, u8>::from_str("#fff");
        assert!(c.is_ok());
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(255, 255, 255));
        let c = Rgb::<Srgb, u8>::from_str("#000000");
        assert!(c.is_ok());
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(0, 0, 0));
        let c = Rgb::<Srgb, u8>::from_str("");
        assert!(c.is_err());
        let c = Rgb::<Srgb, u8>::from_str("#123456");
        assert!(c.is_ok());
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(18, 52, 86));
        let c = Rgb::<Srgb, u8>::from_str("#iii");
        assert!(c.is_err());
        assert_eq!(
            format!("{}", c.err().unwrap()),
            "invalid digit found in string"
        );
        let c = Rgb::<Srgb, u8>::from_str("#08f");
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(0, 136, 255));
        let c = Rgb::<Srgb, u8>::from_str("08f");
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(0, 136, 255));
        let c = Rgb::<Srgb, u8>::from_str("ffffff");
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(255, 255, 255));
        let c = Rgb::<Srgb, u8>::from_str("#12");
        assert!(c.is_err());
        assert_eq!(
            format!("{}", c.err().unwrap()),
            "invalid hex code format, \
             please use format \'#fff\', \'fff\', \'#ffffff\' or \'ffffff\'."
        );
        let c = Rgb::<Srgb, u8>::from_str("da0bce");
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(218, 11, 206));
        let c = Rgb::<Srgb, u8>::from_str("f034e6");
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(240, 52, 230));
        let c = Rgb::<Srgb, u8>::from_str("abc");
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(170, 187, 204));
    }

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Rgb::<Srgb, f32>::min_red(), 0.0);
        assert_relative_eq!(Rgb::<Srgb, f32>::min_green(), 0.0);
        assert_relative_eq!(Rgb::<Srgb, f32>::min_blue(), 0.0);
        assert_relative_eq!(Rgb::<Srgb, f32>::max_red(), 1.0);
        assert_relative_eq!(Rgb::<Srgb, f32>::max_green(), 1.0);
        assert_relative_eq!(Rgb::<Srgb, f32>::max_blue(), 1.0);
    }

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Rgb<Srgb, f32> {
            red: (0.0, 1.0),
            green: (0.0, 1.0),
            blue: (0.0, 1.0)
        },
        min: Rgb::new(0.0f32, 0.0, 0.0),
        max: Rgb::new(1.0, 1.0, 1.0)
    }
}
