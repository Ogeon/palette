use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use core::ops::{Add, AddAssign, Sub, SubAssign};

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
    clamp, contrast_ratio, from_f64, Alpha, Clamp, Component, FloatComponent, FromColor, GetHue,
    Hue, Mix, Oklab, OklabHue, Pixel, RelativeContrast, Saturate, Shade, Xyz,
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
#[derive(Debug, PartialEq, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklab, Oklch, Xyz)
)]
#[repr(C)]
pub struct Oklch<T = f32>
where
    T: FloatComponent,
{
    /// L is the lightness of the color. 0 gives absolute black and 1 gives the brightest white.
    pub l: T,

    /// C is the colorfulness of the color, from greyscale at 0 to the most colorful at 1.
    pub chroma: T,

    /// h is the hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: OklabHue<T>,
}

impl<T> Copy for Oklch<T> where T: FloatComponent {}

impl<T> Clone for Oklch<T>
where
    T: FloatComponent,
{
    fn clone(&self) -> Oklch<T> {
        *self
    }
}

impl<T> AbsDiffEq for Oklch<T>
where
    T: FloatComponent + AbsDiffEq,
    T::Epsilon: Copy + FloatComponent,
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
    T::Epsilon: Copy + FloatComponent,
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
    T::Epsilon: Copy + FloatComponent,
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

impl<T> Oklch<T>
where
    T: FloatComponent,
{
    /// Create an Oklch color.
    pub fn new<H: Into<OklabHue<T>>>(l: T, chroma: T, hue: H) -> Oklch<T> {
        Oklch {
            l,
            chroma,
            hue: hue.into(),
        }
    }

    /// Convert to a `(L, C, h)` tuple.
    pub fn into_components(self) -> (T, T, OklabHue<T>) {
        (self.l, self.chroma, self.hue)
    }

    /// Convert from a `(L, C, h)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((l, chroma, hue): (T, T, H)) -> Self {
        Self::new(l, chroma, hue)
    }

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

impl<T> Eq for Oklch<T> where T: FloatComponent + Eq {}

///<span id="Oklcha"></span>[`Oklcha`](crate::Oklcha) implementations.
impl<T, A> Alpha<Oklch<T>, A>
where
    T: FloatComponent,
    A: Component,
{
    /// Oklch and transparency.
    pub fn new<H: Into<OklabHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Alpha {
            color: Oklch::new(l, chroma, hue),
            alpha,
        }
    }

    /// Convert to a `(L, C, h, alpha)` tuple.
    pub fn into_components(self) -> (T, T, OklabHue<T>, A) {
        (self.l, self.chroma, self.hue, self.alpha)
    }

    /// Convert from a `(L, C, h, alpha)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((l, chroma, hue, alpha): (T, T, H, A)) -> Self {
        Self::new(l, chroma, hue, alpha)
    }
}

impl<T> FromColorUnclamped<Oklch<T>> for Oklch<T>
where
    T: FloatComponent,
{
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

impl<T: FloatComponent, H: Into<OklabHue<T>>> From<(T, T, H)> for Oklch<T> {
    fn from(components: (T, T, H)) -> Self {
        Self::from_components(components)
    }
}

impl<T: FloatComponent> Into<(T, T, OklabHue<T>)> for Oklch<T> {
    fn into(self) -> (T, T, OklabHue<T>) {
        self.into_components()
    }
}

impl<T: FloatComponent, H: Into<OklabHue<T>>, A: Component> From<(T, T, H, A)>
    for Alpha<Oklch<T>, A>
{
    fn from(components: (T, T, H, A)) -> Self {
        Self::from_components(components)
    }
}

impl<T: FloatComponent, A: Component> Into<(T, T, OklabHue<T>, A)> for Alpha<Oklch<T>, A> {
    fn into(self) -> (T, T, OklabHue<T>, A) {
        self.into_components()
    }
}

impl<T> Clamp for Oklch<T>
where
    T: FloatComponent,
{
    fn is_within_bounds(&self) -> bool {
        self.l >= from_f64(0.0)
            && self.l <= from_f64(1.0)
            && self.chroma >= from_f64(0.0)
            && self.chroma <= from_f64(1.0)
    }

    fn clamp(&self) -> Oklch<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, from_f64(0.0), from_f64(1.0));
        self.chroma = clamp(self.chroma, from_f64(0.0), from_f64(1.0));
    }
}

