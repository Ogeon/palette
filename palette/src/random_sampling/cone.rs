use core::marker::PhantomData;

use crate::{
    bool_mask::LazySelect,
    hues::{LuvHue, RgbHue},
    num::{Arithmetics, Cbrt, One, PartialCmp, Powi, Real, Sqrt},
    Hsl, Hsluv, Hsv, Okhsl, Okhsv, OklabHue,
};

// Based on https://stackoverflow.com/q/4778147 and https://math.stackexchange.com/q/18686,
// picking A = (0, 0), B = (0, 1), C = (1, 1) gives us:
//
// (  sqrt(r1) * r2  ,  sqrt(r1) * (1 - r2) + sqrt(r1) * r2  ) =
// (  sqrt(r1) * r2  ,  sqrt(r1) - sqrt(r1) * r2 + sqrt(r1) * r2  ) =
// (  sqrt(r1) * r2  ,  sqrt(r1)  )
//
// `sqrt(r1)` gives us the scale of the triangle, `r2` the radius.
// Substituting, we get `x = scale * radius, y = scale` and thus for cone
// sampling: `scale = powf(r1, 1.0/3.0)` and `radius = sqrt(r2)`.

#[inline]
pub fn sample_hsv<S, T>(hue: RgbHue<T>, r1: T, r2: T) -> Hsv<S, T>
where
    T: Cbrt + Sqrt,
{
    let (value, saturation) = (r1.cbrt(), r2.sqrt());

    Hsv {
        hue,
        saturation,
        value,
        standard: PhantomData,
    }
}

#[inline]
pub fn sample_okhsv<T>(hue: OklabHue<T>, r1: T, r2: T) -> Okhsv<T>
where
    T: Cbrt + Sqrt,
{
    let (value, saturation) = (r1.cbrt(), r2.sqrt());

    Okhsv {
        hue,
        saturation,
        value,
    }
}

#[inline]
pub fn sample_hsl<S, T>(hue: RgbHue<T>, r1: T, r2: T) -> Hsl<S, T>
where
    T: Real + One + Cbrt + Sqrt + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T> + Clone,
{
    Hsl {
        hue,
        saturation: r2.sqrt(),
        lightness: sample_bicone_height(r1),
        standard: PhantomData,
    }
}

#[inline]
pub fn sample_okhsl<T>(hue: OklabHue<T>, r1: T, r2: T) -> Okhsl<T>
where
    T: Real + One + Cbrt + Sqrt + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T> + Clone,
{
    Okhsl {
        hue,
        saturation: r2.sqrt(),
        lightness: sample_bicone_height(r1),
    }
}

#[inline]
pub fn sample_hsluv<Wp, T>(hue: LuvHue<T>, r1: T, r2: T) -> Hsluv<Wp, T>
where
    T: Real + One + Cbrt + Sqrt + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T> + Clone,
{
    Hsluv {
        hue,
        saturation: r2.sqrt() * T::from_f64(100.0),
        l: sample_bicone_height(r1) * T::from_f64(100.0),
        white_point: PhantomData,
    }
}

#[inline]
fn sample_bicone_height<T>(r1: T) -> T
where
    T: Real + One + Cbrt + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T> + Clone,
{
    let mask = r1.lt_eq(&T::from_f64(0.5));

    // Scale it up to [0, 1] or [0, 1)
    let r1 = lazy_select! {
        if mask.clone() => r1.clone(),
        else => T::one() - &r1,
    } * T::from_f64(2.0);

    let height = r1.cbrt();

    // Turn the height back to [0, 0.5] or (0.5, 1.0]
    let height = height * T::from_f64(0.5);
    lazy_select! {
        if mask => height.clone(),
        else => T::one() - &height,
    }
}

#[inline]
pub fn invert_hsl_sample<T>(saturation: T, lightness: T) -> (T, T)
where
    T: Real + Powi + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    let r1 = invert_bicone_height_sample(lightness);

    // saturation is first multiplied, then divided by h before squaring.
    // h can be completely eliminated, leaving only the saturation.
    let r2 = saturation.powi(2);

    (r1, r2)
}

#[inline]
pub fn invert_hsluv_sample<T>(saturation: T, lightness: T) -> (T, T)
where
    T: Real + Powi + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    let r1 = invert_bicone_height_sample(lightness / T::from_f64(100.0));

    // saturation is first multiplied, then divided by h before squaring.
    // h can be completely eliminated, leaving only the saturation.
    let r2 = (saturation / T::from_f64(100.0)).powi(2);

    (r1, r2)
}

