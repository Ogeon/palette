use core::any::TypeId;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num_traits::Zero;
#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::convert::FromColorUnclamped;
use crate::encoding::pixel::RawPixel;
use crate::encoding::Srgb;
use crate::rgb::{Rgb, RgbSpace, RgbStandard};
use crate::{
    clamp, clamp_assign, clamp_min_assign, contrast_ratio, from_f64, Alpha, Clamp, ClampAssign,
    Component, FloatComponent, GetHue, Hsv, Hue, IsWithinBounds, Lighten, LightenAssign, Mix,
    MixAssign, Pixel, RelativeContrast, RgbHue, Saturate, Xyz,
};
#[cfg(feature = "random")]
use crate::{float::Float, FromF64};

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
#[derive(Debug, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    rgb_standard = "S",
    component = "T",
    skip_derives(Rgb, Hsv, Hsl)
)]
#[repr(C)]
pub struct Hsl<S = Srgb, T = f32> {
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

impl<S, T: Copy> Copy for Hsl<S, T> {}

impl<S, T: Clone> Clone for Hsl<S, T> {
    fn clone(&self) -> Hsl<S, T> {
        Hsl {
            hue: self.hue.clone(),
            saturation: self.saturation.clone(),
            lightness: self.lightness.clone(),
            standard: PhantomData,
        }
    }
}

impl<T> Hsl<Srgb, T> {
    /// Create an sRGB HSL color. This method can be used instead of `Hsl::new`
    /// to help type inference.
    pub fn new_srgb<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T) -> Self {
        Self::new_const(hue.into(), saturation, lightness)
    }

    /// Create an sRGB HSL color. This is the same as `Hsl::new_srgb` without
    /// the generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_srgb_const(hue: RgbHue<T>, saturation: T, lightness: T) -> Self {
        Self::new_const(hue, saturation, lightness)
    }
}

impl<S, T> Hsl<S, T> {
    /// Create an HSL color.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T) -> Self {
        Self::new_const(hue.into(), saturation, lightness)
    }

    /// Create an HSL color. This is the same as `Hsl::new` without the generic
    /// hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(hue: RgbHue<T>, saturation: T, lightness: T) -> Self {
        Hsl {
            hue,
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
        Self::new(hue, saturation, lightness)
    }

    #[inline]
    fn reinterpret_as<St: RgbStandard<T>>(self) -> Hsl<St, T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation,
            lightness: self.lightness,
            standard: PhantomData,
        }
    }
}

impl<S, T> Hsl<S, T>
where
    T: Component,
{
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

impl<S, T> PartialEq for Hsl<S, T>
where
    T: PartialEq,
    RgbHue<T>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.hue == other.hue
            && self.saturation == other.saturation
            && self.lightness == other.lightness
    }
}

impl<S, T> Eq for Hsl<S, T>
where
    T: Eq,
    RgbHue<T>: PartialEq,
{
}

///<span id="Hsla"></span>[`Hsla`](crate::Hsla) implementations.
impl<T, A> Alpha<Hsl<Srgb, T>, A> {
    /// Create an sRGB HSL color with transparency. This method can be used
    /// instead of `Hsla::new` to help type inference.
    pub fn new_srgb<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T, alpha: A) -> Self {
        Self::new_const(hue.into(), saturation, lightness, alpha)
    }

    /// Create an sRGB HSL color with transparency. This is the same as
    /// `Hsla::new_srgb` without the generic hue type. It's temporary until
    /// `const fn` supports traits.
    pub const fn new_srgb_const(hue: RgbHue<T>, saturation: T, lightness: T, alpha: A) -> Self {
        Self::new_const(hue, saturation, lightness, alpha)
    }
}

///<span id="Hsla"></span>[`Hsla`](crate::Hsla) implementations.
impl<S, T, A> Alpha<Hsl<S, T>, A> {
    /// Create an HSL color with transparency.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T, alpha: A) -> Self {
        Self::new_const(hue.into(), saturation, lightness, alpha)
    }

    /// Create an HSL color with transparency. This is the same as `Hsla::new`
    /// without the generic hue type. It's temporary until `const fn` supports
    /// traits.
    pub const fn new_const(hue: RgbHue<T>, saturation: T, lightness: T, alpha: A) -> Self {
        Alpha {
            color: Hsl::new_const(hue, saturation, lightness),
            alpha,
        }
    }

    /// Convert to a `(hue, saturation, lightness, alpha)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T, A) {
        (
            self.color.hue,
            self.color.saturation,
            self.color.lightness,
            self.alpha,
        )
    }

    /// Convert from a `(hue, saturation, lightness, alpha)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>(
        (hue, saturation, lightness, alpha): (H, T, T, A),
    ) -> Self {
        Self::new(hue, saturation, lightness, alpha)
    }
}

