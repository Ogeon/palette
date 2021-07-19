use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use core::ops::{Add, AddAssign, Sub, SubAssign};
use num_traits::Zero;

#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::convert::{FromColorUnclamped, IntoColorUnclamped};
use crate::encoding::pixel::RawPixel;
use crate::white_point::D65;
use crate::{
    clamp, clamp_assign, contrast_ratio, from_f64, Alpha, Clamp, ClampAssign, FloatComponent,
    FromColor, FromF64, GetHue, Hue, IsWithinBounds, Mix, Oklab, OklabHue, Pixel, RelativeContrast,
    Saturate, Shade, Xyz,
};

/// Oklch with an alpha component. See the [`Oklcha` implementation in
/// `Alpha`](crate::Alpha#Oklcha).
pub type Oklcha<T = f32> = Alpha<Oklch<T>, T>;

/// Oklch, a polar version of [Oklab](crate::Oklab).
///
/// It is Oklab’s equivalent of [CIE L\*C\*h°](crate::Lch).
///
/// It's a cylindrical color space, like [HSL](crate::Hsl) and
/// [HSV](crate::Hsv). This gives it the same ability to directly change
/// the hue and colorfulness of a color, while preserving other visual aspects.
///
/// It assumes a D65 whitepoint and normal well-lit viewing conditions,
/// like Oklab.
#[derive(Debug, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklab, Oklch, Xyz)
)]
#[repr(C)]
pub struct Oklch<T = f32> {
    /// L is the lightness of the color. 0 gives absolute black and 1 gives the brightest white.
    pub l: T,

    /// C is the colorfulness of the color, from greyscale at 0 to the most colorful at 1.
    pub chroma: T,

    /// h is the hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: OklabHue<T>,
}

impl<T> Copy for Oklch<T> where T: Copy {}

impl<T> Clone for Oklch<T>
where
    T: Clone,
{
    fn clone(&self) -> Oklch<T> {
        Oklch {
            l: self.l.clone(),
            chroma: self.chroma.clone(),
            hue: self.hue.clone(),
        }
    }
}

impl<T> PartialEq for Oklch<T>
where
    T: PartialEq,
    OklabHue<T>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.l == other.l && self.chroma == other.chroma && self.hue == other.hue
    }
}

impl<T> Eq for Oklch<T>
where
    T: Eq,
    OklabHue<T>: Eq,
{
}

impl<T> AbsDiffEq for Oklch<T>
where
    T: FloatComponent + AbsDiffEq,
    T::Epsilon: FloatComponent,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        self.l.abs_diff_eq(&other.l, epsilon)
            && self.chroma.abs_diff_eq(&other.chroma, epsilon)
            && self.hue.abs_diff_eq(&other.hue, epsilon)
    }
}

impl<T> RelativeEq for Oklch<T>
where
    T: FloatComponent + RelativeEq,
    T::Epsilon: FloatComponent,
{
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        self.l.relative_eq(&other.l, epsilon, max_relative)
            && self
                .chroma
                .relative_eq(&other.chroma, epsilon, max_relative)
            && self.hue.relative_eq(&other.hue, epsilon, max_relative)
    }
}

impl<T> UlpsEq for Oklch<T>
where
    T: FloatComponent + UlpsEq,
    T::Epsilon: FloatComponent,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        self.l.ulps_eq(&other.l, epsilon, max_ulps)
            && self.chroma.ulps_eq(&other.chroma, epsilon, max_ulps)
            && self.hue.ulps_eq(&other.hue, epsilon, max_ulps)
    }
}

impl<T> Oklch<T> {
    /// Create an Oklch color.
    pub fn new<H: Into<OklabHue<T>>>(l: T, chroma: T, hue: H) -> Self {
        Self::new_const(l, chroma, hue.into())
    }

    /// Create an Oklch color. This is the same as `Oklch::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(l: T, chroma: T, hue: OklabHue<T>) -> Self {
        Oklch { l, chroma, hue }
    }

    /// Convert to a `(L, C, h)` tuple.
    pub fn into_components(self) -> (T, T, OklabHue<T>) {
        (self.l, self.chroma, self.hue)
    }

    /// Convert from a `(L, C, h)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((l, chroma, hue): (T, T, H)) -> Self {
        Self::new(l, chroma, hue)
    }
}

impl<T> Oklch<T>
where
    T: Zero + FromF64,
{
    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::zero()
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        from_f64(1.0)
    }

    /// Return the `chroma` value minimum.
    pub fn min_chroma() -> T {
        T::zero()
    }

    /// Return the `chroma` value maximum.
    pub fn max_chroma() -> T {
        from_f64(1.0)
    }
}

