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
    clamp, contrast_ratio, from_f64, Alpha, Clamp, Component, FloatComponent, FromColor, FromF64,
    GetHue, Hsl, Hue, Hwb, Mix, Pixel, RelativeContrast, RgbHue, Saturate, Shade, Xyz,
};

/// Linear HSV with an alpha component. See the [`Hsva` implementation in
/// `Alpha`](crate::Alpha#Hsva).
pub type Hsva<S = Srgb, T = f32> = Alpha<Hsv<S, T>, T>;

/// HSV color space.
///
/// HSV is a cylindrical version of [RGB](crate::rgb::Rgb) and it's very
/// similar to [HSL](crate::Hsl). The difference is that the `value`
/// component in HSV determines the _brightness_ of the color, and not the
/// _lightness_. The difference is that, for example, red (100% R, 0% G, 0% B)
/// and white (100% R, 100% G, 100% B) has the same brightness (or value), but
/// not the same lightness.
#[derive(Debug, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "<S::Space as RgbSpace>::WhitePoint",
    rgb_standard = "S",
    component = "T",
    skip_derives(Rgb, Hsl, Hwb, Hsv)
)]
#[repr(C)]
#[doc(alias = "hsb")]
pub struct Hsv<S = Srgb, T = f32>
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

    /// Decides how bright the color will look. 0.0 will be black, and 1.0 will
    /// give a bright an clear color that goes towards white when `saturation`
    /// goes towards 0.0.
    pub value: T,

    /// The white point and RGB primaries this color is adapted to. The default
    /// is the sRGB standard.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub standard: PhantomData<S>,
}

impl<S, T> Copy for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
}

impl<S, T> Clone for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    fn clone(&self) -> Hsv<S, T> {
        *self
    }
}

impl<T> Hsv<Srgb, T>
where
    T: FloatComponent,
{
    /// HSV for linear sRGB.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, value: T) -> Hsv<Srgb, T> {
        Hsv {
            hue: hue.into(),
            saturation,
            value,
            standard: PhantomData,
        }
    }
}

impl<S, T> Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    /// Linear HSV.
    pub fn with_wp<H: Into<RgbHue<T>>>(hue: H, saturation: T, value: T) -> Hsv<S, T> {
        Hsv {
            hue: hue.into(),
            saturation,
            value,
            standard: PhantomData,
        }
    }

    /// Convert to a `(hue, saturation, value)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T) {
        (self.hue, self.saturation, self.value)
    }

    /// Convert from a `(hue, saturation, value)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>((hue, saturation, value): (H, T, T)) -> Self {
        Self::with_wp(hue, saturation, value)
    }

    #[inline]
    fn reinterpret_as<St: RgbStandard>(self) -> Hsv<St, T> {
        Hsv {
            hue: self.hue,
            saturation: self.saturation,
            value: self.value,
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

    /// Return the `value` value minimum.
    pub fn min_value() -> T {
        T::zero()
    }

    /// Return the `value` value maximum.
    pub fn max_value() -> T {
        T::max_intensity()
    }
}

impl<S, T> PartialEq for Hsv<S, T>
where
    T: FloatComponent + PartialEq,
    S: RgbStandard,
{
    fn eq(&self, other: &Self) -> bool {
        self.hue == other.hue && self.saturation == other.saturation && self.value == other.value
    }
}

impl<S, T> Eq for Hsv<S, T>
where
    T: FloatComponent + Eq,
    S: RgbStandard,
{
}

///<span id="Hsva"></span>[`Hsva`](crate::Hsva) implementations.
impl<T, A> Alpha<Hsv<Srgb, T>, A>
where
    T: FloatComponent,
    A: Component,
{
    /// HSV and transparency for linear sRGB.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, value: T, alpha: A) -> Self {
        Alpha {
            color: Hsv::new(hue, saturation, value),
            alpha,
        }
    }
}

