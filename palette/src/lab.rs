use core::{
    marker::PhantomData,
    ops::{Add, AddAssign, BitAnd, BitOr, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[cfg(feature = "approx")]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
#[cfg(feature = "random")]
use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};

use crate::{
    angle::RealAngle,
    blend::{PreAlpha, Premultiply},
    bool_mask::{HasBoolMask, LazySelect},
    clamp, clamp_assign,
    color_difference::{get_ciede_difference, ColorDifference, LabColorDiff},
    contrast_ratio,
    convert::FromColorUnclamped,
    num::{
        self, Abs, Arithmetics, Cbrt, Exp, FromScalarArray, IntoScalarArray, IsValidDivisor,
        MinMax, One, PartialCmp, Powi, Real, Sqrt, Trigonometry, Zero,
    },
    stimulus::Stimulus,
    white_point::{WhitePoint, D65},
    Alpha, Clamp, ClampAssign, FromColor, GetHue, IsWithinBounds, LabHue, Lch, Lighten,
    LightenAssign, Mix, MixAssign, RelativeContrast, Xyz,
};

/// CIE L\*a\*b\* (CIELAB) with an alpha component. See the [`Laba`
/// implementation in `Alpha`](crate::Alpha#Laba).
pub type Laba<Wp = D65, T = f32> = Alpha<Lab<Wp, T>, T>;

/// The CIE L\*a\*b\* (CIELAB) color space.
///
/// CIE L\*a\*b\* is a device independent color space which includes all
/// perceivable colors. It's sometimes used to convert between other color
/// spaces, because of its ability to represent all of their colors, and
/// sometimes in color manipulation, because of its perceptual uniformity. This
/// means that the perceptual difference between two colors is equal to their
/// numerical difference. It was, however, [never designed for the perceptual
/// qualities required for gamut mapping](http://www.brucelindbloom.com/UPLab.html).
/// For perceptually uniform color manipulation the newer color spaces based on
/// [`Oklab`](crate::Oklab) are preferable:
/// [`Oklch`](crate::Oklch), [`Okhsv`](crate::Okhsv), [`Okhsl`](crate::Okhsl),
/// [`Okhwb`](crate::Okhwb) (Note that the latter three are tied to the sRGB gamut
/// and reference white).
///
/// The parameters of L\*a\*b\* are quite different, compared to many other
/// color spaces, so manipulating them manually may be unintuitive.
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Xyz, Lab, Lch)
)]
#[repr(C)]
pub struct Lab<Wp = D65, T = f32> {
    /// L\* is the lightness of the color. 0.0 gives absolute black and 100
    /// give the brightest white.
    pub l: T,

    /// a\* goes from red at -128 to green at 127.
    pub a: T,

