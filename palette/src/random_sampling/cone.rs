use core::marker::PhantomData;

use crate::float::Float;
use crate::hues::{LuvHue, RgbHue};
use crate::rgb::RgbStandard;
use crate::white_point::WhitePoint;
use crate::{from_f64, FloatComponent, Hsl, Hsluv, Hsv};

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

pub fn sample_hsv<S, T>(hue: RgbHue<T>, r1: T, r2: T) -> Hsv<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    let (value, saturation) = (Float::cbrt(r1), Float::sqrt(r2));

    Hsv {
        hue,
        saturation,
        value,
        standard: PhantomData,
    }
}

pub fn sample_hsl<S, T>(hue: RgbHue<T>, r1: T, r2: T) -> Hsl<S, T>
where
    T: FloatComponent,
    S: RgbStandard,
{
    let (saturation, lightness) = if r1 <= from_f64::<T>(0.5) {
        // Scale it up to [0, 1]
        let r1 = r1 * from_f64::<T>(2.0);
        let h = Float::cbrt(r1);
        let r = Float::sqrt(r2);
        // Scale the lightness back to [0, 0.5]
        (r, h * from_f64::<T>(0.5))
    } else {
        // Scale and shift it to [0, 1).
        let r1 = (from_f64::<T>(1.0) - r1) * from_f64::<T>(2.0);
        let h = Float::cbrt(r1);
        let r = Float::sqrt(r2);
        // Turn the cone upside-down and scale the lightness back to (0.5, 1.0]
        (r, (from_f64::<T>(2.0) - h) * from_f64::<T>(0.5))
    };

    Hsl {
        hue,
        saturation,
        lightness,
        standard: PhantomData,
    }
}

pub fn sample_hsluv<Wp, T>(hue: LuvHue<T>, r1: T, r2: T) -> Hsluv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    let (saturation, l) = if r1 <= from_f64::<T>(0.5) {
        // Scale it up to [0, 1]
        let r1 = r1 * from_f64::<T>(2.0);
        let h = Float::cbrt(r1);
        let r = Float::sqrt(r2) * from_f64::<T>(100.0);
        // Scale the lightness back to [0, 0.5]
        (r, h * from_f64::<T>(50.0))
    } else {
        // Scale and shift it to [0, 1).
        let r1 = (from_f64::<T>(1.0) - r1) * from_f64::<T>(2.0);
        let h = Float::cbrt(r1);
        let r = Float::sqrt(r2) * from_f64::<T>(100.0);
        // Turn the cone upside-down and scale the lightness back to (0.5, 1.0]
        (r, (from_f64::<T>(2.0) - h) * from_f64::<T>(50.0))
    };

    Hsluv {
        hue,
        saturation,
        l,
        white_point: PhantomData,
    }
}

pub fn invert_hsl_sample<S, T>(color: Hsl<S, T>) -> (T, T)
where
    T: FloatComponent,
    S: RgbStandard,
{
    let r1 = if color.lightness <= from_f64::<T>(0.5) {
        // ((x * 2)^3) / 2 = x^3 * 4.
        // lightness is multiplied by 2 to scale it up to [0, 1], becoming h.
        // h is cubed to make it r1. r1 is divided by 2 to take it back to [0, 0.5].
        color.lightness * color.lightness * color.lightness * from_f64::<T>(4.0)
    } else {
        let x = color.lightness - from_f64::<T>(1.0);
        x * x * x * from_f64::<T>(4.0) + from_f64::<T>(1.0)
    };

    // saturation is first multiplied, then divided by h before squaring.
    // h can be completely eliminated, leaving only the saturation.
    let r2 = color.saturation * color.saturation;

    (r1, r2)
}

pub fn invert_hsluv_sample<Wp, T>(color: Hsluv<Wp, T>) -> (T, T)
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    let lightness: T = color.l / from_f64::<T>(100.0);
    let r1 = if lightness <= from_f64::<T>(0.5) {
        // ((x * 2)^3) / 2 = x^3 * 4.
        // l is multiplied by 2 to scale it up to [0, 1], becoming h.
        // h is cubed to make it r1. r1 is divided by 2 to take it back to [0, 0.5].
        lightness * lightness * lightness * from_f64::<T>(4.0)
    } else {
        let x = lightness - from_f64::<T>(1.0);
        x * x * x * from_f64::<T>(4.0) + from_f64::<T>(1.0)
    };

    // saturation is first multiplied, then divided by h before squaring.
    // h can be completely eliminated, leaving only the saturation.
    let r2 = (color.saturation / from_f64::<T>(100.0)).powi(2);

    (r1, r2)
}

#[cfg(test)]
mod test {
    use super::{invert_hsl_sample, invert_hsluv_sample, sample_hsl, sample_hsluv, sample_hsv};
    use crate::encoding::Srgb;
    use crate::hues::{LuvHue, RgbHue};
    use crate::white_point::D65;
    use crate::{Hsl, Hsluv, Hsv};

    #[cfg(feature = "random")]
    #[test]
    fn sample_max_min() {
        let a = sample_hsv(RgbHue::from(0.0), 0.0, 0.0);
        let b = sample_hsv(RgbHue::from(360.0), 1.0, 1.0);
        assert_relative_eq!(Hsv::new(0.0, 0.0, 0.0), a);
        assert_relative_eq!(Hsv::new(360.0, 1.0, 1.0), b);
        let a = sample_hsl(RgbHue::from(0.0), 0.0, 0.0);
        let b = sample_hsl(RgbHue::from(360.0), 1.0, 1.0);
        assert_relative_eq!(Hsl::new(0.0, 0.0, 0.0), a);
        assert_relative_eq!(Hsl::new(360.0, 1.0, 1.0), b);
        let a = sample_hsluv(LuvHue::from(0.0), 0.0, 0.0);
        let b = sample_hsluv(LuvHue::from(360.0), 1.0, 1.0);
        assert_relative_eq!(Hsluv::new(0.0, 0.0, 0.0), a);
        assert_relative_eq!(Hsluv::new(360.0, 100.0, 100.0), b);
    }

    #[cfg(feature = "random")]
    #[test]
    fn hsl_sampling() {
        // Sanity check that sampling and inverting from sample are equivalent
        macro_rules! test_hsl {
            ( $x:expr, $y:expr ) => {{
                let a = invert_hsl_sample::<Srgb, _>(sample_hsl(RgbHue::from(0.0), $x, $y));
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
    #[test]
    fn hsluv_sampling() {
        // Sanity check that sampling and inverting from sample are equivalent
        macro_rules! test_hsluv {
            ( $x:expr, $y:expr ) => {{
                let a = invert_hsluv_sample::<D65, _>(sample_hsluv(LuvHue::from(0.0), $x, $y));
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
