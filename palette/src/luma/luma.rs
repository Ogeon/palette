use core::any::TypeId;
use core::fmt;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::blend::PreAlpha;
use crate::convert::FromColorUnclamped;
use crate::encoding::linear::LinearFn;
use crate::encoding::pixel::RawPixel;
use crate::encoding::{Linear, Srgb, TransferFn};
use crate::luma::LumaStandard;
use crate::{
    clamp, contrast_ratio, Alpha, Blend, Clamp, Component, ComponentWise, FloatComponent,
    FromComponent, Mix, Pixel, RelativeContrast, Shade, Xyz, Yxy,
};

/// Luminance with an alpha component. See the [`Lumaa` implementation
/// in `Alpha`](crate::Alpha#Lumaa).
pub type Lumaa<S = Srgb, T = f32> = Alpha<Luma<S, T>, T>;

/// Luminance.
///
/// Luma is a purely gray scale color space, which is included more for
/// completeness than anything else, and represents how bright a color is
/// perceived to be. It's basically the `Y` component of [CIE
/// XYZ](crate::Xyz). The lack of any form of hue representation limits
/// the set of operations that can be performed on it.
#[derive(Debug, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "S::WhitePoint",
    component = "T",
    skip_derives(Xyz, Yxy, Luma)
)]
#[repr(C)]
#[doc(alias = "gray")]
#[doc(alias = "grey")]
pub struct Luma<S = Srgb, T = f32>
where
    T: Component,
    S: LumaStandard,
{
    /// The lightness of the color. 0.0 is black and 1.0 is white.
    pub luma: T,

    /// The kind of RGB standard. sRGB is the default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
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

impl<S, T> Luma<S, T>
where
    T: Component,
    S: LumaStandard,
{
    /// Create a luminance color.
    pub fn new(luma: T) -> Luma<S, T> {
        Luma {
            luma,
            standard: PhantomData,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U>(self) -> Luma<S, U>
    where
        U: Component + FromComponent<T>,
    {
        Luma {
            luma: U::from_component(self.luma),
            standard: PhantomData,
        }
    }

    /// Convert from another component type.
    pub fn from_format<U>(color: Luma<S, U>) -> Self
    where
        U: Component,
        T: FromComponent<U>,
    {
        color.into_format()
    }

    /// Convert to a `(luma,)` tuple.
    pub fn into_components(self) -> (T,) {
        (self.luma,)
    }

    /// Convert from a `(luma,)` tuple.
    pub fn from_components((luma,): (T,)) -> Self {
        Self::new(luma)
    }

    /// Return the `luma` value minimum.
    pub fn min_luma() -> T {
        T::zero()
    }

    /// Return the `luma` value maximum.
    pub fn max_luma() -> T {
        T::max_intensity()
    }

    fn reinterpret_as<S2: LumaStandard<WhitePoint = S::WhitePoint>>(&self) -> Luma<S2, T> {
        Luma {
            luma: self.luma,
            standard: PhantomData,
        }
    }
}

impl<S, T> Luma<S, T>
where
    T: FloatComponent,
    S: LumaStandard,
{
    /// Convert the color to linear luminance.
    pub fn into_linear(self) -> Luma<Linear<S::WhitePoint>, T> {
        Luma::new(S::TransferFn::into_linear(self.luma))
    }

    /// Convert linear luminance to non-linear luminance.
    pub fn from_linear(color: Luma<Linear<S::WhitePoint>, T>) -> Luma<S, T> {
        Luma::new(S::TransferFn::from_linear(color.luma))
    }

    /// Convert the color to a different encoding.
    pub fn into_encoding<St: LumaStandard<WhitePoint = S::WhitePoint>>(self) -> Luma<St, T> {
        Luma::new(St::TransferFn::from_linear(S::TransferFn::into_linear(
            self.luma,
        )))
    }

    /// Convert luminance from a different encoding.
    pub fn from_encoding<St: LumaStandard<WhitePoint = S::WhitePoint>>(
        color: Luma<St, T>,
    ) -> Luma<S, T> {
        Luma::new(S::TransferFn::from_linear(St::TransferFn::into_linear(
            color.luma,
        )))
    }
}

impl<S, T> PartialEq for Luma<S, T>
where
    T: Component + PartialEq,
    S: LumaStandard,
{
    fn eq(&self, other: &Self) -> bool {
        self.luma == other.luma
    }
}

impl<S, T> Eq for Luma<S, T>
where
    T: Component + Eq,
    S: LumaStandard,
{
}

///<span id="Lumaa"></span>[`Lumaa`](crate::luma::Lumaa) implementations.
impl<S, T, A> Alpha<Luma<S, T>, A>
where
    T: Component,
    A: Component,
    S: LumaStandard,
{
    /// Create a luminance color with transparency.
    pub fn new(luma: T, alpha: A) -> Self {
        Alpha {
            color: Luma::new(luma),
            alpha,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U, B>(self) -> Alpha<Luma<S, U>, B>
    where
        U: Component + FromComponent<T>,
        B: Component + FromComponent<A>,
    {
        Alpha::<Luma<S, U>, B>::new(U::from_component(self.luma), B::from_component(self.alpha))
    }

    /// Convert from another component type.
    pub fn from_format<U, B>(color: Alpha<Luma<S, U>, B>) -> Self
    where
        T: FromComponent<U>,
        U: Component,
        A: FromComponent<B>,
        B: Component,
    {
        color.into_format()
    }

    /// Convert to a `(luma, alpha)` tuple.
    pub fn into_components(self) -> (T, A) {
        (self.luma, self.alpha)
    }

    /// Convert from a `(luma, alpha)` tuple.
    pub fn from_components((luma, alpha): (T, A)) -> Self {
        Self::new(luma, alpha)
    }
}

///<span id="Lumaa"></span>[`Lumaa`](crate::luma::Lumaa) implementations.
impl<S, T, A> Alpha<Luma<S, T>, A>
where
    T: FloatComponent,
    A: Component,
    S: LumaStandard,
{
    /// Convert the color to linear luminance with transparency.
    pub fn into_linear(self) -> Alpha<Luma<Linear<S::WhitePoint>, T>, A> {
        Alpha::<Luma<Linear<S::WhitePoint>, T>, A>::new(
            S::TransferFn::into_linear(self.luma),
            self.alpha,
        )
    }

    /// Convert linear luminance to non-linear luminance with transparency.
    pub fn from_linear(color: Alpha<Luma<Linear<S::WhitePoint>, T>, A>) -> Alpha<Luma<S, T>, A> {
        Alpha::<Luma<S, T>, A>::new(S::TransferFn::from_linear(color.luma), color.alpha)
    }

    /// Convert the color to a different encoding with transparency.
    pub fn into_encoding<St: LumaStandard<WhitePoint = S::WhitePoint>>(
        self,
    ) -> Alpha<Luma<St, T>, A> {
        Alpha::<Luma<St, T>, A>::new(
            St::TransferFn::from_linear(S::TransferFn::into_linear(self.luma)),
            self.alpha,
        )
    }

    /// Convert luminance from a different encoding with transparency.
    pub fn from_encoding<St: LumaStandard<WhitePoint = S::WhitePoint>>(
        color: Alpha<Luma<St, T>, A>,
    ) -> Alpha<Luma<S, T>, A> {
        Alpha::<Luma<S, T>, A>::new(
            S::TransferFn::from_linear(St::TransferFn::into_linear(color.luma)),
            color.alpha,
        )
    }
}

impl<S1, S2, T> FromColorUnclamped<Luma<S2, T>> for Luma<S1, T>
where
    S1: LumaStandard,
    S2: LumaStandard<WhitePoint = S1::WhitePoint>,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Luma<S2, T>) -> Self {
        if TypeId::of::<S1>() == TypeId::of::<S2>() {
            color.reinterpret_as()
        } else {
            Self::from_linear(color.into_linear().reinterpret_as())
        }
    }
}

impl<S, T> FromColorUnclamped<Xyz<S::WhitePoint, T>> for Luma<S, T>
where
    S: LumaStandard,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Xyz<S::WhitePoint, T>) -> Self {
        Self::from_linear(Luma {
            luma: color.y,
            standard: PhantomData,
        })
    }
}

impl<S, T> FromColorUnclamped<Yxy<S::WhitePoint, T>> for Luma<S, T>
where
    S: LumaStandard,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Yxy<S::WhitePoint, T>) -> Self {
        Self::from_linear(Luma {
            luma: color.luma,
            standard: PhantomData,
        })
    }
}

