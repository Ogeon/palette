use core::ops::{Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use core::fmt::Debug;
#[cfg(feature = "random")]
use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};

use crate::convert::IntoColorUnclamped;
use crate::num::{FromScalar, Powi, Recip, Sqrt};
use crate::ok_utils::{toe_inv, ChromaValues, LC, ST};
use crate::{
    angle::RealAngle,
    blend::{PreAlpha, Premultiply},
    bool_mask::{HasBoolMask, LazySelect},
    clamp, clamp_assign, contrast_ratio,
    convert::FromColorUnclamped,
    matrix::multiply_xyz,
    num::{
        self, Arithmetics, Cbrt, FromScalarArray, IntoScalarArray, IsValidDivisor, MinMax, One,
        PartialCmp, Real, Trigonometry, Zero,
    },
    stimulus::Stimulus,
    white_point::D65,
    Alpha, Clamp, ClampAssign, FromColor, GetHue, IsWithinBounds, Lighten, LightenAssign, LinSrgb,
    Mat3, Mix, MixAssign, Okhsl, Okhsv, OklabHue, Oklch, RelativeContrast, Xyz,
};

// Using recalculated matrix values from
// https://github.com/LeaVerou/color.js/blob/master/src/spaces/oklab.js
//
// see https://github.com/w3c/csswg-drafts/issues/6642#issuecomment-943521484
// and the following https://github.com/w3c/csswg-drafts/issues/6642#issuecomment-945714988

/// XYZ to LSM transformation matrix
#[rustfmt::skip]
fn m1<T: Real>() -> Mat3<T> {
    [
        T::from_f64(0.8190224432164319), T::from_f64(0.3619062562801221), T::from_f64(-0.12887378261216414),
        T::from_f64(0.0329836671980271), T::from_f64(0.9292868468965546), T::from_f64(0.03614466816999844),
        T::from_f64(0.048177199566046255), T::from_f64(0.26423952494422764), T::from_f64(0.6335478258136937),
    ]
}

/// LMS to XYZ transformation matrix
#[rustfmt::skip]
pub(crate) fn m1_inv<T: Real>() -> Mat3<T> {
    [
        T::from_f64(1.2268798733741557), T::from_f64(-0.5578149965554813), T::from_f64(0.28139105017721583),
        T::from_f64(-0.04057576262431372), T::from_f64(1.1122868293970594), T::from_f64(-0.07171106666151701),
        T::from_f64(-0.07637294974672142), T::from_f64(-0.4214933239627914), T::from_f64(1.5869240244272418),
    ]
}

/// LMS to Oklab transformation matrix
#[rustfmt::skip]
fn m2<T: Real>() -> Mat3<T> {
    [
        T::from_f64(0.2104542553), T::from_f64(0.7936177850), T::from_f64(-0.0040720468),
        T::from_f64(1.9779984951), T::from_f64(-2.4285922050), T::from_f64(0.4505937099),
        T::from_f64(0.0259040371), T::from_f64(0.7827717662), T::from_f64(-0.8086757660),
    ]
}

