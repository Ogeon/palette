use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use num_traits::Zero;
#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::convert::FromColorUnclamped;
use crate::encoding::pixel::RawPixel;
use crate::luv_bounds::LuvBounds;
use crate::white_point::{WhitePoint, D65};
use crate::{
    clamp, clamp_assign, clamp_min_assign, contrast_ratio, from_f64, Alpha, Clamp, ClampAssign,
    FloatComponent, FromColor, FromF64, GetHue, Hsluv, IsWithinBounds, Lighten, LightenAssign, Luv,
    LuvHue, Mix, MixAssign, Pixel, RelativeContrast, Saturate, SaturateAssign, SetHue, ShiftHue,
    ShiftHueAssign, WithHue, Xyz,
};

/// CIE L\*C\*uv h°uv with an alpha component. See the [`Lchuva` implementation in
/// `Alpha`](crate::Alpha#Lchuva).
pub type Lchuva<Wp = D65, T = f32> = Alpha<Lchuv<Wp, T>, T>;

/// CIE L\*C\*uv h°uv, a polar version of [CIE L\*u\*v\*](crate::Lab).
///
/// L\*C\*uv h°uv shares its range and perceptual uniformity with L\*u\*v\*, but
/// it's a cylindrical color space, like [HSL](crate::Hsl) and
/// [HSV](crate::Hsv). This gives it the same ability to directly change
/// the hue and colorfulness of a color, while preserving other visual aspects.
#[derive(Debug, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Luv, Lchuv, Hsluv)
)]
#[repr(C)]
pub struct Lchuv<Wp = D65, T = f32> {
    /// L\* is the lightness of the color. 0.0 gives absolute black and 100.0
    /// gives the brightest white.
    pub l: T,

    /// C\*uv is the colorfulness of the color. It's similar to
    /// saturation. 0.0 gives gray scale colors, and numbers around
    /// 130-180 gives fully saturated colors, depending on the
    /// hue. The upper limit of 180 should include the whole
    /// L\*u\*v\*.
    pub chroma: T,

