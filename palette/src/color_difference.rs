use core::ops::{BitAnd, BitOr};

use crate::{
    angle::RealAngle,
    bool_mask::LazySelect,
    convert::IntoColorUnclamped,
    num::{Abs, Arithmetics, Exp, Hypot, One, PartialCmp, Powi, Real, Sqrt, Trigonometry, Zero},
    Lab, Lch,
};

/// A trait for calculating the color difference between two colors.
pub trait ColorDifference {
    /// The type of the calculated color difference.
    type Scalar;

    /// Return the difference or distance between two colors.
    #[must_use]
    fn get_color_difference(self, other: Self) -> Self::Scalar;
}

/// Container of components necessary to calculate CIEDE color difference
pub struct LabColorDiff<T> {
    /// Lab color lightness
    pub l: T,
    /// Lab color a* value
    pub a: T,
    /// Lab color b* value
    pub b: T,
    /// Lab color chroma value
    pub chroma: T,
}

impl<Wp, T> From<Lab<Wp, T>> for LabColorDiff<T>
where
    T: Hypot + Clone,
{
    #[inline]
    fn from(color: Lab<Wp, T>) -> Self {
        // Color difference calculation requires Lab and chroma components. This
        // function handles the conversion into those components which are then
        // passed to `get_ciede_difference()` where calculation is completed.
        LabColorDiff {
            l: color.l,
            a: color.a.clone(),
            b: color.b.clone(),
            chroma: color.a.hypot(color.b),
        }
    }
}

impl<Wp, T> From<Lch<Wp, T>> for LabColorDiff<T>
where
    T: Clone,
    Lch<Wp, T>: IntoColorUnclamped<Lab<Wp, T>>,
{
    #[inline]
    fn from(color: Lch<Wp, T>) -> Self {
        let chroma = color.chroma.clone();
        let Lab { l, a, b, .. } = color.into_color_unclamped();

        LabColorDiff { l, a, b, chroma }
    }
}