/// Oklab to LMS transformation matrix
#[rustfmt::skip]
pub(crate) fn m2_inv<T: Real>() -> Mat3<T> {
    [
        T::from_f64(0.99999999845051981432), T::from_f64(0.39633779217376785678), T::from_f64(0.21580375806075880339),
        T::from_f64(1.0000000088817607767), T::from_f64(-0.1055613423236563494), T::from_f64(-0.063854174771705903402),
        T::from_f64(1.0000000546724109177), T::from_f64(-0.089484182094965759684), T::from_f64(-1.2914855378640917399),
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
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklab, Oklch, Okhsv, Okhsl, Xyz)
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

//FIXME:https://colorjs.io/docs/spaces.html#oklab uses different values to the documentation above and these min- and max- values
// From my understanding Chroma in Oklab is unlimited, as it is only limited, when used in reference to the gamut of a specific  display technology.
// So here, a and b should be unlimited. Only in Okhsl and Okhsv should chroma be limited, as they reference the sRGB gamut.
// For a different Okhsl and Okhsv, which references HDR (https://bottosson.github.io/posts/colorpicker/#ideas-for-future-work) they'd need to be different again.
impl<T> Oklab<T>
where
    T: Real,
{
    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::from_f64(0.0)
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        T::from_f64(1.0)
    }

    /// Return the `a` value minimum.
    pub fn min_a() -> T {
        T::from_f64(-1.0)
    }

    /// Return the `a` value maximum.
    pub fn max_a() -> T {
        T::from_f64(1.0)
    }

    /// Return the `b` value minimum.
    pub fn min_b() -> T {
        T::from_f64(-1.0)
    }

    /// Return the `b` value maximum.
    pub fn max_b() -> T {
        T::from_f64(1.0)
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
    T: Real + Cbrt + Arithmetics,
{
    fn from_color_unclamped(color: Xyz<D65, T>) -> Self {
        let m1 = m1();
        let m2 = m2();

        let Xyz {
            x: l, y: m, z: s, ..
        } = multiply_xyz(m1, color.with_white_point());

        let l_m_s_ = Xyz::new(l.cbrt(), m.cbrt(), s.cbrt());

        let Xyz {
            x: l, y: a, z: b, ..
        } = multiply_xyz(m2, l_m_s_);

        Self::new(l, a, b)
    }
}

impl<T> FromColorUnclamped<Oklch<T>> for Oklab<T>
where
    T: RealAngle + Zero + MinMax + Trigonometry + Mul<Output = T> + Clone,
{
    fn from_color_unclamped(color: Oklch<T>) -> Self {
        let (sin_hue, cos_hue) = color.hue.into_raw_radians().sin_cos();
        let chroma = color.chroma.max(T::zero());

        Oklab {
            l: color.l,
            a: cos_hue * chroma.clone(),
            b: sin_hue * chroma,
        }
    }
}

/// # See
/// See [`okhsl_to_srgb`](https://bottosson.github.io/posts/colorpicker/#hsl-2)
impl<T> FromColorUnclamped<Okhsl<T>> for Oklab<T>
where
    T: Real
        + Debug
        + AbsDiffEq
        + One
        + Zero
        + Arithmetics
        + Sqrt
        + MinMax
        + Copy
        + PartialOrd
        + HasBoolMask<Mask = bool>
        + Powi
        + Cbrt
        + Trigonometry
        + FromScalar
        + RealAngle,
    T::Scalar: Real
        + Zero
        + One
        + Recip
        + IsValidDivisor<Mask = bool>
        + Arithmetics
        + Clone
        + FromScalar<Scalar = T::Scalar>,
{
    fn from_color_unclamped(hsl: Okhsl<T>) -> Self {
        let h = hsl.hue;
        let s = hsl.saturation;
        let l = hsl.lightness;

        if l == T::one() {
            return Oklab::new(T::one(), T::zero(), T::zero());
        } else if l == T::zero() {
            return Oklab::new(T::zero(), T::zero(), T::zero());
        }

        let h_radians = h.into_raw_radians();
        let a_ = T::cos(h_radians);
        let b_ = T::sin(h_radians);
        let L = toe_inv(l);

        let cs = ChromaValues::from_normalized(L, a_, b_);

        // Interpolate the three values for C so that:
        // At s=0: dC/ds = cs.zero, C = 0
        // At s=0.8: C = cs.mid
        // At s=1.0: C = cs.max

        let mid = T::from_f64(0.8);
        let mid_inv = T::from_f64(1.25);

        let C = if s < mid {
            let t = mid_inv * s;

            let k_1 = mid * cs.zero;
            let k_2 = T::one() - k_1 / cs.mid;

            t * k_1 / (T::one() - k_2 * t)
        } else {
            let t = (s - mid) / (T::one() - mid);

            let k_0 = cs.mid;
            let k_1 = (T::one() - mid) * cs.mid * cs.mid * mid_inv * mid_inv / cs.zero;
            let k_2 = T::one() - (k_1) / (cs.max - cs.mid);

            k_0 + t * k_1 / (T::one() - k_2 * t)
        };

        Oklab::new(L, C * a_, C * b_)
    }
}

impl<T> FromColorUnclamped<Okhsv<T>> for Oklab<T>
where
    T: Real
        + AbsDiffEq
        + PartialOrd
        + HasBoolMask<Mask = bool>
        + MinMax
        + Powi
        + Arithmetics
        + Copy
        + One
        + Zero
        + Sqrt
        + Cbrt
        + Trigonometry
        + RealAngle
        + FromScalar
        + Debug,
    T::Scalar: Real
        + PartialOrd
        + Zero
        + One
        + Recip
        + IsValidDivisor<Mask = bool>
        + Arithmetics
        + Clone
        + FromScalar<Scalar = T::Scalar>,
{
    fn from_color_unclamped(hsv: Okhsv<T>) -> Self {
        if hsv.saturation == T::zero() {
            // totally desaturated color -- the triangle is just the 0-chroma-line
            if hsv.value == T::zero() {
                // pure black
                return Self {
                    l: T::zero(),
                    a: T::zero(),
                    b: T::zero(),
                };
            }
            let l = toe_inv(hsv.value);
            return Self {
                l,
                a: T::zero(),
                b: T::zero(),
            };
        }

        let h_radians = hsv.hue.into_raw_radians();
        let a_ = T::cos(h_radians);
        let b_ = T::sin(h_radians);

        let cusp = LC::find_cusp(a_, b_);
        let cusp: ST<T> = cusp.into();
        let S_0 = T::from_f64(0.5);
        let k = T::one() - S_0 / cusp.s;

        // first we compute L and V as if the gamut is a perfect triangle

        // L, C, when v == 1:
        let L_v = T::one() - hsv.saturation * S_0 / (S_0 + cusp.t - cusp.t * k * hsv.saturation);
        let C_v = hsv.saturation * cusp.t * S_0 / (S_0 + cusp.t - cusp.t * k * hsv.saturation);

        // then we compensate for both toe and the curved top part of the triangle:
        let L_vt = toe_inv(L_v);
        let C_vt = C_v * L_vt / L_v;

        let mut L = hsv.value * L_v;
        let mut C = hsv.value * C_v;
        let L_new = toe_inv(L);
        C = C * L_new / L;
        // the values may be outside the normal range
        let rgb_scale: LinSrgb<T> = Oklab::new(L_vt, a_ * C_vt, b_ * C_vt).into_color_unclamped();
        let scale_L = T::cbrt(
            T::one()
                / T::max(
                    T::max(rgb_scale.red, rgb_scale.green),
                    T::max(rgb_scale.blue, T::zero()),
                ),
        );

        L = L_new * scale_L;
        C = C * scale_L;

        Oklab::new(L, C * a_, C * b_)
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

impl<T, A> From<(T, T, T, A)> for Alpha<Oklab<T>, A> {
    fn from(components: (T, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<T, A> From<Alpha<Oklab<T>, A>> for (T, T, T, A) {
    fn from(color: Alpha<Oklab<T>, A>) -> (T, T, T, A) {
        color.into_components()
    }
}

impl_is_within_bounds! {
    Oklab {
        l => [Self::min_l(), Self::max_l()],
        a => [Self::min_a(), Self::max_a()],
        b => [Self::min_b(), Self::max_b()]
    }
    where T: Real
}

impl<T> Clamp for Oklab<T>
where
    T: Real + num::Clamp,
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

impl<T> ClampAssign for Oklab<T>
where
    T: Real + num::ClampAssign,
{
    #[inline]
    fn clamp_assign(&mut self) {
        clamp_assign(&mut self.l, Self::min_l(), Self::max_l());
        clamp_assign(&mut self.a, Self::min_a(), Self::max_a());
        clamp_assign(&mut self.b, Self::min_b(), Self::max_b());
    }
}

impl_mix!(Oklab);
impl_lighten!(Oklab increase {l => [Self::min_l(), Self::max_l()]} other {a, b});
impl_premultiply!(Oklab { l, a, b });

impl<T> GetHue for Oklab<T>
where
    T: RealAngle + Trigonometry + Clone,
{
    type Hue = OklabHue<T>;

    fn get_hue(&self) -> OklabHue<T> {
        OklabHue::from_radians(self.b.clone().atan2(self.a.clone()))
    }
}

impl<T> HasBoolMask for Oklab<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
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

impl_array_casts!(Oklab<T>, [T; 3]);
impl_simd_array_conversion!(Oklab, [l, a, b]);

impl_eq!(Oklab, [l, a, b]);

impl<T> RelativeContrast for Oklab<T>
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
impl<T> Distribution<Oklab<T>> for Standard
where
    T: Real + Mul<Output = T> + Sub<Output = T>,
    Standard: Distribution<T>,
{
    // `a` and `b` both range from (-1.0, 1.0)
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Oklab<T>
where {
        Oklab::new(
            rng.gen(),
            rng.gen() * T::from_f64(2.0) - T::from_f64(1.0),
            rng.gen() * T::from_f64(2.0) - T::from_f64(1.0),
        )
    }
}

#[cfg(feature = "random")]
pub struct UniformOklab<T>
where
    T: SampleUniform,
{
    l: Uniform<T>,
    a: Uniform<T>,
    b: Uniform<T>,
}

#[cfg(feature = "random")]
impl<T> SampleUniform for Oklab<T>
where
    T: Clone + SampleUniform,
{
    type Sampler = UniformOklab<T>;
}

#[cfg(feature = "random")]
impl<T> UniformSampler for UniformOklab<T>
where
    T: Clone + SampleUniform,
{
    type X = Oklab<T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

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
        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

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
    use crate::rgb::Rgb;
    use crate::{FromColor, LinSrgb, Srgb};
    use std::str::FromStr;

    #[test]
    fn blue_srgb() {
        // use f64 to be comparable to javascript
        let rgb: Srgb<f64> = Rgb::from_str("#0000ff").unwrap().into_format();
        let lin_rgb = LinSrgb::from_color_unclamped(rgb);
        let oklab = Oklab::from_color_unclamped(lin_rgb);

        // values from Ok Color Picker, which seems to use  Björn Ottosson's original
        // algorithm, but from the direct srgb2oklab conversion
        // (not via the XYZ color space)
        assert!(abs_diff_eq!(oklab.l, 0.4520137183853429, epsilon = 1e-3));
        assert!(abs_diff_eq!(oklab.a, -0.03245698416876397, epsilon = 1e-3));
        assert!(abs_diff_eq!(oklab.b, -0.3115281476783751, epsilon = 1e-3));
    }

    #[test]
    fn red() {
        let a = Oklab::from_color(LinSrgb::new(1.0, 0.0, 0.0));
        let b = Oklab::new(0.6279886758522074, 0.22487499084122475, 0.12585297511892374);
        assert_relative_eq!(a, b, epsilon = 1e-12);
    }

    #[test]
    fn green() {
        let a = Oklab::from_color(LinSrgb::new(0.0, 1.0, 0.0));
        let b = Oklab::new(
            0.8664329386540478,
            -0.23388577290357765,
            0.17949709748981812,
        );
        assert_relative_eq!(a, b, epsilon = 1e-12);
    }

    #[test]
    fn blue() {
        let a = Oklab::from_color(LinSrgb::new(0.0, 0.0, 1.0));
        let b = Oklab::new(
            0.45197756295615854,
            -0.0324543880170432,
            -0.3115032293331476,
        );
        assert_relative_eq!(a, b, epsilon = 1e-12);
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
