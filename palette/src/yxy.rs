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
pub struct Yxy<Wp = D65, T = f32> {
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

impl<Wp, T> Copy for Yxy<Wp, T> where T: Copy {}

impl<Wp, T> Clone for Yxy<Wp, T>
where
    T: Clone,
{
    fn clone(&self) -> Yxy<Wp, T> {
        Yxy {
            x: self.x.clone(),
            y: self.y.clone(),
            luma: self.luma.clone(),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Yxy<Wp, T> {
    /// Create a CIE Yxy color.
    pub const fn new(x: T, y: T, luma: T) -> Yxy<Wp, T> {
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
        Self::new(x, y, luma)
    }

    /// Changes the reference white point without changing the color value.
    ///
    /// This function doesn't change the numerical values, and thus the color it
    /// represents in an absolute sense. However, the appearance of the color
    /// may not be the same when observed with the new white point. The effect
    /// would be similar to taking a photo with an incorrect white balance.
    ///
    /// See [chromatic_adaptation](crate::chromatic_adaptation) for operations
    /// that can change the white point while preserving the color's appearance.
    #[inline]
    pub fn with_white_point<NewWp>(self) -> Yxy<NewWp, T> {
        Yxy::new(self.x, self.y, self.luma)
    }
}

impl<Wp, T> Yxy<Wp, T>
where
    T: Component,
{
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

///<span id="Yxya"></span>[`Yxya`](crate::Yxya) implementations.
impl<Wp, T, A> Alpha<Yxy<Wp, T>, A> {
    /// Create a CIE Yxy color with transparency.
    pub const fn new(x: T, y: T, luma: T, alpha: A) -> Self {
        Alpha {
            color: Yxy::new(x, y, luma),
            alpha,
        }
    }

    /// Convert to a `(x, y, luma)`, a.k.a. `(x, y, Y)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (self.color.x, self.color.y, self.color.luma, self.alpha)
    }

    /// Convert from a `(x, y, luma)`, a.k.a. `(x, y, Y)` tuple.
    pub fn from_components((x, y, luma, alpha): (T, T, T, A)) -> Self {
        Self::new(x, y, luma, alpha)
    }

    /// Changes the reference white point without changing the color value.
    ///
    /// This function doesn't change the numerical values, and thus the color it
    /// represents in an absolute sense. However, the appearance of the color
    /// may not be the same when observed with the new white point. The effect
    /// would be similar to taking a photo with an incorrect white balance.
    ///
    /// See [chromatic_adaptation](crate::chromatic_adaptation) for operations
    /// that can change the white point while preserving the color's appearance.
    #[inline]
    pub fn with_white_point<NewWp>(self) -> Alpha<Yxy<NewWp, T>, A> {
        Alpha::<Yxy<NewWp, T>, A>::new(self.color.x, self.color.y, self.color.luma, self.alpha)
    }
}

impl<Wp, T> From<(T, T, T)> for Yxy<Wp, T> {
    fn from(components: (T, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp, T> From<Yxy<Wp, T>> for (T, T, T) {
    fn from(color: Yxy<Wp, T>) -> (T, T, T) {
        color.into_components()
    }
}

impl<Wp, T, A> From<(T, T, T, A)> for Alpha<Yxy<Wp, T>, A> {
    fn from(components: (T, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp, T, A> From<Alpha<Yxy<Wp, T>, A>> for (T, T, T, A) {
    fn from(color: Alpha<Yxy<Wp, T>, A>) -> (T, T, T, A) {
        color.into_components()
    }
}

impl<Wp, T> FromColorUnclamped<Yxy<Wp, T>> for Yxy<Wp, T> {
    fn from_color_unclamped(color: Yxy<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T> FromColorUnclamped<Xyz<Wp, T>> for Yxy<Wp, T>
where
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
    S: LumaStandard<T>,
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
{
    #[rustfmt::skip]
    #[inline]
    fn is_within_bounds(&self) -> bool {
        self.x >= Self::min_x() && self.x <= Self::max_x() &&
        self.y >= Self::min_y() && self.y <= Self::max_y() &&
        self.luma >= Self::min_luma() && self.luma <= Self::max_luma()
    }

    #[inline]
    fn clamp(self) -> Self {
        Self::new(
            clamp(self.x, Self::min_x(), Self::max_x()),
            clamp(self.y, Self::min_y(), Self::max_y()),
            clamp(self.luma, Self::min_luma(), Self::max_luma()),
        )
    }
}

impl<Wp, T> Mix for Yxy<Wp, T>
where
    T: FloatComponent,
{
    type Scalar = T;

    #[inline]
    fn mix(self, other: Yxy<Wp, T>, factor: T) -> Yxy<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());
        self + (other - self) * factor
    }
}

impl<Wp, T> Shade for Yxy<Wp, T>
where
    T: FloatComponent,
{
    type Scalar = T;

    #[inline]
    fn lighten(self, factor: T) -> Yxy<Wp, T> {
        let difference = if factor >= T::zero() {
            Self::max_luma() - self.luma
        } else {
            self.luma
        };

        let delta = difference.max(T::zero()) * factor;

        Yxy {
            x: self.x,
            y: self.y,
            luma: (self.luma + delta).max(Self::min_luma()),
            white_point: PhantomData,
        }
    }

    #[inline]
    fn lighten_fixed(self, amount: T) -> Yxy<Wp, T> {
        Yxy {
            x: self.x,
            y: self.y,
            luma: (self.luma + Self::max_luma() * amount).max(Self::min_luma()),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> ComponentWise for Yxy<Wp, T>
where
    T: Clone,
{
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Yxy<Wp, T>, mut f: F) -> Yxy<Wp, T> {
        Yxy {
            x: f(self.x.clone(), other.x.clone()),
            y: f(self.y.clone(), other.y.clone()),
            luma: f(self.luma.clone(), other.luma.clone()),
            white_point: PhantomData,
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Yxy<Wp, T> {
        Yxy {
            x: f(self.x.clone()),
            y: f(self.y.clone()),
            luma: f(self.luma.clone()),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Yxy<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint<T>,
{
    fn default() -> Yxy<Wp, T> {
        // The default for x and y are the white point x and y ( from the default D65).
        // Since Y (luma) is 0.0, this makes the default color black just like for
        // other colors. The reason for not using 0 for x and y is that this
        // outside the usual color gamut and might cause scaling issues.
        Yxy {
            luma: T::zero(),
            ..Wp::get_xyz().with_white_point().into_color_unclamped()
        }
    }
}

impl_color_add!(Yxy<Wp, T>, [x, y, luma], white_point);
impl_color_sub!(Yxy<Wp, T>, [x, y, luma], white_point);
impl_color_mul!(Yxy<Wp, T>, [x, y, luma], white_point);
impl_color_div!(Yxy<Wp, T>, [x, y, luma], white_point);

impl<Wp, T, P> AsRef<P> for Yxy<Wp, T>
where
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<Wp, T, P> AsMut<P> for Yxy<Wp, T>
where
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<Wp, T> RelativeContrast for Yxy<Wp, T>
where
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
    T: SampleUniform,
{
    x: Uniform<T>,
    y: Uniform<T>,
    luma: Uniform<T>,
    white_point: PhantomData<Wp>,
}

#[cfg(feature = "random")]
impl<Wp, T> SampleUniform for Yxy<Wp, T>
where
    T: Clone + SampleUniform,
{
    type Sampler = UniformYxy<Wp, T>;
}

#[cfg(feature = "random")]
impl<Wp, T> UniformSampler for UniformYxy<Wp, T>
where
    T: Clone + SampleUniform,
{
    type X = Yxy<Wp, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow();
        let high = high_b.borrow();

        UniformYxy {
            x: Uniform::new::<_, T>(low.x.clone(), high.x.clone()),
            y: Uniform::new::<_, T>(low.y.clone(), high.y.clone()),
            luma: Uniform::new::<_, T>(low.luma.clone(), high.luma.clone()),
            white_point: PhantomData,
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow();
        let high = high_b.borrow();

        UniformYxy {
            x: Uniform::new_inclusive::<_, T>(low.x.clone(), high.x.clone()),
            y: Uniform::new_inclusive::<_, T>(low.y.clone(), high.y.clone()),
            luma: Uniform::new_inclusive::<_, T>(low.luma.clone(), high.luma.clone()),
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
unsafe impl<Wp, T> bytemuck::Zeroable for Yxy<Wp, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp: 'static, T> bytemuck::Pod for Yxy<Wp, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use super::Yxy;
    use crate::white_point::D65;
    use crate::{FromColor, LinLuma, LinSrgb};

    #[test]
    fn luma() {
        let a = Yxy::<D65>::from_color(LinLuma::new(0.5));
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
        let serialized = ::serde_json::to_string(&Yxy::<D65>::new(0.3, 0.8, 0.1)).unwrap();

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
