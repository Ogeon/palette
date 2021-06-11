use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

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
    clamp, contrast_ratio, from_f64, Alpha, Clamp, Component, FloatComponent, FromColor, GetHue,
    Hsluv, Hue, Luv, LuvHue, Mix, Pixel, RelativeContrast, Saturate, Shade, Xyz,
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
pub struct Lchuv<Wp = D65, T = f32>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
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

impl<Wp, T> Copy for Lchuv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
}

impl<Wp, T> Clone for Lchuv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn clone(&self) -> Lchuv<Wp, T> {
        *self
    }
}

impl<T> Lchuv<D65, T>
where
    T: FloatComponent,
{
    /// CIE L\*C\*uv h°uv with white point D65.
    pub fn new<H: Into<LuvHue<T>>>(l: T, chroma: T, hue: H) -> Lchuv<D65, T> {
        Lchuv {
            l,
            chroma,
            hue: hue.into(),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Lchuv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    /// CIE L\*C\*uv h°uv
    pub fn with_wp<H: Into<LuvHue<T>>>(l: T, chroma: T, hue: H) -> Lchuv<Wp, T> {
        Lchuv {
            l,
            chroma,
            hue: hue.into(),
            white_point: PhantomData,
        }
    }

    /// Convert to a `(L\*, C\*uv, h°uv)` tuple.
    pub fn into_components(self) -> (T, T, LuvHue<T>) {
        (self.l, self.chroma, self.hue)
    }

    /// Convert from a `(L\*, C\*uv, h°uv)` tuple.
    pub fn from_components<H: Into<LuvHue<T>>>((l, chroma, hue): (T, T, H)) -> Self {
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

    /// Return the `chroma` value maximum.
    pub fn max_chroma() -> T {
        from_f64(180.0)
    }
}

impl<Wp, T> PartialEq for Lchuv<Wp, T>
where
    T: FloatComponent + PartialEq,
    Wp: WhitePoint,
{
    fn eq(&self, other: &Self) -> bool {
        self.l == other.l && self.chroma == other.chroma && self.hue == other.hue
    }
}

impl<Wp, T> Eq for Lchuv<Wp, T>
where
    T: FloatComponent + Eq,
    Wp: WhitePoint,
{
}

///<span id="Lchuva"></span>[`Lchuva`](crate::Lchuva) implementations.
impl<T, A> Alpha<Lchuv<D65, T>, A>
where
    T: FloatComponent,
    A: Component,
{
    /// CIE L\*C\*uv h°uv and transparency with white point D65.
    pub fn new<H: Into<LuvHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Alpha {
            color: Lchuv::new(l, chroma, hue),
            alpha,
        }
    }
}

///<span id="Lchuva"></span>[`Lchuva`](crate::Lchuva) implementations.
impl<Wp, T, A> Alpha<Lchuv<Wp, T>, A>
where
    T: FloatComponent,
    A: Component,
    Wp: WhitePoint,
{
    /// CIE L\*C\*uv h°uv and transparency.
    pub fn with_wp<H: Into<LuvHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Alpha {
            color: Lchuv::with_wp(l, chroma, hue),
            alpha,
        }
    }

    /// Convert to a `(L\*, C\*uv, h°uv, alpha)` tuple.
    pub fn into_components(self) -> (T, T, LuvHue<T>, A) {
        (self.l, self.chroma, self.hue, self.alpha)
    }

    /// Convert from a `(L\*, C\*uv, h°uv, alpha)` tuple.
    pub fn from_components<H: Into<LuvHue<T>>>((l, chroma, hue, alpha): (T, T, H, A)) -> Self {
        Self::with_wp(l, chroma, hue, alpha)
    }
}

impl<Wp, T> FromColorUnclamped<Lchuv<Wp, T>> for Lchuv<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Lchuv<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T> FromColorUnclamped<Luv<Wp, T>> for Lchuv<Wp, T>
where
    Wp: WhitePoint,
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
    Wp: WhitePoint,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Hsluv<Wp, T>) -> Self {
        // Apply the given saturation as a percentage of the max
        // chroma for that hue.
        let max_chroma = LuvBounds::from_lightness(color.l).max_chroma_at_hue(color.hue);

        Lchuv::with_wp(
            color.l,
            color.saturation * max_chroma * T::from_f64(0.01),
            color.hue,
        )
    }
}

