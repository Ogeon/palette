use approx::AbsDiffEq;
use std::fmt::Debug;

use crate::num::{FromScalar, Hypot, Powi, Recip, Sqrt};
use crate::okhsv::{assert_normalized_hue, toe, toe_inv, LC, ST};
use crate::{
    angle::RealAngle,
    convert::FromColorUnclamped,
    num::{Arithmetics, Cbrt, IsValidDivisor, MinMax, One, Real, Trigonometry, Zero},
    Alpha, Oklab,
};

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

// Finds intersection of the line defined by
// L = L0 * (1 - t) + t * L1;
// C = t * C1;
// a and b must be normalized so a^2 + b^2 == 1
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

    // Find the intersection for upper and lower half seprately
    if ((L1 - L0) * cusp.c - (cusp.l - L0) * C1) <= T::zero() {
        // Lower half

        cusp.c * L0 / (C1 * cusp.l + cusp.c * (L0 - L1))
    } else {
        // Upper half

        // First intersect with triangle
        let t = cusp.c * (L0 - T::one()) / (C1 * (cusp.l - T::one()) + cusp.c * (L0 - L1));

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

struct Cs<T> {
    C_0: T,
    C_mid: T,
    C_max: T,
}

impl<T> Cs<T>
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
        Self { C_0, C_mid, C_max }
    }
}

/// Okhsl with an alpha component.
pub type Okhsla<T = f32> = Alpha<Okhsl<T>, T>;

/// A Hue/Saturation/Lightness representation of [`Oklab`].
///
/// Allows
/// * changing hue/chroma/saturation, while keeping perceived lightness constant (like HSLuv)
/// * changing lightness/chroma/saturation, while keeping perceived hue constant
/// * changing the perceived saturation (more or less) proportionally with the numerical
/// amount of change (unlike HSLuv)
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
pub struct Okhsl<T = f32> {
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

    /// The saturation (freedom of black or white) of the color.
    ///
    /// * `0.0` corresponds to pure mixture of black and white without any color.
    /// The black to white relation depends on v.
    /// * `1.0` to a fully saturated color without any white.
    ///
    /// For v == 0 the saturation is irrelevant.
    pub s: T,

    /// The amount of black and white "paint in the mixture".
    /// While changes do not affect the saturation, they do affect
    /// * `0.0` corresponds to pure black
    /// * `1.0` corresponds to a maximally bright colour
    pub l: T,
}

impl<T> Okhsl<T>
where
    T: Real + Zero + One + PartialOrd + Debug + Copy + Arithmetics,
{
    /// Create an Okhsl color.
    // FIXMe: cannot make constructor constant because zero and one are not constant
    pub fn new(hue: T, mut saturation: T, mut lightness: T) -> Self {
        println!(
            "Creating Okhsl({:?}, {:?}, {:?})",
            hue, saturation, lightness
        );
        let EPSILON: T = T::from_f64(1e-4);
        if lightness < T::zero() && lightness > -EPSILON {
            lightness = T::zero()
        }
        if lightness > T::one() && lightness < T::one() + EPSILON {
            lightness = T::one()
        }
        if saturation < T::zero() && saturation > -EPSILON {
            saturation = T::zero()
        }
        if saturation > T::one() && saturation < T::one() + EPSILON {
            saturation = T::one()
        }

        debug_assert!(hue == hue, "hue is NaN");
        debug_assert!(saturation == saturation, "saturation is NaN");
        debug_assert!(lightness == lightness, "lightness is NaN");
        debug_assert!(saturation >= T::zero(), "saturation {:?} < 0", saturation);
        debug_assert!(saturation <= T::one(), "saturation {:?} > 1", saturation);
        debug_assert!(lightness >= T::zero(), "lightness {:?} < 0", lightness);
        debug_assert!(lightness <= T::one(), "lightness {:?} > 1", lightness);
        Self {
            h: hue,
            s: saturation,
            l: lightness,
        }
    }

    /// Convert to a `(h, s, l)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.h, self.s, self.l)
    }

    /// Convert from a `(h, s, l)` tuple.
    pub fn from_components((h, s, l): (T, T, T)) -> Self {
        Self::new(h, s, l)
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
        let h = hsl.h;
        let s = hsl.s;
        let l = hsl.l;

        if l == T::one() {
            return Oklab::new(T::one(), T::zero(), T::zero());
        } else if l == T::zero() {
            return Oklab::new(T::zero(), T::zero(), T::zero());
        }

        let h_radians = h.degrees_to_radians();
        let a_ = T::cos(h_radians);
        let b_ = T::sin(h_radians);
        let L = toe_inv(l);

        let cs = Cs::from_normalized(L, a_, b_);
        let C_0 = cs.C_0;
        let C_mid = cs.C_mid;
        let C_max = cs.C_max;

        // Interpolate the three values for C so that:
        // At s=0: dC/ds = C_0, C=0
        // At s=0.8: C=C_mid
        // At s=1.0: C=C_max

        let mid = T::from_f64(0.8);
        let mid_inv = T::from_f64(1.25);

        let C = if s < mid {
            println!("Saturation {s:?} < {mid:?}");
            let t = mid_inv * s;

            let k_1 = mid * C_0;
            let k_2 = T::one() - k_1 / C_mid;

            t * k_1 / (T::one() - k_2 * t)
        } else {
            println!("Saturation {s:?} >= {mid:?}");
            let t = (s - mid) / (T::one() - mid);

            let k_0 = C_mid;
            let k_1 = (T::one() - mid) * C_mid * C_mid * mid_inv * mid_inv / C_0;
            let k_2 = T::one() - (k_1) / (C_max - C_mid);

            k_0 + t * k_1 / (T::one() - k_2 * t)
        };

        Oklab::new(L, C * a_, C * b_)
    }
}