///<span id="Hsva"></span>[`Hsva`](crate::Hsva) implementations.
impl<S, T, A> Alpha<Hsv<S, T>, A>
where
    T: FloatComponent,
    A: Component,
    S: RgbStandard,
{
    /// Linear HSV and transparency.
    pub fn with_wp<H: Into<RgbHue<T>>>(hue: H, saturation: T, value: T, alpha: A) -> Self {
        Alpha {
            color: Hsv::with_wp(hue, saturation, value),
            alpha,
        }
    }

    /// Convert to a `(hue, saturation, value, alpha)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T, A) {
        (self.hue, self.saturation, self.value, self.alpha)
    }

    /// Convert from a `(hue, saturation, value, alpha)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>(
        (hue, saturation, value, alpha): (H, T, T, A),
    ) -> Self {
        Self::with_wp(hue, saturation, value, alpha)
    }
}

impl<S1, S2, T> FromColorUnclamped<Hsv<S1, T>> for Hsv<S2, T>
where
    T: FloatComponent,
    S1: RgbStandard,
    S2: RgbStandard,
    S1::Space: RgbSpace<WhitePoint = <S2::Space as RgbSpace>::WhitePoint>,
{
    fn from_color_unclamped(hsv: Hsv<S1, T>) -> Self {
        if TypeId::of::<S1>() == TypeId::of::<S2>() {
            hsv.reinterpret_as()
        } else {
            let rgb = Rgb::<S1, T>::from_color_unclamped(hsv);
            let converted_rgb = Rgb::<S2, T>::from_color_unclamped(rgb);
            Self::from_color_unclamped(converted_rgb)
        }
    }
}

impl<S, T> FromColorUnclamped<Rgb<S, T>> for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
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
        let v = max;

        if max != min {
            let d = max - min;
            s = d / max;
            h = ((sep / d) + coeff) * from_f64(60.0);
        };

        Hsv {
            hue: h.into(),
            saturation: s,
            value: v,
            standard: PhantomData,
        }
    }
}

impl<S, T> FromColorUnclamped<Hsl<S, T>> for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    fn from_color_unclamped(hsl: Hsl<S, T>) -> Self {
        let x = hsl.saturation
            * if hsl.lightness < from_f64(0.5) {
                hsl.lightness
            } else {
                T::one() - hsl.lightness
            };
        let mut s = T::zero();

        // avoid divide by zero
        let denom = hsl.lightness + x;
        if denom.is_normal() {
            s = x * from_f64(2.0) / denom;
        }
        Hsv {
            hue: hsl.hue,
            saturation: s,
            value: hsl.lightness + x,
            standard: PhantomData,
        }
    }
}

impl<S, T> FromColorUnclamped<Hwb<S, T>> for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    fn from_color_unclamped(hwb: Hwb<S, T>) -> Self {
        let inv = T::one() - hwb.blackness;
        // avoid divide by zero
        let s = if inv.is_normal() {
            T::one() - (hwb.whiteness / inv)
        } else {
            T::zero()
        };
        Hsv {
            hue: hwb.hue,
            saturation: s,
            value: inv,
            standard: PhantomData,
        }
    }
}