impl<S1, S2, T> FromColorUnclamped<Hsl<S1, T>> for Hsl<S2, T>
where
    T: FloatComponent,
    S1: RgbStandard<T>,
    S2: RgbStandard<T>,
    S1::Space: RgbSpace<T, WhitePoint = <S2::Space as RgbSpace<T>>::WhitePoint>,
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
{
    fn from_color_unclamped(mut rgb: Rgb<S, T>) -> Self {
        // Avoid negative numbers
        rgb.red = rgb.red.max(T::zero());
        rgb.green = rgb.green.max(T::zero());
        rgb.blue = rgb.blue.max(T::zero());

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

impl<S, T, H: Into<RgbHue<T>>> From<(H, T, T)> for Hsl<S, T> {
    fn from(components: (H, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<S, T> From<Hsl<S, T>> for (RgbHue<T>, T, T) {
    fn from(color: Hsl<S, T>) -> (RgbHue<T>, T, T) {
        color.into_components()
    }
}

impl<S, T, H: Into<RgbHue<T>>, A> From<(H, T, T, A)> for Alpha<Hsl<S, T>, A> {
    fn from(components: (H, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<S, T, A> From<Alpha<Hsl<S, T>, A>> for (RgbHue<T>, T, T, A) {
    fn from(color: Alpha<Hsl<S, T>, A>) -> (RgbHue<T>, T, T, A) {
        color.into_components()
    }
}

impl<S, T> IsWithinBounds for Hsl<S, T>
where
    T: Component,
{
    #[rustfmt::skip]
    #[inline]
    fn is_within_bounds(&self) -> bool {
        self.saturation >= T::zero() && self.saturation <= T::max_intensity() &&
        self.lightness >= T::zero() && self.lightness <= T::max_intensity()
    }
}

impl<S, T> Clamp for Hsl<S, T>
where
    T: Component,
{
    #[inline]
    fn clamp(self) -> Self {
        Self::new(
            self.hue,
            clamp(self.saturation, T::zero(), T::max_intensity()),
            clamp(self.lightness, T::zero(), T::max_intensity()),
        )
    }
}

impl<S, T> ClampAssign for Hsl<S, T>
where
    T: Component,
{
    #[inline]
    fn clamp_assign(&mut self) {
        clamp_assign(&mut self.saturation, T::zero(), T::max_intensity());
        clamp_assign(&mut self.lightness, T::zero(), T::max_intensity());
    }
}

impl<S, T> Mix for Hsl<S, T>
where
    T: FloatComponent,
{
    type Scalar = T;

    #[inline]
    fn mix(self, other: Self, factor: T) -> Self {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff = (other.hue - self.hue).to_degrees();

        Hsl {
            hue: self.hue + factor * hue_diff,
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            lightness: self.lightness + factor * (other.lightness - self.lightness),
            standard: PhantomData,
        }
    }
}

impl<S, T> MixAssign for Hsl<S, T>
where
    T: FloatComponent + AddAssign,
{
    type Scalar = T;

    #[inline]
    fn mix_assign(&mut self, other: Self, factor: T) {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff = (other.hue - self.hue).to_degrees();

        self.hue += factor * hue_diff;
        self.saturation += factor * (other.saturation - self.saturation);
        self.lightness += factor * (other.lightness - self.lightness);
    }
}

impl<S, T> Lighten for Hsl<S, T>
where
    T: FloatComponent,
{
    type Scalar = T;

    #[inline]
    fn lighten(self, factor: T) -> Hsl<S, T> {
        let difference = if factor >= T::zero() {
            Self::max_lightness() - self.lightness
        } else {
            self.lightness
        };

        let delta = difference.max(T::zero()) * factor;

        Hsl {
            hue: self.hue,
            saturation: self.saturation,
            lightness: (self.lightness + delta).max(Self::min_lightness()),
            standard: PhantomData,
        }
    }

    #[inline]
    fn lighten_fixed(self, amount: T) -> Hsl<S, T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation,
            lightness: (self.lightness + Self::max_lightness() * amount).max(Self::min_lightness()),
            standard: PhantomData,
        }
    }
}

impl<S, T> LightenAssign for Hsl<S, T>
where
    T: FloatComponent + AddAssign,
{
    type Scalar = T;

    #[inline]
    fn lighten_assign(&mut self, factor: T) {
        let difference = if factor >= T::zero() {
            Self::max_lightness() - self.lightness
        } else {
            self.lightness
        };

        self.lightness += difference.max(T::zero()) * factor;
        clamp_min_assign(&mut self.lightness, Self::min_lightness());
    }

    #[inline]
    fn lighten_fixed_assign(&mut self, amount: T) {
        self.lightness += Self::max_lightness() * amount;
        clamp_min_assign(&mut self.lightness, Self::min_lightness());
    }
}

impl<S, T> GetHue for Hsl<S, T>
where
    T: Zero + PartialOrd + Clone,
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        if self.saturation <= T::zero() {
            None
        } else {
            Some(self.hue.clone())
        }
    }
}

impl<S, T> Hue for Hsl<S, T>
where
    T: Zero + PartialOrd + Add<Output = T> + Clone,
{
    #[inline]
    fn with_hue<H: Into<Self::Hue>>(mut self, hue: H) -> Self {
        self.hue = hue.into();
        self
    }

    #[inline]
    fn shift_hue<H: Into<Self::Hue>>(mut self, amount: H) -> Self {
        self.hue = self.hue + amount.into();
        self
    }
}

impl<S, T> Saturate for Hsl<S, T>
where
    T: FloatComponent,
{
    type Scalar = T;

    #[inline]
    fn saturate(self, factor: T) -> Hsl<S, T> {
        let difference = if factor >= T::zero() {
            T::max_intensity() - self.saturation
        } else {
            self.saturation
        };

        let delta = difference.max(T::zero()) * factor;

        Hsl {
            hue: self.hue,
            saturation: (self.saturation + delta).max(T::zero()),
            lightness: self.lightness,
            standard: PhantomData,
        }
    }

    #[inline]
    fn saturate_fixed(self, amount: T) -> Hsl<S, T> {
        Hsl {
            hue: self.hue,
            saturation: (self.saturation + T::max_intensity() * amount).max(T::zero()),
            lightness: self.lightness,
            standard: PhantomData,
        }
    }
}

impl<S, T> Default for Hsl<S, T>
where
    T: Zero,
{
    fn default() -> Hsl<S, T> {
        Hsl::new(RgbHue::from(T::zero()), T::zero(), T::zero())
    }
}

impl_color_add!(Hsl<S, T>, [hue, saturation, lightness], standard);
impl_color_sub!(Hsl<S, T>, [hue, saturation, lightness], standard);

impl<S, T, P> AsRef<P> for Hsl<S, T>
where
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<S, T, P> AsMut<P> for Hsl<S, T>
where
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<S, T> AbsDiffEq for Hsl<S, T>
where
    T: AbsDiffEq,
    RgbHue<T>: AbsDiffEq<Epsilon = T::Epsilon>,
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[rustfmt::skip]
    fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        self.hue.abs_diff_eq(&other.hue, epsilon.clone())
            && self.saturation.abs_diff_eq(&other.saturation, epsilon.clone())
            && self.lightness.abs_diff_eq(&other.lightness, epsilon)
    }
}

impl<S, T> RelativeEq for Hsl<S, T>
where
    T: RelativeEq,
    RgbHue<T>: RelativeEq + AbsDiffEq<Epsilon = T::Epsilon>,
    T::Epsilon: Clone,
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
        self.hue.relative_eq(&other.hue, epsilon.clone(), max_relative.clone())
            && self.saturation.relative_eq(&other.saturation, epsilon.clone(), max_relative.clone())
            && self.lightness.relative_eq(&other.lightness, epsilon, max_relative)
    }
}