    /// b\* goes from yellow at -128 to blue at 127.
    pub b: T,

    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Lab<Wp, T> where T: Copy {}

impl<Wp, T> Clone for Lab<Wp, T>
where
    T: Clone,
{
    fn clone(&self) -> Lab<Wp, T> {
        Lab {
            l: self.l.clone(),
            a: self.a.clone(),
            b: self.b.clone(),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Lab<Wp, T> {
    /// Create a CIE L\*a\*b\* color.
    pub const fn new(l: T, a: T, b: T) -> Lab<Wp, T> {
        Lab {
            l,
            a,
            b,
            white_point: PhantomData,
        }
    }

    /// Convert to a `(L\*, a\*, b\*)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.l, self.a, self.b)
    }

    /// Convert from a `(L\*, a\*, b\*)` tuple.
    pub fn from_components((l, a, b): (T, T, T)) -> Self {
        Self::new(l, a, b)
    }
}

impl<Wp, T> Lab<Wp, T>
where
    T: Zero + Real,
{
    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::zero()
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        T::from_f64(100.0)
    }

    /// Return the `a` value minimum.
    pub fn min_a() -> T {
        T::from_f64(-128.0)
    }

    /// Return the `a` value maximum.
    pub fn max_a() -> T {
        T::from_f64(127.0)
    }

    /// Return the `b` value minimum.
    pub fn min_b() -> T {
        T::from_f64(-128.0)
    }

    /// Return the `b` value maximum.
    pub fn max_b() -> T {
        T::from_f64(127.0)
    }
}

///<span id="Laba"></span>[`Laba`](crate::Laba) implementations.
impl<Wp, T, A> Alpha<Lab<Wp, T>, A> {
    /// Create a CIE L\*a\*b\* with transparency.
    pub const fn new(l: T, a: T, b: T, alpha: A) -> Self {
        Alpha {
            color: Lab::new(l, a, b),
            alpha,
        }
    }

    /// Convert to a `(L\*, a\*, b\*, alpha)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (self.color.l, self.color.a, self.color.b, self.alpha)
    }

    /// Convert from a `(L\*, a\*, b\*, alpha)` tuple.
    pub fn from_components((l, a, b, alpha): (T, T, T, A)) -> Self {
        Self::new(l, a, b, alpha)
    }
}

impl<Wp, T> FromColorUnclamped<Lab<Wp, T>> for Lab<Wp, T> {
    fn from_color_unclamped(color: Lab<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T> FromColorUnclamped<Xyz<Wp, T>> for Lab<Wp, T>
where
    Wp: WhitePoint<T>,
    T: Real + Powi + Cbrt + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    fn from_color_unclamped(color: Xyz<Wp, T>) -> Self {
        let Xyz { x, y, z, .. } = color / Wp::get_xyz().with_white_point();

        let epsilon = T::from_f64(6.0 / 29.0).powi(3);
        let kappa: T = T::from_f64(841.0 / 108.0);
        let delta: T = T::from_f64(4.0 / 29.0);

        let convert = |c: T| {
            lazy_select! {
                if c.gt(&epsilon) => c.clone().cbrt(),
                else => (kappa.clone() * &c) + &delta,
            }
        };

        let x = convert(x);
        let y = convert(y);
        let z = convert(z);

        Lab {
            l: ((y.clone() * T::from_f64(116.0)) - T::from_f64(16.0)),
            a: ((x - &y) * T::from_f64(500.0)),
            b: ((y - z) * T::from_f64(200.0)),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> FromColorUnclamped<Lch<Wp, T>> for Lab<Wp, T>
where
    T: RealAngle + Zero + MinMax + Trigonometry + Mul<Output = T> + Clone,
{
    fn from_color_unclamped(color: Lch<Wp, T>) -> Self {
        let (hue_sin, hue_cos) = color.hue.into_raw_radians().sin_cos();
        let chroma = color.chroma.max(T::zero());

        Lab {
            l: color.l,
            a: hue_cos * chroma.clone(),
            b: hue_sin * chroma,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> From<(T, T, T)> for Lab<Wp, T> {
    fn from(components: (T, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp, T> From<Lab<Wp, T>> for (T, T, T) {
    fn from(color: Lab<Wp, T>) -> (T, T, T) {
        color.into_components()
    }
}

impl<Wp, T, A> From<(T, T, T, A)> for Alpha<Lab<Wp, T>, A> {
    fn from(components: (T, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp, T, A> From<Alpha<Lab<Wp, T>, A>> for (T, T, T, A) {
    fn from(color: Alpha<Lab<Wp, T>, A>) -> (T, T, T, A) {
        color.into_components()
    }
}

impl_is_within_bounds! {
    Lab<Wp> {
        l => [Self::min_l(), Self::max_l()],
        a => [Self::min_a(), Self::max_a()],
        b => [Self::min_b(), Self::max_b()]
    }
    where T: Real + Zero
}

impl<Wp, T> Clamp for Lab<Wp, T>
where
    T: Zero + Real + num::Clamp,
{
    #[inline]
    fn clamp(self) -> Self {
        Self::new(
            clamp(self.l, Self::min_l(), Self::max_l()),
            clamp(self.a, Self::min_a(), Self::max_a()),
            clamp(self.b, Self::min_b(), Self::max_b()),
        )
    }
}

impl<Wp, T> ClampAssign for Lab<Wp, T>
where
    T: Zero + Real + num::ClampAssign,
{
    #[inline]
    fn clamp_assign(&mut self) {
        clamp_assign(&mut self.l, Self::min_l(), Self::max_l());
        clamp_assign(&mut self.a, Self::min_a(), Self::max_a());
        clamp_assign(&mut self.b, Self::min_b(), Self::max_b());
    }
}

impl_mix!(Lab<Wp>);
impl_lighten!(Lab<Wp> increase {l => [Self::min_l(), Self::max_l()]} other {a, b} phantom: white_point);
impl_premultiply!(Lab<Wp> {l, a, b} phantom: white_point);

impl<Wp, T> GetHue for Lab<Wp, T>
where
    T: RealAngle + Trigonometry + Add<T, Output = T> + Neg<Output = T> + Clone,
{
    type Hue = LabHue<T>;

    fn get_hue(&self) -> LabHue<T> {
        LabHue::from_cartesian(self.a.clone(), self.b.clone())
    }
}

impl<Wp, T> ColorDifference for Lab<Wp, T>
where
    T: Real
        + RealAngle
        + One
        + Zero
        + Powi
        + Exp
        + Trigonometry
        + Abs
        + Sqrt
        + Arithmetics
        + PartialCmp
        + Clone,
    T::Mask: LazySelect<T> + BitAnd<Output = T::Mask> + BitOr<Output = T::Mask>,
    Self: Into<LabColorDiff<T>>,
{
    type Scalar = T;

    #[inline]
    fn get_color_difference(self, other: Lab<Wp, T>) -> Self::Scalar {
        get_ciede_difference(self.into(), other.into())
    }
}

impl<Wp, T> HasBoolMask for Lab<Wp, T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<Wp, T> Default for Lab<Wp, T>
where
    T: Zero,
{
    fn default() -> Lab<Wp, T> {
        Lab::new(T::zero(), T::zero(), T::zero())
    }
}

impl_color_add!(Lab<Wp, T>, [l, a, b], white_point);
impl_color_sub!(Lab<Wp, T>, [l, a, b], white_point);
impl_color_mul!(Lab<Wp, T>, [l, a, b], white_point);
impl_color_div!(Lab<Wp, T>, [l, a, b], white_point);

impl_array_casts!(Lab<Wp, T>, [T; 3]);
impl_simd_array_conversion!(Lab<Wp>, [l, a, b], white_point);

impl_eq!(Lab<Wp>, [l, a, b]);

impl<Wp, T> RelativeContrast for Lab<Wp, T>
where
    T: Real + Arithmetics + PartialCmp,
    T::Mask: LazySelect<T>,
    Xyz<Wp, T>: FromColor<Self>,
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
impl<Wp, T> Distribution<Lab<Wp, T>> for Standard
where
    T: Real + Sub<Output = T> + Mul<Output = T>,
    Standard: Distribution<T>,
{
    // `a` and `b` both range from (-128.0, 127.0)
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Lab<Wp, T> {
        Lab {
            l: rng.gen() * T::from_f64(100.0),
            a: rng.gen() * T::from_f64(255.0) - T::from_f64(128.0),
            b: rng.gen() * T::from_f64(255.0) - T::from_f64(128.0),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformLab<Wp, T>
where
    T: SampleUniform,
{
    l: Uniform<T>,
    a: Uniform<T>,
    b: Uniform<T>,
    white_point: PhantomData<Wp>,
}

#[cfg(feature = "random")]
impl<Wp, T> SampleUniform for Lab<Wp, T>
where
    T: Clone + SampleUniform,
{
    type Sampler = UniformLab<Wp, T>;
}

#[cfg(feature = "random")]
impl<Wp, T> UniformSampler for UniformLab<Wp, T>
where
    T: Clone + SampleUniform,
{
    type X = Lab<Wp, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

        UniformLab {
            l: Uniform::new::<_, T>(low.l, high.l),
            a: Uniform::new::<_, T>(low.a, high.a),
            b: Uniform::new::<_, T>(low.b, high.b),
            white_point: PhantomData,
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

        UniformLab {
            l: Uniform::new_inclusive::<_, T>(low.l, high.l),
            a: Uniform::new_inclusive::<_, T>(low.a, high.a),
            b: Uniform::new_inclusive::<_, T>(low.b, high.b),
            white_point: PhantomData,
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Lab<Wp, T> {
        Lab {
            l: self.l.sample(rng),
            a: self.a.sample(rng),
            b: self.b.sample(rng),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Zeroable for Lab<Wp, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp: 'static, T> bytemuck::Pod for Lab<Wp, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use super::Lab;
    use crate::white_point::D65;
    use crate::{FromColor, LinSrgb};

    test_convert_into_from_xyz!(Lab);

    #[test]
    fn red() {
        let a = Lab::from_color(LinSrgb::new(1.0, 0.0, 0.0));
        let b = Lab::new(53.23288, 80.09246, 67.2031);
        assert_relative_eq!(a, b, epsilon = 0.01);
    }

    #[test]
    fn green() {
        let a = Lab::from_color(LinSrgb::new(0.0, 1.0, 0.0));
        let b = Lab::new(87.73704, -86.184654, 83.18117);
        assert_relative_eq!(a, b, epsilon = 0.01);
    }

    #[test]
    fn blue() {
        let a = Lab::from_color(LinSrgb::new(0.0, 0.0, 1.0));
        let b = Lab::new(32.302586, 79.19668, -107.863686);
        assert_relative_eq!(a, b, epsilon = 0.01);
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Lab<D65, f64>;
            clamped {
                l: 0.0 => 100.0,
                a: -128.0 => 127.0,
                b: -128.0 => 127.0
            }
            clamped_min {}
            unclamped {}
        }
    }

    raw_pixel_conversion_tests!(Lab<D65>: l, a, b);
    raw_pixel_conversion_fail_tests!(Lab<D65>: l, a, b);

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Lab::<D65, f32>::min_l(), 0.0);
        assert_relative_eq!(Lab::<D65, f32>::min_a(), -128.0);
        assert_relative_eq!(Lab::<D65, f32>::min_b(), -128.0);
        assert_relative_eq!(Lab::<D65, f32>::max_l(), 100.0);
        assert_relative_eq!(Lab::<D65, f32>::max_a(), 127.0);
        assert_relative_eq!(Lab::<D65, f32>::max_b(), 127.0);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Lab::<D65>::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"l":0.3,"a":0.8,"b":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Lab = ::serde_json::from_str(r#"{"l":0.3,"a":0.8,"b":0.1}"#).unwrap();

        assert_eq!(deserialized, Lab::new(0.3, 0.8, 0.1));
    }

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Lab<D65, f32> {
            l: (0.0, 100.0),
            a: (-128.0, 127.0),
            b: (-128.0, 127.0)
        },
        min: Lab::new(0.0f32, -128.0, -128.0),
        max: Lab::new(100.0, 127.0, 127.0)
    }
}