/// # See
/// See [`srgb_to_okhsl`](https://bottosson.github.io/posts/colorpicker/#hsl-2)
impl<T> FromColorUnclamped<Oklab<T>> for Okhsl<T>
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
        + Powi
        + Cbrt
        + Hypot
        + Trigonometry
        + RealAngle
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
    fn from_color_unclamped(lab: Oklab<T>) -> Self {
        let L = lab.l;
        let l = toe(L);

        if lab.a == T::zero() && lab.b == T::zero() {
            // `a` describes how green/red the color is, `b` how blue/yellow the color is
            // both are zero -> the color is totally desaturated.
            return Self::new(T::zero(), T::zero(), l);
        }

        let C = T::hypot(lab.a, lab.b);
        let a_ = lab.a / C;
        let b_ = lab.b / C;

        let h = T::from_f64(180.0) + T::atan2(-lab.b, -lab.a).radians_to_degrees();

        let cs = Cs::from_normalized(L, a_, b_);
        let C_0 = cs.C_0;
        let C_mid = cs.C_mid;
        let C_max = cs.C_max;

        // Inverse of the interpolation in okhsl_to_srgb:

        let mid = T::from_f64(0.8);
        let mid_inv = T::from_f64(1.25);

        let s = if C < C_mid {
            println!("Chroma {C:?} < C_mid {C_mid:?}");
            let k_1 = mid * C_0;
            let k_2 = T::one() - k_1 / C_mid;

            let t = C / (k_1 + k_2 * C);
            t * mid
        } else {
            println!("Chroma {C:?} >= C_mid {C_mid:?}");
            let k_0 = C_mid;
            let k_1 = (T::one() - mid) * (C_mid * mid_inv).powi(2) / C_0;
            let k_2 = T::one() - (k_1) / (C_max - C_mid);

            let t = (C - k_0) / (k_1 + k_2 * (C - k_0));
            mid + (T::one() - mid) * t
        };

        Self::new(h, s, l)
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::FromColorUnclamped;
    use crate::rgb::Rgb;
    use crate::{encoding, LinSrgb, Okhsl, Oklab, Srgb};
    use std::str::FromStr;

    #[test]
    fn test_roundtrip_okhsl_oklab_is_original() {
        let colors = [
            (
                "red",
                Oklab::from_color_unclamped(LinSrgb::new(1.0, 0.0, 0.0)),
            ),
            (
                "green",
                Oklab::from_color_unclamped(LinSrgb::new(0.0, 1.0, 0.0)),
            ),
            (
                "cyan",
                Oklab::from_color_unclamped(LinSrgb::new(0.0, 1.0, 1.0)),
            ),
            (
                "magenta",
                Oklab::from_color_unclamped(LinSrgb::new(1.0, 0.0, 1.0)),
            ),
            (
                "white",
                Oklab::from_color_unclamped(LinSrgb::new(1.0, 1.0, 1.0)),
            ),
            (
                "black",
                Oklab::from_color_unclamped(LinSrgb::new(0.0, 0.0, 0.0)),
            ),
            (
                "grey",
                Oklab::from_color_unclamped(LinSrgb::new(0.5, 0.5, 0.5)),
            ),
            (
                "yellow",
                Oklab::from_color_unclamped(LinSrgb::new(1.0, 1.0, 0.0)),
            ),
            (
                "blue",
                Oklab::from_color_unclamped(LinSrgb::new(0.0, 0.0, 1.0)),
            ),
        ];
        for (name, color) in colors {
            let rgb: Rgb<encoding::Srgb, u8> =
                crate::Srgb::<f32>::from_color_unclamped(color).into_format();
            println!(
                "\n\
            roundtrip of {name} (#{:x} / {:?})\n\
            =================================================",
                rgb, color
            );
            let roundtrip_color = Oklab::from_color_unclamped(Okhsl::from_color_unclamped(color));
            assert!(
                relative_eq!(roundtrip_color, color, epsilon = 1e-3),
                "'{name}' failed. {:?} != {:?}",
                roundtrip_color,
                color
            );
        }
    }

    #[test]
    fn test_blue() {
        let lab = Oklab::new(
            0.45201371519623734_f64,
            -0.03245697990291002,
            -0.3115281336419824,
        );
        let okhsl = Okhsl::<f64>::from_color_unclamped(lab);
        assert!(
            abs_diff_eq!(okhsl.h, 360.0 * 0.7334778365225699, epsilon = 1e-10),
            "{} != {}",
            okhsl.h,
            360.0 * 0.7334778365225699
        );
        assert!(
            abs_diff_eq!(okhsl.s, 0.9999999897262261, epsilon = 1e-10),
            "{} != {}",
            okhsl.s,
            0.9999999897262261
        );
        assert!(
            abs_diff_eq!(okhsl.l, 0.366565335813274, epsilon = 1e-10),
            "{} != {}",
            okhsl.l,
            0.366565335813274
        );
    }

    #[test]
    fn test_srgb_to_okhsl() {
        let red_hex = "#834941";
        let rgb: Srgb<f64> = Srgb::from_str(red_hex).unwrap().into_format();
        let lin_rgb = LinSrgb::<f64>::from_color_unclamped(rgb);
        // FIXME: test data from Ok Color picker, that's hex to lin-rgb slightly differs.
        let lin_rgb = LinSrgb::new(
            0.22696587351009836,
            0.06662593864377289,
            0.052860647023180246,
        );
        let oklab = Oklab::from_color_unclamped(lin_rgb);
        println!(
            "RGB: {rgb:?}\n\
        LinRgb: {lin_rgb:?}\n\
        Oklab: {oklab:?}"
        );
        let okhsl = Okhsl::from_color_unclamped(oklab);

        // test data from Ok Color picker
        // FIXME: results strangely are not very similar. Is there a way to reduce the epsilons?
        assert_relative_eq!(
            okhsl.h,
            360.0 * 0.07992730371382328,
            epsilon = 1e-3,
            max_relative = 1e-3
        );
        assert_relative_eq!(okhsl.s, 0.4629217183454986, epsilon = 1e-4);
        assert_relative_eq!(okhsl.l, 0.3900998146147427, epsilon = 1e-4);
    }

    #[test]
    fn test_okhsl_to_srgb() {
        let okhsl = Okhsl::new(0.0_f32, 0.5, 0.5);
        let oklab = Oklab::from_color_unclamped(okhsl);
        let rgb = Srgb::from_color_unclamped(oklab);
        let rgb8: Rgb<encoding::Srgb, u8> = rgb.into_format();
        let hex_str = format!("{:x}", rgb8);
        assert_eq!(hex_str, "aa5a74");
    }
}
