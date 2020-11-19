use core::any::TypeId;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::convert::FromColorUnclamped;
use crate::encoding::pixel::RawPixel;
use crate::encoding::Srgb;
use crate::float::Float;
use crate::rgb::{Rgb, RgbSpace, RgbStandard};
use crate::{
    clamp, contrast_ratio, from_f64, Alpha, Component, FloatComponent, FromF64, GetHue, Hsv, Hue,
    Limited, Mix, Pixel, RelativeContrast, RgbHue, Saturate, Shade, Xyz,
};

/// Linear HSL with an alpha component. See the [`Hsla` implementation in
/// `Alpha`](crate::Alpha#Hsla).
pub type Hsla<S = Srgb, T = f32> = Alpha<Hsl<S, T>, T>;

/// HSL color space.
///
/// The HSL color space can be seen as a cylindrical version of
/// [RGB](crate::rgb::Rgb), where the `hue` is the angle around the color
/// cylinder, the `saturation` is the distance from the center, and the
/// `lightness` is the height from the bottom. Its composition makes it
/// especially good for operations like changing green to red, making a color
/// more gray, or making it darker.
///
/// See [HSV](crate::Hsv) for a very similar color space, with brightness
/// instead of lightness.
#[derive(Debug, PartialEq, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    rgb_standard = "S",
    white_point = "<S::Space as RgbSpace>::WhitePoint",
    component = "T",
    skip_derives(Rgb, Hsv, Hsl)
)]
#[repr(C)]
pub struct Hsl<S = Srgb, T = f32>
where
    T: FloatComponent,
    S: RgbStandard,
{
    /// The hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: RgbHue<T>,

    /// The colorfulness of the color. 0.0 gives gray scale colors and 1.0 will
    /// give absolutely clear colors.
    pub saturation: T,

    /// Decides how light the color will look. 0.0 will be black, 0.5 will give
    /// a clear color, and 1.0 will give white.
    pub lightness: T,

    /// The white point and RGB primaries this color is adapted to. The default
    /// is the sRGB standard.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub standard: PhantomData<S>,
}

impl<S, T> Copy for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
}

impl<S, T> Clone for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    fn clone(&self) -> Hsl<S, T> {
        *self
    }
}

impl<T> Hsl<Srgb, T>
where
    T: FloatComponent,
{
    /// HSL for linear sRGB.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T) -> Hsl<Srgb, T> {
        Hsl {
            hue: hue.into(),
            saturation,
            lightness,
            standard: PhantomData,
        }
    }
}

impl<S, T> Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    /// Linear HSL.
    pub fn with_wp<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T) -> Hsl<S, T> {
        Hsl {
            hue: hue.into(),
            saturation,
            lightness,
            standard: PhantomData,
        }
    }

    /// Convert to a `(hue, saturation, lightness)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T) {
        (self.hue, self.saturation, self.lightness)
    }

    /// Convert from a `(hue, saturation, lightness)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>((hue, saturation, lightness): (H, T, T)) -> Self {
        Self::with_wp(hue, saturation, lightness)
    }

    #[inline]
    fn reinterpret_as<St: RgbStandard>(self) -> Hsl<St, T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation,
            lightness: self.lightness,
            standard: PhantomData,
        }
    }

    /// Return the `saturation` value minimum.
    pub fn min_saturation() -> T {
        T::zero()
    }

    /// Return the `saturation` value maximum.
    pub fn max_saturation() -> T {
        T::max_intensity()
    }

    /// Return the `lightness` value minimum.
    pub fn min_lightness() -> T {
        T::zero()
    }

    /// Return the `lightness` value maximum.
    pub fn max_lightness() -> T {
        T::max_intensity()
    }
}

///<span id="Hsla"></span>[`Hsla`](crate::Hsla) implementations.
impl<T, A> Alpha<Hsl<Srgb, T>, A>
where
    T: FloatComponent,
    A: Component,
{
    /// HSL and transparency for linear sRGB.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T, alpha: A) -> Self {
        Alpha {
            color: Hsl::new(hue, saturation, lightness),
            alpha,
        }
    }
}