///<span id="Oklcha"></span>[`Oklcha`](crate::Oklcha) implementations.
impl<T, A> Alpha<Oklch<T>, A> {
    /// Create an Oklch color with transparency.
    pub fn new<H: Into<OklabHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Self::new_const(l, chroma, hue.into(), alpha)
    }

    /// Create an Oklch color with transparency. This is the same as
    /// `Oklcha::new` without the generic hue type. It's temporary until `const
    /// fn` supports traits.
    pub const fn new_const(l: T, chroma: T, hue: OklabHue<T>, alpha: A) -> Self {
        Alpha {
            color: Oklch::new_const(l, chroma, hue),
            alpha,
        }
    }

    /// Convert to a `(L, C, h, alpha)` tuple.
    pub fn into_components(self) -> (T, T, OklabHue<T>, A) {
        (self.color.l, self.color.chroma, self.color.hue, self.alpha)
    }

    /// Convert from a `(L, C, h, alpha)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((l, chroma, hue, alpha): (T, T, H, A)) -> Self {
        Self::new(l, chroma, hue, alpha)
    }
}

impl<T> FromColorUnclamped<Oklch<T>> for Oklch<T> {
    fn from_color_unclamped(color: Oklch<T>) -> Self {
        color
    }
}

impl<T> FromColorUnclamped<Xyz<D65, T>> for Oklch<T>
where
    T: FloatComponent,
{
    fn from_color_unclamped(color: Xyz<D65, T>) -> Self {
        let lab: Oklab<T> = color.into_color_unclamped();
        Self::from_color_unclamped(lab)
    }
}

impl<T> FromColorUnclamped<Oklab<T>> for Oklch<T>
where
    T: FloatComponent,
{
    fn from_color_unclamped(color: Oklab<T>) -> Self {
        Oklch {
            l: color.l,
            chroma: (color.a * color.a + color.b * color.b).sqrt(),
            hue: color.get_hue().unwrap_or_else(|| OklabHue::from(T::zero())),
        }
    }
}

impl<T, H: Into<OklabHue<T>>> From<(T, T, H)> for Oklch<T> {
    fn from(components: (T, T, H)) -> Self {
        Self::from_components(components)
    }
}

impl<T> From<Oklch<T>> for (T, T, OklabHue<T>) {
    fn from(color: Oklch<T>) -> (T, T, OklabHue<T>) {
        color.into_components()
    }
}

impl<T, H: Into<OklabHue<T>>, A> From<(T, T, H, A)> for Alpha<Oklch<T>, A> {
    fn from(components: (T, T, H, A)) -> Self {
        Self::from_components(components)
    }
}

impl<T, A> From<Alpha<Oklch<T>, A>> for (T, T, OklabHue<T>, A) {
    fn from(color: Alpha<Oklch<T>, A>) -> (T, T, OklabHue<T>, A) {
        color.into_components()
    }
}

impl<T> IsWithinBounds for Oklch<T>
where
    T: Zero + FromF64 + PartialOrd,
{
    #[inline]
    fn is_within_bounds(&self) -> bool {
        self.l >= Self::min_l()
            && self.l <= Self::max_l()
            && self.chroma >= Self::min_chroma()
            && self.chroma <= Self::max_chroma()
    }
}

impl<T> Clamp for Oklch<T>
where
    T: Zero + FromF64 + PartialOrd,
{
    #[inline]
    fn clamp(self) -> Self {
        Self::new(
            clamp(self.l, Self::min_l(), Self::max_l()),
            clamp(self.chroma, Self::min_chroma(), Self::max_chroma()),
            self.hue,
        )
    }
}

impl<T> ClampAssign for Oklch<T>
where
    T: Zero + FromF64 + PartialOrd,
{
    #[inline]
    fn clamp_assign(&mut self) {
        clamp_assign(&mut self.l, Self::min_l(), Self::max_l());
        clamp_assign(&mut self.chroma, Self::min_chroma(), Self::max_chroma());
    }
}

impl<T> Mix for Oklch<T>
where
    T: FloatComponent,
{
    type Scalar = T;

    #[inline]
    fn mix(self, other: Oklch<T>, factor: T) -> Oklch<T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();
        Oklch {
            l: self.l + factor * (other.l - self.l),
            chroma: self.chroma + factor * (other.chroma - self.chroma),
            hue: self.hue + factor * hue_diff,
        }
    }
}

impl<T> Shade for Oklch<T>
where
    T: FloatComponent,
{
    type Scalar = T;

    #[inline]
    fn lighten(self, factor: T) -> Oklch<T> {
        let difference = if factor >= T::zero() {
            Self::max_l() - self.l
        } else {
            self.l
        };

        let delta = difference.max(T::zero()) * factor;

        Oklch {
            l: (self.l + delta).max(Self::min_l()),
            chroma: self.chroma,
            hue: self.hue,
        }
    }

    #[inline]
    fn lighten_fixed(self, amount: T) -> Oklch<T> {
        Oklch {
            l: (self.l + Self::max_l() * amount).max(Self::min_l()),
            chroma: self.chroma,
            hue: self.hue,
        }
    }
}

impl<T> GetHue for Oklch<T>
where
    T: Zero + PartialOrd + Clone,
{
    type Hue = OklabHue<T>;

    fn get_hue(&self) -> Option<OklabHue<T>> {
        if self.chroma <= T::zero() {
            None
        } else {
            Some(self.hue.clone())
        }
    }
}

