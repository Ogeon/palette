use core::ops::{Add, AddAssign, BitAnd, Sub, SubAssign};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
#[cfg(feature = "random")]
use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};

#[cfg(feature = "random")]
use crate::num::Sqrt;

use crate::{
    angle::{RealAngle, SignedAngle},
    bool_mask::{HasBoolMask, LazySelect},
    clamp, clamp_assign, contrast_ratio,
    convert::FromColorUnclamped,
    num::{
        self, Arithmetics, FromScalarArray, Hypot, IntoScalarArray, MinMax, One, PartialCmp, Real,
        Zero,
    },
    white_point::D65,
    Alpha, Clamp, ClampAssign, FromColor, GetHue, IsWithinBounds, Lighten, LightenAssign, Mix,
    MixAssign, Oklab, OklabHue, RelativeContrast, Saturate, SaturateAssign, SetHue, ShiftHue,
    ShiftHueAssign, WithHue, Xyz,
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
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
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
    T: Zero + One,
{
    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::zero()
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        T::one()
    }

    /// Return the `chroma` value minimum.
    pub fn min_chroma() -> T {
        T::zero()
    }

    /// Return the `chroma` value maximum.
    pub fn max_chroma() -> T {
        T::one()
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
    Oklab<T>: FromColorUnclamped<Xyz<D65, T>>,
    Self: FromColorUnclamped<Oklab<T>>,
{
    fn from_color_unclamped(color: Xyz<D65, T>) -> Self {
        let lab = Oklab::<T>::from_color_unclamped(color);
        Self::from_color_unclamped(lab)
    }
}

impl<T> FromColorUnclamped<Oklab<T>> for Oklch<T>
where
    T: Zero + Hypot,
    Oklab<T>: GetHue<Hue = OklabHue<T>>,
{
    fn from_color_unclamped(color: Oklab<T>) -> Self {
        Oklch {
            hue: color.get_hue(),
            l: color.l,
            chroma: color.a.hypot(color.b),
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

impl_is_within_bounds! {
    Oklch {
        l => [Self::min_l(), Self::max_l()],
        chroma => [Self::min_chroma(), Self::max_chroma()]
    }
    where T: Zero + One
}

impl<T> Clamp for Oklch<T>
where
    T: Zero + One + num::Clamp,
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
    T: Zero + One + num::ClampAssign,
{
    #[inline]
    fn clamp_assign(&mut self) {
        clamp_assign(&mut self.l, Self::min_l(), Self::max_l());
        clamp_assign(&mut self.chroma, Self::min_chroma(), Self::max_chroma());
    }
}

impl_mix_hue!(Oklch { l, chroma });
impl_lighten!(Oklch increase {l => [Self::min_l(), Self::max_l()]} other {hue, chroma} where T: One);
impl_saturate!(Oklch increase {chroma => [Self::min_chroma(), Self::max_chroma()]} other {hue, l} where T: One);

impl<T> GetHue for Oklch<T>
where
    T: Clone,
{
    type Hue = OklabHue<T>;

    #[inline]
    fn get_hue(&self) -> OklabHue<T> {
        self.hue.clone()
    }
}

impl<T, H> WithHue<H> for Oklch<T>
where
    H: Into<OklabHue<T>>,
{
    #[inline]
    fn with_hue(mut self, hue: H) -> Self {
        self.hue = hue.into();
        self
    }
}

impl<T, H> SetHue<H> for Oklch<T>
where
    H: Into<OklabHue<T>>,
{
    #[inline]
    fn set_hue(&mut self, hue: H) {
        self.hue = hue.into();
    }
}

impl<T> ShiftHue for Oklch<T>
where
    T: Add<Output = T>,
{
    type Scalar = T;

    #[inline]
    fn shift_hue(mut self, amount: Self::Scalar) -> Self {
        self.hue = self.hue + amount;
        self
    }
}

impl<T> ShiftHueAssign for Oklch<T>
where
    T: AddAssign,
{
    type Scalar = T;

    #[inline]
    fn shift_hue_assign(&mut self, amount: Self::Scalar) {
        self.hue += amount;
    }
}

impl<T> HasBoolMask for Oklch<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<T> Default for Oklch<T>
where
    T: Zero + One,
    OklabHue<T>: Default,
{
    fn default() -> Oklch<T> {
        Oklch::new(Self::min_l(), Self::min_chroma(), OklabHue::default())
    }
}

impl_color_add!(Oklch<T>, [l, chroma, hue]);
impl_color_sub!(Oklch<T>, [l, chroma, hue]);

impl_array_casts!(Oklch<T>, [T; 3]);
impl_simd_array_conversion_hue!(Oklch, [l, chroma]);

impl_eq_hue!(Oklch, OklabHue, [l, chroma, hue]);

impl<T> RelativeContrast for Oklch<T>
where
    T: Real + Arithmetics + PartialCmp,
    T::Mask: LazySelect<T>,
    Xyz<D65, T>: FromColor<Self>,
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
    T: Sqrt,

    Standard: Distribution<T> + Distribution<OklabHue<T>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Oklch<T> {
        Oklch {
            l: rng.gen(),
            chroma: rng.gen::<T>().sqrt(),
            hue: rng.gen::<OklabHue<T>>(),
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformOklch<T>
where
    T: SampleUniform,
{
    l: Uniform<T>,
    chroma: Uniform<T>,
    hue: crate::hues::UniformOklabHue<T>,
}

#[cfg(feature = "random")]
impl<T> SampleUniform for Oklch<T>
where
    T: Sqrt + core::ops::Mul<Output = T> + Clone + SampleUniform,
    OklabHue<T>: SampleBorrow<OklabHue<T>>,
    crate::hues::UniformOklabHue<T>: UniformSampler<X = OklabHue<T>>,
{
    type Sampler = UniformOklch<T>;
}

#[cfg(feature = "random")]
impl<T> UniformSampler for UniformOklch<T>
where
    T: Sqrt + core::ops::Mul<Output = T> + Clone + SampleUniform,
    OklabHue<T>: SampleBorrow<OklabHue<T>>,
    crate::hues::UniformOklabHue<T>: UniformSampler<X = OklabHue<T>>,
{
    type X = Oklch<T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

        UniformOklch {
            l: Uniform::new::<_, T>(low.l, high.l),
            chroma: Uniform::new::<_, T>(
                low.chroma.clone() * low.chroma,
                high.chroma.clone() * high.chroma,
            ),
            hue: crate::hues::UniformOklabHue::new(low.hue, high.hue),
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

        UniformOklch {
            l: Uniform::new_inclusive::<_, T>(low.l, high.l),
            chroma: Uniform::new_inclusive::<_, T>(
                low.chroma.clone() * low.chroma,
                high.chroma.clone() * high.chroma,
            ),
            hue: crate::hues::UniformOklabHue::new_inclusive(low.hue, high.hue),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Oklch<T> {
        Oklch {
            l: self.l.sample(rng),
            chroma: self.chroma.sample(rng).sqrt(),
            hue: self.hue.sample(rng),
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Oklch<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Oklch<T> where T: bytemuck::Pod {}

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
