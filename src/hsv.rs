use num_traits::Float;
use approx::ApproxEq;

use std::ops::{Add, Sub};
use std::marker::PhantomData;
use std::any::TypeId;

use {Alpha, Hsl, Hwb, Xyz};
use {FromColor, GetHue, Hue, Limited, Mix, Pixel, RgbHue, Saturate, Shade};
use {cast, clamp};
use white_point::WhitePoint;
use rgb::{Linear, Rgb, RgbSpace};
use rgb::standards::Srgb;

/// Linear HSV with an alpha component. See the [`Hsva` implementation in
/// `Alpha`](struct.Alpha.html#Hsva).
pub type Hsva<S = Srgb, T = f32> = Alpha<Hsv<S, T>, T>;

///Linear HSV color space.
///
///HSV is a cylindrical version of [RGB](rgb/struct.LinRgb.html) and it's very
///similar to [HSL](struct.Hsl.html). The difference is that the `value`
///component in HSV determines the _brightness_ of the color, and not the
///_lightness_. The difference is that, for example, red (100% R, 0% G, 0% B)
///and white (100% R, 100% G, 100% B) has the same brightness (or value), but
///not the same lightness.
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Hsv<S = Srgb, T = f32>
where
    T: Float,
    S: RgbSpace,
{
    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc.
    pub hue: RgbHue<T>,

    ///The colorfulness of the color. 0.0 gives gray scale colors and 1.0 will
    ///give absolutely clear colors.
    pub saturation: T,

    ///Decides how bright the color will look. 0.0 will be black, and 1.0 will
    ///give a bright an clear color that goes towards white when `saturation`
    ///goes towards 0.0.
    pub value: T,

    ///The white point and RGB primaries this color is adapted to. The default
    ///is the sRGB standard.
    pub space: PhantomData<S>,
}

impl<S, T> Copy for Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
}

impl<S, T> Clone for Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
    fn clone(&self) -> Hsv<S, T> {
        *self
    }
}

unsafe impl<S: RgbSpace, T: Float> Pixel<T> for Hsv<S, T> {
    const CHANNELS: usize = 3;
}

impl<T> Hsv<Srgb, T>
where
    T: Float,
{
    ///HSV for linear sRGB.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, value: T) -> Hsv<Srgb, T> {
        Hsv {
            hue: hue.into(),
            saturation: saturation,
            value: value,
            space: PhantomData,
        }
    }
}

impl<S, T> Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
    ///Linear HSV.
    pub fn with_wp<H: Into<RgbHue<T>>>(hue: H, saturation: T, value: T) -> Hsv<S, T> {
        Hsv {
            hue: hue.into(),
            saturation: saturation,
            value: value,
            space: PhantomData,
        }
    }

    #[inline]
    fn reinterpret_as<Sp: RgbSpace>(self) -> Hsv<Sp, T> {
        Hsv {
            hue: self.hue,
            saturation: self.saturation,
            value: self.value,
            space: PhantomData,
        }
    }
}

///<span id="Hsva"></span>[`Hsva`](type.Hsva.html) implementations.
impl<T> Alpha<Hsv<Srgb, T>, T>
where
    T: Float,
{
    ///HSV and transparency for linear sRGB.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, value: T, alpha: T) -> Hsva<Srgb, T> {
        Alpha {
            color: Hsv::new(hue, saturation, value),
            alpha: alpha,
        }
    }
}

///<span id="Hsva"></span>[`Hsva`](type.Hsva.html) implementations.
impl<S, T> Alpha<Hsv<S, T>, T>
where
    T: Float,
    S: RgbSpace,
{
    ///Linear HSV and transparency.
    pub fn with_wp<H: Into<RgbHue<T>>>(hue: H, saturation: T, value: T, alpha: T) -> Hsva<S, T> {
        Alpha {
            color: Hsv::with_wp(hue, saturation, value),
            alpha: alpha,
        }
    }
}

