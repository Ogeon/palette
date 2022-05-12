use approx::{abs_diff_eq, AbsDiffEq};
use std::fmt::Debug;

use crate::convert::IntoColorUnclamped;
use crate::num::{FromScalar, Hypot, Powi, Recip, Sqrt};
use crate::{
    angle::RealAngle,
    convert::FromColorUnclamped,
    num::{Arithmetics, Cbrt, IsValidDivisor, MinMax, One, Real, Trigonometry, Zero},
    Alpha, LinSrgb, Oklab,
};

pub(crate) fn assert_normalized_hue<T>(a: T, b: T)
where
    T: One + Powi + Arithmetics + Debug + AbsDiffEq + Copy,
{
    if cfg!(debug_assertions) {
        if !abs_diff_eq!(a.powi(2) + b.powi(2), T::one()) {
            panic!("{:?}²+{:?}² == {:?} != 1", a, b, a.powi(2) + b.powi(2));
        }
    }
}

/// Okhsv with an alpha component. See the [`Okhsva` implementation in
/// `Alpha`](crate::Alpha#Okhsva).
pub type Okhsva<T = f32> = Alpha<Okhsv<T>, T>;

/// A Hue/Saturation/Value representation of [`Oklab`].
///
/// Allows
/// * changing lightness/chroma/saturation while keeping perceived Hue constant
/// (like HSV promises but delivers only partially)  
/// * finding the strongest color (maximum chroma) at s == 1 (like HSV)  
#[derive(
    Debug,
    ArrayCast,
    //FromColorUnclamped,
    WithAlpha,
)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklab, Oklch, Okhsv)
)]
#[repr(C)]
pub struct Okhsv<T = f32> {
    /// The hue of the color, in degrees of a circle, where for all `h`: `h+n*360 ==  h`.
    ///
    /// For fully saturated, bright colors
    /// * 0° corresponds to a kind of magenta-pink (RBG #ff0188),
    /// * 90° to a kind of yellow (RBG RGB #ffcb00)
    /// * 180° to a kind of cyan (RBG #00ffe1) and
    /// * 240° to a kind of blue (RBG #00aefe).
    ///
    /// For s == 0 or v == 0, the hue is irrelevant.
    pub h: T,

    /// The saturation (freedom of whitishness) of the color.
    ///
    /// * `0.0` corresponds to pure mixture of black and white without any color.
    /// The black to white relation depends on v.
    /// * `1.0` to a fully saturated color without any white.
    ///
    /// For v == 0 the saturation is irrelevant.
    pub s: T,

    /// The monochromatic brightness of the color.
    /// * `0.0` corresponds to pure black
    /// * `1.0` corresponds to a maximally bright colour -- be it very colorful or very  white
    pub v: T,
}

impl<T> Copy for Okhsv<T> where T: Copy {}

impl<T> Clone for Okhsv<T>
where
    T: Clone,
{
    fn clone(&self) -> Okhsv<T> {
        Okhsv {
            h: self.h.clone(),
            s: self.s.clone(),
            v: self.v.clone(),
        }
    }
}

impl<T> Okhsv<T>
where
    T: Real + Zero + One + PartialOrd + Debug + Copy + Arithmetics,
{
    /// Create an Okhsv color.
    // FIXMe: cannot make constructor constant because zero and one are not constant
    pub fn new(hue: T, mut saturation: T, mut value: T) -> Self {
        println!("Creating Okhsv({:?}, {:?}, {:?})", hue, saturation, value);
        let EPSILON: T = T::from_f64(1e-4);
        if value < T::zero() && value > -EPSILON {
            value = T::zero()
        }
        if value > T::one() && value < T::from_f64(1.0) + EPSILON {
            value = T::one()
        }
        if saturation < T::zero() && saturation > -EPSILON {
            saturation = T::zero()
        }
        if saturation > T::one() && saturation < T::from_f64(1.0) + EPSILON {
            saturation = T::one()
        }

        debug_assert!(hue == hue, "hue is NaN");
        debug_assert!(saturation == saturation, "saturation is NaN");
        debug_assert!(value == value, "value/brightness is NaN");
        debug_assert!(saturation >= T::zero(), "saturation {:?} < 0", saturation);
        debug_assert!(saturation <= T::one(), "saturation {:?} > 1", saturation);
        debug_assert!(value >= T::zero(), "value {:?} < 0", value);
        debug_assert!(value <= T::one(), "value {:?} > 1", value);
        Self {
            h: hue,
            s: saturation,
            v: value,
        }
    }

    /// Convert to a `(h, s, v)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.h, self.s, self.v)
    }

    /// Convert from a `(h, s, v)` tuple.
    pub fn from_components((h, s, v): (T, T, T)) -> Self {
        Self::new(h, s, v)
    }
}