impl<S: RgbStandard, T: FloatComponent, H: Into<RgbHue<T>>> From<(H, T, T)> for Hsv<S, T> {
    fn from(components: (H, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<S: RgbStandard, T: FloatComponent> Into<(RgbHue<T>, T, T)> for Hsv<S, T> {
    fn into(self) -> (RgbHue<T>, T, T) {
        self.into_components()
    }
}

impl<S: RgbStandard, T: FloatComponent, H: Into<RgbHue<T>>, A: Component> From<(H, T, T, A)>
    for Alpha<Hsv<S, T>, A>
{
    fn from(components: (H, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<S: RgbStandard, T: FloatComponent, A: Component> Into<(RgbHue<T>, T, T, A)>
    for Alpha<Hsv<S, T>, A>
{
    fn into(self) -> (RgbHue<T>, T, T, A) {
        self.into_components()
    }
}

impl<S, T> Clamp for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    #[rustfmt::skip]
    fn is_within_bounds(&self) -> bool {
        self.saturation >= T::zero() && self.saturation <= T::one() &&
        self.value >= T::zero() && self.value <= T::one()
    }

    fn clamp(&self) -> Hsv<S, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.saturation = clamp(self.saturation, T::zero(), T::one());
        self.value = clamp(self.value, T::zero(), T::one());
    }
}

impl<S, T> Mix for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Scalar = T;

    fn mix(&self, other: &Hsv<S, T>, factor: T) -> Hsv<S, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();

        Hsv {
            hue: self.hue + factor * hue_diff,
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            value: self.value + factor * (other.value - self.value),
            standard: PhantomData,
        }
    }
}

impl<S, T> Shade for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Scalar = T;

    fn lighten(&self, factor: T) -> Hsv<S, T> {
        let difference = if factor >= T::zero() {
            T::max_intensity() - self.value
        } else {
            self.value
        };

        let delta = difference.max(T::zero()) * factor;

        Hsv {
            hue: self.hue,
            saturation: self.saturation,
            value: (self.value + delta).max(T::zero()),
            standard: PhantomData,
        }
    }

    fn lighten_fixed(&self, amount: T) -> Hsv<S, T> {
        Hsv {
            hue: self.hue,
            saturation: self.saturation,
            value: (self.value + T::max_intensity() * amount).max(T::zero()),
            standard: PhantomData,
        }
    }
}

impl<S, T> GetHue for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        if self.saturation <= T::zero() || self.value <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<S, T> Hue for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    fn with_hue<H: Into<Self::Hue>>(&self, hue: H) -> Hsv<S, T> {
        Hsv {
            hue: hue.into(),
            saturation: self.saturation,
            value: self.value,
            standard: PhantomData,
        }
    }

    fn shift_hue<H: Into<Self::Hue>>(&self, amount: H) -> Hsv<S, T> {
        Hsv {
            hue: self.hue + amount.into(),
            saturation: self.saturation,
            value: self.value,
            standard: PhantomData,
        }
    }
}

impl<S, T> Saturate for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Scalar = T;

    fn saturate(&self, factor: T) -> Hsv<S, T> {
        let difference = if factor >= T::zero() {
            T::max_intensity() - self.saturation
        } else {
            self.saturation
        };

        let delta = difference.max(T::zero()) * factor;

        Hsv {
            hue: self.hue,
            saturation: (self.saturation + delta).max(T::zero()),
            value: self.value,
            standard: PhantomData,
        }
    }

    fn saturate_fixed(&self, amount: T) -> Hsv<S, T> {
        Hsv {
            hue: self.hue,
            saturation: (self.saturation + T::max_intensity() * amount).max(T::zero()),
            value: self.value,
            standard: PhantomData,
        }
    }
}

impl<S, T> Default for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    fn default() -> Hsv<S, T> {
        Hsv::with_wp(RgbHue::from(T::zero()), T::zero(), T::zero())
    }
}

impl<S, T> Add<Hsv<S, T>> for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Output = Hsv<S, T>;

    fn add(self, other: Hsv<S, T>) -> Self::Output {
        Hsv {
            hue: self.hue + other.hue,
            saturation: self.saturation + other.saturation,
            value: self.value + other.value,
            standard: PhantomData,
        }
    }
}

impl<S, T> Add<T> for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Output = Hsv<S, T>;

    fn add(self, c: T) -> Self::Output {
        Hsv {
            hue: self.hue + c,
            saturation: self.saturation + c,
            value: self.value + c,
            standard: PhantomData,
        }
    }
}

impl<S, T> AddAssign<Hsv<S, T>> for Hsv<S, T>
where
    T: FloatComponent + AddAssign,
    S: RgbStandard,
{
    fn add_assign(&mut self, other: Hsv<S, T>) {
        self.hue += other.hue;
        self.saturation += other.saturation;
        self.value += other.value;
    }
}

