use crate::component::FloatComponent;
use crate::from_f64;

/// A trait for calculating the color difference between two colors.
pub trait ColorDifference {
    /// The type of the calculated color difference
    type Scalar: FloatComponent;

    /// Return the difference or distance between two colors
    fn get_color_difference(&self, other: &Self) -> Self::Scalar;
}

/// Container of components necessary to calculate CIEDE color difference
pub struct LabColorDiff<T: FloatComponent> {
    /// Lab color lightness
    pub l: T,
    /// Lab color a* value
    pub a: T,
    /// Lab color b* value
    pub b: T,
    /// Lab color chroma value
    pub chroma: T,
}

/// Calculate the CIEDE2000 color difference for two colors in Lab color space.
/// There is a "just noticeable difference" between two colors when the delta E
/// is roughly greater than 1. Thus, the color difference is more suited for
/// calculating small distances between colors as opposed to large differences.
#[rustfmt::skip]
pub fn get_ciede_difference<T: FloatComponent>(this: &LabColorDiff<T>, other: &LabColorDiff<T>) -> T {
    let c_bar = (this.chroma + other.chroma) / from_f64(2.0);
    let c_bar_pow_seven = c_bar * c_bar * c_bar * c_bar * c_bar * c_bar * c_bar;
    let twenty_five_pow_seven = from_f64(6103515625.0);
    let pi_over_180 = from_f64::<T>(core::f64::consts::PI / 180.0);

    let g = from_f64::<T>(0.5)
        * (from_f64::<T>(1.0)
            - (c_bar_pow_seven / (c_bar_pow_seven + twenty_five_pow_seven)).sqrt());
    let a_one_prime = this.a * (from_f64::<T>(1.0) + g);
    let a_two_prime = other.a * (from_f64::<T>(1.0) + g);
    let c_one_prime = (a_one_prime * a_one_prime + this.b * this.b).sqrt();
    let c_two_prime = (a_two_prime * a_two_prime + other.b * other.b).sqrt();

    let calc_h_prime = |b: T, a_prime: T| -> T {
        if b == T::zero() && a_prime == T::zero() {
            from_f64(0.0)
        } else {
            let result = b.atan2(a_prime).to_degrees();
            if result < T::zero() {
                result + from_f64(360.0)
            } else {
                result
            }
        }
    };
    let h_one_prime = calc_h_prime(this.b, a_one_prime);
    let h_two_prime = calc_h_prime(other.b, a_two_prime);

    let h_prime_difference = (h_one_prime - h_two_prime).abs();

    let delta_h_prime: T = if c_one_prime == T::zero() || c_two_prime == T::zero() {
        from_f64(0.0)
    } else if h_prime_difference <= from_f64(180.0) {
        h_two_prime - h_one_prime
    } else if h_two_prime <= h_one_prime {
        h_two_prime - h_one_prime + from_f64(360.0)
    } else {
        h_two_prime - h_one_prime - from_f64(360.0)
    };

    let delta_big_h_prime = from_f64::<T>(2.0)
        * (c_one_prime * c_two_prime).sqrt()
        * (delta_h_prime / from_f64(2.0) * pi_over_180).sin();
    let h_bar_prime = if c_one_prime == T::zero() || c_two_prime == T::zero() {
        h_one_prime + h_two_prime
    } else if h_prime_difference > from_f64(180.0) {
        (h_one_prime + h_two_prime + from_f64(360.0)) / from_f64(2.0)
    } else {
        (h_one_prime + h_two_prime) / from_f64(2.0)
    };

    let l_bar = (this.l + other.l) / from_f64(2.0);
    let c_bar_prime = (c_one_prime + c_two_prime) / from_f64(2.0);

    let t: T = from_f64::<T>(1.0)
        - from_f64::<T>(0.17) * ((h_bar_prime - from_f64(30.0)) * pi_over_180).cos()
        + from_f64::<T>(0.24) * ((h_bar_prime * from_f64(2.0)) * pi_over_180).cos()
        + from_f64::<T>(0.32) * ((h_bar_prime * from_f64(3.0) + from_f64(6.0)) * pi_over_180).cos()
        - from_f64::<T>(0.20) * ((h_bar_prime * from_f64(4.0) - from_f64(63.0)) * pi_over_180).cos();
    let s_l = from_f64::<T>(1.0)
        + ((from_f64::<T>(0.015) * (l_bar - from_f64(50.0)) * (l_bar - from_f64(50.0)))
            / ((l_bar - from_f64(50.0)) * (l_bar - from_f64(50.0)) + from_f64(20.0)).sqrt());
    let s_c = from_f64::<T>(1.0) + from_f64::<T>(0.045) * c_bar_prime;
    let s_h = from_f64::<T>(1.0) + from_f64::<T>(0.015) * c_bar_prime * t;

    let delta_theta = from_f64::<T>(30.0)
        * (-(((h_bar_prime - from_f64(275.0)) / from_f64(25.0))
            * ((h_bar_prime - from_f64(275.0)) / from_f64(25.0))))
        .exp();
    let c_bar_prime_pow_seven = c_bar_prime.powi(7);
    let r_c: T = from_f64::<T>(2.0)
        * (c_bar_prime_pow_seven / (c_bar_prime_pow_seven + twenty_five_pow_seven)).sqrt();
    let r_t = -r_c * (from_f64::<T>(2.0) * delta_theta * pi_over_180).sin();

    let one = from_f64::<T>(1.0);
    let k_l = one;
    let k_c = one;
    let k_h = one;
    let delta_l_prime = other.l - this.l;
    let delta_c_prime = c_two_prime - c_one_prime;

    ((delta_l_prime / (k_l * s_l)) * (delta_l_prime / (k_l * s_l))
        + (delta_c_prime / (k_c * s_c)) * (delta_c_prime / (k_c * s_c))
        + (delta_big_h_prime / (k_h * s_h)) * (delta_big_h_prime / (k_h * s_h))
        + (r_t * delta_c_prime * delta_big_h_prime) / (k_c * s_c * k_h * s_h))
        .sqrt()
}