impl<S: LumaStandard, T: Component> From<(T,)> for Luma<S, T> {
    fn from(components: (T,)) -> Self {
        Self::from_components(components)
    }
}

impl<S: LumaStandard, T: Component> Into<(T,)> for Luma<S, T> {
    fn into(self) -> (T,) {
        self.into_components()
    }
}

impl<S: LumaStandard, T: Component, A: Component> From<(T, A)> for Alpha<Luma<S, T>, A> {
    fn from(components: (T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<S: LumaStandard, T: Component, A: Component> Into<(T, A)> for Alpha<Luma<S, T>, A> {
    fn into(self) -> (T, A) {
        self.into_components()
    }
}

impl<S, T> Clamp for Luma<S, T>
where
    T: Component,
    S: LumaStandard,
{
    fn is_within_bounds(&self) -> bool {
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
    T: FloatComponent,
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
    T: FloatComponent,
    S: LumaStandard<TransferFn = LinearFn>,
{
    type Scalar = T;

    fn lighten(&self, factor: T) -> Luma<S, T> {
        let difference = if factor >= T::zero() {
            T::max_intensity() - self.luma
        } else {
            self.luma
        };

        let delta = difference.max(T::zero()) * factor;

        Luma {
            luma: (self.luma + delta).max(T::zero()),
            standard: PhantomData,
        }
    }

    fn lighten_fixed(&self, amount: T) -> Luma<S, T> {
        Luma {
            luma: (self.luma + T::max_intensity() * amount).max(T::zero()),
            standard: PhantomData,
        }
    }
}

impl<S, T> Blend for Luma<S, T>
where
    T: FloatComponent,
    S: LumaStandard<TransferFn = LinearFn>,
{
    type Color = Luma<S, T>;

    fn into_premultiplied(self) -> PreAlpha<Luma<S, T>, T> {
        Lumaa {
            color: self,
            alpha: T::one(),
        }
        .into_premultiplied()
    }

    fn from_premultiplied(color: PreAlpha<Luma<S, T>, T>) -> Self {
        Lumaa::from_premultiplied(color).color
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

impl<S, T> AddAssign<Luma<S, T>> for Luma<S, T>
where
    T: Component + AddAssign,
    S: LumaStandard<TransferFn = LinearFn>,
{
    fn add_assign(&mut self, other: Luma<S, T>) {
        self.luma += other.luma;
    }
}

impl<S, T> AddAssign<T> for Luma<S, T>
where
    T: Component + AddAssign,
    S: LumaStandard<TransferFn = LinearFn>,
{
    fn add_assign(&mut self, c: T) {
        self.luma += c;
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

impl<S, T> SubAssign<Luma<S, T>> for Luma<S, T>
where
    T: Component + SubAssign,
    S: LumaStandard<TransferFn = LinearFn>,
{
    fn sub_assign(&mut self, other: Luma<S, T>) {
        self.luma -= other.luma;
    }
}

impl<S, T> SubAssign<T> for Luma<S, T>
where
    T: Component + SubAssign,
    S: LumaStandard<TransferFn = LinearFn>,
{
    fn sub_assign(&mut self, c: T) {
        self.luma -= c;
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

impl<S, T> MulAssign<Luma<S, T>> for Luma<S, T>
where
    T: Component + MulAssign,
    S: LumaStandard<TransferFn = LinearFn>,
{
    fn mul_assign(&mut self, other: Luma<S, T>) {
        self.luma *= other.luma;
    }
}

impl<S, T> MulAssign<T> for Luma<S, T>
where
    T: Component + MulAssign,
    S: LumaStandard<TransferFn = LinearFn>,
{
    fn mul_assign(&mut self, c: T) {
        self.luma *= c;
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

impl<S, T> DivAssign<Luma<S, T>> for Luma<S, T>
where
    T: Component + DivAssign,
    S: LumaStandard<TransferFn = LinearFn>,
{
    fn div_assign(&mut self, other: Luma<S, T>) {
        self.luma /= other.luma;
    }
}

impl<S, T> DivAssign<T> for Luma<S, T>
where
    T: Component + DivAssign,
    S: LumaStandard<TransferFn = LinearFn>,
{
    fn div_assign(&mut self, c: T) {
        self.luma /= c;
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

impl<S, T> AbsDiffEq for Luma<S, T>
where
    T: Component + AbsDiffEq,
    T::Epsilon: Copy,
    S: LumaStandard + PartialEq,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.luma.abs_diff_eq(&other.luma, epsilon)
    }
}

impl<S, T> RelativeEq for Luma<S, T>
where
    T: Component + RelativeEq,
    T::Epsilon: Copy,
    S: LumaStandard + PartialEq,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.luma.relative_eq(&other.luma, epsilon, max_relative)
    }
}

impl<S, T> UlpsEq for Luma<S, T>
where
    T: Component + UlpsEq,
    T::Epsilon: Copy,
    S: LumaStandard + PartialEq,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.luma.ulps_eq(&other.luma, epsilon, max_ulps)
    }
}

impl<S, T> fmt::LowerHex for Luma<S, T>
where
    T: Component + fmt::LowerHex,
    S: LumaStandard,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = f.width().unwrap_or(::core::mem::size_of::<T>() * 2);
        write!(f, "{:0width$x}", self.luma, width = size)
    }
}

impl<S, T> fmt::UpperHex for Luma<S, T>
where
    T: Component + fmt::UpperHex,
    S: LumaStandard,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = f.width().unwrap_or(::core::mem::size_of::<T>() * 2);
        write!(f, "{:0width$X}", self.luma, width = size)
    }
}

impl<S, T> RelativeContrast for Luma<S, T>
where
    T: FloatComponent,
    S: LumaStandard,
{
    type Scalar = T;

    fn get_contrast_ratio(&self, other: &Self) -> T {
        let luma1 = self.into_linear();
        let luma2 = other.into_linear();

        contrast_ratio(luma1.luma, luma2.luma)
    }
}

#[cfg(feature = "random")]
impl<S, T> Distribution<Luma<S, T>> for Standard
where
    T: Component,
    S: LumaStandard,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Luma<S, T> {
        Luma {
            luma: rng.gen(),
            standard: PhantomData,
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformLuma<S, T>
where
    T: Component + SampleUniform,
    S: LumaStandard,
{
    luma: Uniform<T>,
    standard: PhantomData<S>,
}

#[cfg(feature = "random")]
impl<S, T> SampleUniform for Luma<S, T>
where
    T: Component + SampleUniform,
    S: LumaStandard,
{
    type Sampler = UniformLuma<S, T>;
}

#[cfg(feature = "random")]
impl<S, T> UniformSampler for UniformLuma<S, T>
where
    T: Component + SampleUniform,
    S: LumaStandard,
{
    type X = Luma<S, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        UniformLuma {
            luma: Uniform::new::<_, T>(low.luma, high.luma),
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

        UniformLuma {
            luma: Uniform::new_inclusive::<_, T>(low.luma, high.luma),
            standard: PhantomData,
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Luma<S, T> {
        Luma {
            luma: self.luma.sample(rng),
            standard: PhantomData,
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<S, T> bytemuck::Zeroable for Luma<S, T>
where
    S: LumaStandard,
    T: Component + bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<S, T> bytemuck::Pod for Luma<S, T>
where
    S: LumaStandard,
    T: Component + bytemuck::Pod,
{
}

#[cfg(test)]
mod test {
    use crate::encoding::Srgb;
    use crate::Luma;

    #[test]
    fn ranges() {
        assert_ranges! {
            Luma<Srgb, f64>;
            clamped {
                luma: 0.0 => 1.0
            }
            clamped_min {}
            unclamped {}
        }
    }

    raw_pixel_conversion_tests!(Luma<Srgb>: luma);

    #[test]
    fn lower_hex() {
        assert_eq!(format!("{:x}", Luma::<Srgb, u8>::new(161)), "a1");
    }

    #[test]
    fn lower_hex_small_numbers() {
        assert_eq!(format!("{:x}", Luma::<Srgb, u8>::new(1)), "01");
        assert_eq!(format!("{:x}", Luma::<Srgb, u16>::new(1)), "0001");
        assert_eq!(format!("{:x}", Luma::<Srgb, u32>::new(1)), "00000001");
        assert_eq!(
            format!("{:x}", Luma::<Srgb, u64>::new(1)),
            "0000000000000001"
        );
    }

    #[test]
    fn lower_hex_custom_width() {
        assert_eq!(format!("{:03x}", Luma::<Srgb, u8>::new(1)), "001");
        assert_eq!(format!("{:03x}", Luma::<Srgb, u16>::new(1)), "001");
        assert_eq!(format!("{:03x}", Luma::<Srgb, u32>::new(1)), "001");
        assert_eq!(format!("{:03x}", Luma::<Srgb, u64>::new(1)), "001");
    }

    #[test]
    fn upper_hex() {
        assert_eq!(format!("{:X}", Luma::<Srgb, u8>::new(161)), "A1");
    }

    #[test]
    fn upper_hex_small_numbers() {
        assert_eq!(format!("{:X}", Luma::<Srgb, u8>::new(1)), "01");
        assert_eq!(format!("{:X}", Luma::<Srgb, u16>::new(1)), "0001");
        assert_eq!(format!("{:X}", Luma::<Srgb, u32>::new(1)), "00000001");
        assert_eq!(
            format!("{:X}", Luma::<Srgb, u64>::new(1)),
            "0000000000000001"
        );
    }

    #[test]
    fn upper_hex_custom_width() {
        assert_eq!(format!("{:03X}", Luma::<Srgb, u8>::new(1)), "001");
        assert_eq!(format!("{:03X}", Luma::<Srgb, u16>::new(1)), "001");
        assert_eq!(format!("{:03X}", Luma::<Srgb, u32>::new(1)), "001");
        assert_eq!(format!("{:03X}", Luma::<Srgb, u64>::new(1)), "001");
    }

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Luma::<Srgb, f32>::min_luma(), 0.0);
        assert_relative_eq!(Luma::<Srgb, f32>::max_luma(), 1.0);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Luma::<Srgb>::new(0.3)).unwrap();

        assert_eq!(serialized, r#"{"luma":0.3}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Luma<Srgb> = ::serde_json::from_str(r#"{"luma":0.3}"#).unwrap();

        assert_eq!(deserialized, Luma::<Srgb>::new(0.3));
    }

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Luma<Srgb, f32> {
            luma: (0.0, 1.0)
        },
        min: Luma::new(0.0f32),
        max: Luma::new(1.0)
    }
}
