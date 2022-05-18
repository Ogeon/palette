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
use std::any::TypeId;

use crate::convert::IntoColorUnclamped;
use crate::encoding::{IntoLinear, Srgb};
use crate::num::{FromScalar, Powi, Recip, Sqrt};
use crate::ok_utils::{toe_inv, ChromaValues, LC, ST};
use crate::rgb::{Primaries, Rgb, RgbSpace, RgbStandard};
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
#[allow(clippy::excessive_precision)]
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
///
/// Oklab's chroma is unlimited and Oklab's lightness is unscaled.
/// However colors converted from `sRGB` will be in the `sRGB` gamut.
/// [`Okhsv`], [`Okhwb`](crate::Okhwb) and [`Okhsl`] reference the `sRGB` gamut.
/// The transformation from `Oklab` to one of them is based on the assumption,
/// that the transformed `Oklab` value is also based on `sRGB`, i.e. converted from `sRGB`.
///
/// `Okhsv`, `Okhwb` and `Okhsl` are not applicable to HDR, which also come with
/// color spaces with wider gamuts. They require
/// [additional research](https://bottosson.github.io/posts/colorpicker/#ideas-for-future-work).
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklab, Oklch, Okhsv, Okhsl, Xyz, Rgb)
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

    //FIXME: The -1 and 1 limits for a and b are random.
    // For sRGB the minima/maxima of a and b are interdependent.
    // ok_utils::tests::print_max_srgb_chroma_of_all_hues computes their independent
    // ranges (for SAMPLE_RESOLUTION == 300000, i.e. 1°/300000) at
    // -0.23388759702987336 <= a <= 0.27621677417023
    // -0.3115281582166001 <= b <= 0.19856971549244842
    // (also max chroma 0.3224909769769702 at hue 328.3634133333333°)
    // Values outside these ranges will certainly be outside the sRGB gamut.
    // Values in these ranges may -- depending on the combination of a and b and
    // lightness -- also be outside the sRGB gamut.
    // For Oklab in general -- i.e. independent of the gamut of a reference color
    // space -- a and b are unlimited.

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

fn linear_srgb_to_oklab<T>(c: LinSrgb<T>) -> Oklab<T>
where
    T: Real + Arithmetics + Cbrt + Copy + Debug,
{
    let l = T::from_f64(0.4122214708) * c.red
        + T::from_f64(0.5363325363) * c.green
        + T::from_f64(0.0514459929) * c.blue;
    let m = T::from_f64(0.2119034982) * c.red
        + T::from_f64(0.6806995451) * c.green
        + T::from_f64(0.1073969566) * c.blue;
    let s = T::from_f64(0.0883024619) * c.red
        + T::from_f64(0.2817188376) * c.green
        + T::from_f64(0.6299787005) * c.blue;

    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    let oklab = Oklab::new(
        T::from_f64(0.2104542553) * l_ + T::from_f64(0.7936177850) * m_
            - T::from_f64(0.0040720468) * s_,
        T::from_f64(1.9779984951) * l_ - T::from_f64(2.4285922050) * m_
            + T::from_f64(0.4505937099) * s_,
        T::from_f64(0.0259040371) * l_ + T::from_f64(0.7827717662) * m_
            - T::from_f64(0.8086757660) * s_,
    );
    //println!("linear srgb {c:?} -> {oklab:?}",);
    oklab
}

pub(crate) fn oklab_to_linear_srgb<T>(c: Oklab<T>) -> LinSrgb<T>
where
    T: Real + Arithmetics + Copy,
{
    let l_ = c.l + T::from_f64(0.3963377774) * c.a + T::from_f64(0.2158037573) * c.b;
    let m_ = c.l - T::from_f64(0.1055613458) * c.a - T::from_f64(0.0638541728) * c.b;
    let s_ = c.l - T::from_f64(0.0894841775) * c.a - T::from_f64(1.2914855480) * c.b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    LinSrgb::new(
        T::from_f64(4.0767416621) * l - T::from_f64(3.3077115913) * m
            + T::from_f64(0.2309699292) * s,
        T::from_f64(-1.2684380046) * l + T::from_f64(2.6097574011) * m
            - T::from_f64(0.3413193965) * s,
        T::from_f64(-0.0041960863) * l - T::from_f64(0.7034186147) * m
            + T::from_f64(1.7076147010) * s,
    )
}

