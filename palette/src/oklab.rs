use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use num_traits::Zero;

#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::convert::FromColorUnclamped;
use crate::encoding::pixel::RawPixel;
use crate::matrix::multiply_xyz;
use crate::white_point::D65;
use crate::{
    clamp, contrast_ratio, from_f64, Alpha, Clamp, Component, ComponentWise, FloatComponent,
    FromF64, GetHue, Mat3, Mix, OklabHue, Oklch, Pixel, RelativeContrast, Shade, Xyz,
};

#[rustfmt::skip]
fn m1<T: FromF64>() -> Mat3<T> {
    [
        from_f64(0.8189330101), from_f64(0.3618667424), from_f64(-0.1288597137),
        from_f64(0.0329845436), from_f64(0.9293118715), from_f64(0.0361456387),
        from_f64(0.0482003018), from_f64(0.2643662691), from_f64(0.6338517070),
    ]
}

#[rustfmt::skip]
pub(crate) fn m1_inv<T: FromF64>() -> Mat3<T> {
    [
        from_f64(1.2270138511), from_f64(-0.5577999807), from_f64(0.2812561490),
        from_f64(-0.0405801784), from_f64(1.1122568696), from_f64(-0.0716766787),
        from_f64(-0.0763812845), from_f64(-0.4214819784), from_f64(1.5861632204),
    ]
}

#[rustfmt::skip]
fn m2<T: FromF64>() -> Mat3<T> {
    [
        from_f64(0.2104542553), from_f64(0.7936177850), from_f64(-0.0040720468),
        from_f64(1.9779984951), from_f64(-2.4285922050), from_f64(0.4505937099),
        from_f64(0.0259040371), from_f64(0.7827717662), from_f64(-0.8086757660),
    ]
}

#[rustfmt::skip]
pub(crate) fn m2_inv<T: FromF64>() -> Mat3<T> {
    [
        from_f64(0.9999999985), from_f64(0.3963377922), from_f64(0.2158037581),
        from_f64(1.0000000089), from_f64(-0.1055613423), from_f64(-0.0638541748),
        from_f64(1.0000000547), from_f64(-0.0894841821), from_f64(-1.2914855379),
    ]
}

/// Oklab with an alpha component. See the [`Oklaba` implementation in
/// `Alpha`](crate::Alpha#Oklaba).
pub type Oklaba<T = f32> = Alpha<Oklab<T>, T>;

/// The [Oklab color space](https://bottosson.github.io/posts/oklab/).
///
/// Oklab is a perceptually-uniform color space similar in structure to
/// [L\*a\*b\*](crate::Lab), but tries to have a better perceptual uniformity.
/// It assumes a D65 whitepoint and normal well-lit viewing conditions.
#[derive(Debug, PartialEq, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklab, Oklch, Xyz)
)]
#[repr(C)]
pub struct Oklab<T = f32> {
    /// L is the lightness of the color. 0 gives absolute black and 1 gives the brightest white.
    pub l: T,

    /// a goes from red at -1 to green at 1.
    pub a: T,

    /// b goes from yellow at -1 to blue at 1.
    pub b: T,
}

impl<T> Copy for Oklab<T> where T: Copy {}

impl<T> Clone for Oklab<T>
where
    T: Clone,
{
    fn clone(&self) -> Oklab<T> {
        Oklab {
            l: self.l.clone(),
            a: self.a.clone(),
            b: self.b.clone(),
        }
    }
}

impl<T> AbsDiffEq for Oklab<T>
where
    T: AbsDiffEq,
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        self.l.abs_diff_eq(&other.l, epsilon.clone())
            && self.a.abs_diff_eq(&other.a, epsilon.clone())
            && self.b.abs_diff_eq(&other.b, epsilon)
    }
}

impl<T> RelativeEq for Oklab<T>
where
    T: RelativeEq,
    T::Epsilon: Clone,
{
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    #[rustfmt::skip]
    fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        self.l.relative_eq(&other.l, epsilon.clone(), max_relative.clone())
            && self.a.relative_eq(&other.a, epsilon.clone(), max_relative.clone())
            && self.b.relative_eq(&other.b, epsilon, max_relative)
    }
}

impl<T> UlpsEq for Oklab<T>
where
    T: UlpsEq,
    T::Epsilon: Clone,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        self.l.ulps_eq(&other.l, epsilon.clone(), max_ulps.clone())
            && self.a.ulps_eq(&other.a, epsilon.clone(), max_ulps.clone())
            && self.b.ulps_eq(&other.b, epsilon, max_ulps)
    }
}

impl<T> Oklab<T> {
    /// Create an Oklab color.
    pub const fn new(l: T, a: T, b: T) -> Self {
        Self { l, a, b }
    }

    /// Convert to a `(L, a, b)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.l, self.a, self.b)
    }

    /// Convert from a `(L, a, b)` tuple.
    pub fn from_components((l, a, b): (T, T, T)) -> Self {
        Self::new(l, a, b)
    }
}