/// A lightness/chroma representation of the `sRGB` gamut for a fixed hue.
///
/// Gamut is the range of representable colors of a color space. In this case the
/// `sRGB` color space.
///
/// For each hue the geometrical shape of the `sRGB` gamut forms a specific triangle
/// with a concave upper line.
///
///# See [LC diagram samples](https://bottosson.github.io/posts/gamutclipping/#gamut-clipping)
#[derive(Debug, Copy, Clone)]
pub(crate) struct LC<T> {
    /// The lightness of the color. 0 corresponds to black. 1 corresponds to white
    pub l: T,
    /// The chroma of the color. 0 corresponds to totally desaturated (white, grey or black).
    /// Larger values correspond to colorful values.
    ///
    ///Note: the maximum representable value depends on the lightness and the hue.
    pub c: T,
}

impl<T> LC<T>
where
    T: Real
        + Debug
        + AbsDiffEq
        + PartialOrd
        + MinMax
        + Copy
        + Powi
        + Sqrt
        + Cbrt
        + Arithmetics
        + Trigonometry
        + Zero
        + One
        + FromScalar,
    T::Scalar: Real
        + Zero
        + One
        + Recip
        + IsValidDivisor<Mask = bool>
        + Arithmetics
        + Clone
        + FromScalar<Scalar = T::Scalar>,
{
    /// Returns the cusp of the geometrical shape of representable `sRGB` colors for
    /// normalized `a` and `b` values of a hue in `OKlab`.
    ///
    /// Normalized means, that `a² + b² == 1`
    pub fn find_cusp(a: T, b: T) -> Self {
        assert_normalized_hue(a, b);
        // First, find the maximum saturation (saturation S = C/L)
        let S_cusp = Self::max_saturation(a, b);

        // Convert to linear sRGB to find the first point where at least one of r,g or b >= 1:
        let rgb_at_max: LinSrgb<T> =
            Oklab::new(T::one(), S_cusp * a, S_cusp * b).into_color_unclamped();

        let L_cusp =
            T::cbrt(T::one() / T::max(T::max(rgb_at_max.red, rgb_at_max.green), rgb_at_max.blue));
        Self {
            l: L_cusp,
            c: L_cusp * S_cusp,
        }
    }
    /// Returns the maximum `sRGB`-saturation (chroma / lightness) for a given hue,
    /// described by of vectors `a` and `b`,
    /// where `a` describes the green/redness of the hue
    /// and `b` describes the blue/yellowness of the hue
    /// and `a` and `b` are normalized to a chroma of `1`.
    ///
    /// # Panics
    /// Panics, if /// `a²+b² != 1`
    fn max_saturation(a: T, b: T) -> T {
        assert_normalized_hue(a, b);
        // Max saturation will be when one of r, g or b goes below zero.
        // Select different coefficients depending on which component goes below zero first
        // wl, wm and ws are coefficients for https://en.wikipedia.org/wiki/LMS_color_space
        // -- the color space modelling human perception.
        let (k0, k1, k2, k3, k4, wl, wm, ws) =
            if T::from_f64(-1.88170328) * a - T::from_f64(0.80936493) * b > T::one() {
                println!("RED component goes below zero first");
                // red component
                (
                    T::from_f64(1.19086277),
                    T::from_f64(1.76576728),
                    T::from_f64(0.59662641),
                    T::from_f64(0.75515197),
                    T::from_f64(0.56771245),
                    T::from_f64(4.0767416621),
                    T::from_f64(-3.3077115913),
                    T::from_f64(0.2309699292),
                )
            } else if T::from_f64(1.81444104) * a - T::from_f64(1.19445276) * b > T::one() {
                println!("GREEN component goes below zero");
                // green component
                (
                    T::from_f64(0.73956515),
                    T::from_f64(-0.45954404),
                    T::from_f64(0.08285427),
                    T::from_f64(0.12541070),
                    T::from_f64(0.14503204),
                    T::from_f64(-1.2684380046),
                    T::from_f64(2.6097574011),
                    T::from_f64(-0.3413193965),
                )
            } else {
                println!("BLUE component goes below zero");
                // blue component
                (
                    T::from_f64(1.35733652),
                    T::from_f64(-0.00915799),
                    T::from_f64(-1.15130210),
                    T::from_f64(-0.50559606),
                    T::from_f64(0.00692167),
                    T::from_f64(-0.0041960863),
                    T::from_f64(-0.7034186147),
                    T::from_f64(1.7076147010),
                )
            };

        // Approximate max saturation using a polynomial
        let mut approx_max_saturation = k0 + k1 * a + k2 * b + k3 * a.powi(2) + k4 * a * b;
        println!(
            "Initial approximation to max saturation for a {:?}, b {:?}: {:?}",
            a, b, approx_max_saturation
        );
        // Get closer with Halley's method

        let k_l = T::from_f64(0.3963377774) * a + T::from_f64(0.2158037573) * b;
        let k_m = T::from_f64(-0.1055613458) * a - T::from_f64(0.0638541728) * b;
        let k_s = T::from_f64(-0.0894841775) * a - T::from_f64(1.2914855480) * b;

        // For most hues the first step gives an error less than 10e6.
        // For some blue hues, where the dS/dh is close to infinite, the error is larger.
        // For pure SRGB blue the optimization process oscillates after more than 4 iterations
        // due to rounding errors even with f64.
        // TODO: find the difficult blues and use a 3 or 4 iterations for them
        let MAX_ITER = 2;
        for i in 0..MAX_ITER {
            let l_ = T::one() + approx_max_saturation * k_l;
            let m_ = T::one() + approx_max_saturation * k_m;
            let s_ = T::one() + approx_max_saturation * k_s;

            let l = l_.powi(3);
            let m = m_.powi(3);
            let s = s_.powi(3);

            let l_dS = T::from_f64(3.0) * k_l * l_.powi(2);
            let m_dS = T::from_f64(3.0) * k_m * m_.powi(2);
            let s_dS = T::from_f64(3.0) * k_s * s_.powi(2);

            let l_dS2 = T::from_f64(6.0) * k_l.powi(2) * l_;
            let m_dS2 = T::from_f64(6.0) * k_m.powi(2) * m_;
            let s_dS2 = T::from_f64(6.0) * k_s.powi(2) * s_;

            // let x be the approximate maximum saturation and
            // i the current iteration
            // f = f(x_i), f1 = f'(x_i), f2 = f''(x_i) for
            let f = wl * l + wm * m + ws * s;
            let f1 = wl * l_dS + wm * m_dS + ws * s_dS;
            let f2 = wl * l_dS2 + wm * m_dS2 + ws * s_dS2;

            approx_max_saturation =
                approx_max_saturation - f * f1 / (f1.powi(2) - T::from_f64(0.5) * f * f2);
            println!(
                "Approximation to max saturation after iteration {}: {:?}",
                i, approx_max_saturation
            );
        }
        approx_max_saturation
    }
}

