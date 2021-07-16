use core::any::TypeId;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, DivAssign, Sub, SubAssign};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::convert::FromColorUnclamped;
use crate::encoding::pixel::RawPixel;
use crate::encoding::Srgb;
use crate::rgb::{RgbSpace, RgbStandard};
use crate::{
    clamp, contrast_ratio, Alpha, Clamp, Component, FloatComponent, GetHue, Hsv, Hue, Mix, Pixel,
    RelativeContrast, RgbHue, Shade, Xyz,
};

/// Linear HWB with an alpha component. See the [`Hwba` implementation in
/// `Alpha`](crate::Alpha#Hwba).
pub type Hwba<S = Srgb, T = f32> = Alpha<Hwb<S, T>, T>;

/// HWB color space.
///
/// HWB is a cylindrical version of [RGB](crate::rgb::Rgb) and it's very
/// closely related to [HSV](crate::Hsv). It describes colors with a
/// starting hue, then a degree of whiteness and blackness to mix into that
/// base hue.
///
/// It is very intuitive for humans to use and many color-pickers are based on
/// the HWB color system
#[derive(Debug, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    rgb_standard = "S",
    component = "T",
    skip_derives(Hsv, Hwb)
)]
#[repr(C)]
pub struct Hwb<S = Srgb, T = f32> {
    /// The hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc. Same as the hue for HSL and HSV.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: RgbHue<T>,

    /// The whiteness of the color. It specifies the amount white to mix into
    /// the hue. It varies from 0 to 1, with 1 being always full white and 0
    /// always being the color shade (a mixture of a pure hue with black)
    /// chosen with the other two controls.
    pub whiteness: T,

    /// The blackness of the color. It specifies the amount black to mix into
    /// the hue. It varies from 0 to 1, with 1 being always full black and
    /// 0 always being the color tint (a mixture of a pure hue with white)
    /// chosen with the other two
    //controls.
    pub blackness: T,

    /// The white point and RGB primaries this color is adapted to. The default
    /// is the sRGB standard.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub standard: PhantomData<S>,
}

impl<S, T> Copy for Hwb<S, T> where T: Copy {}

impl<S, T> Clone for Hwb<S, T>
where
    T: Clone,
{
    fn clone(&self) -> Hwb<S, T> {
        Hwb {
            hue: self.hue.clone(),
            whiteness: self.whiteness.clone(),
            blackness: self.blackness.clone(),
            standard: PhantomData,
        }
    }
}

impl<T> Hwb<Srgb, T> {
    /// Create an sRGB HWB color. This method can be used instead of `Hwb::new`
    /// to help type inference.
    pub fn new_srgb<H: Into<RgbHue<T>>>(hue: H, whiteness: T, blackness: T) -> Self {
        Self::new_const(hue.into(), whiteness, blackness)
    }

    /// Create an sRGB HWB color. This is the same as `Hwb::new_srgb` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_srgb_const(hue: RgbHue<T>, whiteness: T, blackness: T) -> Self {
        Self::new_const(hue, whiteness, blackness)
    }
}

impl<S, T> Hwb<S, T> {
    /// Create an HWB color.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, whiteness: T, blackness: T) -> Self {
        Self::new_const(hue.into(), whiteness, blackness)
    }

    /// Create an HWB color. This is the same as `Hwb::new` without the generic
    /// hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(hue: RgbHue<T>, whiteness: T, blackness: T) -> Self {
        Hwb {
            hue,
            whiteness,
            blackness,
            standard: PhantomData,
        }
    }

    /// Convert to a `(hue, whiteness, blackness)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T) {
        (self.hue, self.whiteness, self.blackness)
    }

    /// Convert from a `(hue, whiteness, blackness)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>((hue, whiteness, blackness): (H, T, T)) -> Self {
        Self::new(hue, whiteness, blackness)
    }

    #[inline]
    fn reinterpret_as<St: RgbStandard<T>>(self) -> Hwb<St, T> {
        Hwb {
            hue: self.hue,
            whiteness: self.whiteness,
            blackness: self.blackness,
            standard: PhantomData,
        }
    }
}

