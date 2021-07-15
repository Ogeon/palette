use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use num_traits::Zero;
#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::color_difference::ColorDifference;
use crate::color_difference::{get_ciede_difference, LabColorDiff};
use crate::convert::FromColorUnclamped;
use crate::encoding::pixel::RawPixel;
use crate::white_point::{WhitePoint, D65};
use crate::{
    clamp, contrast_ratio, from_f64, Alpha, Clamp, Float, FloatComponent, FromColor, FromF64,
    GetHue, Hue, Lab, LabHue, Mix, Pixel, RelativeContrast, Saturate, Shade, Xyz,
};

/// CIE L\*C\*h° with an alpha component. See the [`Lcha` implementation in
/// `Alpha`](crate::Alpha#Lcha).
pub type Lcha<Wp = D65, T = f32> = Alpha<Lch<Wp, T>, T>;

/// CIE L\*C\*h°, a polar version of [CIE L\*a\*b\*](crate::Lab).
///
/// L\*C\*h° shares its range and perceptual uniformity with L\*a\*b\*, but
/// it's a cylindrical color space, like [HSL](crate::Hsl) and
/// [HSV](crate::Hsv). This gives it the same ability to directly change
/// the hue and colorfulness of a color, while preserving other visual aspects.
#[derive(Debug, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Lab, Lch)
)]
#[repr(C)]
pub struct Lch<Wp = D65, T = f32> {
    /// L\* is the lightness of the color. 0.0 gives absolute black and 100.0
    /// gives the brightest white.
    pub l: T,

    /// C\* is the colorfulness of the color. It's similar to saturation. 0.0
    /// gives gray scale colors, and numbers around 128-181 gives fully
    /// saturated colors. The upper limit of 128 should
    /// include the whole L\*a\*b\* space and some more.
    pub chroma: T,

    /// The hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: LabHue<T>,

    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Lch<Wp, T> where T: Copy {}

impl<Wp, T> Clone for Lch<Wp, T>
where
    T: Clone,
{
    fn clone(&self) -> Lch<Wp, T> {
        Lch {
            l: self.l.clone(),
            chroma: self.chroma.clone(),
            hue: self.hue.clone(),
            white_point: PhantomData,
        }
    }
}

impl<T> Lch<D65, T> {
    /// CIE L\*C\*h° with white point D65.
    pub fn new<H: Into<LabHue<T>>>(l: T, chroma: T, hue: H) -> Lch<D65, T> {
        Lch {
            l,
            chroma,
            hue: hue.into(),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Lch<Wp, T> {
    /// CIE L\*C\*h°.
    pub fn with_wp<H: Into<LabHue<T>>>(l: T, chroma: T, hue: H) -> Lch<Wp, T> {
        Lch {
            l,
            chroma,
            hue: hue.into(),
            white_point: PhantomData,
        }
    }

    /// Convert to a `(L\*, C\*, h°)` tuple.
    pub fn into_components(self) -> (T, T, LabHue<T>) {
        (self.l, self.chroma, self.hue)
    }

    /// Convert from a `(L\*, C\*, h°)` tuple.
    pub fn from_components<H: Into<LabHue<T>>>((l, chroma, hue): (T, T, H)) -> Self {
        Self::with_wp(l, chroma, hue)
    }
}

impl<Wp, T> Lch<Wp, T>
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

    /// Return the `chroma` value maximum. This value does not cover the entire
    /// color space, but covers enough to be practical for downsampling to
    /// smaller color spaces like sRGB.
    pub fn max_chroma() -> T {
        from_f64(128.0)
    }

    /// Return the `chroma` extended maximum value. This value covers the entire
    /// color space and is included for completeness, but the additional range
    /// should be unnecessary for most use cases.
    pub fn max_extended_chroma() -> T {
        from_f64(crate::float::Float::sqrt(128.0f64 * 128.0 + 128.0 * 128.0))
    }
}

///<span id="Lcha"></span>[`Lcha`](crate::Lcha) implementations.
impl<T, A> Alpha<Lch<D65, T>, A> {
    /// CIE L\*C\*h° and transparency with white point D65.
    pub fn new<H: Into<LabHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Alpha {
            color: Lch::new(l, chroma, hue),
            alpha,
        }
    }
}

///<span id="Lcha"></span>[`Lcha`](crate::Lcha) implementations.
impl<Wp, T, A> Alpha<Lch<Wp, T>, A> {
    /// CIE L\*C\*h° and transparency.
    pub fn with_wp<H: Into<LabHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Alpha {
            color: Lch::with_wp(l, chroma, hue),
            alpha,
        }
    }

