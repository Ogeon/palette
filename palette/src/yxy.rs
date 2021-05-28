use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::convert::{FromColorUnclamped, IntoColorUnclamped};
use crate::encoding::pixel::RawPixel;
use crate::luma::LumaStandard;
use crate::white_point::{WhitePoint, D65};
use crate::{
    clamp, contrast_ratio, Alpha, Clamp, Component, ComponentWise, FloatComponent, Luma, Mix,
    Pixel, RelativeContrast, Shade, Xyz,
};

/// CIE 1931 Yxy (xyY) with an alpha component. See the [`Yxya` implementation
/// in `Alpha`](crate::Alpha#Yxya).
pub type Yxya<Wp = D65, T = f32> = Alpha<Yxy<Wp, T>, T>;

/// The CIE 1931 Yxy (xyY)  color space.
///
/// Yxy is a luminance-chromaticity color space derived from the CIE XYZ
/// color space. It is widely used to define colors. The chromaticity diagrams
/// for the color spaces are a plot of this color space's x and y coordinates.
///
/// Conversions and operations on this color space depend on the white point.
#[derive(Debug, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Xyz, Yxy, Luma)
)]
#[repr(C)]
#[doc(alias = "xyY")]
pub struct Yxy<Wp = D65, T = f32>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    /// x chromaticity co-ordinate derived from XYZ color space as X/(X+Y+Z).
    /// Typical range is between 0 and 1
    pub x: T,

    /// y chromaticity co-ordinate derived from XYZ color space as Y/(X+Y+Z).
    /// Typical range is between 0 and 1
    pub y: T,

    /// luma (Y) was a measure of the brightness or luminance of a color.
    /// It is the same as the Y from the XYZ color space. Its range is from
    ///0 to 1, where 0 is black and 1 is white.
    pub luma: T,

    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
}

impl<Wp, T> Clone for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn clone(&self) -> Yxy<Wp, T> {
        *self
    }
}

impl<T> Yxy<D65, T>
where
    T: FloatComponent,
{
    /// CIE Yxy with white point D65.
    pub fn new(x: T, y: T, luma: T) -> Yxy<D65, T> {
        Yxy {
            x,
            y,
            luma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    /// CIE Yxy.
    pub fn with_wp(x: T, y: T, luma: T) -> Yxy<Wp, T> {
        Yxy {
            x,
            y,
            luma,
            white_point: PhantomData,
        }
    }

    /// Convert to a `(x, y, luma)`, a.k.a. `(x, y, Y)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.x, self.y, self.luma)
    }

    /// Convert from a `(x, y, luma)`, a.k.a. `(x, y, Y)` tuple.
    pub fn from_components((x, y, luma): (T, T, T)) -> Self {
        Self::with_wp(x, y, luma)
    }

    /// Return the `x` value minimum.
    pub fn min_x() -> T {
        T::zero()
    }

    /// Return the `x` value maximum.
    pub fn max_x() -> T {
        T::max_intensity()
    }

    /// Return the `y` value minimum.
    pub fn min_y() -> T {
        T::zero()
    }

    /// Return the `y` value maximum.
    pub fn max_y() -> T {
        T::max_intensity()
    }

    /// Return the `luma` value minimum.
    pub fn min_luma() -> T {
        T::zero()
    }

    /// Return the `luma` value maximum.
    pub fn max_luma() -> T {
        T::max_intensity()
    }
}

impl<Wp, T> PartialEq for Yxy<Wp, T>
where
    T: FloatComponent + PartialEq,
    Wp: WhitePoint,
{
    fn eq(&self, other: &Self) -> bool {
        self.luma == other.luma && self.x == other.x && self.y == other.y
    }
}

impl<Wp, T> Eq for Yxy<Wp, T>
where
    T: FloatComponent + Eq,
    Wp: WhitePoint,
{
}

///<span id="Yxya"></span>[`Yxya`](crate::Yxya) implementations.
impl<T, A> Alpha<Yxy<D65, T>, A>
where
    T: FloatComponent,
    A: Component,
{
    /// CIE Yxy and transparency with white point D65.
    pub fn new(x: T, y: T, luma: T, alpha: A) -> Self {
        Alpha {
            color: Yxy::new(x, y, luma),
            alpha,
        }
    }
}
///<span id="Yxya"></span>[`Yxya`](crate::Yxya) implementations.
impl<Wp, T, A> Alpha<Yxy<Wp, T>, A>
where
    T: FloatComponent,
    A: Component,
    Wp: WhitePoint,
{
    /// CIE Yxy and transparency.
    pub fn with_wp(x: T, y: T, luma: T, alpha: A) -> Self {
        Alpha {
            color: Yxy::with_wp(x, y, luma),
            alpha,
        }
    }

    /// Convert to a `(x, y, luma)`, a.k.a. `(x, y, Y)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (self.x, self.y, self.luma, self.alpha)
    }

    /// Convert from a `(x, y, luma)`, a.k.a. `(x, y, Y)` tuple.
    pub fn from_components((x, y, luma, alpha): (T, T, T, A)) -> Self {
        Self::with_wp(x, y, luma, alpha)
    }
}

