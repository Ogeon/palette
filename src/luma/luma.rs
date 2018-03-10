use approx::ApproxEq;

use num_traits::Float;

use std::ops::{Add, Div, Mul, Sub};
use std::marker::PhantomData;

use {Alpha, Xyz, Yxy};
use {Blend, Component, ComponentWise, FromColor, IntoColor, Limited, Mix, Pixel, Shade};
use luma::LumaStandard;
use encoding::{Linear, Srgb, TransferFn};
use encoding::linear::LinearFn;
use encoding::pixel::RawPixel;
use white_point::WhitePoint;
use clamp;
use blend::PreAlpha;

/// Luminance with an alpha component. See the [`Lumaa` implementation
/// in `Alpha`](struct.Alpha.html#Lumaa).
pub type Lumaa<S = Srgb, T = f32> = Alpha<Luma<S, T>, T>;

///Luminance.
///
///Luma is a purely gray scale color space, which is included more for
///completeness than anything else, and represents how bright a color is
///perceived to be. It's basically the `Y` component of [CIE
///XYZ](struct.Xyz.html). The lack of any form of hue representation limits
///the set of operations that can be performed on it.
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Luma<S = Srgb, T = f32>
where
    T: Component,
    S: LumaStandard,
{
    ///The lightness of the color. 0.0 is black and 1.0 is white.
    pub luma: T,

    /// The kind of RGB standard. sRGB is the default.
    pub standard: PhantomData<S>,
}

impl<S, T> Copy for Luma<S, T>
where
    T: Component,
    S: LumaStandard,
{
}

impl<S, T> Clone for Luma<S, T>
where
    T: Component,
    S: LumaStandard,
{
    fn clone(&self) -> Luma<S, T> {
        *self
    }
}

unsafe impl<S: LumaStandard, T: Component> Pixel<T> for Luma<S, T> {
    const CHANNELS: usize = 1;
}

impl<S, T> Luma<S, T>
where
    T: Component,
    S: LumaStandard,
{
    /// Create a luminance color.
    pub fn new(luma: T) -> Luma<S, T> {
        Luma {
            luma: luma,
            standard: PhantomData,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U: Component>(self) -> Luma<S, U> {
        Luma {
            luma: self.luma.convert(),
            standard: PhantomData,
        }
    }

    /// Convert from another component type.
    pub fn from_format<U: Component>(color: Luma<S, U>) -> Self {
        color.into_format()
    }
}

impl<S, T> Luma<S, T>
where
    T: Component + Float,
    S: LumaStandard,
{
    /// Convert the color to linear luminance.
    pub fn into_linear(self) -> Luma<Linear<S::WhitePoint>, T> {
        Luma::new(S::TransferFn::into_linear(self.luma))
    }

    /// Convert linear luminance to nonlinear luminance.
    pub fn from_linear(color: Luma<Linear<S::WhitePoint>, T>) -> Luma<S, T> {
        Luma::new(S::TransferFn::from_linear(color.luma))
    }
}

///<span id="Lumaa"></span>[`Lumaa`](type.Lumaa.html) implementations.
impl<S, T> Alpha<Luma<S, T>, T>
where
    T: Component,
    S: LumaStandard,
{
    /// Create a luminance color with transparency.
    pub fn new(luma: T, alpha: T) -> Self {
        Alpha {
            color: Luma::new(luma),
            alpha: alpha,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U: Component>(self) -> Lumaa<S, U> {
        Lumaa::new(self.luma.convert(), self.alpha.convert())
    }

    /// Convert from another component type.
    pub fn from_format<U: Component>(color: Lumaa<S, U>) -> Self {
        color.into_format()
    }
}

///<span id="Lumaa"></span>[`Lumaa`](type.Lumaa.html) implementations.
impl<S, T> Alpha<Luma<S, T>, T>
where
    T: Component + Float,
    S: LumaStandard,
{
    /// Convert the color to linear luminance with transparency.
    pub fn into_linear(self) -> Lumaa<Linear<S::WhitePoint>, T> {
        Lumaa::new(S::TransferFn::into_linear(self.luma), self.alpha)
    }

    /// Convert linear luminance to nonlinear luminance with transparency.
    pub fn from_linear(color: Lumaa<Linear<S::WhitePoint>, T>) -> Lumaa<S, T> {
        Lumaa::new(S::TransferFn::from_linear(color.luma), color.alpha)
    }
}

impl<S, Wp, T> FromColor<Wp, T> for Luma<S, T>
where
    S: LumaStandard<WhitePoint = Wp>,
    T: Component + Float,
    Wp: WhitePoint,
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        Self::from_linear(Luma {
            luma: xyz.y,
            standard: PhantomData,
        })
    }

    fn from_yxy(yxy: Yxy<Wp, T>) -> Self {
        Self::from_linear(Luma {
            luma: yxy.luma,
            standard: PhantomData,
        })
    }

    fn from_luma(luma: Luma<Linear<Wp>, T>) -> Self {
        Self::from_linear(luma)
    }
}

impl<S, Wp, T> IntoColor<Wp, T> for Luma<S, T>
where
    S: LumaStandard<WhitePoint = Wp>,
    T: Component + Float,
    Wp: WhitePoint,
{
    fn into_xyz(self) -> Xyz<Wp, T> {
        Xyz::from_luma(self.into_linear())
    }

    fn into_yxy(self) -> Yxy<Wp, T> {
        Yxy::from_luma(self.into_linear())
    }

    fn into_luma(self) -> Luma<Linear<Wp>, T> {
        self.into_linear()
    }
}

impl<S, T> Limited for Luma<S, T>
where
    T: Component,
    S: LumaStandard,
{
    fn is_valid(&self) -> bool {
        self.luma >= T::zero() && self.luma <= T::max_intensity()
    }

    fn clamp(&self) -> Luma<S, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.luma = clamp(self.luma, T::zero(), T::max_intensity());
    }
}

impl<S, T> Mix for Luma<S, T>
where
    T: Component + Float,
    S: LumaStandard<TransferFn = LinearFn>,
{
    type Scalar = T;

    fn mix(&self, other: &Luma<S, T>, factor: T) -> Luma<S, T> {
        let factor = clamp(factor, T::zero(), T::one());

        Luma {
            luma: self.luma + factor * (other.luma - self.luma),
            standard: PhantomData,
        }
    }
}

impl<S, T> Shade for Luma<S, T>
where
    T: Component + Float,
    S: LumaStandard<TransferFn = LinearFn>,
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Luma<S, T> {
        Luma {
            luma: (self.luma + amount).max(T::zero()),
            standard: PhantomData,
        }
    }
}