impl<T> Oklab<T>
where
    T: FromF64,
{
    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        from_f64(0.0)
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        from_f64(1.0)
    }

    /// Return the `a` value minimum.
    pub fn min_a() -> T {
        from_f64(-1.0)
    }

    /// Return the `a` value maximum.
    pub fn max_a() -> T {
        from_f64(1.0)
    }

    /// Return the `b` value minimum.
    pub fn min_b() -> T {
        from_f64(-1.0)
    }

    /// Return the `b` value maximum.
    pub fn max_b() -> T {
        from_f64(1.0)
    }
}

///<span id="Oklaba"></span>[`Oklaba`](crate::Oklaba) implementations.
impl<T, A> Alpha<Oklab<T>, A> {
    /// Create an Oklab color with transparency.
    pub const fn new(l: T, a: T, b: T, alpha: A) -> Self {
        Alpha {
            color: Oklab::new(l, a, b),
            alpha,
        }
    }

    /// Convert to a `(L, a, b, alpha)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (self.color.l, self.color.a, self.color.b, self.alpha)
    }

    /// Convert from a `(L, a, b, alpha)` tuple.
    pub fn from_components((l, a, b, alpha): (T, T, T, A)) -> Self {
        Self::new(l, a, b, alpha)
    }
}

impl<T> FromColorUnclamped<Oklab<T>> for Oklab<T> {
    fn from_color_unclamped(color: Self) -> Self {
        color
    }
}

impl<T> FromColorUnclamped<Xyz<D65, T>> for Oklab<T>
where
    T: FloatComponent,
{
    fn from_color_unclamped(color: Xyz<D65, T>) -> Self {
        let m1 = m1();
        let m2 = m2();

        let Xyz {
            x: l, y: m, z: s, ..
        } = multiply_xyz(&m1, &color.with_white_point());

        let l_m_s_ = Xyz::new(l.cbrt(), m.cbrt(), s.cbrt());

        let Xyz {
            x: l, y: a, z: b, ..
        } = multiply_xyz(&m2, &l_m_s_);

        Self::new(l, a, b)
    }
}

impl<T> FromColorUnclamped<Oklch<T>> for Oklab<T>
where
    T: FloatComponent,
{
    fn from_color_unclamped(color: Oklch<T>) -> Self {
        Oklab {
            l: color.l,
            a: color.chroma.max(T::zero()) * color.hue.to_radians().cos(),
            b: color.chroma.max(T::zero()) * color.hue.to_radians().sin(),
        }
    }
}