impl<S, T> Hwb<S, T>
where
    T: Component,
{
    /// Return the `whiteness` value minimum.
    pub fn min_whiteness() -> T {
        T::zero()
    }

    /// Return the `whiteness` value maximum.
    pub fn max_whiteness() -> T {
        T::max_intensity()
    }

    /// Return the `blackness` value minimum.
    pub fn min_blackness() -> T {
        T::zero()
    }

    /// Return the `blackness` value maximum.
    pub fn max_blackness() -> T {
        T::max_intensity()
    }
}

impl<S, T> PartialEq for Hwb<S, T>
where
    T: PartialEq,
    RgbHue<T>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.hue == other.hue
            && self.whiteness == other.whiteness
            && self.blackness == other.blackness
    }
}

impl<S, T> Eq for Hwb<S, T>
where
    T: Eq,
    RgbHue<T>: Eq,
{
}

///<span id="Hwba"></span>[`Hwba`](crate::Hwba) implementations.
impl<T, A> Alpha<Hwb<Srgb, T>, A> {
    /// Create an sRGB HWB color with transparency. This method can be used
    /// instead of `Hwba::new` to help type inference.
    pub fn new_srgb<H: Into<RgbHue<T>>>(hue: H, whiteness: T, blackness: T, alpha: A) -> Self {
        Self::new_const(hue.into(), whiteness, blackness, alpha)
    }

    /// Create an sRGB HWB color with transparency. This is the same as
    /// `Hwba::new_srgb` without the generic hue type. It's temporary until `const
    /// fn` supports traits.
    pub const fn new_srgb_const(hue: RgbHue<T>, whiteness: T, blackness: T, alpha: A) -> Self {
        Self::new_const(hue, whiteness, blackness, alpha)
    }
}

///<span id="Hwba"></span>[`Hwba`](crate::Hwba) implementations.
impl<S, T, A> Alpha<Hwb<S, T>, A> {
    /// Create an HWB color with transparency.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, whiteness: T, blackness: T, alpha: A) -> Self {
        Self::new_const(hue.into(), whiteness, blackness, alpha)
    }

    /// Create an HWB color with transparency. This is the same as `Hwba::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(hue: RgbHue<T>, whiteness: T, blackness: T, alpha: A) -> Self {
        Alpha {
            color: Hwb::new_const(hue, whiteness, blackness),
            alpha,
        }
    }

    /// Convert to a `(hue, whiteness, blackness, alpha)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T, A) {
        (
            self.color.hue,
            self.color.whiteness,
            self.color.blackness,
            self.alpha,
        )
    }

    /// Convert from a `(hue, whiteness, blackness, alpha)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>(
        (hue, whiteness, blackness, alpha): (H, T, T, A),
    ) -> Self {
        Self::new(hue, whiteness, blackness, alpha)
    }
}

impl<S1, S2, T> FromColorUnclamped<Hwb<S1, T>> for Hwb<S2, T>
where
    S1: RgbStandard<T>,
    S2: RgbStandard<T>,
    S1::Space: RgbSpace<T, WhitePoint = <S2::Space as RgbSpace<T>>::WhitePoint>,
    T: FloatComponent,
{
    fn from_color_unclamped(hwb: Hwb<S1, T>) -> Self {
        if TypeId::of::<S1>() == TypeId::of::<S2>() {
            hwb.reinterpret_as()
        } else {
            let hsv = Hsv::<S1, T>::from_color_unclamped(hwb);
            let converted_hsv = Hsv::<S2, T>::from_color_unclamped(hsv);
            Self::from_color_unclamped(converted_hsv)
        }
    }
}

impl<S, T> FromColorUnclamped<Hsv<S, T>> for Hwb<S, T>
where
    T: FloatComponent,
{
    fn from_color_unclamped(color: Hsv<S, T>) -> Self {
        Hwb {
            hue: color.hue,
            whiteness: (T::one() - color.saturation) * color.value,
            blackness: (T::one() - color.value),
            standard: PhantomData,
        }
    }
}

