use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::color_difference::ColorDifference;
use crate::color_difference::{get_ciede_difference, LabColorDiff};
use crate::convert::{FromColorUnclamped, IntoColorUnclamped};
use crate::encoding::pixel::RawPixel;
use crate::white_point::{WhitePoint, D65};
use crate::{
    clamp, contrast_ratio, from_f64, Alpha, Clamp, Component, FloatComponent, FromColor, GetHue,
    Hue, Lab, LabHue, Mix, Pixel, RelativeContrast, Saturate, Shade, Xyz,
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
    skip_derives(Xyz, Lab, Lch)
)]
#[repr(C)]
pub struct Lch<Wp = D65, T = f32>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
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

impl<Wp, T> Copy for Lch<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
}

impl<Wp, T> Clone for Lch<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn clone(&self) -> Lch<Wp, T> {
        *self
    }
}

impl<T> Lch<D65, T>
where
    T: FloatComponent,
{
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

impl<Wp, T> Lch<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
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

impl<Wp, T> PartialEq for Lch<Wp, T>
where
    T: FloatComponent + PartialEq,
    Wp: WhitePoint,
{
    fn eq(&self, other: &Self) -> bool {
        self.l == other.l && self.chroma == other.chroma && self.hue == other.hue
    }
}

impl<Wp, T> Eq for Lch<Wp, T>
where
    T: FloatComponent + Eq,
    Wp: WhitePoint,
{
}

///<span id="Lcha"></span>[`Lcha`](crate::Lcha) implementations.
impl<T, A> Alpha<Lch<D65, T>, A>
where
    T: FloatComponent,
    A: Component,
{
    /// CIE L\*C\*h° and transparency with white point D65.
    pub fn new<H: Into<LabHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Alpha {
            color: Lch::new(l, chroma, hue),
            alpha,
        }
    }
}

///<span id="Lcha"></span>[`Lcha`](crate::Lcha) implementations.
impl<Wp, T, A> Alpha<Lch<Wp, T>, A>
where
    T: FloatComponent,
    A: Component,
    Wp: WhitePoint,
{
    /// CIE L\*C\*h° and transparency.
    pub fn with_wp<H: Into<LabHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Alpha {
            color: Lch::with_wp(l, chroma, hue),
            alpha,
        }
    }

    /// Convert to a `(L\*, C\*, h°, alpha)` tuple.
    pub fn into_components(self) -> (T, T, LabHue<T>, A) {
        (self.l, self.chroma, self.hue, self.alpha)
    }

    /// Convert from a `(L\*, C\*, h°, alpha)` tuple.
    pub fn from_components<H: Into<LabHue<T>>>((l, chroma, hue, alpha): (T, T, H, A)) -> Self {
        Self::with_wp(l, chroma, hue, alpha)
    }
}

impl<Wp, T> FromColorUnclamped<Lch<Wp, T>> for Lch<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Lch<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T> FromColorUnclamped<Xyz<Wp, T>> for Lch<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Xyz<Wp, T>) -> Self {
        let lab: Lab<Wp, T> = color.into_color_unclamped();
        Self::from_color_unclamped(lab)
    }
}

impl<Wp, T> FromColorUnclamped<Lab<Wp, T>> for Lch<Wp, T>
where
    Wp: WhitePoint,
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