/// Alternative representation of (L_cusp, C_cusp)
/// Encoded so S = C_cusp/L_cusp and T = C_cusp/(1-L_cusp)
/// The maximum value for C in the triangle is then found as fmin(S*L, T*(1-L)), for a given L
pub(crate) struct ST<T> {
    pub s: T,
    pub t: T,
}

impl<T> From<LC<T>> for ST<T>
where
    T: Real + Arithmetics + One + Copy,
{
    fn from(cusp: LC<T>) -> Self {
        ST {
            s: cusp.c / cusp.l,
            t: cusp.c / (T::one() - cusp.l),
        }
    }
}

/// Maps a `Oklab` lightness `x` to a a lightness `L_r`.
///
/// The `Oklab` lightness is scale independent, i.e. `0` is black, `1` is pure white, but
/// the luminosity of pure white is undefined. `Oklab`'s lightness is not limited to the human
/// ability to see nor a displays to produce color. Lightness values may mean different things in
/// different contexts (maximum display luminosity, background color and other viewing conditions).
///
/// `sRGB` however has a well defined dynamic range and a clear reference white luminance.
/// Mapping `1` to that luminance is just a matter of definition. But is say `0.8` `Oklab`
/// lightness equal to `0.5` or `0.9` `sRGB` luminance?
///   
/// The shape and weights of `L_r` are chosen to closely matches the lightness estimate of
/// the `CIELab` color space and be nearly equal at `0.5`.
///
/// # See
/// https://bottosson.github.io/posts/colorpicker/#intermission---a-new-lightness-estimate-for-oklab
pub(crate) fn toe<T>(x: T) -> T
where
    T: Real + Copy + Powi + Sqrt + Arithmetics + One,
{
    let k_1 = T::from_f64(0.206);
    let k_2 = T::from_f64(0.03);
    let k_3 = (T::one() + k_1) / (T::one() + k_2);
    T::from_f64(0.5)
        * (k_3 * x - k_1 + T::sqrt((k_3 * x - k_1).powi(2) + T::from_f64(4.0) * k_2 * k_3 * x))
}