fn invert_bicone_height_sample<T>(height: T) -> T
where
    T: Real + Powi + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    lazy_select! {
        if height.lt_eq(&T::from_f64(0.5)) => {
            // ((x * 2)^3) / 2 = x^3 * 4.
            // height is multiplied by 2 to scale it up to [0, 1], becoming h.
            // h is cubed to make it r1. r1 is divided by 2 to take it back to [0, 0.5].
            height.clone().powi(3) * T::from_f64(4.0)
        },
        else => {
            let x = height.clone() - T::from_f64(1.0);
            x.powi(3) * T::from_f64(4.0) + T::from_f64(1.0)
        },
    }
}

#[cfg(test)]
mod test {
    use super::{invert_hsl_sample, invert_hsluv_sample, sample_hsl, sample_hsluv, sample_hsv};
    use crate::hues::{LuvHue, RgbHue};
    use crate::white_point::D65;
    use crate::{Hsl, Hsluv, Hsv};

    #[cfg(feature = "random")]
    #[test]
    fn sample_max_min() {
        let a = sample_hsv(RgbHue::from(0.0), 0.0, 0.0);
        let b = sample_hsv(RgbHue::from(360.0), 1.0, 1.0);
        assert_relative_eq!(Hsv::new_srgb(0.0, 0.0, 0.0), a);
        assert_relative_eq!(Hsv::new_srgb(360.0, 1.0, 1.0), b);
        let a = sample_hsl(RgbHue::from(0.0), 0.0, 0.0);
        let b = sample_hsl(RgbHue::from(360.0), 1.0, 1.0);
        assert_relative_eq!(Hsl::new_srgb(0.0, 0.0, 0.0), a);
        assert_relative_eq!(Hsl::new_srgb(360.0, 1.0, 1.0), b);
        let a = sample_hsluv(LuvHue::from(0.0), 0.0, 0.0);
        let b = sample_hsluv(LuvHue::from(360.0), 1.0, 1.0);
        assert_relative_eq!(Hsluv::<D65>::new(0.0, 0.0, 0.0), a);
        assert_relative_eq!(Hsluv::<D65>::new(360.0, 100.0, 100.0), b);
    }

    #[cfg(feature = "random")]
    #[allow(clippy::excessive_precision)]
    #[test]
    fn hsl_sampling() {
        // Sanity check that sampling and inverting from sample are equivalent
        macro_rules! test_hsl {
            ( $x:expr, $y:expr ) => {{
                let hsl: Hsl = sample_hsl(RgbHue::from(0.0), $x, $y);
                let a = invert_hsl_sample(hsl.saturation, hsl.lightness);
                assert_relative_eq!(a.0, $x);
                assert_relative_eq!(a.1, $y);
            }};
        }

        test_hsl!(0.8464721407, 0.8271899200);
        test_hsl!(0.8797234442, 0.4924621591);
        test_hsl!(0.9179406120, 0.8771350605);
        test_hsl!(0.5458023108, 0.1154283005);
        test_hsl!(0.2691241774, 0.7881780600);
        test_hsl!(0.2085030453, 0.9975406626);
        test_hsl!(0.8483632811, 0.4955013942);
        test_hsl!(0.0857919040, 0.0652214785);
        test_hsl!(0.7152662838, 0.2788421565);
        test_hsl!(0.2973598808, 0.5585230243);
        test_hsl!(0.0936619602, 0.7289450731);
        test_hsl!(0.4364395449, 0.9362269009);
        test_hsl!(0.9802381158, 0.9742974964);
        test_hsl!(0.1666129293, 0.4396910574);
        test_hsl!(0.6190216210, 0.7175675180);
    }

    #[cfg(feature = "random")]
    #[allow(clippy::excessive_precision)]
    #[test]
    fn hsluv_sampling() {
        // Sanity check that sampling and inverting from sample are equivalent
        macro_rules! test_hsluv {
            ( $x:expr, $y:expr ) => {{
                let hsluv: Hsluv = sample_hsluv(LuvHue::from(0.0), $x, $y);
                let a = invert_hsluv_sample(hsluv.saturation, hsluv.l);
                assert_relative_eq!(a.0, $x);
                assert_relative_eq!(a.1, $y);
            }};
        }

        test_hsluv!(0.8464721407, 0.8271899200);
        test_hsluv!(0.8797234442, 0.4924621591);
        test_hsluv!(0.9179406120, 0.8771350605);
        test_hsluv!(0.5458023108, 0.1154283005);
        test_hsluv!(0.2691241774, 0.7881780600);
        test_hsluv!(0.2085030453, 0.9975406626);
        test_hsluv!(0.8483632811, 0.4955013942);
        test_hsluv!(0.0857919040, 0.0652214785);
        test_hsluv!(0.7152662838, 0.2788421565);
        test_hsluv!(0.2973598808, 0.5585230243);
        test_hsluv!(0.0936619602, 0.7289450731);
        test_hsluv!(0.4364395449, 0.9362269009);
        test_hsluv!(0.9802381158, 0.9742974964);
        test_hsluv!(0.1666129293, 0.4396910574);
        test_hsluv!(0.6190216210, 0.7175675180);
    }
}