impl<S, T> UlpsEq for Hsl<S, T>
where
    T: UlpsEq,
    RgbHue<T>: UlpsEq + AbsDiffEq<Epsilon = T::Epsilon>,
    T::Epsilon: Clone,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[rustfmt::skip]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.hue.ulps_eq(&other.hue, epsilon.clone(), max_ulps)
            && self.saturation.ulps_eq(&other.saturation, epsilon.clone(), max_ulps)
            && self.lightness.ulps_eq(&other.lightness, epsilon, max_ulps)
    }
}

impl<S, T> RelativeContrast for Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard<T>,
{
    type Scalar = T;

    #[inline]
    fn get_contrast_ratio(self, other: Self) -> T {
        use crate::FromColor;

        let xyz1 = Xyz::from_color(self);
        let xyz2 = Xyz::from_color(other);

        contrast_ratio(xyz1.y, xyz2.y)
    }
}

#[cfg(feature = "random")]
impl<S, T> Distribution<Hsl<S, T>> for Standard
where
    T: Float + FromF64,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hsl<S, T> {
        crate::random_sampling::sample_hsl(rng.gen::<RgbHue<T>>(), rng.gen(), rng.gen())
    }
}

#[cfg(feature = "random")]
pub struct UniformHsl<S, T>
where
    T: Float + FromF64 + SampleUniform,
{
    hue: crate::hues::UniformRgbHue<T>,
    u1: Uniform<T>,
    u2: Uniform<T>,
    space: PhantomData<S>,
}