impl<T> Mix for Oklch<T>
where
    T: FloatComponent,
{
    type Scalar = T;

    fn mix(&self, other: &Oklch<T>, factor: T) -> Oklch<T> {
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

    fn lighten(&self, factor: T) -> Oklch<T> {
        let difference = if factor >= T::zero() {
            T::from_f64(1.0) - self.l
        } else {
            self.l
        };

        let delta = difference.max(T::zero()) * factor;

        Oklch {
            l: (self.l + delta).max(T::zero()),
            chroma: self.chroma,
            hue: self.hue,
        }
    }

    fn lighten_fixed(&self, amount: T) -> Oklch<T> {
        Oklch {
            l: (self.l + T::from_f64(1.0) * amount).max(T::zero()),
            chroma: self.chroma,
            hue: self.hue,
        }
    }
}

impl<T> GetHue for Oklch<T>
where
    T: FloatComponent,
{
    type Hue = OklabHue<T>;

    fn get_hue(&self) -> Option<OklabHue<T>> {
        if self.chroma <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<T> Hue for Oklch<T>
where
    T: FloatComponent,
{
    fn with_hue<H: Into<Self::Hue>>(&self, hue: H) -> Oklch<T> {
        Oklch {
            l: self.l,
            chroma: self.chroma,
            hue: hue.into(),
        }
    }

    fn shift_hue<H: Into<Self::Hue>>(&self, amount: H) -> Oklch<T> {
        Oklch {
            l: self.l,
            chroma: self.chroma,
            hue: self.hue + amount.into(),
        }
    }
}

impl<T> Saturate for Oklch<T>
where
    T: FloatComponent,
{
    type Scalar = T;

    fn saturate(&self, factor: T) -> Oklch<T> {
        let difference = if factor >= T::zero() {
            Self::max_chroma() - self.chroma
        } else {
            self.chroma
        };

        let delta = difference.max(T::zero()) * factor;

        Oklch {
            l: self.l,
            chroma: (self.chroma + delta).max(T::zero()),
            hue: self.hue,
        }
    }

    fn saturate_fixed(&self, amount: T) -> Oklch<T> {
        Oklch {
            l: self.l,
            chroma: (self.chroma + Self::max_chroma() * amount).max(T::zero()),
            hue: self.hue,
        }
    }
}

impl<T> Default for Oklch<T>
where
    T: FloatComponent,
{
    fn default() -> Oklch<T> {
        Oklch::new(T::zero(), T::zero(), OklabHue::from(T::zero()))
    }
}

impl<T> Add<Oklch<T>> for Oklch<T>
where
    T: FloatComponent,
{
    type Output = Oklch<T>;

    fn add(self, other: Oklch<T>) -> Self::Output {
        Oklch {
            l: self.l + other.l,
            chroma: self.chroma + other.chroma,
            hue: self.hue + other.hue,
        }
    }
}

impl<T> Add<T> for Oklch<T>
where
    T: FloatComponent,
{
    type Output = Oklch<T>;

    fn add(self, c: T) -> Self::Output {
        Oklch {
            l: self.l + c,
            chroma: self.chroma + c,
            hue: self.hue + c,
        }
    }
}

impl<T> AddAssign<Oklch<T>> for Oklch<T>
where
    T: FloatComponent + AddAssign,
{
    fn add_assign(&mut self, other: Oklch<T>) {
        self.l += other.l;
        self.chroma += other.chroma;
        self.hue += other.hue;
    }
}

impl<T> AddAssign<T> for Oklch<T>
where
    T: FloatComponent + AddAssign,
{
    fn add_assign(&mut self, c: T) {
        self.l += c;
        self.chroma += c;
        self.hue += c;
    }
}

impl<T> Sub<Oklch<T>> for Oklch<T>
where
    T: FloatComponent,
{
    type Output = Oklch<T>;

    fn sub(self, other: Oklch<T>) -> Self::Output {
        Oklch {
            l: self.l - other.l,
            chroma: self.chroma - other.chroma,
            hue: self.hue - other.hue,
        }
    }
}

impl<T> Sub<T> for Oklch<T>
where
    T: FloatComponent,
{
    type Output = Oklch<T>;

    fn sub(self, c: T) -> Self::Output {
        Oklch {
            l: self.l - c,
            chroma: self.chroma - c,
            hue: self.hue - c,
        }
    }
}

impl<T> SubAssign<Oklch<T>> for Oklch<T>
where
    T: FloatComponent + SubAssign,
{
    fn sub_assign(&mut self, other: Oklch<T>) {
        self.l -= other.l;
        self.chroma -= other.chroma;
        self.hue -= other.hue;
    }
}

impl<T> SubAssign<T> for Oklch<T>
where
    T: FloatComponent + SubAssign,
{
    fn sub_assign(&mut self, c: T) {
        self.l -= c;
        self.chroma -= c;
        self.hue -= c;
    }
}

impl<T, P> AsRef<P> for Oklch<T>
where
    T: FloatComponent,

    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<T, P> AsMut<P> for Oklch<T>
where
    T: FloatComponent,

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

    fn get_contrast_ratio(&self, other: &Self) -> T {
        let xyz1 = Xyz::from_color(*self);
        let xyz2 = Xyz::from_color(*other);

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