impl<T> From<(T, T, T)> for Oklab<T> {
    fn from(components: (T, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<T> From<Oklab<T>> for (T, T, T) {
    fn from(color: Oklab<T>) -> (T, T, T) {
        color.into_components()
    }
}

impl<T, A: Component> From<(T, T, T, A)> for Alpha<Oklab<T>, A> {
    fn from(components: (T, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<T, A: Component> From<Alpha<Oklab<T>, A>> for (T, T, T, A) {
    fn from(color: Alpha<Oklab<T>, A>) -> (T, T, T, A) {
        color.into_components()
    }
}

impl<T> Clamp for Oklab<T>
where
    T: FromF64 + PartialOrd,
{
    #[rustfmt::skip]
    #[inline]
    fn is_within_bounds(&self) -> bool {
        self.l >= Self::min_l() && self.l <= Self::max_l() &&
        self.a >= Self::min_a() && self.a <= Self::max_a() &&
        self.b >= Self::min_b() && self.b <= Self::max_b()
    }

    #[inline]
    fn clamp(self) -> Self {
        Self::new(
            clamp(self.l, Self::min_l(), Self::max_l()),
            clamp(self.a, Self::min_a(), Self::max_a()),
            clamp(self.b, Self::min_b(), Self::max_b()),
        )
    }
}

impl<T> Mix for Oklab<T>
where
    T: FloatComponent,
{
    type Scalar = T;

    fn mix(&self, other: &Self, factor: T) -> Self {
        let factor = clamp(factor, T::zero(), T::one());

        Self::new(
            self.l + factor * (other.l - self.l),
            self.a + factor * (other.a - self.a),
            self.b + factor * (other.b - self.b),
        )
    }
}

impl<T> Shade for Oklab<T>
where
    T: FloatComponent,
{
    type Scalar = T;

    fn lighten(&self, factor: T) -> Self {
        let difference = if factor >= T::zero() {
            Self::max_l() - self.l
        } else {
            self.l
        };

        let delta = difference.max(T::zero()) * factor;

        Self::new((self.l + delta).max(Self::min_l()), self.a, self.b)
    }

    fn lighten_fixed(&self, amount: T) -> Self {
        Self::new((self.l + amount).max(Self::min_l()), self.a, self.b)
    }
}

impl<T> GetHue for Oklab<T>
where
    T: FloatComponent,
{
    type Hue = OklabHue<T>;

    fn get_hue(&self) -> Option<OklabHue<T>> {
        if self.a == T::zero() && self.b == T::zero() {
            None
        } else {
            Some(OklabHue::from_radians(self.b.atan2(self.a)))
        }
    }
}

impl<T> ComponentWise for Oklab<T>
where
    T: FloatComponent,
{
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Self, mut f: F) -> Self {
        Self::new(f(self.l, other.l), f(self.a, other.a), f(self.b, other.b))
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Self {
        Self::new(f(self.l), f(self.a), f(self.b))
    }
}

impl<T> Default for Oklab<T>
where
    T: Zero,
{
    fn default() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }
}

impl_color_add!(Oklab<T>, [l, a, b]);
impl_color_sub!(Oklab<T>, [l, a, b]);
impl_color_mul!(Oklab<T>, [l, a, b]);
impl_color_div!(Oklab<T>, [l, a, b]);

impl<T, P> AsRef<P> for Oklab<T>
where
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<T, P> AsMut<P> for Oklab<T>
where
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<T> RelativeContrast for Oklab<T>
where
    T: FloatComponent,
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
impl<T> Distribution<Oklab<T>> for Standard
where
    T: FloatComponent,
    Standard: Distribution<T>,
{
    // `a` and `b` both range from (-1.0, 1.0)
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Oklab<T>
where {
        Oklab::new(
            rng.gen(),
            rng.gen() * from_f64(2.0) - from_f64(1.0),
            rng.gen() * from_f64(2.0) - from_f64(1.0),
        )
    }
}

#[cfg(feature = "random")]
pub struct UniformOklab<T>
where
    T: FloatComponent + SampleUniform,
{
    l: Uniform<T>,
    a: Uniform<T>,
    b: Uniform<T>,
}

#[cfg(feature = "random")]
impl<T> SampleUniform for Oklab<T>
where
    T: FloatComponent + SampleUniform,
{
    type Sampler = UniformOklab<T>;
}

#[cfg(feature = "random")]
impl<T> UniformSampler for UniformOklab<T>
where
    T: FloatComponent + SampleUniform,
{
    type X = Oklab<T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        Self {
            l: Uniform::new::<_, T>(low.l, high.l),
            a: Uniform::new::<_, T>(low.a, high.a),
            b: Uniform::new::<_, T>(low.b, high.b),
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        Self {
            l: Uniform::new_inclusive::<_, T>(low.l, high.l),
            a: Uniform::new_inclusive::<_, T>(low.a, high.a),
            b: Uniform::new_inclusive::<_, T>(low.b, high.b),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Oklab<T>
where {
        Oklab::new(self.l.sample(rng), self.a.sample(rng), self.b.sample(rng))
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Oklab<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Oklab<T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{FromColor, LinSrgb};

    #[test]
    fn red() {
        let a = Oklab::from_color(LinSrgb::new(1.0, 0.0, 0.0));
        let b = Oklab::new(0.627986, 0.224840, 0.125798);
        assert_relative_eq!(a, b, epsilon = 0.00001);
    }

    #[test]
    fn green() {
        let a = Oklab::from_color(LinSrgb::new(0.0, 1.0, 0.0));
        let b = Oklab::new(0.866432, -0.233916, 0.179417);
        assert_relative_eq!(a, b, epsilon = 0.00001);
    }

    #[test]
    fn blue() {
        let a = Oklab::from_color(LinSrgb::new(0.0, 0.0, 1.0));
        let b = Oklab::new(0.451977, -0.032429, -0.311611);
        assert_relative_eq!(a, b, epsilon = 0.00001);
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Oklab<f64>;
            clamped {
                l: 0.0 => 1.0,
                a: -1.0 => 1.0,
                b: -1.0 => 1.0
            }
            clamped_min {}
            unclamped {}
        }
    }

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Oklab::<f32>::min_l(), 0.0);
        assert_relative_eq!(Oklab::<f32>::min_a(), -1.0);
        assert_relative_eq!(Oklab::<f32>::min_b(), -1.0);
        assert_relative_eq!(Oklab::<f32>::max_l(), 1.0);
        assert_relative_eq!(Oklab::<f32>::max_a(), 1.0);
        assert_relative_eq!(Oklab::<f32>::max_b(), 1.0);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Oklab::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"l":0.3,"a":0.8,"b":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Oklab = ::serde_json::from_str(r#"{"l":0.3,"a":0.8,"b":0.1}"#).unwrap();

        assert_eq!(deserialized, Oklab::new(0.3, 0.8, 0.1));
    }

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Oklab {
            l: (0.0, 1.0),
            a: (-1.0, 1.0),
            b: (-1.0, 1.0)
        },
        min: Oklab::new(0.0, -1.0, -1.0),
        max: Oklab::new(1.0, 1.0, 1.0)
    }
}
