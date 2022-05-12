use crate::convert::IntoColorUnclamped;
use crate::num::{
    Arithmetics, Cbrt, FromScalar, IsValidDivisor, MinMax, One, Powi, Real, Recip, Sqrt,
    Trigonometry, Zero,
};
use crate::{HasBoolMask, LinSrgb, Oklab};
use approx::abs_diff_eq;
use approx::AbsDiffEq;
use core::fmt::Debug;

/// Returns a smooth approximation of the location of the cusp.
///
/// This polynomial was created by an optimization process.
/// It has been designed so that `S_mid < S_max` and `T_mid < T_max`
#[rustfmt::skip]
fn get_ST_mid<T>(a_: T, b_: T) -> ST<T>
where
    T: Real + Arithmetics + Copy + One,
{
    let s = T::from_f64(0.11516993) + T::one() / (
            T::from_f64(7.44778970) 
            + T::from_f64(4.15901240) * b_
            + a_ * (T::from_f64(-2.19557347)+ T::from_f64(1.75198401) * b_
            + a_ * (T::from_f64(-2.13704948) - T::from_f64(10.02301043) * b_
            + a_ * (T::from_f64(-4.24894561) + T::from_f64(5.38770819) * b_+ T::from_f64(4.69891013) * a_
            )))
    );

    let t = T::from_f64(0.11239642)+ T::one()/ (
        T::from_f64(1.61320320) - T::from_f64(0.68124379) * b_
        + a_ * (T::from_f64(0.40370612)
        + T::from_f64(0.90148123) * b_
        + a_ * (T::from_f64(-0.27087943) + T::from_f64(0.61223990) * b_
        + a_ * (T::from_f64(0.00299215) - T::from_f64(0.45399568) * b_ - T::from_f64(0.14661872) * a_
        )))
    );
 ST { s, t }
}

/// Finds intersection of the line defined by
///
/// L = L0 * (1 - t) + t * L1;
///
/// C = t * C1;
///
/// a and b must be normalized so a² + b² == 1
fn find_gamut_intersection<T>(a: T, b: T, L1: T, C1: T, L0: T, cusp: Option<LC<T>>) -> T
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
        + PartialEq
        + PartialOrd
        + HasBoolMask<Mask = bool>
        + Powi
        + Cbrt
        + Trigonometry
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
    assert_normalized_hue(a, b);

    // Find the cusp of the gamut triangle
    let cusp = cusp.unwrap_or_else(|| LC::find_cusp(a, b));

    // Find the intersection for upper and lower half separately
    if ((L1 - L0) * cusp.chroma - (cusp.lightness - L0) * C1) <= T::zero() {
        // Lower half

        cusp.chroma * L0 / (C1 * cusp.lightness + cusp.chroma * (L0 - L1))
    } else {
        // Upper half

        // First intersect with triangle
        let t = cusp.chroma * (L0 - T::one())
            / (C1 * (cusp.lightness - T::one()) + cusp.chroma * (L0 - L1));

        // Then one step Halley's method
        {
            let dL = L1 - L0;
            let dC = C1;

            let k_l = T::from_f64(0.3963377774) * a + T::from_f64(0.2158037573) * b;
            let k_m = -T::from_f64(0.1055613458) * a - T::from_f64(0.0638541728) * b;
            let k_s = -T::from_f64(0.0894841775) * a - T::from_f64(1.2914855480) * b;

            let l_dt = dL + dC * k_l;
            let m_dt = dL + dC * k_m;
            let s_dt = dL + dC * k_s;

            // If higher accuracy is required, 2 or 3 iterations of the following block can be used:
            {
                let L = L0 * (T::one() - t) + t * L1;
                let C = t * C1;

                let l_ = L + C * k_l;
                let m_ = L + C * k_m;
                let s_ = L + C * k_s;

                let l = l_ * l_ * l_;
                let m = m_ * m_ * m_;
                let s = s_ * s_ * s_;

                let ldt = T::from_f64(3.0) * l_dt * l_ * l_;
                let mdt = T::from_f64(3.0) * m_dt * m_ * m_;
                let sdt = T::from_f64(3.0) * s_dt * s_ * s_;

                let ldt2 = T::from_f64(6.0) * l_dt * l_dt * l_;
                let mdt2 = T::from_f64(6.0) * m_dt * m_dt * m_;
                let sdt2 = T::from_f64(6.0) * s_dt * s_dt * s_;

                let r = T::from_f64(4.0767416621) * l - T::from_f64(3.3077115913) * m
                    + T::from_f64(0.2309699292) * s
                    - T::one();
                let r1 = T::from_f64(4.0767416621) * ldt - T::from_f64(3.3077115913) * mdt
                    + T::from_f64(0.2309699292) * sdt;
                let r2 = T::from_f64(4.0767416621) * ldt2 - T::from_f64(3.3077115913) * mdt2
                    + T::from_f64(0.2309699292) * sdt2;

                let u_r = r1 / (r1 * r1 - T::from_f64(0.5) * r * r2);
                let mut t_r = -r * u_r;

                let g = -T::from_f64(1.2684380046) * l + T::from_f64(2.6097574011) * m
                    - T::from_f64(0.3413193965) * s
                    - T::one();
                let g1 = -T::from_f64(1.2684380046) * ldt + T::from_f64(2.6097574011) * mdt
                    - T::from_f64(0.3413193965) * sdt;
                let g2 = -T::from_f64(1.2684380046) * ldt2 + T::from_f64(2.6097574011) * mdt2
                    - T::from_f64(0.3413193965) * sdt2;

                let u_g = g1 / (g1 * g1 - T::from_f64(0.5) * g * g2);
                let mut t_g = -g * u_g;

                let b = -T::from_f64(0.0041960863) * l - T::from_f64(0.7034186147) * m
                    + T::from_f64(1.7076147010) * s
                    - T::one();
                let b1 = -T::from_f64(0.0041960863) * ldt - T::from_f64(0.7034186147) * mdt
                    + T::from_f64(1.7076147010) * sdt;
                let b2 = -T::from_f64(0.0041960863) * ldt2 - T::from_f64(0.7034186147) * mdt2
                    + T::from_f64(1.7076147010) * sdt2;

                let u_b = b1 / (b1 * b1 - T::from_f64(0.5) * b * b2);
                let mut t_b = -b * u_b;

                let FLT_MAX = T::from_f64(10e5);

                t_r = if u_r >= T::zero() { t_r } else { FLT_MAX };
                t_g = if u_g >= T::zero() { t_g } else { FLT_MAX };
                t_b = if u_b >= T::zero() { t_b } else { FLT_MAX };

                t + T::min(t_r, T::min(t_g, t_b))
            }
        }
    }
}