impl<S, T> FromColorUnclamped<Rgb<S, T>> for Oklab<T>
where
    T: Real + Cbrt + Arithmetics + FromScalar + Copy + Debug,
    T::Scalar: Recip
        + IsValidDivisor<Mask = bool>
        + Arithmetics
        + FromScalar<Scalar = T::Scalar>
        + Real
        + Zero
        + One
        + Clone,
    S: RgbStandard,
    S::TransferFn: IntoLinear<T, T>,
    S::Space: RgbSpace<WhitePoint = D65> + 'static,
    <S::Space as RgbSpace>::Primaries: Primaries<T::Scalar>,
{
    fn from_color_unclamped(rgb: Rgb<S, T>) -> Self {
        if TypeId::of::<<S as RgbStandard>::Space>() == TypeId::of::<Srgb>() {
            // Use direct sRGB to Oklab conversion
            // fixme: why does the direct conversion produce relevant differences
            // to the conversion via XYZ?
            linear_srgb_to_oklab(rgb.into_linear().reinterpret_as())
        } else {
            // Convert via XYZ
            Xyz::from_color_unclamped(rgb).into_color_unclamped()
        }
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
        let lightness = toe_inv(l);

        let cs = ChromaValues::from_normalized(lightness, a_, b_);

        // Interpolate the three values for C so that:
        // At s=0: dC/ds = cs.zero, C = 0
        // At s=0.8: C = cs.mid
        // At s=1.0: C = cs.max

        let mid = T::from_f64(0.8);
        let mid_inv = T::from_f64(1.25);

        let chroma = if s < mid {
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

        Oklab::new(lightness, chroma * a_, chroma * b_)
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
        let s_0 = T::from_f64(0.5);
        let k = T::one() - s_0 / cusp.s;

        // first we compute L and V as if the gamut is a perfect triangle

        // L, C, when v == 1:
        let l_v = T::one() - hsv.saturation * s_0 / (s_0 + cusp.t - cusp.t * k * hsv.saturation);
        let c_v = hsv.saturation * cusp.t * s_0 / (s_0 + cusp.t - cusp.t * k * hsv.saturation);

        // then we compensate for both toe and the curved top part of the triangle:
        let l_vt = toe_inv(l_v);
        let c_vt = c_v * l_vt / l_v;

        let mut lightness = hsv.value * l_v;
        let mut chroma = hsv.value * c_v;
        let lightness_new = toe_inv(lightness);
        chroma = chroma * lightness_new / lightness;
        // the values may be outside the normal range
        let rgb_scale: LinSrgb<T> = Oklab::new(l_vt, a_ * c_vt, b_ * c_vt).into_color_unclamped();
        let lightness_scale_factor = T::cbrt(
            T::one()
                / T::max(
                    T::max(rgb_scale.red, rgb_scale.green),
                    T::max(rgb_scale.blue, T::zero()),
                ),
        );

        lightness = lightness_new * lightness_scale_factor;
        chroma = chroma * lightness_scale_factor;

        Oklab::new(lightness, chroma * a_, chroma * b_)
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

impl<T> Oklab<T>
where
    T: AbsDiffEq + One + Zero,
    T::Epsilon: Clone + Real + PartialOrd,
{
    /// Returns true, if `lightness == 1`
    ///
    /// **Note:** `sRGB` to `Oklab` conversion uses `f32` constants.
    /// A tolerance `epsilon >= 1e-8` is required to reliably detect white.
    /// Conversion of `sRGB` via XYZ requires `epsilon >= 1e-5`
    pub fn is_white(&self, epsilon: T::Epsilon) -> bool {
        self.l.abs_diff_eq(&T::one(), epsilon)
    }

    pub fn is_black(&self, epsilon: T::Epsilon) -> bool {
        self.l.abs_diff_eq(&T::zero(), epsilon)
    }

    pub fn is_grey(&self, epsilon: T::Epsilon) -> bool {
        self.a.abs_diff_eq(&T::zero(), epsilon.clone()) && self.b.abs_diff_eq(&T::zero(), epsilon)
    }

    fn both_black_or_both_white(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        self.is_white(epsilon.clone()) && other.is_white(epsilon.clone())
            || self.is_black(epsilon.clone()) && other.is_black(epsilon)
    }
}

impl<T> PartialEq for Oklab<T>
where
    T: PartialEq,
{
    /// Returns true, f `l`, `a` and `b` of `self` and `other` are identical.
    /// This is equality in name only, as with computed floating point numbers there
    /// always is an error, that must be accounted for, using a tolerance.   
    fn eq(&self, other: &Self) -> bool {
        self.l == other.l && self.a == other.a && self.b == other.b
    }
}
impl<T> Eq for Oklab<T> where T: Eq {}
impl<T> AbsDiffEq for Oklab<T>
where
    T: AbsDiffEq + One + Zero,
    T::Epsilon: Clone + Real + PartialOrd,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    /// Returns true, if `self ` and `other` are visually indiscernible, even
    /// if they hold are both black or both white and their `a` and `b` values differ.
    ///
    /// `epsilon` must be large enough to detect white (see [Oklab::is_white])
    fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        self.both_black_or_both_white(other, epsilon.clone())
            || self.l.abs_diff_eq(&other.l, epsilon.clone())
                && self.a.abs_diff_eq(&other.a, epsilon.clone())
                && self.b.abs_diff_eq(&other.b, epsilon)
    }
    fn abs_diff_ne(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        !self.both_black_or_both_white(other, epsilon.clone())
            && (self.l.abs_diff_ne(&other.l, epsilon.clone())
                || self.a.abs_diff_ne(&other.a, epsilon.clone())
                || self.b.abs_diff_ne(&other.b, epsilon))
    }
}
impl<T> RelativeEq for Oklab<T>
where
    T: RelativeEq + One + Zero,
    T::Epsilon: Clone + Real + PartialOrd,
{
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        self.both_black_or_both_white(other, epsilon.clone())
            || self
                .l
                .relative_eq(&other.l, epsilon.clone(), max_relative.clone())
                && self
                    .a
                    .relative_eq(&other.a, epsilon.clone(), max_relative.clone())
                && self.b.relative_eq(&other.b, epsilon, max_relative)
    }
    fn relative_ne(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        !self.both_black_or_both_white(other, epsilon.clone())
            && (self
                .l
                .relative_ne(&other.l, epsilon.clone(), max_relative.clone())
                || self
                    .a
                    .relative_ne(&other.a, epsilon.clone(), max_relative.clone())
                || self.b.relative_ne(&other.b, epsilon, max_relative))
    }
}
impl<T> UlpsEq for Oklab<T>
where
    T: UlpsEq + One + Zero,
    T::Epsilon: Clone + Real + PartialOrd,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        self.both_black_or_both_white(other, epsilon.clone())
            || self.l.ulps_eq(&other.l, epsilon.clone(), max_ulps)
                && self.a.ulps_eq(&other.a, epsilon.clone(), max_ulps)
                && self.b.ulps_eq(&other.b, epsilon, max_ulps)
    }
    fn ulps_ne(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        !self.both_black_or_both_white(other, epsilon.clone())
            && (self.l.ulps_ne(&other.l, epsilon.clone(), max_ulps)
                || self.a.ulps_ne(&other.a, epsilon.clone(), max_ulps)
                || self.b.ulps_ne(&other.b, epsilon, max_ulps))
    }
}

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
        // from https://github.com/bottosson/bottosson.github.io/blob/master/misc/ok_color.h
        let b = Oklab::new(0.6279553606145516, 0.22486306106597395, 0.1258462985307351);
        assert_relative_eq!(a, b, epsilon = 1e-8);
    }

    #[test]
    fn green() {
        let a = Oklab::from_color(LinSrgb::new(0.0, 1.0, 0.0));
        // from https://github.com/bottosson/bottosson.github.io/blob/master/misc/ok_color.h
        let b = Oklab::new(
            0.8664396115356694,
            -0.23388757418790812,
            0.17949847989672985,
        );
        assert_relative_eq!(a, b, epsilon = 1e-8);
    }

    #[test]
    fn blue() {
        let a = Oklab::from_color(LinSrgb::new(0.0, 0.0, 1.0));
        // from https://github.com/bottosson/bottosson.github.io/blob/master/misc/ok_color.h
        let b = Oklab::new(0.4520137183853429, -0.0324569841687640, -0.3115281476783751);
        assert_relative_eq!(a, b, epsilon = 1e-8);
    }

    #[test]
    fn black_eq_different_black() {
        assert_abs_diff_eq!(
            Oklab::new(0.0, 1.0, 0.0),
            Oklab::new(0.0, 0.0, 1.0),
            epsilon = 1e-8
        );
    }

    #[test]
    fn white_eq_different_white() {
        assert_abs_diff_eq!(
            Oklab::new(1.0, 1.0, 0.0),
            Oklab::new(1.0, 0.0, 1.0),
            epsilon = 1e-8
        );
    }

    #[test]
    fn white_ne_black() {
        assert_abs_diff_ne!(
            Oklab::new(1.0, 1.0, 0.0),
            Oklab::new(0.0, 0.0, 1.0),
            epsilon = 1e-8
        );
        assert_abs_diff_ne!(
            Oklab::new(1.0, 1.0, 0.0),
            Oklab::new(0.0, 1.0, 0.0),
            epsilon = 1e-8
        );
    }

    #[test]
    fn non_bw_neq_different_non_bw() {
        assert_abs_diff_ne!(
            Oklab::new(0.3, 1.0, 0.0),
            Oklab::new(0.3, 0.0, 1.0),
            epsilon = 1e-8
        );
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
