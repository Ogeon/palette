use core::fmt::Debug;

pub use alpha::Okhsva;
#[cfg(feature = "random")]
pub use random::UniformOkhsv;

use crate::angle::FromAngle;
use crate::convert::IntoColorUnclamped;
use crate::num::{
    Arithmetics, Cbrt, FromScalar, Hypot, IsValidDivisor, MinMax, One, Powi, Real, Recip, Sqrt,
    Trigonometry, Zero,
};
use crate::ok_utils::{LC, ST};
use crate::stimulus::{FromStimulus, Stimulus};
use crate::white_point::D65;
use crate::{
    angle::RealAngle, convert::FromColorUnclamped, ok_utils, HasBoolMask, LinSrgb, Okhwb, Oklab,
    OklabHue,
};

mod alpha;
mod properties;
#[cfg(feature = "random")]
mod random;

/// A Hue/Saturation/Value representation of [`Oklab`] in the `sRGB` color space.
///
/// Allows
/// * changing lightness/chroma/saturation while keeping perceived Hue constant
/// (like HSV promises but delivers only partially)  
/// * finding the strongest color (maximum chroma) at s == 1 (like HSV)  
#[derive(Debug, Copy, Clone, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklab, Okhwb)
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
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: OklabHue<T>,

    /// The saturation (freedom of whitishness) of the color.
    ///
    /// * `0.0` corresponds to pure mixture of black and white without any color.
    /// The black to white relation depends on v.
    /// * `1.0` to a fully saturated color without any white.
    ///
    /// For v == 0 the saturation is irrelevant.
    pub saturation: T,

    /// The monochromatic brightness of the color.
    /// * `0.0` corresponds to pure black
    /// * `1.0` corresponds to a maximally bright colour -- be it very colorful or very  white
    ///
    /// `Okhsl`'s `lightness` component goes from black to white.
    /// `Okhsv`'s `value` component goes from black to non-black -- a maximally bright color..
    pub value: T,
}