#[cfg(feature = "random")]
impl<S, T> SampleUniform for Hsl<S, T>
where
    T: Float + FromF64 + SampleUniform,
{
    type Sampler = UniformHsl<S, T>;
}

#[cfg(feature = "random")]
impl<S, T> UniformSampler for UniformHsl<S, T>
where
    T: Float + FromF64 + SampleUniform,
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

#[cfg(feature = "bytemuck")]
unsafe impl<S, T> bytemuck::Zeroable for Hsl<S, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<S: 'static, T> bytemuck::Pod for Hsl<S, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use super::Hsl;
    use crate::{FromColor, Hsv, Srgb};

    #[test]
    fn red() {
        let a = Hsl::from_color(Srgb::new(1.0, 0.0, 0.0));
        let b = Hsl::new_srgb(0.0, 1.0, 0.5);
        let c = Hsl::from_color(Hsv::new_srgb(0.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn orange() {
        let a = Hsl::from_color(Srgb::new(1.0, 0.5, 0.0));
        let b = Hsl::new_srgb(30.0, 1.0, 0.5);
        let c = Hsl::from_color(Hsv::new_srgb(30.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn green() {
        let a = Hsl::from_color(Srgb::new(0.0, 1.0, 0.0));
        let b = Hsl::new_srgb(120.0, 1.0, 0.5);
        let c = Hsl::from_color(Hsv::new_srgb(120.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn blue() {
        let a = Hsl::from_color(Srgb::new(0.0, 0.0, 1.0));
        let b = Hsl::new_srgb(240.0, 1.0, 0.5);
        let c = Hsl::from_color(Hsv::new_srgb(240.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn purple() {
        let a = Hsl::from_color(Srgb::new(0.5, 0.0, 1.0));
        let b = Hsl::new_srgb(270.0, 1.0, 0.5);
        let c = Hsl::from_color(Hsv::new_srgb(270.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Hsl<crate::encoding::Srgb, f64>;
            clamped {
                saturation: 0.0 => 1.0,
                lightness: 0.0 => 1.0
            }
            clamped_min {}
            unclamped {
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
        let serialized = ::serde_json::to_string(&Hsl::new_srgb(0.3, 0.8, 0.1)).unwrap();

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

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Hsl<crate::encoding::Srgb, f32> as crate::rgb::Rgb {
            red: (0.0, 1.0),
            green: (0.0, 1.0),
            blue: (0.0, 1.0)
        },
        min: Hsl::new(0.0f32, 0.0, 0.0),
        max: Hsl::new(360.0, 1.0, 1.0)
    }

    /// Sanity check to make sure the test doesn't start accepting known
    /// non-uniform distributions.
    #[cfg(feature = "random")]
    #[test]
    #[should_panic(expected = "is not uniform enough")]
    fn uniform_distribution_fail() {
        use rand::Rng;

        const BINS: usize = crate::random_sampling::test_utils::BINS;
        const SAMPLES: usize = crate::random_sampling::test_utils::SAMPLES;

        let mut red = [0; BINS];
        let mut green = [0; BINS];
        let mut blue = [0; BINS];

        let mut rng = rand_mt::Mt::new(1234); // We want the same seed on every run to avoid random fails

        for _ in 0..SAMPLES {
            let color = Hsl::<crate::encoding::Srgb, f32>::new(
                rng.gen::<f32>() * 360.0,
                rng.gen(),
                rng.gen(),
            );
            let color: crate::rgb::Rgb = crate::IntoColor::into_color(color);
            red[((color.red * BINS as f32) as usize).min(9)] += 1;
            green[((color.green * BINS as f32) as usize).min(9)] += 1;
            blue[((color.blue * BINS as f32) as usize).min(9)] += 1;
        }

        assert_uniform_distribution!(red);
        assert_uniform_distribution!(green);
        assert_uniform_distribution!(blue);
    }
}