/// Maps a lightness based on a defined dynamic range and a reference white luminance
/// to `Oklab`s scale-free luminance.
///
/// Inverse of [`toe`]
pub(crate) fn toe_inv<T>(x: T) -> T
where
    T: Real + Copy + Powi + Arithmetics + One,
{
    let k_1 = T::from_f64(0.206);
    let k_2 = T::from_f64(0.03);
    let k_3 = (T::one() + k_1) / (T::one() + k_2);
    (x.powi(2) + k_1 * x) / (k_3 * (x + k_2))
}

/// Converts `lab` to `Okhsv` in the bounds of sRGB.
///
/// # See
/// See [`srgb_to_okhsv`](https://bottosson.github.io/posts/colorpicker/#hsv-2).
/// This implementation differs from srgb_to_okhsv in that it starts with the `lab`
/// value and produces hues in degrees, whereas `srgb_to_okhsv` produces degree/360.
impl<T> FromColorUnclamped<Oklab<T>> for Okhsv<T>
where
    T: Real
        + AbsDiffEq
        + PartialOrd
        + MinMax
        + Copy
        + Powi
        + Sqrt
        + Cbrt
        + Arithmetics
        + Trigonometry
        + Zero
        + Hypot
        + One
        + FromScalar
        + Debug
        + RealAngle,
    T::Scalar: Real
        + Zero
        + One
        + Recip
        + Hypot
        + IsValidDivisor<Mask = bool>
        + Arithmetics
        + Clone
        + FromScalar<Scalar = T::Scalar>
        + Debug,
{
    fn from_color_unclamped(lab: Oklab<T>) -> Self {
        println!("Converting {:?} to Okhsl", lab);
        if lab.a == T::zero() && lab.b == T::zero() {
            // `a` describes how green/red the color is, `b` how blue/yellow the color is
            // both are zero -> the color is totally desaturated.
            let v = toe(lab.l);
            println!("{lab:?} is greyscale with l {:?} -> v {v:?}", lab.l);

            return Self::new(T::zero(), T::zero(), v);
        }

        // compute hue and chroma as for OkLCh
        // we will use h as is.
        let mut C = T::hypot(lab.a, lab.b);
        let a_ = lab.a / C;
        let b_ = lab.b / C;

        let mut L = lab.l;
        let h = T::from_f64(180.0) + T::atan2(-lab.b, -lab.a).radians_to_degrees();

        // For each hue the sRGB gamut can be drawn on a 2-dimensional space.
        // Let L_r, the lightness in relation to the possible luminance of sRGB, be spread
        // along the y-axis (bottom is black, top is bright) and Chroma along the x-axis
        // (left is desaturated, right is colorful). The gamut then takes a triangular shape,
        // with a concave top side and a cusp to the right.
        // To use saturation and brightness values, the gamut must be mapped to a square.
        // The lower point of the triangle is expanded to the lower side of the square.
        // The left side remains unchanged and the cusp of the triangle moves to the upper right.
        let cusp = LC::find_cusp(a_, b_);
        println!(
            "Cusp of hue {:?}°: Lightness {:?}, Chroma {:?}",
            h, cusp.l, cusp.c
        );
        let ST_max: ST<T> = cusp.into();
        let S_0 = T::from_f64(0.5);
        let k = T::one() - S_0 / ST_max.s;

        // first we find L_v, C_v, L_vt and C_vt
        let t = ST_max.t / (C + L * ST_max.t);
        let L_v = t * L;
        let C_v = t * C;

        let L_vt = toe_inv(L_v);
        let C_vt = C_v * L_vt / L_v;

        // we can then use these to invert the step that compensates for the toe and the curved top part of the triangle:
        let rgb_scale: LinSrgb<T> = Oklab::new(L_vt, a_ * C_vt, b_ * C_vt).into_color_unclamped();
        let scale_L = T::cbrt(
            T::one()
                / T::max(
                    T::max(rgb_scale.red, rgb_scale.green),
                    T::max(rgb_scale.blue, T::zero()),
                ),
        );

        L = L / scale_L;
        C = C / scale_L;

        // use L_r instead of L and also scale C by L_r/L
        let L_r = toe(L);
        C = C * L_r / L;
        L = L_r;

        // we can now compute v and s:
        let v = L / L_v;
        let s = (S_0 + ST_max.t) * C_v / ((ST_max.t * S_0) + ST_max.t * k * C_v);

        Self::new(h, s, v)
    }
}