impl<S, T> AddAssign<T> for Hsv<S, T>
where
    T: FloatComponent + AddAssign,
    S: RgbStandard,
{
    fn add_assign(&mut self, c: T) {
        self.hue += c;
        self.saturation += c;
        self.value += c;
    }
}

impl<S, T> Sub<Hsv<S, T>> for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Output = Hsv<S, T>;

    fn sub(self, other: Hsv<S, T>) -> Self::Output {
        Hsv {
            hue: self.hue - other.hue,
            saturation: self.saturation - other.saturation,
            value: self.value - other.value,
            standard: PhantomData,
        }
    }
}

impl<S, T> Sub<T> for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Output = Hsv<S, T>;

    fn sub(self, c: T) -> Self::Output {
        Hsv {
            hue: self.hue - c,
            saturation: self.saturation - c,
            value: self.value - c,
            standard: PhantomData,
        }
    }
}

impl<S, T> SubAssign<Hsv<S, T>> for Hsv<S, T>
where
    T: FloatComponent + SubAssign,
    S: RgbStandard,
{
    fn sub_assign(&mut self, other: Hsv<S, T>) {
        self.hue -= other.hue;
        self.saturation -= other.saturation;
        self.value -= other.value;
    }
}

impl<S, T> SubAssign<T> for Hsv<S, T>
where
    T: FloatComponent + SubAssign,
    S: RgbStandard,
{
    fn sub_assign(&mut self, c: T) {
        self.hue -= c;
        self.saturation -= c;
        self.value -= c;
    }
}

impl<S, T, P> AsRef<P> for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<S, T, P> AsMut<P> for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<S, T> AbsDiffEq for Hsv<S, T>
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
            && self.value.abs_diff_eq(&other.value, epsilon)
    }
}

impl<S, T> RelativeEq for Hsv<S, T>
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
            self.value.relative_eq(&other.value, epsilon, max_relative)
    }
}

impl<S, T> UlpsEq for Hsv<S, T>
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
            self.value.ulps_eq(&other.value, epsilon, max_ulps)
    }
}

impl<S, T> RelativeContrast for Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    type Scalar = T;

    fn get_contrast_ratio(&self, other: &Self) -> T {
        let xyz1 = Xyz::from_color(*self);
        let xyz2 = Xyz::from_color(*other);

        contrast_ratio(xyz1.y, xyz2.y)
    }
}

#[cfg(feature = "random")]
impl<S, T> Distribution<Hsv<S, T>> for Standard
where
    T: FloatComponent,
    S: RgbStandard,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hsv<S, T> {
        crate::random_sampling::sample_hsv(rng.gen::<RgbHue<T>>(), rng.gen(), rng.gen())
    }
}

#[cfg(feature = "random")]
pub struct UniformHsv<S, T>
where
    T: FloatComponent + SampleUniform,
    S: RgbStandard,
{
    hue: crate::hues::UniformRgbHue<T>,
    u1: Uniform<T>,
    u2: Uniform<T>,
    space: PhantomData<S>,
}

#[cfg(feature = "random")]
impl<S, T> SampleUniform for Hsv<S, T>
where
    T: FloatComponent + SampleUniform,
    S: RgbStandard,
{
    type Sampler = UniformHsv<S, T>;
}

