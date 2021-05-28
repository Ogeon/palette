use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::Distribution;
#[cfg(feature = "random")]
use rand::Rng;

use crate::encoding::pixel::RawPixel;
use crate::luv_bounds::LuvBounds;
use crate::{
    clamp, contrast_ratio,
    convert::FromColorUnclamped,
    white_point::{WhitePoint, D65},
    Alpha, Clamp, Component, FloatComponent, GetHue, Hue, Lchuv, LuvHue, Mix, Pixel,
    RelativeContrast, Saturate, Shade, Xyz,
};

/// HSLuv with an alpha component. See the [`Hsluva` implementation in
/// `Alpha`](crate::Alpha#Hsluva).
pub type Hsluva<Wp = D65, T = f32> = Alpha<Hsluv<Wp, T>, T>;

/// HSLuv color space.
///
/// The HSLuv color space can be seen as a cylindrical version of
/// [CIELUV](crate::luv::Luv), similar to
/// [LCHuv](crate::lchuv::Lchuv), with the additional benefit of
/// streching the chroma values to a uniform saturation range [0.0,
/// 100.0]. This makes HSLuv much more convenient for generating
/// colors than Lchuv, as the set of valid saturation values is
/// independent of lightness and hue.
#[derive(Debug, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Lchuv, Hsluv)
)]
#[repr(C)]
pub struct Hsluv<Wp = D65, T = f32>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    /// The hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: LuvHue<T>,

    /// The colorfulness of the color, as a percentage of the maximum
    /// available chroma. 0.0 gives gray scale colors and 100.0 will
    /// give absolutely clear colors.
    pub saturation: T,

    /// Decides how light the color will look. 0.0 will be black, 50.0 will give
    /// a clear color, and 100.0 will give white.
    pub l: T,

    /// The white point and RGB primaries this color is adapted to. The default
    /// is the sRGB standard.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
}

impl<Wp, T> Clone for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn clone(&self) -> Hsluv<Wp, T> {
        *self
    }
}

impl<T> Hsluv<D65, T>
where
    T: FloatComponent,
{
    /// HSLuv with standard D65 whitepoint
    pub fn new<H: Into<LuvHue<T>>>(hue: H, saturation: T, l: T) -> Hsluv<D65, T> {
        Hsluv {
            hue: hue.into(),
            saturation,
            l,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    /// HSLuv with custom whitepoint.
    pub fn with_wp<H: Into<LuvHue<T>>>(hue: H, saturation: T, l: T) -> Hsluv<Wp, T> {
        Hsluv {
            hue: hue.into(),
            saturation,
            l,
            white_point: PhantomData,
        }
    }

    /// Convert to a `(hue, saturation, l)` tuple.
    pub fn into_components(self) -> (LuvHue<T>, T, T) {
        (self.hue, self.saturation, self.l)
    }

    /// Convert from a `(hue, saturation, l)` tuple.
    pub fn from_components<H: Into<LuvHue<T>>>((hue, saturation, l): (H, T, T)) -> Self {
        Self::with_wp(hue, saturation, l)
    }

    /// Return the `saturation` value minimum.
    pub fn min_saturation() -> T {
        T::zero()
    }

    /// Return the `saturation` value maximum.
    pub fn max_saturation() -> T {
        T::from_f64(100.0)
    }

    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::zero()
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        T::from_f64(100.0)
    }
}

impl<Wp, T> PartialEq for Hsluv<Wp, T>
where
    T: FloatComponent + PartialEq,
    Wp: WhitePoint,
{
    fn eq(&self, other: &Self) -> bool {
        self.hue == other.hue && self.saturation == other.saturation && self.l == other.l
    }
}

impl<Wp, T> Eq for Hsluv<Wp, T>
where
    T: FloatComponent + Eq,
    Wp: WhitePoint,
{
}

///<span id="Hsluva"></span>[`Hsluva`](crate::Hsluva) implementations.
impl<T, A> Alpha<Hsluv<D65, T>, A>
where
    T: FloatComponent,
    A: Component,
{
    /// HSLuv and transparency with standard D65 whitepoint.
    pub fn new<H: Into<LuvHue<T>>>(hue: H, saturation: T, l: T, alpha: A) -> Self {
        Alpha {
            color: Hsluv::new(hue, saturation, l),
            alpha,
        }
    }
}

///<span id="Hsluva"></span>[`Hsluva`](crate::Hsluva) implementations.
impl<Wp, T, A> Alpha<Hsluv<Wp, T>, A>
where
    T: FloatComponent,
    A: Component,
    Wp: WhitePoint,
{
    /// HSLuv and transparency.
    pub fn with_wp<H: Into<LuvHue<T>>>(hue: H, saturation: T, l: T, alpha: A) -> Self {
        Alpha {
            color: Hsluv::with_wp(hue, saturation, l),
            alpha,
        }
    }

    /// Convert to a `(hue, saturation, l, alpha)` tuple.
    pub fn into_components(self) -> (LuvHue<T>, T, T, A) {
        (self.hue, self.saturation, self.l, self.alpha)
    }

    /// Convert from a `(hue, saturation, l, alpha)` tuple.
    pub fn from_components<H: Into<LuvHue<T>>>((hue, saturation, l, alpha): (H, T, T, A)) -> Self {
        Self::with_wp(hue, saturation, l, alpha)
    }
}

impl<Wp, T> FromColorUnclamped<Hsluv<Wp, T>> for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn from_color_unclamped(hsluv: Hsluv<Wp, T>) -> Self {
        hsluv
    }
}