impl<T> FromColorUnclamped<Okhsv<T>> for Oklab<T>
where
    T: Real
        + AbsDiffEq
        + PartialOrd
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
        if hsv.s == T::zero() {
            // totally desaturated color -- the triangle is just the 0-chroma-line
            if hsv.v == T::zero() {
                // pure black
                return Self {
                    l: T::zero(),
                    a: T::zero(),
                    b: T::zero(),
                };
            }
            let l = toe_inv(hsv.v);
            println!("{hsv:?} is greyscale with v {:?} -> l {l:?}", hsv.v);
            return Self {
                l,
                a: T::zero(),
                b: T::zero(),
            };
        }

        let h_radians = hsv.h.degrees_to_radians();
        let a_ = T::cos(h_radians);
        let b_ = T::sin(h_radians);

        let cusp = LC::find_cusp(a_, b_);
        let cusp: ST<T> = cusp.into();
        let S_0 = T::from_f64(0.5);
        let k = T::one() - S_0 / cusp.s;

        // first we compute L and V as if the gamut is a perfect triangle

        // L, C, when v == 1:
        let L_v = T::one() - hsv.s * S_0 / (S_0 + cusp.t - cusp.t * k * hsv.s);
        let C_v = hsv.s * cusp.t * S_0 / (S_0 + cusp.t - cusp.t * k * hsv.s);

        // then we compensate for both toe and the curved top part of the triangle:
        let L_vt = toe_inv(L_v);
        let C_vt = C_v * L_vt / L_v;

        let mut L = hsv.v * L_v;
        let mut C = hsv.v * C_v;
        let L_new = toe_inv(L);
        C = C * L_new / L;
        println!("Creating Oklab {:?} {:?} {:?}", L_vt, a_ * C_vt, b_ * C_vt);
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