impl<Wp: WhitePoint, T: FloatComponent> From<(T, T, T)> for Yxy<Wp, T> {
    fn from(components: (T, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent> Into<(T, T, T)> for Yxy<Wp, T> {
    fn into(self) -> (T, T, T) {
        self.into_components()
    }
}

impl<Wp: WhitePoint, T: FloatComponent, A: Component> From<(T, T, T, A)> for Alpha<Yxy<Wp, T>, A> {
    fn from(components: (T, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent, A: Component> Into<(T, T, T, A)> for Alpha<Yxy<Wp, T>, A> {
    fn into(self) -> (T, T, T, A) {
        self.into_components()
    }
}

impl<Wp, T> FromColorUnclamped<Yxy<Wp, T>> for Yxy<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Yxy<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T> FromColorUnclamped<Xyz<Wp, T>> for Yxy<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    fn from_color_unclamped(xyz: Xyz<Wp, T>) -> Self {
        let mut yxy = Yxy {
            x: T::zero(),
            y: T::zero(),
            luma: xyz.y,
            white_point: PhantomData,
        };
        let sum = xyz.x + xyz.y + xyz.z;
        // If denominator is zero, NAN or INFINITE leave x and y at the default 0
        if sum.is_normal() {
            yxy.x = xyz.x / sum;
            yxy.y = xyz.y / sum;
        }
        yxy
    }
}

impl<T, S> FromColorUnclamped<Luma<S, T>> for Yxy<S::WhitePoint, T>
where
    T: FloatComponent,
    S: LumaStandard,
{
    fn from_color_unclamped(luma: Luma<S, T>) -> Self {
        Yxy {
            luma: luma.into_linear().luma,
            ..Default::default()
        }
    }
}

impl<Wp, T> Clamp for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    #[rustfmt::skip]
    fn is_within_bounds(&self) -> bool {
        self.x >= T::zero() && self.x <= T::one() &&
        self.y >= T::zero() && self.y <= T::one() &&
        self.luma >= T::zero() && self.luma <= T::one()
    }

    fn clamp(&self) -> Yxy<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.x = clamp(self.x, T::zero(), T::one());
        self.y = clamp(self.y, T::zero(), T::one());
        self.luma = clamp(self.luma, T::zero(), T::one());
    }
}

impl<Wp, T> Mix for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn mix(&self, other: &Yxy<Wp, T>, factor: T) -> Yxy<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());

        Yxy {
            x: self.x + factor * (other.x - self.x),
            y: self.y + factor * (other.y - self.y),
            luma: self.luma + factor * (other.luma - self.luma),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn lighten(&self, factor: T) -> Yxy<Wp, T> {
        let difference = if factor >= T::zero() {
            T::max_intensity() - self.luma
        } else {
            self.luma
        };

        let delta = difference.max(T::zero()) * factor;

        Yxy {
            x: self.x,
            y: self.y,
            luma: (self.luma + delta).max(T::zero()),
            white_point: PhantomData,
        }
    }

    fn lighten_fixed(&self, amount: T) -> Yxy<Wp, T> {
        Yxy {
            x: self.x,
            y: self.y,
            luma: (self.luma + T::max_intensity() * amount).max(T::zero()),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> ComponentWise for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Yxy<Wp, T>, mut f: F) -> Yxy<Wp, T> {
        Yxy {
            x: f(self.x, other.x),
            y: f(self.y, other.y),
            luma: f(self.luma, other.luma),
            white_point: PhantomData,
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Yxy<Wp, T> {
        Yxy {
            x: f(self.x),
            y: f(self.y),
            luma: f(self.luma),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn default() -> Yxy<Wp, T> {
        // The default for x and y are the white point x and y ( from the default D65).
        // Since Y (luma) is 0.0, this makes the default color black just like for
        // other colors. The reason for not using 0 for x and y is that this
        // outside the usual color gamut and might cause scaling issues.
        Yxy {
            luma: T::zero(),
            ..Wp::get_xyz().into_color_unclamped()
        }
    }
}

impl<Wp, T> Add<Yxy<Wp, T>> for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Yxy<Wp, T>;

    fn add(self, other: Yxy<Wp, T>) -> Self::Output {
        Yxy {
            x: self.x + other.x,
            y: self.y + other.y,
            luma: self.luma + other.luma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Add<T> for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Yxy<Wp, T>;

    fn add(self, c: T) -> Self::Output {
        Yxy {
            x: self.x + c,
            y: self.y + c,
            luma: self.luma + c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> AddAssign<Yxy<Wp, T>> for Yxy<Wp, T>
where
    T: FloatComponent + AddAssign,
    Wp: WhitePoint,
{
    fn add_assign(&mut self, other: Yxy<Wp, T>) {
        self.x += other.x;
        self.y += other.y;
        self.luma += other.luma;
    }
}

impl<Wp, T> AddAssign<T> for Yxy<Wp, T>
where
    T: FloatComponent + AddAssign,
    Wp: WhitePoint,
{
    fn add_assign(&mut self, c: T) {
        self.x += c;
        self.y += c;
        self.luma += c;
    }
}

impl<Wp, T> Sub<Yxy<Wp, T>> for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Yxy<Wp, T>;

    fn sub(self, other: Yxy<Wp, T>) -> Self::Output {
        Yxy {
            x: self.x - other.x,
            y: self.y - other.y,
            luma: self.luma - other.luma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<T> for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Yxy<Wp, T>;

    fn sub(self, c: T) -> Self::Output {
        Yxy {
            x: self.x - c,
            y: self.y - c,
            luma: self.luma - c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> SubAssign<Yxy<Wp, T>> for Yxy<Wp, T>
where
    T: FloatComponent + SubAssign,
    Wp: WhitePoint,
{
    fn sub_assign(&mut self, other: Yxy<Wp, T>) {
        self.x -= other.x;
        self.y -= other.y;
        self.luma -= other.luma;
    }
}

impl<Wp, T> SubAssign<T> for Yxy<Wp, T>
where
    T: FloatComponent + SubAssign,
    Wp: WhitePoint,
{
    fn sub_assign(&mut self, c: T) {
        self.x -= c;
        self.y -= c;
        self.luma -= c;
    }
}

impl<Wp, T> Mul<Yxy<Wp, T>> for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Yxy<Wp, T>;

    fn mul(self, other: Yxy<Wp, T>) -> Self::Output {
        Yxy {
            x: self.x * other.x,
            y: self.y * other.y,
            luma: self.luma * other.luma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Mul<T> for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Yxy<Wp, T>;

    fn mul(self, c: T) -> Self::Output {
        Yxy {
            x: self.x * c,
            y: self.y * c,
            luma: self.luma * c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> MulAssign<Yxy<Wp, T>> for Yxy<Wp, T>
where
    T: FloatComponent + MulAssign,
    Wp: WhitePoint,
{
    fn mul_assign(&mut self, other: Yxy<Wp, T>) {
        self.x *= other.x;
        self.y *= other.y;
        self.luma *= other.luma;
    }
}

impl<Wp, T> MulAssign<T> for Yxy<Wp, T>
where
    T: FloatComponent + MulAssign,
    Wp: WhitePoint,
{
    fn mul_assign(&mut self, c: T) {
        self.x *= c;
        self.y *= c;
        self.luma *= c;
    }
}

impl<Wp, T> Div<Yxy<Wp, T>> for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Yxy<Wp, T>;

    fn div(self, other: Yxy<Wp, T>) -> Self::Output {
        Yxy {
            x: self.x / other.x,
            y: self.y / other.y,
            luma: self.luma / other.luma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Div<T> for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Yxy<Wp, T>;

    fn div(self, c: T) -> Self::Output {
        Yxy {
            x: self.x / c,
            y: self.y / c,
            luma: self.luma / c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> DivAssign<Yxy<Wp, T>> for Yxy<Wp, T>
where
    T: FloatComponent + DivAssign,
    Wp: WhitePoint,
{
    fn div_assign(&mut self, other: Yxy<Wp, T>) {
        self.x /= other.x;
        self.y /= other.y;
        self.luma /= other.luma;
    }
}

impl<Wp, T> DivAssign<T> for Yxy<Wp, T>
where
    T: FloatComponent + DivAssign,
    Wp: WhitePoint,
{
    fn div_assign(&mut self, c: T) {
        self.x /= c;
        self.y /= c;
        self.luma /= c;
    }
}

impl<Wp, T, P> AsRef<P> for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<Wp, T, P> AsMut<P> for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<Wp, T> RelativeContrast for Yxy<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    type Scalar = T;

    fn get_contrast_ratio(&self, other: &Self) -> T {
        contrast_ratio(self.luma, other.luma)
    }
}

#[cfg(feature = "random")]
impl<Wp, T> Distribution<Yxy<Wp, T>> for Standard
where
    T: FloatComponent,
    Wp: WhitePoint,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Yxy<Wp, T> {
        Yxy {
            x: rng.gen(),
            y: rng.gen(),
            luma: rng.gen(),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformYxy<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    x: Uniform<T>,
    y: Uniform<T>,
    luma: Uniform<T>,
    white_point: PhantomData<Wp>,
}

#[cfg(feature = "random")]
impl<Wp, T> SampleUniform for Yxy<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    type Sampler = UniformYxy<Wp, T>;
}

#[cfg(feature = "random")]
impl<Wp, T> UniformSampler for UniformYxy<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    type X = Yxy<Wp, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        UniformYxy {
            x: Uniform::new::<_, T>(low.x, high.x),
            y: Uniform::new::<_, T>(low.y, high.y),
            luma: Uniform::new::<_, T>(low.luma, high.luma),
            white_point: PhantomData,
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        UniformYxy {
            x: Uniform::new_inclusive::<_, T>(low.x, high.x),
            y: Uniform::new_inclusive::<_, T>(low.y, high.y),
            luma: Uniform::new_inclusive::<_, T>(low.luma, high.luma),
            white_point: PhantomData,
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Yxy<Wp, T> {
        Yxy {
            x: self.x.sample(rng),
            y: self.y.sample(rng),
            luma: self.luma.sample(rng),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Zeroable for Yxy<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent + bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Pod for Yxy<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent + bytemuck::Pod,
{
}

#[cfg(test)]
mod test {
    use super::Yxy;
    use crate::white_point::D65;
    use crate::{FromColor, LinLuma, LinSrgb};

    #[test]
    fn luma() {
        let a = Yxy::from_color(LinLuma::new(0.5));
        let b = Yxy::new(0.312727, 0.329023, 0.5);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn red() {
        let a = Yxy::from_color(LinSrgb::new(1.0, 0.0, 0.0));
        let b = Yxy::new(0.64, 0.33, 0.212673);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn green() {
        let a = Yxy::from_color(LinSrgb::new(0.0, 1.0, 0.0));
        let b = Yxy::new(0.3, 0.6, 0.715152);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn blue() {
        let a = Yxy::from_color(LinSrgb::new(0.0, 0.0, 1.0));
        let b = Yxy::new(0.15, 0.06, 0.072175);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Yxy<D65, f64>;
            clamped {
                x: 0.0 => 1.0,
                y: 0.0 => 1.0,
                luma: 0.0 => 1.0
            }
            clamped_min {}
            unclamped {}
        }
    }

    raw_pixel_conversion_tests!(Yxy<D65>: x, y, luma);
    raw_pixel_conversion_fail_tests!(Yxy<D65>: x, y, luma);

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Yxy::<D65>::min_x(), 0.0);
        assert_relative_eq!(Yxy::<D65>::min_y(), 0.0);
        assert_relative_eq!(Yxy::<D65>::min_luma(), 0.0);
        assert_relative_eq!(Yxy::<D65>::max_x(), 1.0);
        assert_relative_eq!(Yxy::<D65>::max_y(), 1.0);
        assert_relative_eq!(Yxy::<D65>::max_luma(), 1.0);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Yxy::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"x":0.3,"y":0.8,"luma":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Yxy = ::serde_json::from_str(r#"{"x":0.3,"y":0.8,"luma":0.1}"#).unwrap();

        assert_eq!(deserialized, Yxy::new(0.3, 0.8, 0.1));
    }

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Yxy<D65, f32> {
            x: (0.0, 1.0),
            y: (0.0, 1.0),
            luma: (0.0, 1.0)
        },
        min: Yxy::new(0.0f32, 0.0, 0.0),
        max: Yxy::new(1.0, 1.0, 1.0),
    }
}