    /// Convert to a `(L\*, C\*, h°, alpha)` tuple.
    pub fn into_components(self) -> (T, T, LabHue<T>, A) {
        (self.color.l, self.color.chroma, self.color.hue, self.alpha)
    }

    /// Convert from a `(L\*, C\*, h°, alpha)` tuple.
    pub fn from_components<H: Into<LabHue<T>>>((l, chroma, hue, alpha): (T, T, H, A)) -> Self {
        Self::with_wp(l, chroma, hue, alpha)
    }
}

impl<Wp, T> FromColorUnclamped<Lch<Wp, T>> for Lch<Wp, T> {
    fn from_color_unclamped(color: Lch<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T> FromColorUnclamped<Lab<Wp, T>> for Lch<Wp, T>
where
    T: FloatComponent,
{
    fn from_color_unclamped(color: Lab<Wp, T>) -> Self {
        Lch {
            l: color.l,
            chroma: (color.a * color.a + color.b * color.b).sqrt(),
            hue: color.get_hue().unwrap_or_else(|| LabHue::from(T::zero())),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T, H: Into<LabHue<T>>> From<(T, T, H)> for Lch<Wp, T> {
    fn from(components: (T, T, H)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp, T> From<Lch<Wp, T>> for (T, T, LabHue<T>) {
    fn from(color: Lch<Wp, T>) -> (T, T, LabHue<T>) {
        color.into_components()
    }
}

impl<Wp, T, H: Into<LabHue<T>>, A> From<(T, T, H, A)> for Alpha<Lch<Wp, T>, A> {
    fn from(components: (T, T, H, A)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp, T, A> From<Alpha<Lch<Wp, T>, A>> for (T, T, LabHue<T>, A) {
    fn from(color: Alpha<Lch<Wp, T>, A>) -> (T, T, LabHue<T>, A) {
        color.into_components()
    }
}

impl<Wp, T> Clamp for Lch<Wp, T>
where
    T: Zero + FromF64 + PartialOrd + Clone,
{
    fn is_within_bounds(&self) -> bool {
        self.l >= Self::min_l() && self.l <= Self::max_l() && self.chroma >= Self::min_chroma()
    }

    fn clamp(&self) -> Lch<Wp, T> {
        let mut c = self.clone();
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l.clone(), Self::min_l(), Self::max_l());
        if self.chroma < Self::min_chroma() {
            self.chroma = Self::min_chroma()
        }
    }
}

impl<Wp, T> Mix for Lch<Wp, T>
where
    T: FloatComponent,
{
    type Scalar = T;

    fn mix(&self, other: &Lch<Wp, T>, factor: T) -> Lch<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();
        Lch {
            l: self.l + factor * (other.l - self.l),
            chroma: self.chroma + factor * (other.chroma - self.chroma),
            hue: self.hue + factor * hue_diff,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Lch<Wp, T>
where
    T: FloatComponent,
{
    type Scalar = T;

    fn lighten(&self, factor: T) -> Lch<Wp, T> {
        let difference = if factor >= T::zero() {
            Self::max_l() - self.l
        } else {
            self.l
        };

        let delta = difference.max(T::zero()) * factor;

        Lch {
            l: (self.l + delta).max(Self::min_l()),
            chroma: self.chroma,
            hue: self.hue,
            white_point: PhantomData,
        }
    }

    fn lighten_fixed(&self, amount: T) -> Lch<Wp, T> {
        Lch {
            l: (self.l + Self::max_l() * amount).max(Self::min_l()),
            chroma: self.chroma,
            hue: self.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> GetHue for Lch<Wp, T>
where
    T: Float + FromF64 + PartialOrd,
{
    type Hue = LabHue<T>;

    fn get_hue(&self) -> Option<LabHue<T>> {
        if self.chroma <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<Wp, T> Hue for Lch<Wp, T>
where
    T: Float + FromF64 + PartialOrd,
{
    fn with_hue<H: Into<Self::Hue>>(&self, hue: H) -> Lch<Wp, T> {
        Lch {
            l: self.l,
            chroma: self.chroma,
            hue: hue.into(),
            white_point: PhantomData,
        }
    }

    fn shift_hue<H: Into<Self::Hue>>(&self, amount: H) -> Lch<Wp, T> {
        Lch {
            l: self.l,
            chroma: self.chroma,
            hue: self.hue + amount.into(),
            white_point: PhantomData,
        }
    }
}

/// CIEDE2000 distance metric for color difference.
impl<Wp, T> ColorDifference for Lch<Wp, T>
where
    T: FloatComponent,
{
    type Scalar = T;

    fn get_color_difference(&self, other: &Lch<Wp, T>) -> Self::Scalar {
        // Prepare a* and b* from Lch components to calculate color difference
        let self_a = clamp(
            self.chroma.max(T::zero()) * self.hue.to_radians().cos(),
            Lab::<Wp, T>::min_a(),
            Lab::<Wp, T>::max_a(),
        );
        let self_b = clamp(
            self.chroma.max(T::zero()) * self.hue.to_radians().sin(),
            Lab::<Wp, T>::min_b(),
            Lab::<Wp, T>::max_b(),
        );
        let other_a = clamp(
            other.chroma.max(T::zero()) * other.hue.to_radians().cos(),
            Lab::<Wp, T>::min_a(),
            Lab::<Wp, T>::max_a(),
        );
        let other_b = clamp(
            other.chroma.max(T::zero()) * other.hue.to_radians().sin(),
            Lab::<Wp, T>::min_b(),
            Lab::<Wp, T>::max_b(),
        );
        let self_params = LabColorDiff {
            l: self.l,
            a: self_a,
            b: self_b,
            chroma: self.chroma,
        };
        let other_params = LabColorDiff {
            l: other.l,
            a: other_a,
            b: other_b,
            chroma: other.chroma,
        };

        get_ciede_difference(&self_params, &other_params)
    }
}

impl<Wp, T> Saturate for Lch<Wp, T>
where
    T: FloatComponent,
{
    type Scalar = T;

    fn saturate(&self, factor: T) -> Lch<Wp, T> {
        let difference = if factor >= T::zero() {
            Self::max_chroma() - self.chroma
        } else {
            self.chroma
        };

        let delta = difference.max(T::zero()) * factor;

        Lch {
            l: self.l,
            chroma: (self.chroma + delta).max(Self::min_chroma()),
            hue: self.hue,
            white_point: PhantomData,
        }
    }

    fn saturate_fixed(&self, amount: T) -> Lch<Wp, T> {
        Lch {
            l: self.l,
            chroma: (self.chroma + Self::max_chroma() * amount).max(Self::min_chroma()),
            hue: self.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Lch<Wp, T>
where
    T: Zero,
{
    fn default() -> Lch<Wp, T> {
        Lch::with_wp(T::zero(), T::zero(), LabHue::from(T::zero()))
    }
}

impl_color_add!(Lch<Wp, T>, [l, chroma, hue], white_point);
impl_color_sub!(Lch<Wp, T>, [l, chroma, hue], white_point);

impl<Wp, T, P> AsRef<P> for Lch<Wp, T>
where
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<Wp, T, P> AsMut<P> for Lch<Wp, T>
where
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<Wp, T> RelativeContrast for Lch<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    type Scalar = T;

    fn get_contrast_ratio(&self, other: &Self) -> T {
        let xyz1 = Xyz::from_color(*self);
        let xyz2 = Xyz::from_color(*other);

        contrast_ratio(xyz1.y, xyz2.y)
    }
}

#[cfg(feature = "random")]
impl<Wp, T> Distribution<Lch<Wp, T>> for Standard
where
    T: FloatComponent,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Lch<Wp, T> {
        Lch {
            l: rng.gen() * Lch::<Wp, T>::max_l(),
            chroma: crate::Float::sqrt(rng.gen()) * Lch::<Wp, T>::max_chroma(),
            hue: rng.gen::<LabHue<T>>(),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformLch<Wp, T>
where
    T: FloatComponent + SampleUniform,
{
    l: Uniform<T>,
    chroma: Uniform<T>,
    hue: crate::hues::UniformLabHue<T>,
    white_point: PhantomData<Wp>,
}

#[cfg(feature = "random")]
impl<Wp, T> SampleUniform for Lch<Wp, T>
where
    T: FloatComponent + SampleUniform,
{
    type Sampler = UniformLch<Wp, T>;
}

#[cfg(feature = "random")]
impl<Wp, T> UniformSampler for UniformLch<Wp, T>
where
    T: FloatComponent + SampleUniform,
{
    type X = Lch<Wp, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        UniformLch {
            l: Uniform::new::<_, T>(low.l, high.l),
            chroma: Uniform::new::<_, T>(low.chroma * low.chroma, high.chroma * high.chroma),
            hue: crate::hues::UniformLabHue::new(low.hue, high.hue),
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

        UniformLch {
            l: Uniform::new_inclusive::<_, T>(low.l, high.l),
            chroma: Uniform::new_inclusive::<_, T>(
                low.chroma * low.chroma,
                high.chroma * high.chroma,
            ),
            hue: crate::hues::UniformLabHue::new_inclusive(low.hue, high.hue),
            white_point: PhantomData,
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Lch<Wp, T> {
        Lch {
            l: self.l.sample(rng),
            chroma: crate::Float::sqrt(self.chroma.sample(rng)),
            hue: self.hue.sample(rng),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Zeroable for Lch<Wp, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp: 'static, T> bytemuck::Pod for Lch<Wp, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use crate::white_point::D65;
    use crate::Lch;

    #[test]
    fn ranges() {
        assert_ranges! {
            Lch<D65, f64>;
            clamped {
                l: 0.0 => 100.0
            }
            clamped_min {
                chroma: 0.0 => 200.0
            }
            unclamped {
                hue: -360.0 => 360.0
            }
        }
    }

    raw_pixel_conversion_tests!(Lch<D65>: l, chroma, hue);
    raw_pixel_conversion_fail_tests!(Lch<D65>: l, chroma, hue);

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Lch::<D65, f32>::min_l(), 0.0);
        assert_relative_eq!(Lch::<D65, f32>::max_l(), 100.0);
        assert_relative_eq!(Lch::<D65, f32>::min_chroma(), 0.0);
        assert_relative_eq!(Lch::<D65, f32>::max_chroma(), 128.0);
        assert_relative_eq!(Lch::<D65, f32>::max_extended_chroma(), 181.01933598375618);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Lch::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"l":0.3,"chroma":0.8,"hue":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Lch =
            ::serde_json::from_str(r#"{"l":0.3,"chroma":0.8,"hue":0.1}"#).unwrap();

        assert_eq!(deserialized, Lch::new(0.3, 0.8, 0.1));
    }

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Lch<D65, f32> as crate::Lab {
            l: (0.0, 100.0),
            a: (-89.0, 89.0),
            b: (-89.0, 89.0),
        },
        min: Lch::new(0.0f32, 0.0, 0.0),
        max: Lch::new(100.0, 128.0, 360.0)
    }
}