impl<Wp, T> FromColorUnclamped<Lchuv<Wp, T>> for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn from_color_unclamped(color: Lchuv<Wp, T>) -> Self {
        // convert the chroma to a saturation based on the max
        // saturation at a particular hue.
        let max_chroma = LuvBounds::from_lightness(color.l).max_chroma_at_hue(color.hue);

        Hsluv::with_wp(
            color.hue,
            color.chroma / max_chroma * T::from_f64(100.0),
            color.l,
        )
    }
}

impl<Wp: WhitePoint, T: FloatComponent, H: Into<LuvHue<T>>> From<(H, T, T)> for Hsluv<Wp, T> {
    fn from(components: (H, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent> Into<(LuvHue<T>, T, T)> for Hsluv<Wp, T> {
    fn into(self) -> (LuvHue<T>, T, T) {
        self.into_components()
    }
}

impl<Wp: WhitePoint, T: FloatComponent, H: Into<LuvHue<T>>, A: Component> From<(H, T, T, A)>
    for Alpha<Hsluv<Wp, T>, A>
{
    fn from(components: (H, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent, A: Component> Into<(LuvHue<T>, T, T, A)>
    for Alpha<Hsluv<Wp, T>, A>
{
    fn into(self) -> (LuvHue<T>, T, T, A) {
        self.into_components()
    }
}

impl<Wp, T> Clamp for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    #[rustfmt::skip]
    fn is_within_bounds(&self) -> bool {
        self.saturation >= Self::min_saturation() && self.saturation <= Self::max_saturation() &&
        self.l >= Self::min_l() && self.l <= Self::max_l()
    }

    fn clamp(&self) -> Hsluv<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.saturation = clamp(
            self.saturation,
            Self::min_saturation(),
            Self::max_saturation(),
        );
        self.l = clamp(self.l, Self::min_l(), Self::max_l());
    }
}

impl<Wp, T> Mix for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn mix(&self, other: &Hsluv<Wp, T>, factor: T) -> Hsluv<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();

        Hsluv {
            hue: self.hue + factor * hue_diff,
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            l: self.l + factor * (other.l - self.l),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn lighten(&self, factor: T) -> Hsluv<Wp, T> {
        let difference = if factor >= T::zero() {
            Self::max_l() - self.l
        } else {
            self.l
        };

        let delta = difference.max(T::zero()) * factor;

        Hsluv {
            hue: self.hue,
            saturation: self.saturation,
            l: (self.l + delta).max(T::zero()),
            white_point: PhantomData,
        }
    }

    fn lighten_fixed(&self, amount: T) -> Hsluv<Wp, T> {
        Hsluv {
            hue: self.hue,
            saturation: self.saturation,
            l: (self.l + Self::max_l() * amount).max(T::zero()),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> GetHue for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Hue = LuvHue<T>;

    fn get_hue(&self) -> Option<LuvHue<T>> {
        if self.saturation <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<Wp, T> Hue for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn with_hue<H: Into<Self::Hue>>(&self, hue: H) -> Hsluv<Wp, T> {
        Hsluv {
            hue: hue.into(),
            saturation: self.saturation,
            l: self.l,
            white_point: PhantomData,
        }
    }

    fn shift_hue<H: Into<Self::Hue>>(&self, amount: H) -> Hsluv<Wp, T> {
        Hsluv {
            hue: self.hue + amount.into(),
            saturation: self.saturation,
            l: self.l,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Saturate for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn saturate(&self, factor: T) -> Hsluv<Wp, T> {
        let difference = if factor >= T::zero() {
            Self::max_saturation() - self.saturation
        } else {
            self.saturation
        };

        let delta = difference.max(T::zero()) * factor;

        Hsluv {
            hue: self.hue,
            saturation: clamp(
                self.saturation + delta,
                Self::min_saturation(),
                Self::max_saturation(),
            ),
            l: self.l,
            white_point: PhantomData,
        }
    }

    fn saturate_fixed(&self, amount: T) -> Hsluv<Wp, T> {
        Hsluv {
            hue: self.hue,
            saturation: clamp(
                self.saturation + Self::max_saturation() * amount,
                Self::min_saturation(),
                Self::max_saturation(),
            ),
            l: self.l,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn default() -> Hsluv<Wp, T> {
        Hsluv::with_wp(LuvHue::from(T::zero()), T::zero(), T::zero())
    }
}

impl_color_add!(Hsluv, [hue, saturation, l], white_point);
impl_color_sub!(Hsluv, [hue, saturation, l], white_point);

impl<Wp, T, P> AsRef<P> for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<Wp, T, P> AsMut<P> for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<Wp, T> RelativeContrast for Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
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
pub struct UniformHsluv<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    hue: crate::hues::UniformLuvHue<T>,
    u1: Uniform<T>,
    u2: Uniform<T>,
    space: PhantomData<Wp>,
}

#[cfg(feature = "random")]
impl<Wp, T> SampleUniform for Hsluv<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    type Sampler = UniformHsluv<Wp, T>;
}

#[cfg(feature = "random")]
impl<Wp, T> UniformSampler for UniformHsluv<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    type X = Hsluv<Wp, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        use crate::random_sampling::invert_hsluv_sample;

        let low = *low_b.borrow();
        let high = *high_b.borrow();

        let (r1_min, r2_min): (T, T) = invert_hsluv_sample(low);
        let (r1_max, r2_max): (T, T) = invert_hsluv_sample(high);

        UniformHsluv {
            hue: crate::hues::UniformLuvHue::new(low.hue, high.hue),
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
        use crate::random_sampling::invert_hsluv_sample;

        let low = *low_b.borrow();
        let high = *high_b.borrow();

        let (r1_min, r2_min) = invert_hsluv_sample(low);
        let (r1_max, r2_max) = invert_hsluv_sample(high);

        UniformHsluv {
            hue: crate::hues::UniformLuvHue::new_inclusive(low.hue, high.hue),
            u1: Uniform::new_inclusive::<_, T>(r1_min, r1_max),
            u2: Uniform::new_inclusive::<_, T>(r2_min, r2_max),
            space: PhantomData,
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hsluv<Wp, T> {
        crate::random_sampling::sample_hsluv(
            self.hue.sample(rng),
            self.u1.sample(rng),
            self.u2.sample(rng),
        )
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Zeroable for Hsluv<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent + bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Pod for Hsluv<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent + bytemuck::Pod,
{
}

#[cfg(test)]
mod test {
    use super::Hsluv;
    use crate::{white_point::D65, FromColor, Lchuv, LuvHue, Saturate};

    #[test]
    fn lchuv_round_trip() {
        for hue in (0..=20).map(|x| x as f64 * 18.0) {
            for sat in (0..=20).map(|x| x as f64 * 5.0) {
                for l in (1..=20).map(|x| x as f64 * 5.0) {
                    let hsluv = Hsluv::new(hue, sat, l);
                    let lchuv = Lchuv::from_color(hsluv);
                    let mut to_hsluv = Hsluv::from_color(lchuv);
                    if to_hsluv.l < 1e-8 {
                        to_hsluv.hue = LuvHue::from(0.0);
                    }
                    assert_relative_eq!(hsluv, to_hsluv, epsilon = 1e-5);
                }
            }
        }
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Hsluv<D65, f64>;
            clamped {
                saturation: 0.0 => 100.0,
                l: 0.0 => 100.0
            }
            clamped_min {}
            unclamped {
                hue: -360.0 => 360.0
            }
        }
    }

    /// Check that the arithmetic operations (add/sub) are all
    /// implemented.
    #[test]
    fn test_arithmetic() {
        let hsl = Hsluv::new(120.0, 40.0, 30.0);
        let hsl2 = Hsluv::new(200.0, 30.0, 40.0);
        let mut _hsl3 = hsl + hsl2;
        _hsl3 += hsl2;
        let mut _hsl4 = hsl2 + 0.3;
        _hsl4 += 0.1;

        _hsl3 = hsl2 - hsl;
        _hsl3 = _hsl4 - 0.1;
        _hsl4 -= _hsl3;
        _hsl3 -= 0.1;
    }

    #[test]
    fn saturate() {
        for sat in (0..=10).map(|s| s as f64 * 10.0) {
            for a in (0..=10).map(|l| l as f64 * 10.0) {
                let hsl = Hsluv::new(150.0, sat, a);
                let hsl_sat_fixed = hsl.saturate_fixed(0.1);
                let expected_sat_fixed = Hsluv::new(150.0, (sat + 10.0).min(100.0), a);
                assert_relative_eq!(hsl_sat_fixed, expected_sat_fixed);

                let hsl_sat = hsl.saturate(0.1);
                let expected_sat = Hsluv::new(150.0, (sat + (100.0 - sat) * 0.1).min(100.0), a);
                assert_relative_eq!(hsl_sat, expected_sat);
            }
        }
    }

    raw_pixel_conversion_tests!(Hsluv<D65>: hue, saturation, lightness);
    raw_pixel_conversion_fail_tests!(Hsluv<D65>: hue, saturation, lightness);

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Hsluv::<D65>::min_saturation(), 0.0);
        assert_relative_eq!(Hsluv::<D65>::min_l(), 0.0);
        assert_relative_eq!(Hsluv::<D65>::max_saturation(), 100.0);
        assert_relative_eq!(Hsluv::<D65>::max_l(), 100.0);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Hsluv::new(120.0, 80.0, 60.0)).unwrap();

        assert_eq!(serialized, r#"{"hue":120.0,"saturation":80.0,"l":60.0}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Hsluv =
            ::serde_json::from_str(r#"{"hue":120.0,"saturation":80.0,"l":60.0}"#).unwrap();

        assert_eq!(deserialized, Hsluv::new(120.0, 80.0, 60.0));
    }
}