impl<S, T, H: Into<RgbHue<T>>> From<(H, T, T)> for Hwb<S, T> {
    fn from(components: (H, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<S, T> From<Hwb<S, T>> for (RgbHue<T>, T, T) {
    fn from(color: Hwb<S, T>) -> (RgbHue<T>, T, T) {
        color.into_components()
    }
}

impl<S, T, H: Into<RgbHue<T>>, A> From<(H, T, T, A)> for Alpha<Hwb<S, T>, A> {
    fn from(components: (H, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<S, T, A> From<Alpha<Hwb<S, T>, A>> for (RgbHue<T>, T, T, A) {
    fn from(color: Alpha<Hwb<S, T>, A>) -> (RgbHue<T>, T, T, A) {
        color.into_components()
    }
}

impl<S, T> Clamp for Hwb<S, T>
where
    T: Component + DivAssign,
{
    #[rustfmt::skip]
    fn is_within_bounds(&self) -> bool {
        self.blackness >= Self::min_blackness() && self.blackness <= Self::max_blackness() &&
        self.whiteness >= Self::min_whiteness() && self.whiteness <= Self::max_blackness() &&
        self.whiteness + self.blackness <= T::max_intensity()
    }

    fn clamp(&self) -> Hwb<S, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.whiteness = if self.whiteness < Self::min_whiteness() {
            Self::min_whiteness()
        } else {
            self.whiteness
        };
        self.blackness = if self.blackness < Self::min_blackness() {
            Self::min_blackness()
        } else {
            self.blackness
        };
        let sum = self.blackness + self.whiteness;
        if sum > T::max_intensity() {
            self.whiteness /= sum;
            self.blackness /= sum;
        }
    }
}

impl<S, T> Mix for Hwb<S, T>
where
    T: FloatComponent,
{
    type Scalar = T;

    fn mix(&self, other: &Hwb<S, T>, factor: T) -> Hwb<S, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();

        Hwb {
            hue: self.hue + factor * hue_diff,
            whiteness: self.whiteness + factor * (other.whiteness - self.whiteness),
            blackness: self.blackness + factor * (other.blackness - self.blackness),
            standard: PhantomData,
        }
    }
}

impl<S, T> Shade for Hwb<S, T>
where
    T: FloatComponent,
{
    type Scalar = T;

    fn lighten(&self, factor: T) -> Hwb<S, T> {
        let difference_whiteness = if factor >= T::zero() {
            T::max_intensity() - self.whiteness
        } else {
            self.whiteness
        };
        let delta_whiteness = difference_whiteness.max(T::zero()) * factor;

        let difference_blackness = if factor >= T::zero() {
            self.blackness
        } else {
            T::max_intensity() - self.blackness
        };
        let delta_blackness = difference_blackness.max(T::zero()) * factor;

        Hwb {
            hue: self.hue,
            whiteness: (self.whiteness + delta_whiteness).max(T::zero()),
            blackness: (self.blackness - delta_blackness).max(T::zero()),
            standard: PhantomData,
        }
    }

    fn lighten_fixed(&self, amount: T) -> Hwb<S, T> {
        Hwb {
            hue: self.hue,
            whiteness: (self.whiteness + T::max_intensity() * amount).max(T::zero()),
            blackness: (self.blackness - T::max_intensity() * amount).max(T::zero()),
            standard: PhantomData,
        }
    }
}

impl<S, T> GetHue for Hwb<S, T>
where
    T: Component,
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        if self.whiteness + self.blackness >= T::max_intensity() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<S, T> Hue for Hwb<S, T>
where
    T: Component,
{
    fn with_hue<H: Into<Self::Hue>>(&self, hue: H) -> Hwb<S, T> {
        Hwb {
            hue: hue.into(),
            whiteness: self.whiteness,
            blackness: self.blackness,
            standard: PhantomData,
        }
    }

    fn shift_hue<H: Into<Self::Hue>>(&self, amount: H) -> Hwb<S, T> {
        Hwb {
            hue: self.hue + amount.into(),
            whiteness: self.whiteness,
            blackness: self.blackness,
            standard: PhantomData,
        }
    }
}

impl<S, T> Default for Hwb<S, T>
where
    T: Component,
{
    fn default() -> Hwb<S, T> {
        Hwb::new(
            RgbHue::from(T::zero()),
            Self::min_whiteness(),
            Self::max_blackness(),
        )
    }
}

impl_color_add!(Hwb<S, T>, [hue, whiteness, blackness], standard);
impl_color_sub!(Hwb<S, T>, [hue, whiteness, blackness], standard);

impl<S, T, P> AsRef<P> for Hwb<S, T>
where
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<S, T, P> AsMut<P> for Hwb<S, T>
where
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<S, T> AbsDiffEq for Hwb<S, T>
where
    T: Component + AbsDiffEq,
    RgbHue<T>: AbsDiffEq<Epsilon = T::Epsilon>,
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[rustfmt::skip]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let equal_shade = self.whiteness.abs_diff_eq(&other.whiteness, epsilon.clone())
            && self.blackness.abs_diff_eq(&other.blackness, epsilon.clone());

        // The hue doesn't matter that much when the color is gray, and may fluctuate
        // due to precision errors. This is a blunt tool, but works for now.
        let is_gray = self.blackness + self.whiteness >= T::max_intensity()
            || other.blackness + other.whiteness >= T::max_intensity();
        if is_gray {
            equal_shade
        } else {
            self.hue.abs_diff_eq(&other.hue, epsilon) && equal_shade
        }
    }
}

impl<S, T> RelativeEq for Hwb<S, T>
where
    T: Component + RelativeEq,
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
        let equal_shade = self.whiteness.relative_eq(&other.whiteness, epsilon.clone(), max_relative.clone())
            && self.blackness.relative_eq(&other.blackness, epsilon.clone(), max_relative.clone());

        // The hue doesn't matter that much when the color is gray, and may fluctuate
        // due to precision errors. This is a blunt tool, but works for now.
        let is_gray = self.blackness + self.whiteness >= T::max_intensity()
            || other.blackness + other.whiteness >= T::max_intensity();
        if is_gray {
            equal_shade
        } else {
            self.hue.relative_eq(&other.hue, epsilon, max_relative) && equal_shade
        }
    }
}

