use core::any::TypeId;
use core::fmt;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use float::Float;

use alpha::Alpha;
use blend::PreAlpha;
use convert::{FromColor, IntoColor};
use encoding::linear::LinearFn;
use encoding::pixel::RawPixel;
use encoding::{Linear, Srgb};
use luma::LumaStandard;
use matrix::{matrix_inverse, multiply_xyz_to_rgb, rgb_to_xyz_matrix};
use rgb::{RgbSpace, RgbStandard, TransferFn};
use white_point::WhitePoint;
use {cast, clamp};
use {Blend, Component, ComponentWise, GetHue, Limited, Mix, Pixel, Shade};
use {Hsl, Hsv, Hwb, Lab, Lch, Luma, RgbHue, Xyz, Yxy};

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
/// Many conversions and operations on this color space requires that it's
/// linear, meaning that gamma correction is required when converting to and
/// from a displayable RGB, such as sRGB. See the [`pixel`](pixel/index.html)
/// module for encoding formats.
#[derive(Debug, PartialEq, FromColor, Pixel)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette_internal]
#[palette_rgb_space = "S::Space"]
#[palette_white_point = "<S::Space as RgbSpace>::WhitePoint"]
#[palette_component = "T"]
#[palette_manual_from(Xyz, Hsv, Hsl, Luma, Rgb = "from_rgb_internal")]
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
    #[palette_unsafe_zero_sized]
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

    /// Convert to a `(red, green, blue)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.red, self.green, self.blue)
    }

    /// Convert from a `(red, green, blue)` tuple.
    pub fn from_components((red, green, blue): (T, T, T)) -> Self {
        Self::new(red, green, blue)
    }
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

    fn from_rgb_internal<Sp>(rgb: Rgb<Linear<Sp>, T>) -> Self
    where
        Sp: RgbSpace<WhitePoint = <S::Space as RgbSpace>::WhitePoint>,
    {
        if TypeId::of::<Sp::Primaries>() == TypeId::of::<<S::Space as RgbSpace>::Primaries>() {
            Self::from_linear(rgb.reinterpret_as())
        } else {
            Self::from_xyz(Xyz::from_rgb(rgb))
        }
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
impl<S: RgbStandard, T: Component, A: Component> Alpha<Rgb<S, T>, A> {
    /// Nonlinear RGB.
    pub fn new(red: T, green: T, blue: T, alpha: A) -> Self {
        Alpha {
            color: Rgb::new(red, green, blue),
            alpha: alpha,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U: Component, B: Component>(self) -> Alpha<Rgb<S, U>, B> {
        Alpha::<Rgb<S, U>, B>::new(
            self.red.convert(),
            self.green.convert(),
            self.blue.convert(),
            self.alpha.convert(),
        )
    }

    /// Convert from another component type.
    pub fn from_format<U: Component, B: Component>(color: Alpha<Rgb<S, U>, B>) -> Self {
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

/// <span id="Rgba"></span>[`Rgba`](rgb/type.Rgba.html) implementations.
impl<S: RgbStandard, T: Component + Float, A: Component> Alpha<Rgb<S, T>, A> {
    /// Convert the color to linear RGB with transparency.
    pub fn into_linear(self) -> Alpha<Rgb<Linear<S::Space>, T>, A> {
        Alpha::<Rgb<Linear<S::Space>, T>, A>::new(
            S::TransferFn::into_linear(self.red),
            S::TransferFn::into_linear(self.green),
            S::TransferFn::into_linear(self.blue),
            self.alpha,
        )
    }

    /// Convert linear RGB to nonlinear RGB with transparency.
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

impl<S, Wp, T> From<Xyz<Wp, T>> for Rgb<S, T>
where
    S: RgbStandard,
    T: Component + Float,
    Wp: WhitePoint,
    S::Space: RgbSpace<WhitePoint = Wp>,
{
    fn from(color: Xyz<Wp, T>) -> Self {
        let transform_matrix = matrix_inverse(&rgb_to_xyz_matrix::<S::Space, T>());
        Self::from_linear(multiply_xyz_to_rgb(&transform_matrix, &color))
    }
}

impl<S, T, Sp, Wp> From<Hsl<Sp, T>> for Rgb<S, T>
where
    S: RgbStandard,
    T: Component + Float,
    Wp: WhitePoint,
    S::Space: RgbSpace<WhitePoint = Wp>,
    Sp: RgbSpace<WhitePoint = Wp>,
{
    fn from(color: Hsl<Sp, T>) -> Self {
        let hsl = Hsl::<S::Space, T>::from_hsl(color);

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
}

impl<S, T, Sp, Wp> From<Hsv<Sp, T>> for Rgb<S, T>
where
    S: RgbStandard,
    T: Component + Float,
    Wp: WhitePoint,
    S::Space: RgbSpace<WhitePoint = Wp>,
    Sp: RgbSpace<WhitePoint = Wp>,
{
    fn from(color: Hsv<Sp, T>) -> Self {
        let hsv = Hsv::<S::Space, T>::from_hsv(color);

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
}

impl<S, T, St, Wp> From<Luma<St, T>> for Rgb<S, T>
where
    S: RgbStandard,
    T: Component + Float,
    Wp: WhitePoint,
    S::Space: RgbSpace<WhitePoint = Wp>,
    St: LumaStandard<WhitePoint = Wp>,
{
    fn from(color: Luma<St, T>) -> Self {
        let luma = color.into_linear();

        Self::from_linear(Rgb {
            red: luma.luma,
            green: luma.luma,
            blue: luma.luma,
            standard: PhantomData,
        })
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
        self.red.abs_diff_eq(&other.red, epsilon) &&
            self.green.abs_diff_eq(&other.green, epsilon) &&
            self.blue.abs_diff_eq(&other.blue, epsilon)
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
}