impl<T> Hue for Oklch<T>
where
    T: Zero + PartialOrd + Clone,
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

impl<T> Saturate for Oklch<T>
where
    T: FloatComponent,
{
    type Scalar = T;

    #[inline]
    fn saturate(self, factor: T) -> Oklch<T> {
        let difference = if factor >= T::zero() {
            Self::max_chroma() - self.chroma
        } else {
            self.chroma
        };

        let delta = difference.max(T::zero()) * factor;

        Oklch {
            l: self.l,
            chroma: (self.chroma + delta).max(Self::min_chroma()),
            hue: self.hue,
        }
    }

    #[inline]
    fn saturate_fixed(self, amount: T) -> Oklch<T> {
        Oklch {
            l: self.l,
            chroma: (self.chroma + Self::max_chroma() * amount).max(Self::min_chroma()),
            hue: self.hue,
        }
    }
}

impl<T> Default for Oklch<T>
where
    T: Zero,
{
    fn default() -> Oklch<T> {
        Oklch::new(T::zero(), T::zero(), OklabHue::from(T::zero()))
    }
}

impl_color_add!(Oklch<T>, [l, chroma, hue]);
impl_color_sub!(Oklch<T>, [l, chroma, hue]);

impl<T, P> AsRef<P> for Oklch<T>
where
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<T, P> AsMut<P> for Oklch<T>
where
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<T> RelativeContrast for Oklch<T>
where
    T: FloatComponent,
{
    type Scalar = T;

    #[inline]
    fn get_contrast_ratio(self, other: Self) -> T {
        let xyz1 = Xyz::from_color(self);
        let xyz2 = Xyz::from_color(other);

        contrast_ratio(xyz1.y, xyz2.y)
    }
}

#[cfg(feature = "random")]
impl<T> Distribution<Oklch<T>> for Standard
where
    T: FloatComponent,

    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Oklch<T> {
        Oklch {
            l: rng.gen(),
            chroma: crate::Float::sqrt(rng.gen()),
            hue: rng.gen::<OklabHue<T>>(),
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformOklch<T>
where
    T: FloatComponent + SampleUniform,
{
    l: Uniform<T>,
    chroma: Uniform<T>,
    hue: crate::hues::UniformOklabHue<T>,
}

#[cfg(feature = "random")]
impl<T> SampleUniform for Oklch<T>
where
    T: FloatComponent + SampleUniform,
{
    type Sampler = UniformOklch<T>;
}

#[cfg(feature = "random")]
impl<T> UniformSampler for UniformOklch<T>
where
    T: FloatComponent + SampleUniform,
{
    type X = Oklch<T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        UniformOklch {
            l: Uniform::new::<_, T>(low.l, high.l),
            chroma: Uniform::new::<_, T>(low.chroma * low.chroma, high.chroma * high.chroma),
            hue: crate::hues::UniformOklabHue::new(low.hue, high.hue),
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        UniformOklch {
            l: Uniform::new_inclusive::<_, T>(low.l, high.l),
            chroma: Uniform::new_inclusive::<_, T>(
                low.chroma * low.chroma,
                high.chroma * high.chroma,
            ),
            hue: crate::hues::UniformOklabHue::new_inclusive(low.hue, high.hue),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Oklch<T> {
        Oklch {
            l: self.l.sample(rng),
            chroma: crate::Float::sqrt(self.chroma.sample(rng)),
            hue: self.hue.sample(rng),
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Oklch<T> where T: FloatComponent + bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Oklch<T> where T: FloatComponent + bytemuck::Pod {}

#[cfg(test)]
mod test {
    use crate::Oklch;

    #[test]
    fn ranges() {
        assert_ranges! {
            Oklch< f64>;
            clamped {
                l: 0.0 => 1.0,
                chroma: 0.0 => 1.0
            }
            clamped_min {}
            unclamped {
                hue: -360.0 => 360.0
            }
        }
    }

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Oklch::<f32>::min_l(), 0.0);
        assert_relative_eq!(Oklch::<f32>::max_l(), 1.0);
        assert_relative_eq!(Oklch::<f32>::min_chroma(), 0.0);
        assert_relative_eq!(Oklch::<f32>::max_chroma(), 1.0);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Oklch::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"l":0.3,"chroma":0.8,"hue":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Oklch =
            ::serde_json::from_str(r#"{"l":0.3,"chroma":0.8,"hue":0.1}"#).unwrap();

        assert_eq!(deserialized, Oklch::new(0.3, 0.8, 0.1));
    }

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Oklch<f32> as crate::Oklab {
            l: (0.0, 1.0),
            a: (-0.7, 0.7),
            b: (-0.7, 0.7),
        },
        min: Oklch::new(0.0f32, 0.0, 0.0),
        max: Oklch::new(1.0, 1.0, 360.0)
    }
}