impl<S, Wp, T> FromColor<Wp, T> for Hsv<S, T>
where
    T: Float,
    S: RgbSpace<WhitePoint = Wp>,
    Wp: WhitePoint,
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        let rgb: Rgb<Linear<S>, T> = Rgb::from_xyz(xyz);
        Self::from_rgb(rgb)
    }

    fn from_rgb<Sp: RgbSpace<WhitePoint = Wp>>(rgb: Rgb<Linear<Sp>, T>) -> Self {
        let rgb = Rgb::<Linear<S>, T>::from_rgb(rgb);

        let (max, min, sep, coeff) = {
            let (max, min, sep, coeff) = if rgb.red > rgb.green {
                (rgb.red, rgb.green, rgb.green - rgb.blue, T::zero())
            } else {
                (rgb.green, rgb.red, rgb.blue - rgb.red, cast(2.0))
            };
            if rgb.blue > max {
                (rgb.blue, min, rgb.red - rgb.green, cast(4.0))
            } else {
                let min_val = if rgb.blue < min {
                    rgb.blue
                } else {
                    min
                };
                (max, min_val, sep, coeff)
            }
        };

        let mut h = T::zero();
        let mut s = T::zero();
        let v = max;

        if max != min {
            let d = max - min;
            s = d / max;
            h = ((sep / d) + coeff) * cast(60.0);
        };

        Hsv {
            hue: h.into(),
            saturation: s,
            value: v,
            space: PhantomData,
        }
    }

    fn from_hsl<Sp: RgbSpace<WhitePoint = Wp>>(hsl: Hsl<Sp, T>) -> Self {
        let hsl = Hsl::<S, T>::from_hsl(hsl);

        let x = hsl.saturation * if hsl.lightness < cast(0.5) {
            hsl.lightness
        } else {
            T::one() - hsl.lightness
        };
        let mut s = T::zero();

        // avoid divide by zero
        let denom = hsl.lightness + x;
        if denom.is_normal() {
            s = x * cast(2.0) / denom;
        }
        Hsv {
            hue: hsl.hue,
            saturation: s,
            value: hsl.lightness + x,
            space: PhantomData,
        }
    }

    fn from_hsv<Sp: RgbSpace<WhitePoint = Wp>>(hsv: Hsv<Sp, T>) -> Self {
        if TypeId::of::<Sp::Primaries>() == TypeId::of::<S::Primaries>() {
            hsv.reinterpret_as()
        } else {
            Self::from_rgb(Rgb::<Linear<Sp>, T>::from_hsv(hsv))
        }
    }

    fn from_hwb<Sp: RgbSpace<WhitePoint = Wp>>(hwb: Hwb<Sp, T>) -> Self {
        let hwb = Hwb::<S, T>::from_hwb(hwb);

        let inv = T::one() - hwb.blackness;
        // avoid divide by zero
        let s = if inv.is_normal() {
            T::one() - (hwb.whiteness / inv)
        } else {
            T::zero()
        };
        Hsv {
            hue: hwb.hue,
            saturation: s,
            value: inv,
            space: PhantomData,
        }
    }
}

impl<S, T> Limited for Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn is_valid(&self) -> bool {
        self.saturation >= T::zero() && self.saturation <= T::one() &&
        self.value >= T::zero() && self.value <= T::one()
    }

    fn clamp(&self) -> Hsv<S, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.saturation = clamp(self.saturation, T::zero(), T::one());
        self.value = clamp(self.value, T::zero(), T::one());
    }
}

impl<S, T> Mix for Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Scalar = T;

    fn mix(&self, other: &Hsv<S, T>, factor: T) -> Hsv<S, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();

        Hsv {
            hue: self.hue + factor * hue_diff,
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            value: self.value + factor * (other.value - self.value),
            space: PhantomData,
        }
    }
}

impl<S, T> Shade for Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Hsv<S, T> {
        Hsv {
            hue: self.hue,
            saturation: self.saturation,
            value: self.value + amount,
            space: PhantomData,
        }
    }
}

impl<S, T> GetHue for Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        if self.saturation <= T::zero() || self.value <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<S, T> Hue for Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
    fn with_hue<H: Into<Self::Hue>>(&self, hue: H) -> Hsv<S, T> {
        Hsv {
            hue: hue.into(),
            saturation: self.saturation,
            value: self.value,
            space: PhantomData,
        }
    }

    fn shift_hue<H: Into<Self::Hue>>(&self, amount: H) -> Hsv<S, T> {
        Hsv {
            hue: self.hue + amount.into(),
            saturation: self.saturation,
            value: self.value,
            space: PhantomData,
        }
    }
}