///<span id="Hsla"></span>[`Hsla`](crate::Hsla) implementations.
impl<S, T, A> Alpha<Hsl<S, T>, A>
where
    T: FloatComponent,
    A: Component,
    S: RgbStandard,
{
    /// Linear HSL and transparency.
    pub fn with_wp<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T, alpha: A) -> Self {
        Alpha {
            color: Hsl::with_wp(hue, saturation, lightness),
            alpha,
        }
    }

    /// Convert to a `(hue, saturation, lightness, alpha)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T, A) {
        (self.hue, self.saturation, self.lightness, self.alpha)
    }

    /// Convert from a `(hue, saturation, lightness, alpha)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>(
        (hue, saturation, lightness, alpha): (H, T, T, A),
    ) -> Self {
        Self::with_wp(hue, saturation, lightness, alpha)
    }
}

impl<S1, S2, T> FromColorUnclamped<Hsl<S1, T>> for Hsl<S2, T>
where
    T: FloatComponent,
    S1: RgbStandard,
    S2: RgbStandard,
    S1::Space: RgbSpace<WhitePoint = <S2::Space as RgbSpace>::WhitePoint>,
{
    fn from_color_unclamped(hsl: Hsl<S1, T>) -> Self {
        if TypeId::of::<S1>() == TypeId::of::<S2>() {
            hsl.reinterpret_as()
        } else {
            let rgb = Rgb::<S1, T>::from_color_unclamped(hsl);
            let converted_rgb = Rgb::<S2, T>::from_color_unclamped(rgb);
            Self::from_color_unclamped(converted_rgb)
        }
    }
}

impl<S, T> FromColorUnclamped<Rgb<S, T>> for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    fn from_color_unclamped(rgb: Rgb<S, T>) -> Self {
        let (max, min, sep, coeff) = {
            let (max, min, sep, coeff) = if rgb.red > rgb.green {
                (rgb.red, rgb.green, rgb.green - rgb.blue, T::zero())
            } else {
                (rgb.green, rgb.red, rgb.blue - rgb.red, from_f64(2.0))
            };
            if rgb.blue > max {
                (rgb.blue, min, rgb.red - rgb.green, from_f64(4.0))
            } else {
                let min_val = if rgb.blue < min { rgb.blue } else { min };
                (max, min_val, sep, coeff)
            }
        };

        let mut h = T::zero();
        let mut s = T::zero();

        let sum = max + min;
        let l = sum / from_f64(2.0);
        if max != min {
            let d = max - min;
            s = if sum > T::one() {
                d / (from_f64::<T>(2.0) - sum)
            } else {
                d / sum
            };
            h = ((sep / d) + coeff) * from_f64(60.0);
        };

        Hsl {
            hue: h.into(),
            saturation: s,
            lightness: l,
            standard: PhantomData,
        }
    }
}

impl<S, T> FromColorUnclamped<Hsv<S, T>> for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    fn from_color_unclamped(hsv: Hsv<S, T>) -> Self {
        let x = (from_f64::<T>(2.0) - hsv.saturation) * hsv.value;
        let saturation = if !hsv.value.is_normal() {
            T::zero()
        } else if x < T::one() {
            if x.is_normal() {
                hsv.saturation * hsv.value / x
            } else {
                T::zero()
            }
        } else {
            let denom = from_f64::<T>(2.0) - x;
            if denom.is_normal() {
                hsv.saturation * hsv.value / denom
            } else {
                T::zero()
            }
        };

        Hsl {
            hue: hsv.hue,
            saturation,
            lightness: x / from_f64(2.0),
            standard: PhantomData,
        }
    }
}