impl<T, H: Into<OklabHue<T>>> From<(H, T, T)> for Okhsv<T>
where
    T: Zero + MinMax,
{
    fn from(components: (H, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<T> HasBoolMask for Okhsv<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<T> Default for Okhsv<T>
where
    T: Real + Stimulus,
    OklabHue<T>: Default,
{
    fn default() -> Okhsv<T> {
        Okhsv::new(
            OklabHue::default(),
            Self::min_saturation(),
            Self::min_value(),
        )
    }
}

impl<T> Okhsv<T>
where
    T: Real + Stimulus,
{
    /// Return the `saturation` value minimum.
    pub fn min_saturation() -> T {
        T::zero()
    }

    /// Return the `saturation` value maximum.
    pub fn max_saturation() -> T {
        T::max_intensity()
    }

    /// Return the `value` value minimum.
    pub fn min_value() -> T {
        T::zero()
    }

    /// Return the `value` value maximum.
    pub fn max_value() -> T {
        T::max_intensity()
    }
}

impl<T> Okhsv<T> {
    /// Create an `Okhsv` color.
    pub fn new<H: Into<OklabHue<T>>>(hue: H, saturation: T, value: T) -> Self {
        Self {
            hue: hue.into(),
            saturation,
            value,
        }
    }

    /// Create an `Okhsv` color. This is the same as `Okhsv::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub fn new_const(hue: OklabHue<T>, saturation: T, value: T) -> Self {
        Self {
            hue,
            saturation,
            value,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U>(self) -> Okhsv<U>
    where
        U: FromStimulus<T> + FromAngle<T>,
    {
        Okhsv {
            hue: self.hue.into_format(),
            saturation: U::from_stimulus(self.saturation),
            value: U::from_stimulus(self.value),
        }
    }

    /// Convert to a `(h, s, v)` tuple.
    pub fn into_components(self) -> (OklabHue<T>, T, T) {
        (self.hue, self.saturation, self.value)
    }

    /// Convert from a `(h, s, v)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((hue, saturation, value): (H, T, T)) -> Self {
        Self::new(hue, saturation, value)
    }
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
        + Hypot
        + One
        + FromScalar
        + RealAngle,
    T::Scalar: Real
        + Zero
        + One
        + Recip
        + Hypot
        + IsValidDivisor<Mask = bool>
        + Arithmetics
        + Clone
        + FromScalar<Scalar = T::Scalar>,
{
    fn from_color_unclamped(lab: Oklab<T>) -> Self {
        if lab.l == T::zero() {
            // the color is pure black
            return Self::new(T::zero(), T::zero(), T::zero());
        }

        if let Some(hue) = lab.try_hue() {
            let (chroma, normalized_ab) = lab.chroma_and_normalized_ab();
            let (a_, b_) =
                normalized_ab.expect("There is a hue, thus there also are normalized a and b");

            // For each hue the sRGB gamut can be drawn on a 2-dimensional space.
            // Let L_r, the lightness in relation to the possible luminance of sRGB, be spread
            // along the y-axis (bottom is black, top is bright) and Chroma along the x-axis
            // (left is desaturated, right is colorful). The gamut then takes a triangular shape,
            // with a concave top side and a cusp to the right.
            // To use saturation and brightness values, the gamut must be mapped to a square.
            // The lower point of the triangle is expanded to the lower side of the square.
            // The left side remains unchanged and the cusp of the triangle moves to the upper right.
            let cusp = LC::find_cusp(a_, b_);
            let st_max = ST::<T>::from(cusp);

            let s_0 = T::from_f64(0.5);
            let k = T::one() - s_0 / st_max.s;

            // first we find L_v, C_v, L_vt and C_vt
            let t = st_max.t / (chroma + lab.l * st_max.t);
            let l_v = t * lab.l;
            let c_v = t * chroma;

            let l_vt = ok_utils::toe_inv(l_v);
            let c_vt = c_v * l_vt / l_v;

            // we can then use these to invert the step that compensates for the toe and the curved top part of the triangle:
            let rgb_scale: LinSrgb<T> =
                Oklab::new(l_vt, a_ * c_vt, b_ * c_vt).into_color_unclamped();
            let lightness_scale_factor = T::cbrt(
                T::one()
                    / T::max(
                        T::max(rgb_scale.red, rgb_scale.green),
                        T::max(rgb_scale.blue, T::zero()),
                    ),
            );

            //chroma = chroma / lightness_scale_factor;

            // use L_r instead of L and also scale C by L_r/L
            let l_r = ok_utils::toe(lab.l / lightness_scale_factor);
            //chroma = chroma * l_r / (lab.l / lightness_scale_factor);

            // we can now compute v and s:
            let v = l_r / l_v;
            let s = (s_0 + st_max.t) * c_v / ((st_max.t * s_0) + st_max.t * k * c_v);

            Self::new(hue, s, v)
        } else {
            // the color is totally desaturated.
            let v = ok_utils::toe(lab.l);
            Self::new(T::zero(), T::zero(), v)
        }
    }
}
impl<T> FromColorUnclamped<Okhwb<T>> for Okhsv<T>
where
    T: Real
        + PartialOrd
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
        + RealAngle,
    T::Scalar: Real
        + Zero
        + One
        + Recip
        + Hypot
        + IsValidDivisor<Mask = bool>
        + Arithmetics
        + Clone
        + FromScalar<Scalar = T::Scalar>,
{
    fn from_color_unclamped(hwb: Okhwb<T>) -> Self {
        if hwb.blackness == T::one() {
            return Self::new(hwb.hue, T::zero(), T::zero());
        }
        Self::new(
            hwb.hue,
            T::one() - hwb.whiteness / (T::one() - hwb.blackness),
            T::one() - hwb.blackness,
        )
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use crate::convert::FromColorUnclamped;
    use crate::rgb::Rgb;
    use crate::visual::VisuallyEqual;
    use crate::{encoding, Clamp, IsWithinBounds, LinSrgb, Okhsv, Oklab, OklabHue, Srgb};

    #[test]
    fn test_roundtrip_okhsv_oklab_is_original() {
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

        // unlike in okhwb we are using f64 here, which actually works.
        // So we can afford a small tolerance
        const EPSILON: f64 = 1e-10;

        for (name, color) in colors {
            let rgb: Rgb<encoding::Srgb, u8> =
                crate::Srgb::<f64>::from_color_unclamped(color).into_format();
            println!(
                "\n\
            roundtrip of {} (#{:x} / {:?})\n\
            =================================================",
                name, rgb, color
            );

            let okhsv = Okhsv::from_color_unclamped(color);
            println!("Okhsv: {:?}", okhsv);
            let roundtrip_color = Oklab::from_color_unclamped(okhsv);
            assert!(
                Oklab::visually_eq(roundtrip_color, color, EPSILON),
                "'{}' failed.\n{:?}\n!=\n{:?}",
                name,
                roundtrip_color,
                color
            );
        }
    }

    /// Compares results to results for a run of
    /// https://github.com/bottosson/bottosson.github.io/blob/3d3f17644d7f346e1ce1ca08eb8b01782eea97af/misc/ok_color.h
    /// Not to the ideal values, which should be
    /// hue: as is
    /// saturation: 1.0
    /// value: 1.0
    #[test]
    fn blue() {
        let color = Oklab::<f64>::from_color_unclamped(LinSrgb::new(0.0, 0.0, 1.0));
        let okhsv = Okhsv::from_color_unclamped(color);
        println!("Okhsv f64: {:?}\n", okhsv);
        // HSV values of the reference implementation (in C)
        // 1 iteration : 264.0520206380550121, 0.9999910912349018, 0.9999999646150918
        // 2 iterations: 264.0520206380550121, 0.9999999869716002, 0.9999999646150844
        // 3 iterations: 264.0520206380550121, 0.9999999869716024, 0.9999999646150842

        // compare to the reference implementation values
        assert_abs_diff_eq!(
            okhsv.hue,
            OklabHue::new(264.0520206380550121),
            epsilon = 1e-12
        );
        assert_abs_diff_eq!(okhsv.saturation, 0.9999910912349018, epsilon = 1e-12);
        assert_abs_diff_eq!(okhsv.value, 0.9999999646150918, epsilon = 1e-12);

        let color = Oklab::<f32>::from_color_unclamped(LinSrgb::new(0.0, 0.0, 1.0));
        let okhsv = Okhsv::from_color_unclamped(color);
        println!("Okhsv f32: {:?}", okhsv);
        assert_abs_diff_eq!(
            okhsv.hue,
            OklabHue::new(264.0520324707031250),
            epsilon = 1e-6
        );

        // compare to the ideal values
        // FIXME: The algorithm is not robust wrt. floating point errors.
        //  See `ok_utils:LC::max_saturation`.
        //  .
        //  HSV values of the reference implementation (in C) on an unlucky machine
        //  (printed with double-precision despite using float-precision for compuation)
        //  1 iteration : 264.0520324707031250, 0.9239708185195923, 1.0000002384185791
        //  2 iterations: 264.0520324707031250, -1.0219360589981079, 0.9999997615814209
        //  3 iterations: 264.0520324707031250, 0.1297522187232971, 0.9999999403953552
        //  .
        //  With lucky machines (like the integration machine) the okhsv.saturation
        //  and okhsv.value already in the first iteration are 0.9999911
        //  .
        //  If a solution is found, reduce epsilon to 1e-6.
        assert_abs_diff_eq!(okhsv.saturation, 1.0, epsilon = 1e-1);
        assert_abs_diff_eq!(okhsv.value, 1.0, epsilon = 1e-1);
    }

    #[test]
    fn test_srgb_to_okhsv() {
        let red_hex = "#ff0004";
        let rgb: Srgb = Rgb::<encoding::Srgb, _>::from_str(red_hex)
            .unwrap()
            .into_format();
        let oklab = Oklab::from_color_unclamped(rgb);
        let okhsv = Okhsv::from_color_unclamped(oklab);
        assert_relative_eq!(okhsv.saturation, 1.0, epsilon = 1e-3);
        assert_relative_eq!(okhsv.value, 1.0, epsilon = 1e-3);
        assert_relative_eq!(
            okhsv.hue.into_raw_degrees(),
            29.0,
            epsilon = 1e-3,
            max_relative = 1e-3
        );
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

    #[test]
    fn black_eq_different_black() {
        assert!(Okhsv::visually_eq(
            Okhsv::from_color_unclamped(Oklab::new(0.0, 1.0, 0.0)),
            Okhsv::from_color_unclamped(Oklab::new(0.0, 0.0, 1.0)),
            1e-12
        ));
    }

    #[test]
    fn white_eq_different_white() {
        assert!(Okhsv::visually_eq(
            Okhsv::new(240.0, 0.0, 1.0),
            Okhsv::new(24.0, 0.0, 1.0),
            1e-12
        ));
    }

    #[test]
    fn white_ne_grey_or_black() {
        assert!(!Okhsv::visually_eq(
            Okhsv::new(0.0, 0.0, 0.0),
            Okhsv::new(0.0, 0.0, 1.0),
            1e-12
        ));
        assert!(!Okhsv::visually_eq(
            Okhsv::new(0.0, 0.0, 0.3),
            Okhsv::new(0.0, 0.0, 1.0),
            1e-12
        ));
    }

    #[test]
    fn color_neq_different_color() {
        assert!(!Okhsv::visually_eq(
            Okhsv::new(10.0, 0.01, 0.5),
            Okhsv::new(11.0, 0.01, 0.5),
            1e-12
        ));
        assert!(!Okhsv::visually_eq(
            Okhsv::new(10.0, 0.01, 0.5),
            Okhsv::new(10.0, 0.02, 0.5),
            1e-12
        ));
        assert!(!Okhsv::visually_eq(
            Okhsv::new(10.0, 0.01, 0.5),
            Okhsv::new(10.0, 0.01, 0.6),
            1e-12
        ));
    }

    #[test]
    fn grey_vs_grey() {
        // greys of different lightness are not equal
        assert!(!Okhsv::visually_eq(
            Okhsv::new(0.0, 0.0, 0.3),
            Okhsv::new(0.0, 0.0, 0.4),
            1e-12
        ));
        // greys of same lightness but different hue are equal
        assert!(Okhsv::visually_eq(
            Okhsv::new(0.0, 0.0, 0.3),
            Okhsv::new(12.0, 0.0, 0.3),
            1e-12
        ));
    }

    #[test]
    fn srgb_gamut_containment() {
        {
            println!("sRGB Red");
            let oklab = Oklab::from_color_unclamped(LinSrgb::new(1.0, 0.0, 0.0));
            println!("{:?}", oklab);
            let okhsv: Okhsv<f64> = Okhsv::from_color_unclamped(oklab);
            println!("{:?}", okhsv);
            assert!(okhsv.is_within_bounds());
        }

        {
            println!("Double sRGB Red");
            let oklab = Oklab::from_color_unclamped(LinSrgb::new(2.0, 0.0, 0.0));
            println!("{:?}", oklab);
            let okhsv: Okhsv<f64> = Okhsv::from_color_unclamped(oklab);
            println!("{:?}", okhsv);
            assert!(!okhsv.is_within_bounds());
            let clamped_okhsv = okhsv.clamp();
            println!("Clamped: {:?}", clamped_okhsv);
            assert!(clamped_okhsv.is_within_bounds());
            let linsrgb = LinSrgb::from_color_unclamped(clamped_okhsv);
            println!("Clamped as unclamped Linear sRGB: {:?}", linsrgb);
        }

        {
            println!("P3 Yellow");
            // display P3 yellow according to https://colorjs.io/apps/convert/?color=color(display-p3%201%201%200)&precision=17
            let oklab = Oklab::from_color_unclamped(LinSrgb::new(1.0, 1.0, -0.098273600140966));
            println!("{:?}", oklab);
            let okhsv: Okhsv<f64> = Okhsv::from_color_unclamped(oklab);
            println!("{:?}", okhsv);
            assert!(!okhsv.is_within_bounds());
            let clamped_okhsv = okhsv.clamp();
            println!("Clamped: {:?}", clamped_okhsv);
            assert!(clamped_okhsv.is_within_bounds());
            let linsrgb = LinSrgb::from_color_unclamped(clamped_okhsv);
            println!(
                "Clamped as unclamped Linear sRGB: {:?}\n\
                May be different, but should be visually indistinguishable from\n\
                color.js' gamut mapping red: 1 green: 0.9876530763223166 blue: 0",
                linsrgb
            );
        }
    }
}