impl<Wp: WhitePoint, T: FloatComponent, H: Into<LabHue<T>>> From<(T, T, H)> for Lch<Wp, T> {
    fn from(components: (T, T, H)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent> Into<(T, T, LabHue<T>)> for Lch<Wp, T> {
    fn into(self) -> (T, T, LabHue<T>) {
        self.into_components()
    }
}

impl<Wp: WhitePoint, T: FloatComponent, H: Into<LabHue<T>>, A: Component> From<(T, T, H, A)>
    for Alpha<Lch<Wp, T>, A>
{
    fn from(components: (T, T, H, A)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent, A: Component> Into<(T, T, LabHue<T>, A)>
    for Alpha<Lch<Wp, T>, A>
{
    fn into(self) -> (T, T, LabHue<T>, A) {
        self.into_components()
    }
}

impl<Wp, T> Clamp for Lch<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn is_within_bounds(&self) -> bool {
        self.l >= T::zero() && self.l <= from_f64(100.0) && self.chroma >= T::zero()
    }

    fn clamp(&self) -> Lch<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, T::zero(), from_f64(100.0));
        self.chroma = self.chroma.max(T::zero())
    }
}

impl<Wp, T> Mix for Lch<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
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
    Wp: WhitePoint,
{
    type Scalar = T;

    fn lighten(&self, factor: T) -> Lch<Wp, T> {
        let difference = if factor >= T::zero() {
            T::from_f64(100.0) - self.l
        } else {
            self.l
        };

        let delta = difference.max(T::zero()) * factor;

        Lch {
            l: (self.l + delta).max(T::zero()),
            chroma: self.chroma,
            hue: self.hue,
            white_point: PhantomData,
        }
    }

    fn lighten_fixed(&self, amount: T) -> Lch<Wp, T> {
        Lch {
            l: (self.l + T::from_f64(100.0) * amount).max(T::zero()),
            chroma: self.chroma,
            hue: self.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> GetHue for Lch<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
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
    T: FloatComponent,
    Wp: WhitePoint,
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
    Wp: WhitePoint,
{
    type Scalar = T;

    fn get_color_difference(&self, other: &Lch<Wp, T>) -> Self::Scalar {
        // Prepare a* and b* from Lch components to calculate color difference
        let self_a = clamp(
            self.chroma.max(T::zero()) * self.hue.to_radians().cos(),
            from_f64(-128.0),
            from_f64(127.0),
        );
        let self_b = clamp(
            self.chroma.max(T::zero()) * self.hue.to_radians().sin(),
            from_f64(-128.0),
            from_f64(127.0),
        );
        let other_a = clamp(
            other.chroma.max(T::zero()) * other.hue.to_radians().cos(),
            from_f64(-128.0),
            from_f64(127.0),
        );
        let other_b = clamp(
            other.chroma.max(T::zero()) * other.hue.to_radians().sin(),
            from_f64(-128.0),
            from_f64(127.0),
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
    Wp: WhitePoint,
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
            chroma: (self.chroma + delta).max(T::zero()),
            hue: self.hue,
            white_point: PhantomData,
        }
    }

    fn saturate_fixed(&self, amount: T) -> Lch<Wp, T> {
        Lch {
            l: self.l,
            chroma: (self.chroma + Self::max_chroma() * amount).max(T::zero()),
            hue: self.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Lch<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn default() -> Lch<Wp, T> {
        Lch::with_wp(T::zero(), T::zero(), LabHue::from(T::zero()))
    }
}

impl<Wp, T> Add<Lch<Wp, T>> for Lch<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Lch<Wp, T>;

    fn add(self, other: Lch<Wp, T>) -> Self::Output {
        Lch {
            l: self.l + other.l,
            chroma: self.chroma + other.chroma,
            hue: self.hue + other.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Add<T> for Lch<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Lch<Wp, T>;

    fn add(self, c: T) -> Self::Output {
        Lch {
            l: self.l + c,
            chroma: self.chroma + c,
            hue: self.hue + c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> AddAssign<Lch<Wp, T>> for Lch<Wp, T>
where
    T: FloatComponent + AddAssign,
    Wp: WhitePoint,
{
    fn add_assign(&mut self, other: Lch<Wp, T>) {
        self.l += other.l;
        self.chroma += other.chroma;
        self.hue += other.hue;
    }
}

impl<Wp, T> AddAssign<T> for Lch<Wp, T>
where
    T: FloatComponent + AddAssign,
    Wp: WhitePoint,
{
    fn add_assign(&mut self, c: T) {
        self.l += c;
        self.chroma += c;
        self.hue += c;
    }
}

impl<Wp, T> Sub<Lch<Wp, T>> for Lch<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Lch<Wp, T>;

    fn sub(self, other: Lch<Wp, T>) -> Self::Output {
        Lch {
            l: self.l - other.l,
            chroma: self.chroma - other.chroma,
            hue: self.hue - other.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<T> for Lch<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Lch<Wp, T>;

    fn sub(self, c: T) -> Self::Output {
        Lch {
            l: self.l - c,
            chroma: self.chroma - c,
            hue: self.hue - c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> SubAssign<Lch<Wp, T>> for Lch<Wp, T>
where
    T: FloatComponent + SubAssign,
    Wp: WhitePoint,
{
    fn sub_assign(&mut self, other: Lch<Wp, T>) {
        self.l -= other.l;
        self.chroma -= other.chroma;
        self.hue -= other.hue;
    }
}

impl<Wp, T> SubAssign<T> for Lch<Wp, T>
where
    T: FloatComponent + SubAssign,
    Wp: WhitePoint,
{
    fn sub_assign(&mut self, c: T) {
        self.l -= c;
        self.chroma -= c;
        self.hue -= c;
    }
}

impl<Wp, T, P> AsRef<P> for Lch<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<Wp, T, P> AsMut<P> for Lch<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
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
    Wp: WhitePoint,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Lch<Wp, T> {
        Lch {
            l: rng.gen() * from_f64(100.0),
            chroma: crate::Float::sqrt(rng.gen()) * from_f64(128.0),
            hue: rng.gen::<LabHue<T>>(),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformLch<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
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
    Wp: WhitePoint,
{
    type Sampler = UniformLch<Wp, T>;
}

#[cfg(feature = "random")]
impl<Wp, T> UniformSampler for UniformLch<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
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
unsafe impl<Wp, T> bytemuck::Zeroable for Lch<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent + bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Pod for Lch<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent + bytemuck::Pod,
{
}

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