impl<S: RgbStandard, T: FloatComponent, H: Into<RgbHue<T>>> From<(H, T, T)> for Hsl<S, T> {
    fn from(components: (H, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<S: RgbStandard, T: FloatComponent> Into<(RgbHue<T>, T, T)> for Hsl<S, T> {
    fn into(self) -> (RgbHue<T>, T, T) {
        self.into_components()
    }
}

impl<S: RgbStandard, T: FloatComponent, H: Into<RgbHue<T>>, A: Component> From<(H, T, T, A)>
    for Alpha<Hsl<S, T>, A>
{
    fn from(components: (H, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<S: RgbStandard, T: FloatComponent, A: Component> Into<(RgbHue<T>, T, T, A)>
    for Alpha<Hsl<S, T>, A>
{
    fn into(self) -> (RgbHue<T>, T, T, A) {
        self.into_components()
    }
}

impl<S, T> Limited for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    #[rustfmt::skip]
    fn is_valid(&self) -> bool {
        self.saturation >= T::zero() && self.saturation <= T::one() &&
        self.lightness >= T::zero() && self.lightness <= T::one()
    }

    fn clamp(&self) -> Hsl<S, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.saturation = clamp(self.saturation, T::zero(), T::one());
        self.lightness = clamp(self.lightness, T::zero(), T::one());
    }
}

impl<S, T> Mix for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Scalar = T;

    fn mix(&self, other: &Hsl<S, T>, factor: T) -> Hsl<S, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();

        Hsl {
            hue: self.hue + factor * hue_diff,
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            lightness: self.lightness + factor * (other.lightness - self.lightness),
            standard: PhantomData,
        }
    }
}

impl<S, T> Shade for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Hsl<S, T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation,
            lightness: self.lightness + amount,
            standard: PhantomData,
        }
    }
}

impl<S, T> GetHue for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        if self.saturation <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<S, T> Hue for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    fn with_hue<H: Into<Self::Hue>>(&self, hue: H) -> Hsl<S, T> {
        Hsl {
            hue: hue.into(),
            saturation: self.saturation,
            lightness: self.lightness,
            standard: PhantomData,
        }
    }

    fn shift_hue<H: Into<Self::Hue>>(&self, amount: H) -> Hsl<S, T> {
        Hsl {
            hue: self.hue + amount.into(),
            saturation: self.saturation,
            lightness: self.lightness,
            standard: PhantomData,
        }
    }
}

impl<S, T> Saturate for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Scalar = T;

    fn saturate(&self, factor: T) -> Hsl<S, T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation * (T::one() + factor),
            lightness: self.lightness,
            standard: PhantomData,
        }
    }
}

impl<S, T> Default for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    fn default() -> Hsl<S, T> {
        Hsl::with_wp(RgbHue::from(T::zero()), T::zero(), T::zero())
    }
}

impl<S, T> Add<Hsl<S, T>> for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Output = Hsl<S, T>;

    fn add(self, other: Hsl<S, T>) -> Self::Output {
        Hsl {
            hue: self.hue + other.hue,
            saturation: self.saturation + other.saturation,
            lightness: self.lightness + other.lightness,
            standard: PhantomData,
        }
    }
}

impl<S, T> Add<T> for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Output = Hsl<S, T>;

    fn add(self, c: T) -> Self::Output {
        Hsl {
            hue: self.hue + c,
            saturation: self.saturation + c,
            lightness: self.lightness + c,
            standard: PhantomData,
        }
    }
}

impl<S, T> AddAssign<Hsl<S, T>> for Hsl<S, T>
where
    T: FloatComponent + AddAssign,
    S: RgbStandard,
{
    fn add_assign(&mut self, other: Hsl<S, T>) {
        self.hue += other.hue;
        self.saturation += other.saturation;
        self.lightness += other.lightness;
    }
}

impl<S, T> AddAssign<T> for Hsl<S, T>
where
    T: FloatComponent + AddAssign,
    S: RgbStandard,
{
    fn add_assign(&mut self, c: T) {
        self.hue += c;
        self.saturation += c;
        self.lightness += c;
    }
}