impl<S, T> Saturate for Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Scalar = T;

    fn saturate(&self, factor: T) -> Hsv<S, T> {
        Hsv {
            hue: self.hue,
            saturation: self.saturation * (T::one() + factor),
            value: self.value,
            space: PhantomData,
        }
    }
}

impl<S, T> Default for Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
    fn default() -> Hsv<S, T> {
        Hsv::with_wp(RgbHue::from(T::zero()), T::zero(), T::zero())
    }
}

impl<S, T> Add<Hsv<S, T>> for Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Output = Hsv<S, T>;

    fn add(self, other: Hsv<S, T>) -> Hsv<S, T> {
        Hsv {
            hue: self.hue + other.hue,
            saturation: self.saturation + other.saturation,
            value: self.value + other.value,
            space: PhantomData,
        }
    }
}

impl<S, T> Add<T> for Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Output = Hsv<S, T>;

    fn add(self, c: T) -> Hsv<S, T> {
        Hsv {
            hue: self.hue + c,
            saturation: self.saturation + c,
            value: self.value + c,
            space: PhantomData,
        }
    }
}

impl<S, T> Sub<Hsv<S, T>> for Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Output = Hsv<S, T>;

    fn sub(self, other: Hsv<S, T>) -> Hsv<S, T> {
        Hsv {
            hue: self.hue - other.hue,
            saturation: self.saturation - other.saturation,
            value: self.value - other.value,
            space: PhantomData,
        }
    }
}

impl<S, T> Sub<T> for Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
    type Output = Hsv<S, T>;

    fn sub(self, c: T) -> Hsv<S, T> {
        Hsv {
            hue: self.hue - c,
            saturation: self.saturation - c,
            value: self.value - c,
            space: PhantomData,
        }
    }
}

impl<S, T> From<Alpha<Hsv<S, T>, T>> for Hsv<S, T>
where
    T: Float,
    S: RgbSpace,
{
    fn from(color: Alpha<Hsv<S, T>, T>) -> Hsv<S, T> {
        color.color
    }
}

impl<S, T> ApproxEq for Hsv<S, T>
where
    T: Float + ApproxEq,
    T::Epsilon: Copy + Float,
    S: RgbSpace,
{
    type Epsilon = <T as ApproxEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.hue.relative_eq(&other.hue, epsilon, max_relative) &&
        self.saturation.relative_eq(&other.saturation, epsilon, max_relative) &&
        self.value.relative_eq(&other.value, epsilon, max_relative)
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.hue.ulps_eq(&other.hue, epsilon, max_ulps) &&
        self.saturation.ulps_eq(&other.saturation, epsilon, max_ulps) &&
        self.value.ulps_eq(&other.value, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod test {
    use super::Hsv;
    use {Hsl, LinSrgb};
    use rgb::standards::Srgb;

    #[test]
    fn red() {
        let a = Hsv::from(LinSrgb::new(1.0, 0.0, 0.0));
        let b = Hsv::new(0.0, 1.0, 1.0);
        let c = Hsv::from(Hsl::new(0.0, 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn orange() {
        let a = Hsv::from(LinSrgb::new(1.0, 0.5, 0.0));
        let b = Hsv::new(30.0, 1.0, 1.0);
        let c = Hsv::from(Hsl::new(30.0, 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn green() {
        let a = Hsv::from(LinSrgb::new(0.0, 1.0, 0.0));
        let b = Hsv::new(120.0, 1.0, 1.0);
        let c = Hsv::from(Hsl::new(120.0, 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn blue() {
        let a = Hsv::from(LinSrgb::new(0.0, 0.0, 1.0));
        let b = Hsv::new(240.0, 1.0, 1.0);
        let c = Hsv::from(Hsl::new(240.0, 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn purple() {
        let a = Hsv::from(LinSrgb::new(0.5, 0.0, 1.0));
        let b = Hsv::new(270.0, 1.0, 1.0);
        let c = Hsv::from(Hsl::new(270.0, 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn ranges() {
        assert_ranges!{
            Hsv<Srgb, f64>;
            limited {
                saturation: 0.0 => 1.0,
                value: 0.0 => 1.0
            }
            limited_min {}
            unlimited {
                hue: -360.0 => 360.0
            }
        }
    }

    raw_pixel_conversion_tests!(Hsv<Srgb>: hue, saturation, value);
    raw_pixel_conversion_fail_tests!(Hsv<Srgb>: hue, saturation, value);
}