impl<S, T> UlpsEq for Hwb<S, T>
where
    T: Component + UlpsEq,
    RgbHue<T>: UlpsEq + AbsDiffEq<Epsilon = T::Epsilon>,
    T::Epsilon: Clone,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[rustfmt::skip]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let equal_shade = self.whiteness.ulps_eq(&other.whiteness, epsilon.clone(), max_ulps.clone())
            && self.blackness.ulps_eq(&other.blackness, epsilon.clone(), max_ulps.clone());

        // The hue doesn't matter that much when the color is gray, and may fluctuate
        // due to precision errors. This is a blunt tool, but works for now.
        let is_gray = self.blackness + self.whiteness >= T::max_intensity()
            || other.blackness + other.whiteness >= T::max_intensity();
        if is_gray {
            equal_shade
        } else {
            self.hue.ulps_eq(&other.hue, epsilon, max_ulps) && equal_shade
        }
    }
}

impl<S, T> RelativeContrast for Hwb<S, T>
where
    T: FloatComponent,
    S: RgbStandard<T>,
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
impl<S, T> Distribution<Hwb<S, T>> for Standard
where
    T: FloatComponent,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hwb<S, T> {
        Hwb::from_color_unclamped(rng.gen::<Hsv<S, T>>())
    }
}

#[cfg(feature = "random")]
pub struct UniformHwb<S, T>
where
    T: FloatComponent + SampleUniform,
{
    sampler: crate::hsv::UniformHsv<S, T>,
    space: PhantomData<S>,
}

#[cfg(feature = "random")]
impl<S, T> SampleUniform for Hwb<S, T>
where
    T: FloatComponent + SampleUniform,
{
    type Sampler = UniformHwb<S, T>;
}

