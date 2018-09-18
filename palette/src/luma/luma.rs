use core::fmt;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use float::Float;

use blend::PreAlpha;
use clamp;
use encoding::linear::LinearFn;
use encoding::pixel::RawPixel;
use encoding::{Linear, Srgb, TransferFn};
use luma::LumaStandard;
use white_point::WhitePoint;
use {Alpha, Xyz, Yxy};
use {Blend, Component, ComponentWise, FromColor, IntoColor, Limited, Mix, Pixel, Shade};

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
#[derive(Debug, PartialEq, FromColor, Pixel)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette_internal]
#[palette_white_point = "S::WhitePoint"]
#[palette_component = "T"]
#[palette_manual_from(Xyz, Yxy, Luma = "from_linear")]
#[repr(C)]
pub struct Luma<S = Srgb, T = f32>
where
    T: Component,
    S: LumaStandard,
{
    ///The lightness of the color. 0.0 is black and 1.0 is white.
    pub luma: T,

    /// The kind of RGB standard. sRGB is the default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette_unsafe_zero_sized]
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

    /// Convert to a `(luma,)` tuple.
    pub fn into_components(self) -> (T,) {
        (self.luma,)
    }

    /// Convert from a `(luma,)` tuple.
    pub fn from_components((luma,): (T,)) -> Self {
        Self::new(luma)
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

///<span id="Lumaa"></span>[`Lumaa`](type.Lumaa.html) implementations.
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
            alpha: alpha,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U: Component, B: Component>(self) -> Alpha<Luma<S, U>, B> {
        Alpha::<Luma<S, U>, B>::new(self.luma.convert(), self.alpha.convert())
    }

    /// Convert from another component type.
    pub fn from_format<U: Component, B: Component>(color: Alpha<Luma<S, U>, B>) -> Self {
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

///<span id="Lumaa"></span>[`Lumaa`](type.Lumaa.html) implementations.
impl<S, T, A> Alpha<Luma<S, T>, A>
where
    T: Component + Float,
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

    /// Convert linear luminance to nonlinear luminance with transparency.
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

impl<S, T> From<Xyz<S::WhitePoint, T>> for Luma<S, T>
where
    S: LumaStandard,
    T: Component + Float,
{
    fn from(color: Xyz<S::WhitePoint, T>) -> Self {
        Self::from_linear(Luma {
            luma: color.y,
            standard: PhantomData,
        })
    }
}

impl<S, T> From<Yxy<S::WhitePoint, T>> for Luma<S, T>
where
    S: LumaStandard,
    T: Component + Float,
{
    fn from(color: Yxy<S::WhitePoint, T>) -> Self {
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

#[cfg(test)]
mod test {
    use encoding::Srgb;
    use Luma;

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
}