impl<S, T> Sub<Hsl<S, T>> for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Output = Hsl<S, T>;

    fn sub(self, other: Hsl<S, T>) -> Self::Output {
        Hsl {
            hue: self.hue - other.hue,
            saturation: self.saturation - other.saturation,
            lightness: self.lightness - other.lightness,
            standard: PhantomData,
        }
    }
}

impl<S, T> Sub<T> for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Output = Hsl<S, T>;

    fn sub(self, c: T) -> Self::Output {
        Hsl {
            hue: self.hue - c,
            saturation: self.saturation - c,
            lightness: self.lightness - c,
            standard: PhantomData,
        }
    }
}

impl<S, T> SubAssign<Hsl<S, T>> for Hsl<S, T>
where
    T: FloatComponent + SubAssign,
    S: RgbStandard,
{
    fn sub_assign(&mut self, other: Hsl<S, T>) {
        self.hue -= other.hue;
        self.saturation -= other.saturation;
        self.lightness -= other.lightness;
    }
}

impl<S, T> SubAssign<T> for Hsl<S, T>
where
    T: FloatComponent + SubAssign,
    S: RgbStandard,
{
    fn sub_assign(&mut self, c: T) {
        self.hue -= c;
        self.saturation -= c;
        self.lightness -= c;
    }
}

impl<S, T, P> AsRef<P> for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<S, T, P> AsMut<P> for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<S, T> AbsDiffEq for Hsl<S, T>
where
    T: FloatComponent + AbsDiffEq,
    T::Epsilon: Copy + Float + FromF64,
    S: RgbStandard + PartialEq,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        self.hue.abs_diff_eq(&other.hue, epsilon)
            && self.saturation.abs_diff_eq(&other.saturation, epsilon)
            && self.lightness.abs_diff_eq(&other.lightness, epsilon)
    }
}

impl<S, T> RelativeEq for Hsl<S, T>
where
    T: FloatComponent + RelativeEq,
    T::Epsilon: Copy + Float + FromF64,
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
        self.hue.relative_eq(&other.hue, epsilon, max_relative) &&
            self.saturation.relative_eq(&other.saturation, epsilon, max_relative) &&
            self.lightness.relative_eq(&other.lightness, epsilon, max_relative)
    }
}

impl<S, T> UlpsEq for Hsl<S, T>
where
    T: FloatComponent + UlpsEq,
    T::Epsilon: Copy + Float + FromF64,
    S: RgbStandard + PartialEq,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[rustfmt::skip]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.hue.ulps_eq(&other.hue, epsilon, max_ulps) &&
            self.saturation.ulps_eq(&other.saturation, epsilon, max_ulps) &&
            self.lightness.ulps_eq(&other.lightness, epsilon, max_ulps)
    }
}

impl<S, T> RelativeContrast for Hsl<S, T>
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
impl<S, T> Distribution<Hsl<S, T>> for Standard
where
    T: FloatComponent,
    S: RgbStandard,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hsl<S, T> {
        crate::random_sampling::sample_hsl(rng.gen::<RgbHue<T>>(), rng.gen(), rng.gen())
    }
}

#[cfg(feature = "random")]
pub struct UniformHsl<S, T>
where
    T: FloatComponent + SampleUniform,
    S: RgbStandard + SampleUniform,
{
    hue: crate::hues::UniformRgbHue<T>,
    u1: Uniform<T>,
    u2: Uniform<T>,
    space: PhantomData<S>,
}

#[cfg(feature = "random")]
impl<S, T> SampleUniform for Hsl<S, T>
where
    T: FloatComponent + SampleUniform,
    S: RgbStandard + SampleUniform,
{
    type Sampler = UniformHsl<S, T>;
}