#[cfg(feature = "random")]
impl<S, T> UniformSampler for UniformHwb<S, T>
where
    T: FloatComponent + SampleUniform,
{
    type X = Hwb<S, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low_input = Hsv::from_color_unclamped(*low_b.borrow());
        let high_input = Hsv::from_color_unclamped(*high_b.borrow());

        let low = Hsv::new(
            low_input.hue,
            low_input.saturation.min(high_input.saturation),
            low_input.value.min(high_input.value),
        );
        let high = Hsv::new(
            high_input.hue,
            low_input.saturation.max(high_input.saturation),
            low_input.value.max(high_input.value),
        );
        let sampler = crate::hsv::UniformHsv::<S, _>::new(low, high);

        UniformHwb {
            sampler,
            space: PhantomData,
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low_input = Hsv::from_color_unclamped(*low_b.borrow());
        let high_input = Hsv::from_color_unclamped(*high_b.borrow());

        let low = Hsv::new(
            low_input.hue,
            low_input.saturation.min(high_input.saturation),
            low_input.value.min(high_input.value),
        );
        let high = Hsv::new(
            high_input.hue,
            low_input.saturation.max(high_input.saturation),
            low_input.value.max(high_input.value),
        );

        let sampler = crate::hsv::UniformHsv::<S, _>::new_inclusive(low, high);

        UniformHwb {
            sampler,
            space: PhantomData,
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hwb<S, T> {
        Hwb::from_color_unclamped(self.sampler.sample(rng))
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<S, T> bytemuck::Zeroable for Hwb<S, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<S: 'static, T> bytemuck::Pod for Hwb<S, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use super::Hwb;
    use crate::{Clamp, FromColor, Srgb};

    #[test]
    fn red() {
        let a = Hwb::from_color(Srgb::new(1.0, 0.0, 0.0));
        let b = Hwb::new_srgb(0.0, 0.0, 0.0);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn orange() {
        let a = Hwb::from_color(Srgb::new(1.0, 0.5, 0.0));
        let b = Hwb::new_srgb(30.0, 0.0, 0.0);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn green() {
        let a = Hwb::from_color(Srgb::new(0.0, 1.0, 0.0));
        let b = Hwb::new_srgb(120.0, 0.0, 0.0);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn blue() {
        let a = Hwb::from_color(Srgb::new(0.0, 0.0, 1.0));
        let b = Hwb::new_srgb(240.0, 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn purple() {
        let a = Hwb::from_color(Srgb::new(0.5, 0.0, 1.0));
        let b = Hwb::new_srgb(270.0, 0.0, 0.0);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn clamp_invalid() {
        let expected = Hwb::new_srgb(240.0, 0.0, 0.0);

        let a = Hwb::new_srgb(240.0, -3.0, -4.0);
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);
    }

    #[test]
    fn clamp_none() {
        let expected = Hwb::new_srgb(240.0, 0.3, 0.7);

        let a = Hwb::new_srgb(240.0, 0.3, 0.7);
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);
    }
    #[test]
    fn clamp_over_one() {
        let expected = Hwb::new_srgb(240.0, 0.2, 0.8);

        let a = Hwb::new_srgb(240.0, 5.0, 20.0);
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);
    }
    #[test]
    fn clamp_under_one() {
        let expected = Hwb::new_srgb(240.0, 0.3, 0.1);

        let a = Hwb::new_srgb(240.0, 0.3, 0.1);
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);
    }

    raw_pixel_conversion_tests!(Hwb<crate::encoding::Srgb>: hue, whiteness, blackness);
    raw_pixel_conversion_fail_tests!(Hwb<crate::encoding::Srgb>: hue, whiteness, blackness);

    #[test]
    fn check_min_max_components() {
        use crate::encoding::Srgb;

        assert_relative_eq!(Hwb::<Srgb>::min_whiteness(), 0.0,);
        assert_relative_eq!(Hwb::<Srgb>::min_blackness(), 0.0,);
        assert_relative_eq!(Hwb::<Srgb>::max_whiteness(), 1.0,);
        assert_relative_eq!(Hwb::<Srgb>::max_blackness(), 1.0,);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Hwb::new_srgb(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"hue":0.3,"whiteness":0.8,"blackness":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Hwb =
            ::serde_json::from_str(r#"{"hue":0.3,"whiteness":0.8,"blackness":0.1}"#).unwrap();

        assert_eq!(deserialized, Hwb::new(0.3, 0.8, 0.1));
    }

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Hwb<crate::encoding::Srgb, f32> as crate::rgb::Rgb {
            red: (0.0, 1.0),
            green: (0.0, 1.0),
            blue: (0.0, 1.0)
        },
        min: Hwb::new(0.0f32, 0.0, 0.0),
        max: Hwb::new(360.0, 1.0, 1.0)
    }
}