pub struct ChromaValues<T> {
    pub zero: T,
    pub mid: T,
    pub max: T,
}

impl<T> ChromaValues<T>
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
    pub fn from_normalized(L: T, a_: T, b_: T) -> Self {
        assert_normalized_hue(a_, b_);
        let cusp = LC::find_cusp(a_, b_);

        let C_max = find_gamut_intersection(a_, b_, L, T::one(), L, Some(cusp));
        let ST_max = ST::from(cusp);

        // Scale factor to compensate for the curved part of gamut shape:
        let k = C_max / T::min(L * ST_max.s, (T::one() - L) * ST_max.t);

        let C_mid = {
            let ST_mid = get_ST_mid(a_, b_);

            // Use a soft minimum function, instead of a sharp triangle shape to get a smooth value for chroma.
            let C_a = L * ST_mid.s;
            let C_b = (T::one() - L) * ST_mid.t;
            T::from_f64(0.9)
                * k
                * T::sqrt(T::sqrt(
                    T::one()
                        / (T::one() / (C_a * C_a * C_a * C_a) + T::one() / (C_b * C_b * C_b * C_b)),
                ))
        };

        let C_0 = {
            // for C_0, the shape is independent of hue, so ST are constant. Values picked to roughly be the average values of ST.
            let C_a = L * T::from_f64(0.4);
            let C_b = (T::one() - L) * T::from_f64(0.8);

            // Use a soft minimum function, instead of a sharp triangle shape to get a smooth value for chroma.
            T::sqrt(T::one() / (T::one() / (C_a * C_a) + T::one() / (C_b * C_b)))
        };
        Self {
            zero: C_0,
            mid: C_mid,
            max: C_max,
        }
    }
}

pub(crate) fn assert_normalized_hue<T>(a: T, b: T)
where
    T: One + Powi + Arithmetics + Debug + AbsDiffEq + Copy,
{
    if cfg!(debug_assertions) && !abs_diff_eq!(a.powi(2) + b.powi(2), T::one()) {
        panic!("{:?}²+{:?}² == {:?} != 1", a, b, a.powi(2) + b.powi(2));
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
    pub lightness: T,
    /// The chroma of the color. 0 corresponds to totally desaturated (white, grey or black).
    /// Larger values correspond to colorful values.
    ///
    ///Note: the maximum representable value depends on the lightness and the hue.
    pub chroma: T,
}

impl<T> LC<T>
where
    T: Real
        + Debug
        + AbsDiffEq
        + PartialOrd
        + HasBoolMask<Mask = bool>
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
            lightness: L_cusp,
            chroma: L_cusp * S_cusp,
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
        for _i in 0..MAX_ITER {
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
            s: cusp.chroma / cusp.lightness,
            t: cusp.chroma / (T::one() - cusp.lightness),
        }
    }
}

/// Maps a `oklab_lightness` to a a lightness `L_r`.
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
pub(crate) fn toe<T>(oklab_lightness: T) -> T
where
    T: Real + Copy + Powi + Sqrt + Arithmetics + One,
{
    let k_1 = T::from_f64(0.206);
    let k_2 = T::from_f64(0.03);
    let k_3 = (T::one() + k_1) / (T::one() + k_2);
    T::from_f64(0.5)
        * (k_3 * oklab_lightness - k_1
            + T::sqrt(
                (k_3 * oklab_lightness - k_1).powi(2)
                    + T::from_f64(4.0) * k_2 * k_3 * oklab_lightness,
            ))
}

/// Maps a lightness based on a defined dynamic range and a reference white luminance
/// to `Oklab`s scale-free luminance.
///
/// Inverse of [`toe`]
pub(crate) fn toe_inv<T>(l_r: T) -> T
where
    T: Real + Copy + Powi + Arithmetics + One,
{
    let k_1 = T::from_f64(0.206);
    let k_2 = T::from_f64(0.03);
    let k_3 = (T::one() + k_1) / (T::one() + k_2);
    (l_r.powi(2) + k_1 * l_r) / (k_3 * (l_r + k_2))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::convert::FromColorUnclamped;
    use crate::rgb::Rgb;
    use crate::{encoding, Oklab, Srgb};
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
}