#[cfg(feature = "random")]
impl<S, T> UniformSampler for UniformHsv<S, T>
where
    T: FloatComponent + SampleUniform,
    S: RgbStandard,
{
    type X = Hsv<S, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        let (r1_min, r2_min) = (
            low.value * low.value * low.value,
            low.saturation * low.saturation,
        );
        let (r1_max, r2_max) = (
            high.value * high.value * high.value,
            high.saturation * high.saturation,
        );

        UniformHsv {
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
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        let (r1_min, r2_min) = (
            low.value * low.value * low.value,
            low.saturation * low.saturation,
        );
        let (r1_max, r2_max) = (
            high.value * high.value * high.value,
            high.saturation * high.saturation,
        );

        UniformHsv {
            hue: crate::hues::UniformRgbHue::new_inclusive(low.hue, high.hue),
            u1: Uniform::new_inclusive::<_, T>(r1_min, r1_max),
            u2: Uniform::new_inclusive::<_, T>(r2_min, r2_max),
            space: PhantomData,
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hsv<S, T> {
        crate::random_sampling::sample_hsv(
            self.hue.sample(rng),
            self.u1.sample(rng),
            self.u2.sample(rng),
        )
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<S, T> bytemuck::Zeroable for Hsv<S, T>
where
    S: RgbStandard,
    T: FloatComponent + bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<S, T> bytemuck::Pod for Hsv<S, T>
where
    S: RgbStandard,
    T: FloatComponent + bytemuck::Pod,
{
}

#[cfg(test)]
mod test {
    use super::Hsv;
    use crate::{FromColor, Hsl, Srgb};

    #[test]
    fn red() {
        let a = Hsv::from_color(Srgb::new(1.0, 0.0, 0.0));
        let b = Hsv::new(0.0, 1.0, 1.0);
        let c = Hsv::from_color(Hsl::new(0.0, 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn orange() {
        let a = Hsv::from_color(Srgb::new(1.0, 0.5, 0.0));
        let b = Hsv::new(30.0, 1.0, 1.0);
        let c = Hsv::from_color(Hsl::new(30.0, 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn green() {
        let a = Hsv::from_color(Srgb::new(0.0, 1.0, 0.0));
        let b = Hsv::new(120.0, 1.0, 1.0);
        let c = Hsv::from_color(Hsl::new(120.0, 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn blue() {
        let a = Hsv::from_color(Srgb::new(0.0, 0.0, 1.0));
        let b = Hsv::new(240.0, 1.0, 1.0);
        let c = Hsv::from_color(Hsl::new(240.0, 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn purple() {
        let a = Hsv::from_color(Srgb::new(0.5, 0.0, 1.0));
        let b = Hsv::new(270.0, 1.0, 1.0);
        let c = Hsv::from_color(Hsl::new(270.0, 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Hsv<crate::encoding::Srgb, f64>;
            clamped {
                saturation: 0.0 => 1.0,
                value: 0.0 => 1.0
            }
            clamped_min {}
            unclamped {
                hue: -360.0 => 360.0
            }
        }
    }

    raw_pixel_conversion_tests!(Hsv<crate::encoding::Srgb>: hue, saturation, value);
    raw_pixel_conversion_fail_tests!(Hsv<crate::encoding::Srgb>: hue, saturation, value);

    #[test]
    fn check_min_max_components() {
        use crate::encoding::Srgb;

        assert_relative_eq!(Hsv::<Srgb>::min_saturation(), 0.0,);
        assert_relative_eq!(Hsv::<Srgb>::min_value(), 0.0,);
        assert_relative_eq!(Hsv::<Srgb>::max_saturation(), 1.0,);
        assert_relative_eq!(Hsv::<Srgb>::max_value(), 1.0,);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Hsv::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"hue":0.3,"saturation":0.8,"value":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Hsv =
            ::serde_json::from_str(r#"{"hue":0.3,"saturation":0.8,"value":0.1}"#).unwrap();

        assert_eq!(deserialized, Hsv::new(0.3, 0.8, 0.1));
    }

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Hsv<crate::encoding::Srgb, f32> as crate::rgb::Rgb {
            red: (0.0, 1.0),
            green: (0.0, 1.0),
            blue: (0.0, 1.0)
        },
        min: Hsv::new(0.0f32, 0.0, 0.0),
        max: Hsv::new(360.0, 1.0, 1.0)
    }
}