#[cfg(feature = "random")]
impl<S, T> UniformSampler for UniformHsl<S, T>
where
    T: FloatComponent + SampleUniform,
    S: RgbStandard + SampleUniform,
{
    type X = Hsl<S, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        use crate::random_sampling::invert_hsl_sample;

        let low = *low_b.borrow();
        let high = *high_b.borrow();

        let (r1_min, r2_min) = invert_hsl_sample(low);
        let (r1_max, r2_max) = invert_hsl_sample(high);

        UniformHsl {
            hue: crate::hues::UniformRgbHue::new(low.hue, high.hue),
            u1: Uniform::new::<_, T>(r1_min, r1_max),
            u2: Uniform::new::<_, T>(r2_min, r2_max),
            space: PhantomData,
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        use crate::random_sampling::invert_hsl_sample;

        let low = *low_b.borrow();
        let high = *high_b.borrow();

        let (r1_min, r2_min) = invert_hsl_sample(low);
        let (r1_max, r2_max) = invert_hsl_sample(high);

        UniformHsl {
            hue: crate::hues::UniformRgbHue::new_inclusive(low.hue, high.hue),
            u1: Uniform::new_inclusive::<_, T>(r1_min, r1_max),
            u2: Uniform::new_inclusive::<_, T>(r2_min, r2_max),
            space: PhantomData,
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hsl<S, T> {
        crate::random_sampling::sample_hsl(
            self.hue.sample(rng),
            self.u1.sample(rng),
            self.u2.sample(rng),
        )
    }
}

#[cfg(test)]
mod test {
    use super::Hsl;
    use crate::{FromColor, Hsv, Srgb};

    #[test]
    fn red() {
        let a = Hsl::from_color(Srgb::new(1.0, 0.0, 0.0));
        let b = Hsl::new(0.0, 1.0, 0.5);
        let c = Hsl::from_color(Hsv::new(0.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn orange() {
        let a = Hsl::from_color(Srgb::new(1.0, 0.5, 0.0));
        let b = Hsl::new(30.0, 1.0, 0.5);
        let c = Hsl::from_color(Hsv::new(30.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn green() {
        let a = Hsl::from_color(Srgb::new(0.0, 1.0, 0.0));
        let b = Hsl::new(120.0, 1.0, 0.5);
        let c = Hsl::from_color(Hsv::new(120.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn blue() {
        let a = Hsl::from_color(Srgb::new(0.0, 0.0, 1.0));
        let b = Hsl::new(240.0, 1.0, 0.5);
        let c = Hsl::from_color(Hsv::new(240.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn purple() {
        let a = Hsl::from_color(Srgb::new(0.5, 0.0, 1.0));
        let b = Hsl::new(270.0, 1.0, 0.5);
        let c = Hsl::from_color(Hsv::new(270.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Hsl<crate::encoding::Srgb, f64>;
            limited {
                saturation: 0.0 => 1.0,
                lightness: 0.0 => 1.0
            }
            limited_min {}
            unlimited {
                hue: -360.0 => 360.0
            }
        }
    }

    raw_pixel_conversion_tests!(Hsl<crate::encoding::Srgb>: hue, saturation, lightness);
    raw_pixel_conversion_fail_tests!(Hsl<crate::encoding::Srgb>: hue, saturation, lightness);

    #[test]
    fn check_min_max_components() {
        use crate::encoding::Srgb;

        assert_relative_eq!(Hsl::<Srgb>::min_saturation(), 0.0);
        assert_relative_eq!(Hsl::<Srgb>::min_lightness(), 0.0);
        assert_relative_eq!(Hsl::<Srgb>::max_saturation(), 1.0);
        assert_relative_eq!(Hsl::<Srgb>::max_lightness(), 1.0);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Hsl::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(
            serialized,
            r#"{"hue":0.3,"saturation":0.8,"lightness":0.1}"#
        );
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Hsl =
            ::serde_json::from_str(r#"{"hue":0.3,"saturation":0.8,"lightness":0.1}"#).unwrap();

        assert_eq!(deserialized, Hsl::new(0.3, 0.8, 0.1));
    }
}