/// Calculate the CIEDE2000 color difference for two colors in Lab color space.
/// There is a "just noticeable difference" between two colors when the delta E
/// is roughly greater than 1. Thus, the color difference is more suited for
/// calculating small distances between colors as opposed to large differences.
#[rustfmt::skip]
pub fn get_ciede_difference<T>(this: LabColorDiff<T>, other: LabColorDiff<T>) -> T
where
    T: Real
        + RealAngle
        + One
        + Zero
        + Trigonometry
        + Abs
        + Sqrt
        + Powi
        + Exp
        + Arithmetics
        + PartialCmp
        + Clone,
    T::Mask: LazySelect<T> + BitAnd<Output = T::Mask> + BitOr<Output = T::Mask>
{
    let c_bar = (this.chroma + other.chroma) / T::from_f64(2.0);
    let c_bar_pow_seven = c_bar.powi(7);
    let twenty_five_pow_seven = T::from_f64(6103515625.0);
    let pi_over_180 = T::from_f64(core::f64::consts::PI / 180.0);

    let g = T::from_f64(0.5)
        * (T::one() - (c_bar_pow_seven.clone() / (c_bar_pow_seven + &twenty_five_pow_seven)).sqrt());
    let a_one_prime = this.a * (T::one() + &g);
    let a_two_prime = other.a * (T::one() + g);
    let c_one_prime = (a_one_prime.clone() * &a_one_prime + this.b.clone() * &this.b).sqrt();
    let c_two_prime = (a_two_prime.clone() * &a_two_prime + other.b.clone() * &other.b).sqrt();

    let calc_h_prime = |b: T, a_prime: T| -> T {
        lazy_select! {
            if b.eq(&T::zero()) & a_prime.eq(&T::zero()) => T::zero(),
            else => {
                let result = T::radians_to_degrees(b.atan2(a_prime));
                lazy_select! {
                    if result.lt(&T::zero()) => result.clone() + T::from_f64(360.0),
                    else => result.clone(),
                }
            },
        }
    };
    let h_one_prime = calc_h_prime(this.b, a_one_prime);
    let h_two_prime = calc_h_prime(other.b, a_two_prime);

    let h_prime_diff = h_two_prime.clone() - &h_one_prime;
    let h_prime_abs_diff = h_prime_diff.clone().abs();

    let delta_h_prime: T = lazy_select! {
        if c_one_prime.eq(&T::zero()) | c_two_prime.eq(&T::zero()) => T::zero(),
        if h_prime_abs_diff.lt_eq(&T::from_f64(180.0)) => h_prime_diff.clone(),
        if h_two_prime.lt_eq(&h_one_prime) => h_prime_diff.clone() + T::from_f64(360.0),
        else => h_prime_diff.clone() - T::from_f64(360.0),
    };

    let delta_big_h_prime = T::from_f64(2.0)
        * (c_one_prime.clone() * &c_two_prime).sqrt()
        * (delta_h_prime / T::from_f64(2.0) * &pi_over_180).sin();
    let h_prime_sum = h_one_prime + h_two_prime;
    let h_bar_prime = lazy_select! {
        if c_one_prime.eq(&T::zero()) | c_two_prime.eq(&T::zero()) => h_prime_sum.clone(),
        if h_prime_abs_diff.gt(&T::from_f64(180.0)) => {
            (h_prime_sum.clone() + T::from_f64(360.0)) / T::from_f64(2.0)
        },
        else => h_prime_sum.clone() / T::from_f64(2.0),
    };

    let l_bar = (this.l.clone() + &other.l) / T::from_f64(2.0);
    let c_bar_prime = (c_one_prime.clone() + &c_two_prime) / T::from_f64(2.0);

    let t: T = T::one()
        - T::from_f64(0.17) * ((h_bar_prime.clone() - T::from_f64(30.0)) * &pi_over_180).cos()
        + T::from_f64(0.24) * ((h_bar_prime.clone() * T::from_f64(2.0)) * &pi_over_180).cos()
        + T::from_f64(0.32) * ((h_bar_prime.clone() * T::from_f64(3.0) + T::from_f64(6.0)) * &pi_over_180).cos()
        - T::from_f64(0.20) * ((h_bar_prime.clone() * T::from_f64(4.0) - T::from_f64(63.0)) * &pi_over_180).cos();
    let s_l = T::one()
        + ((T::from_f64(0.015) * (l_bar.clone() - T::from_f64(50.0)) * (l_bar.clone() - T::from_f64(50.0)))
            / ((l_bar.clone() - T::from_f64(50.0)) * (l_bar - T::from_f64(50.0)) + T::from_f64(20.0)).sqrt());
    let s_c = T::one() + T::from_f64(0.045) * &c_bar_prime;
    let s_h = T::one() + T::from_f64(0.015) * &c_bar_prime * t;

    let delta_theta = T::from_f64(30.0)
        * (-(((h_bar_prime.clone() - T::from_f64(275.0)) / T::from_f64(25.0))
            * ((h_bar_prime - T::from_f64(275.0)) / T::from_f64(25.0))))
        .exp();
    let c_bar_prime_pow_seven = c_bar_prime.powi(7);
    let r_c: T = T::from_f64(2.0)
        * (c_bar_prime_pow_seven.clone() / (c_bar_prime_pow_seven + twenty_five_pow_seven)).sqrt();
    let r_t = -r_c * (T::from_f64(2.0) * delta_theta * pi_over_180).sin();

    let k_l = T::one();
    let k_c = T::one();
    let k_h = T::one();
    let delta_l_prime = other.l - this.l;
    let delta_c_prime = c_two_prime - c_one_prime;

    ((delta_l_prime.clone() / (k_l.clone() * &s_l)) * (delta_l_prime / (k_l * s_l))
        + (delta_c_prime.clone() / (k_c.clone() * &s_c)) * (delta_c_prime.clone() / (k_c.clone() * &s_c))
        + (delta_big_h_prime.clone() / (k_h.clone() * &s_h)) * (delta_big_h_prime.clone() / (k_h.clone() * &s_h))
        + (r_t * delta_c_prime * delta_big_h_prime) / (k_c * s_c * k_h * s_h))
        .sqrt()
}