impl<S, T> Blend for Luma<S, T>
where
    T: Component + Float,
    S: LumaStandard<TransferFn = LinearFn>,
{
    type Color = Luma<S, T>;

    fn into_premultiplied(self) -> PreAlpha<Luma<S, T>, T> {
        Lumaa::from(self).into()
    }

    fn from_premultiplied(color: PreAlpha<Luma<S, T>, T>) -> Self {
        Lumaa::from(color).into()
    }
}

impl<S, T> ComponentWise for Luma<S, T>
where
    T: Component,
    S: LumaStandard,
{
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Luma<S, T>, mut f: F) -> Luma<S, T> {
        Luma {
            luma: f(self.luma, other.luma),
            standard: PhantomData,
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Luma<S, T> {
        Luma {
            luma: f(self.luma),
            standard: PhantomData,
        }
    }
}

impl<S, T> Default for Luma<S, T>
where
    T: Component,
    S: LumaStandard,
{
    fn default() -> Luma<S, T> {
        Luma::new(T::zero())
    }
}

impl<S, T> Add<Luma<S, T>> for Luma<S, T>
where
    T: Component + Add,
    S: LumaStandard<TransferFn = LinearFn>,
    <T as Add>::Output: Component,
{
    type Output = Luma<S, <T as Add>::Output>;

    fn add(self, other: Luma<S, T>) -> Self::Output {
        Luma {
            luma: self.luma + other.luma,
            standard: PhantomData,
        }
    }
}

impl<S, T> Add<T> for Luma<S, T>
where
    T: Component + Add,
    S: LumaStandard<TransferFn = LinearFn>,
    <T as Add>::Output: Component,
{
    type Output = Luma<S, <T as Add>::Output>;

    fn add(self, c: T) -> Self::Output {
        Luma {
            luma: self.luma + c,
            standard: PhantomData,
        }
    }
}

impl<S, T> Sub<Luma<S, T>> for Luma<S, T>
where
    T: Component + Sub,
    S: LumaStandard<TransferFn = LinearFn>,
    <T as Sub>::Output: Component,
{
    type Output = Luma<S, <T as Sub>::Output>;

    fn sub(self, other: Luma<S, T>) -> Self::Output {
        Luma {
            luma: self.luma - other.luma,
            standard: PhantomData,
        }
    }
}

impl<S, T> Sub<T> for Luma<S, T>
where
    T: Component + Sub,
    S: LumaStandard<TransferFn = LinearFn>,
    <T as Sub>::Output: Component,
{
    type Output = Luma<S, <T as Sub>::Output>;

    fn sub(self, c: T) -> Self::Output {
        Luma {
            luma: self.luma - c,
            standard: PhantomData,
        }
    }
}

impl<S, T> Mul<Luma<S, T>> for Luma<S, T>
where
    T: Component + Mul,
    S: LumaStandard<TransferFn = LinearFn>,
    <T as Mul>::Output: Component,
{
    type Output = Luma<S, <T as Mul>::Output>;

    fn mul(self, other: Luma<S, T>) -> Self::Output {
        Luma {
            luma: self.luma * other.luma,
            standard: PhantomData,
        }
    }
}

impl<S, T> Mul<T> for Luma<S, T>
where
    T: Component + Mul,
    S: LumaStandard<TransferFn = LinearFn>,
    <T as Mul>::Output: Component,
{
    type Output = Luma<S, <T as Mul>::Output>;

    fn mul(self, c: T) -> Self::Output {
        Luma {
            luma: self.luma * c,
            standard: PhantomData,
        }
    }
}

impl<S, T> Div<Luma<S, T>> for Luma<S, T>
where
    T: Component + Div,
    S: LumaStandard<TransferFn = LinearFn>,
    <T as Div>::Output: Component,
{
    type Output = Luma<S, <T as Div>::Output>;

    fn div(self, other: Luma<S, T>) -> Self::Output {
        Luma {
            luma: self.luma / other.luma,
            standard: PhantomData,
        }
    }
}

impl<S, T> Div<T> for Luma<S, T>
where
    T: Component + Div,
    S: LumaStandard<TransferFn = LinearFn>,
    <T as Div>::Output: Component,
{
    type Output = Luma<S, <T as Div>::Output>;

    fn div(self, c: T) -> Self::Output {
        Luma {
            luma: self.luma / c,
            standard: PhantomData,
        }
    }
}

impl<S, T, P> AsRef<P> for Luma<S, T>
where
    T: Component,
    S: LumaStandard,
    P: RawPixel<T> + ?Sized,
{
    /// Convert to a raw pixel format.
    ///
    /// ```rust
    /// use palette::SrgbLuma;
    ///
    /// let luma = SrgbLuma::new(100);
    /// let raw: &[u8] = luma.as_ref();
    ///
    /// assert_eq!(raw[0], 100);
    /// ```
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<S, T, P> AsMut<P> for Luma<S, T>
where
    T: Component,
    S: LumaStandard,
    P: RawPixel<T> + ?Sized,
{
    /// Convert to a raw pixel format.
    ///
    /// ```rust
    /// use palette::SrgbLuma;
    ///
    /// let mut luma = SrgbLuma::new(100);
    /// {
    ///     let raw: &mut [u8] = luma.as_mut();
    ///     raw[0] = 5;
    /// }
    ///
    /// assert_eq!(luma.luma, 5);
    /// ```
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<S, T> From<Alpha<Luma<S, T>, T>> for Luma<S, T>
where
    T: Component,
    S: LumaStandard,
{
    fn from(color: Alpha<Luma<S, T>, T>) -> Luma<S, T> {
        color.color
    }
}

impl<S, T> ApproxEq for Luma<S, T>
where
    T: Component + ApproxEq,
    T::Epsilon: Copy,
    S: LumaStandard,
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

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.luma.relative_eq(&other.luma, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.luma.ulps_eq(&other.luma, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod test {
    use Luma;
    use encoding::Srgb;

    #[test]
    fn ranges() {
        assert_ranges!{
            Luma<Srgb, f64>;
            limited {
                luma: 0.0 => 1.0
            }
            limited_min {}
            unlimited {}
        }
    }

    raw_pixel_conversion_tests!(Luma<Srgb>: luma);
}