    /// The hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: LuvHue<T>,

    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Lchuv<Wp, T> where T: Copy {}

impl<Wp, T> Clone for Lchuv<Wp, T>
where
    T: Clone,
{
    fn clone(&self) -> Lchuv<Wp, T> {
        Lchuv {
            l: self.l.clone(),
            chroma: self.chroma.clone(),
            hue: self.hue.clone(),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Lchuv<Wp, T> {
    /// Create a CIE L\*C\*uv h°uv color.
    pub fn new<H: Into<LuvHue<T>>>(l: T, chroma: T, hue: H) -> Self {
        Self::new_const(l, chroma, hue.into())
    }

    /// Create a CIE L\*C\*uv h°uv color. This is the same as `Lchuv::new`
    /// without the generic hue type. It's temporary until `const fn` supports
    /// traits.
    pub const fn new_const(l: T, chroma: T, hue: LuvHue<T>) -> Self {
        Lchuv {
            l,
            chroma,
            hue,
            white_point: PhantomData,
        }
    }

    /// Convert to a `(L\*, C\*uv, h°uv)` tuple.
    pub fn into_components(self) -> (T, T, LuvHue<T>) {
        (self.l, self.chroma, self.hue)
    }

    /// Convert from a `(L\*, C\*uv, h°uv)` tuple.
    pub fn from_components<H: Into<LuvHue<T>>>((l, chroma, hue): (T, T, H)) -> Self {
        Self::new(l, chroma, hue)
    }
}

impl<Wp, T> Lchuv<Wp, T>
where
    T: Zero + FromF64,
{
    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::zero()
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        from_f64(100.0)
    }

    /// Return the `chroma` value minimum.
    pub fn min_chroma() -> T {
        T::zero()
    }

    /// Return the `chroma` value maximum.
    pub fn max_chroma() -> T {
        from_f64(180.0)
    }
}

///<span id="Lchuva"></span>[`Lchuva`](crate::Lchuva) implementations.
impl<Wp, T, A> Alpha<Lchuv<Wp, T>, A> {
    /// Create a CIE L\*C\*uv h°uv color with transparency.
    pub fn new<H: Into<LuvHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Self::new_const(l, chroma, hue.into(), alpha)
    }

    /// Create a CIE L\*C\*uv h°uv color with transparency. This is the same as
    /// `Lchuva::new` without the generic hue type. It's temporary until `const
    /// fn` supports traits.
    pub const fn new_const(l: T, chroma: T, hue: LuvHue<T>, alpha: A) -> Self {
        Alpha {
            color: Lchuv::new_const(l, chroma, hue),
            alpha,
        }
    }

    /// Convert to a `(L\*, C\*uv, h°uv, alpha)` tuple.
    pub fn into_components(self) -> (T, T, LuvHue<T>, A) {
        (self.color.l, self.color.chroma, self.color.hue, self.alpha)
    }

    /// Convert from a `(L\*, C\*uv, h°uv, alpha)` tuple.
    pub fn from_components<H: Into<LuvHue<T>>>((l, chroma, hue, alpha): (T, T, H, A)) -> Self {
        Self::new(l, chroma, hue, alpha)
    }
}

impl<Wp, T> FromColorUnclamped<Lchuv<Wp, T>> for Lchuv<Wp, T> {
    fn from_color_unclamped(color: Lchuv<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T> FromColorUnclamped<Luv<Wp, T>> for Lchuv<Wp, T>
where
    T: FloatComponent,
{
    fn from_color_unclamped(color: Luv<Wp, T>) -> Self {
        Lchuv {
            l: color.l,
            chroma: color.u.hypot(color.v),
            hue: color.get_hue().unwrap_or_else(|| LuvHue::from(T::zero())),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> FromColorUnclamped<Hsluv<Wp, T>> for Lchuv<Wp, T>
where
    T: FloatComponent,
{
    fn from_color_unclamped(color: Hsluv<Wp, T>) -> Self {
        // Apply the given saturation as a percentage of the max
        // chroma for that hue.
        let max_chroma = LuvBounds::from_lightness(color.l).max_chroma_at_hue(color.hue);

        Lchuv::new(
            color.l,
            color.saturation * max_chroma * T::from_f64(0.01),
            color.hue,
        )
    }
}

impl<Wp, T, H: Into<LuvHue<T>>> From<(T, T, H)> for Lchuv<Wp, T> {
    fn from(components: (T, T, H)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp, T> From<Lchuv<Wp, T>> for (T, T, LuvHue<T>) {
    fn from(color: Lchuv<Wp, T>) -> (T, T, LuvHue<T>) {
        color.into_components()
    }
}

impl<Wp, T, H: Into<LuvHue<T>>, A> From<(T, T, H, A)> for Alpha<Lchuv<Wp, T>, A> {
    fn from(components: (T, T, H, A)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp, T, A> From<Alpha<Lchuv<Wp, T>, A>> for (T, T, LuvHue<T>, A) {
    fn from(color: Alpha<Lchuv<Wp, T>, A>) -> (T, T, LuvHue<T>, A) {
        color.into_components()
    }
}

impl<Wp, T> IsWithinBounds for Lchuv<Wp, T>
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

impl<Wp, T> Clamp for Lchuv<Wp, T>
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

impl<Wp, T> ClampAssign for Lchuv<Wp, T>
where
    T: Zero + FromF64 + PartialOrd,
{
    #[inline]
    fn clamp_assign(&mut self) {
        clamp_assign(&mut self.l, Self::min_l(), Self::max_l());
        clamp_assign(&mut self.chroma, Self::min_chroma(), Self::max_chroma());
    }
}

impl<Wp, T> Mix for Lchuv<Wp, T>
where
    T: FloatComponent,
{
    type Scalar = T;

    #[inline]
    fn mix(self, other: Self, factor: T) -> Self {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff = (other.hue - self.hue).to_degrees();

        Lchuv {
            l: self.l + factor * (other.l - self.l),
            chroma: self.chroma + factor * (other.chroma - self.chroma),
            hue: self.hue + factor * hue_diff,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> MixAssign for Lchuv<Wp, T>
where
    T: FloatComponent + AddAssign,
{
    type Scalar = T;

    #[inline]
    fn mix_assign(&mut self, other: Self, factor: T) {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff = (other.hue - self.hue).to_degrees();

        self.l += factor * (other.l - self.l);
        self.chroma += factor * (other.chroma - self.chroma);
        self.hue += factor * hue_diff;
    }
}

impl<Wp, T> Lighten for Lchuv<Wp, T>
where
    T: FloatComponent,
{
    type Scalar = T;

    #[inline]
    fn lighten(self, factor: T) -> Self {
        let difference = if factor >= T::zero() {
            Self::max_l() - self.l
        } else {
            self.l
        };

        let delta = difference.max(T::zero()) * factor;

        Lchuv {
            l: (self.l + delta).max(Self::min_l()),
            chroma: self.chroma,
            hue: self.hue,
            white_point: PhantomData,
        }
    }

    #[inline]
    fn lighten_fixed(self, amount: T) -> Self {
        Lchuv {
            l: (self.l + Self::max_l() * amount).max(Self::min_l()),
            chroma: self.chroma,
            hue: self.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> LightenAssign for Lchuv<Wp, T>
where
    T: FloatComponent + AddAssign,
{
    type Scalar = T;

    #[inline]
    fn lighten_assign(&mut self, factor: T) {
        let difference = if factor >= T::zero() {
            Self::max_l() - self.l
        } else {
            self.l
        };

        self.l += difference.max(T::zero()) * factor;
        clamp_min_assign(&mut self.l, Self::min_l());
    }

    #[inline]
    fn lighten_fixed_assign(&mut self, amount: T) {
        self.l += Self::max_l() * amount;
        clamp_min_assign(&mut self.l, Self::min_l());
    }
}

impl<Wp, T> GetHue for Lchuv<Wp, T>
where
    T: Zero + PartialOrd + Clone,
{
    type Hue = LuvHue<T>;

    #[inline]
    fn get_hue(&self) -> Option<LuvHue<T>> {
        if self.chroma <= T::zero() {
            None
        } else {
            Some(self.hue.clone())
        }
    }
}

impl<Wp, T, H> WithHue<H> for Lchuv<Wp, T>
where
    H: Into<LuvHue<T>>,
{
    #[inline]
    fn with_hue(mut self, hue: H) -> Self {
        self.hue = hue.into();
        self
    }
}

impl<Wp, T, H> SetHue<H> for Lchuv<Wp, T>
where
    H: Into<LuvHue<T>>,
{
    #[inline]
    fn set_hue(&mut self, hue: H) {
        self.hue = hue.into();
    }
}

impl<Wp, T> ShiftHue for Lchuv<Wp, T>
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

impl<Wp, T> ShiftHueAssign for Lchuv<Wp, T>
where
    T: AddAssign,
{
    type Scalar = T;

    #[inline]
    fn shift_hue_assign(&mut self, amount: Self::Scalar) {
        self.hue += amount;
    }
}

impl<Wp, T> Saturate for Lchuv<Wp, T>
where
    T: FloatComponent,
{
    type Scalar = T;

    #[inline]
    fn saturate(self, factor: T) -> Self {
        let difference = if factor >= T::zero() {
            Self::max_chroma() - self.chroma
        } else {
            self.chroma
        };

        let delta = difference.max(T::zero()) * factor;

        Lchuv {
            l: self.l,
            chroma: (self.chroma + delta).max(T::zero()),
            hue: self.hue,
            white_point: PhantomData,
        }
    }

    #[inline]
    fn saturate_fixed(self, amount: T) -> Self {
        Lchuv {
            l: self.l,
            chroma: (self.chroma + Self::max_chroma() * amount).max(T::zero()),
            hue: self.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> SaturateAssign for Lchuv<Wp, T>
where
    T: FloatComponent + AddAssign,
{
    type Scalar = T;

    #[inline]
    fn saturate_assign(&mut self, factor: T) {
        let difference = if factor >= T::zero() {
            Self::max_chroma() - self.chroma
        } else {
            self.chroma
        };

        self.chroma += difference.max(T::zero()) * factor;
        clamp_min_assign(&mut self.chroma, Self::min_chroma());
    }

    #[inline]
    fn saturate_fixed_assign(&mut self, amount: T) {
        self.chroma += Self::max_chroma() * amount;
        clamp_min_assign(&mut self.chroma, Self::min_chroma());
    }
}

impl<Wp, T> Default for Lchuv<Wp, T>
where
    T: Zero,
{
    fn default() -> Lchuv<Wp, T> {
        Lchuv::new(T::zero(), T::zero(), LuvHue::from(T::zero()))
    }
}

impl_color_add!(Lchuv<Wp, T>, [l, chroma, hue], white_point);
impl_color_sub!(Lchuv<Wp, T>, [l, chroma, hue], white_point);

impl<Wp, T, P> AsRef<P> for Lchuv<Wp, T>
where
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<Wp, T, P> AsMut<P> for Lchuv<Wp, T>
where
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<Wp, T> RelativeContrast for Lchuv<Wp, T>
where
    Wp: WhitePoint<T>,
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
impl<Wp, T> Distribution<Lchuv<Wp, T>> for Standard
where
    T: FloatComponent,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Lchuv<Wp, T> {
        Lchuv {
            l: rng.gen() * Lchuv::<Wp, T>::max_l(),
            chroma: crate::Float::sqrt(rng.gen()) * Lchuv::<Wp, T>::max_chroma(),
            hue: rng.gen::<LuvHue<T>>(),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformLchuv<Wp, T>
where
    T: FloatComponent + SampleUniform,
{
    l: Uniform<T>,
    chroma: Uniform<T>,
    hue: crate::hues::UniformLuvHue<T>,
    white_point: PhantomData<Wp>,
}

#[cfg(feature = "random")]
impl<Wp, T> SampleUniform for Lchuv<Wp, T>
where
    T: FloatComponent + SampleUniform,
{
    type Sampler = UniformLchuv<Wp, T>;
}

#[cfg(feature = "random")]
impl<Wp, T> UniformSampler for UniformLchuv<Wp, T>
where
    T: FloatComponent + SampleUniform,
{
    type X = Lchuv<Wp, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        UniformLchuv {
            l: Uniform::new::<_, T>(low.l, high.l),
            chroma: Uniform::new::<_, T>(low.chroma * low.chroma, high.chroma * high.chroma),
            hue: crate::hues::UniformLuvHue::new(low.hue, high.hue),
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

        UniformLchuv {
            l: Uniform::new_inclusive::<_, T>(low.l, high.l),
            chroma: Uniform::new_inclusive::<_, T>(
                low.chroma * low.chroma,
                high.chroma * high.chroma,
            ),
            hue: crate::hues::UniformLuvHue::new_inclusive(low.hue, high.hue),
            white_point: PhantomData,
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Lchuv<Wp, T> {
        Lchuv {
            l: self.l.sample(rng),
            chroma: crate::Float::sqrt(self.chroma.sample(rng)),
            hue: self.hue.sample(rng),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Zeroable for Lchuv<Wp, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp: 'static, T> bytemuck::Pod for Lchuv<Wp, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use crate::white_point::D65;
    use crate::Lchuv;

    #[test]
    fn ranges() {
        assert_ranges! {
            Lchuv<D65, f64>;
            clamped {
                l: 0.0 => 100.0,
                chroma: 0.0 => 180.0
            }
            clamped_min {
            }
            unclamped {
                hue: -360.0 => 360.0
            }
        }
    }

    /// Check that the arithmetic operations (add/sub) are all
    /// implemented.
    #[test]
    fn test_arithmetic() {
        let lchuv = Lchuv::<D65>::new(120.0, 40.0, 30.0);
        let lchuv2 = Lchuv::new(200.0, 30.0, 40.0);
        let mut _lchuv3 = lchuv + lchuv2;
        _lchuv3 += lchuv2;
        let mut _lchuv4 = lchuv2 + 0.3;
        _lchuv4 += 0.1;

        _lchuv3 = lchuv2 - lchuv;
        _lchuv3 = _lchuv4 - 0.1;
        _lchuv4 -= _lchuv3;
        _lchuv3 -= 0.1;
    }

    raw_pixel_conversion_tests!(Lchuv<D65>: l, chroma, hue);
    raw_pixel_conversion_fail_tests!(Lchuv<D65>: l, chroma, hue);

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Lchuv::<D65, f32>::min_l(), 0.0);
        assert_relative_eq!(Lchuv::<D65, f32>::max_l(), 100.0);
        assert_relative_eq!(Lchuv::<D65, f32>::min_chroma(), 0.0);
        assert_relative_eq!(Lchuv::<D65, f32>::max_chroma(), 180.0);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Lchuv::<D65>::new(80.0, 70.0, 130.0)).unwrap();

        assert_eq!(serialized, r#"{"l":80.0,"chroma":70.0,"hue":130.0}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Lchuv =
            ::serde_json::from_str(r#"{"l":70.0,"chroma":80.0,"hue":130.0}"#).unwrap();

        assert_eq!(deserialized, Lchuv::new(70.0, 80.0, 130.0));
    }

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Lchuv<D65, f32> as crate::Luv {
            l: (0.0, 100.0),
            u: (-80.0, 80.0),
            v: (-80.0, 80.0),
        },
        min: Lchuv::new(0.0f32, 0.0, 0.0),
        max: Lchuv::new(100.0, 180.0, 360.0)
    }
}