#[cfg(test)]
mod tests {
    use crate::convert::FromColorUnclamped;
    use crate::okhsv::{toe, toe_inv};
    use crate::rgb::Rgb;
    use crate::{encoding, Okhsv, Oklab, Srgb};
    use std::str::FromStr;

    #[test]
    fn test_roundtrip_toe_is_original() {
        let n = 500;
        for i in 0..n {
            let x = i as f64 / n as f64;
            assert_ulps_eq!(toe_inv(toe(x)), x);
        }

        let x = 1000.0;
        assert_ulps_eq!(toe_inv(toe(x)), x);
    }

    #[test]
    fn test_toe() {
        assert_eq!(toe(0.0), 0.0);
        assert_eq!(toe(1.0), 1.0);
        let grey50srgb: Srgb = Rgb::<encoding::Srgb, u8>::from_str("#777777")
            .unwrap()
            .into_format();
        let grey50oklab = Oklab::from_color_unclamped(grey50srgb);
        println!("grey 50% oklab lightness: {}", grey50oklab.l);
        assert!(relative_eq!(toe(grey50oklab.l), 0.5, epsilon = 1e-3));
    }

    #[test]
    fn test_roundtrip_okhsv_oklab_is_original() {
        let colors = [
            ("red", Oklab::new(0.627955, 0.224863, 0.125846)),
            ("lime", Oklab::new(0.86644, -0.233888, 0.179498)),
            ("cyan", Oklab::new(0.905399, -0.149444, -0.039398)),
            ("magenta", Oklab::new(0.701674, 0.274566, -0.169156)),
            ("white", Oklab::new(1.000000, 0.000000, 0.000000)),
            ("black", Oklab::new(0.000000, 0.000000, 0.000000)),
            ("grey", Oklab::new(0.3, 0.0, 0.0)),
            ("yellow", Oklab::new(0.967983, -0.071369, 0.198570)),
            ("blue", Oklab::new(0.452014, -0.032457, -0.311528)),
        ];
        for (name, color) in colors {
            let rgb: Rgb<encoding::Srgb, u8> =
                crate::Srgb::<f64>::from_color_unclamped(color).into_format();
            println!(
                "\n\
            roundtrip of {name} (#{:x})\n\
            =================================================",
                rgb
            );
            let roundtrip_color = Oklab::from_color_unclamped(Okhsv::from_color_unclamped(color));
            assert!(
                relative_eq!(roundtrip_color, color, epsilon = 1e-3),
                "'{name}' failed. {:?} != {:?}",
                roundtrip_color,
                color
            );
        }
    }

    #[test]
    fn test_srgb_to_okhsv() {
        let red_hex = "#ff0004";
        let rgb: Srgb = Rgb::<encoding::Srgb, _>::from_str(red_hex)
            .unwrap()
            .into_format();
        let oklab = Oklab::from_color_unclamped(rgb);
        let okhsv = Okhsv::from_color_unclamped(oklab);
        assert_relative_eq!(okhsv.s, 1.0, epsilon = 1e-3);
        assert_relative_eq!(okhsv.v, 1.0, epsilon = 1e-3);
        assert_relative_eq!(okhsv.h, 29.0, epsilon = 1e-3, max_relative = 1e-3);
    }

    #[test]
    fn test_okhsv_to_srgb() {
        let okhsv = Okhsv::new(0.0_f32, 0.5, 0.5);
        let oklab = Oklab::from_color_unclamped(okhsv);
        let rgb = Srgb::from_color_unclamped(oklab);
        let rgb8: Rgb<encoding::Srgb, u8> = rgb.into_format();
        let hex_str = format!("{:x}", rgb8);
        assert_eq!(hex_str, "7a4355");
    }
}