impl<Wp: WhitePoint, T: FloatComponent, H: Into<LuvHue<T>>> From<(T, T, H)> for Lchuv<Wp, T> {
    fn from(components: (T, T, H)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent> Into<(T, T, LuvHue<T>)> for Lchuv<Wp, T> {
    fn into(self) -> (T, T, LuvHue<T>) {
        self.into_components()
    }
}

impl<Wp: WhitePoint, T: FloatComponent, H: Into<LuvHue<T>>, A: Component> From<(T, T, H, A)>
    for Alpha<Lchuv<Wp, T>, A>
{
    fn from(components: (T, T, H, A)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent, A: Component> Into<(T, T, LuvHue<T>, A)>
    for Alpha<Lchuv<Wp, T>, A>
{
    fn into(self) -> (T, T, LuvHue<T>, A) {
        self.into_components()
    }
}

impl<Wp, T> Clamp for Lchuv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn is_within_bounds(&self) -> bool {
        self.l >= Self::min_l()
            && self.l <= Self::max_l()
            && self.chroma >= Self::min_chroma()
            && self.chroma <= Self::max_chroma()
    }

    fn clamp(&self) -> Lchuv<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, Self::min_l(), Self::max_l());
        self.chroma = clamp(self.chroma, Self::min_chroma(), Self::max_chroma());
    }
}

impl<Wp, T> Mix for Lchuv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn mix(&self, other: &Lchuv<Wp, T>, factor: T) -> Lchuv<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();
        Lchuv {
            l: self.l + factor * (other.l - self.l),
            chroma: self.chroma + factor * (other.chroma - self.chroma),
            hue: self.hue + factor * hue_diff,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Lchuv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn lighten(&self, factor: T) -> Lchuv<Wp, T> {
        let difference = if factor >= T::zero() {
            T::from_f64(100.0) - self.l
        } else {
            self.l
        };

        let delta = difference.max(T::zero()) * factor;

        Lchuv {
            l: (self.l + delta).max(T::zero()),
            chroma: self.chroma,
            hue: self.hue,
            white_point: PhantomData,
        }
    }

    fn lighten_fixed(&self, amount: T) -> Lchuv<Wp, T> {
        Lchuv {
            l: (self.l + T::from_f64(100.0) * amount).max(T::zero()),
            chroma: self.chroma,
            hue: self.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> GetHue for Lchuv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Hue = LuvHue<T>;

    fn get_hue(&self) -> Option<LuvHue<T>> {
        if self.chroma <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<Wp, T> Hue for Lchuv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn with_hue<H: Into<Self::Hue>>(&self, hue: H) -> Lchuv<Wp, T> {
        Lchuv {
            l: self.l,
            chroma: self.chroma,
            hue: hue.into(),
            white_point: PhantomData,
        }
    }

    fn shift_hue<H: Into<Self::Hue>>(&self, amount: H) -> Lchuv<Wp, T> {
        Lchuv {
            l: self.l,
            chroma: self.chroma,
            hue: self.hue + amount.into(),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Saturate for Lchuv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn saturate(&self, factor: T) -> Lchuv<Wp, T> {
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

    fn saturate_fixed(&self, amount: T) -> Lchuv<Wp, T> {
        Lchuv {
            l: self.l,
            chroma: (self.chroma + Self::max_chroma() * amount).max(T::zero()),
            hue: self.hue,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Lchuv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn default() -> Lchuv<Wp, T> {
        Lchuv::with_wp(T::zero(), T::zero(), LuvHue::from(T::zero()))
    }
}

impl_color_add!(Lchuv, [l, chroma, hue], white_point);
impl_color_sub!(Lchuv, [l, chroma, hue], white_point);

impl<Wp, T, P> AsRef<P> for Lchuv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<Wp, T, P> AsMut<P> for Lchuv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<Wp, T> RelativeContrast for Lchuv<Wp, T>
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
impl<Wp, T> Distribution<Lchuv<Wp, T>> for Standard
where
    T: FloatComponent,
    Wp: WhitePoint,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Lchuv<Wp, T> {
        Lchuv {
            l: rng.gen() * from_f64(100.0),
            chroma: crate::Float::sqrt(rng.gen()) * from_f64(180.0),
            hue: rng.gen::<LuvHue<T>>(),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformLchuv<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
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
    Wp: WhitePoint,
{
    type Sampler = UniformLchuv<Wp, T>;
}

#[cfg(feature = "random")]
impl<Wp, T> UniformSampler for UniformLchuv<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
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
unsafe impl<Wp, T> bytemuck::Zeroable for Lchuv<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent + bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Pod for Lchuv<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent + bytemuck::Pod,
{
}

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
        let lchuv = Lchuv::new(120.0, 40.0, 30.0);
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
        let serialized = ::serde_json::to_string(&Lchuv::new(80.0, 70.0, 130.0)).unwrap();

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
