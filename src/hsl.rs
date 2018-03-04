use num_traits::Float;
use approx::ApproxEq;

use std::ops::{Add, Sub};
use std::marker::PhantomData;
use std::any::TypeId;

use {cast, clamp, Alpha, Component, FromColor, GetHue, Hsv, Hue, IntoColor, Limited, Mix, Pixel,
     RgbHue, Saturate, Shade, Xyz};
use white_point::WhitePoint;
use rgb::{Linear, Rgb, RgbSpace};
use rgb::standards::Srgb;

/// Linear HSL with an alpha component. See the [`Hsla` implementation in
/// `Alpha`](struct.Alpha.html#Hsla).
pub type Hsla<S = Srgb, T = f32> = Alpha<Hsl<S, T>, T>;

///Linear HSL color space.
///
///The HSL color space can be seen as a cylindrical version of
///[RGB](rgb/struct.LinRgb.html), where the `hue` is the angle around the color
///cylinder, the `saturation` is the distance from the center, and the
///`lightness` is the height from the bottom. Its composition makes it
///especially good for operations like changing green to red, making a color
///more gray, or making it darker.
///
///See [HSV](struct.Hsv.html) for a very similar color space, with brightness instead of lightness.
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Hsl<S = Srgb, T = f32>
where
    T: Component + Float,
    S: RgbSpace,
{
    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc.
    pub hue: RgbHue<T>,

    ///The colorfulness of the color. 0.0 gives gray scale colors and 1.0 will
    ///give absolutely clear colors.
    pub saturation: T,

    ///Decides how light the color will look. 0.0 will be black, 0.5 will give
    ///a clear color, and 1.0 will give white.
    pub lightness: T,

    ///The white point and RGB primaries this color is adapted to. The default
    ///is the sRGB standard.
    pub space: PhantomData<S>,
}

impl<S, T> Copy for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
}

impl<S, T> Clone for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    fn clone(&self) -> Hsl<S, T> {
        *self
    }
}

unsafe impl<S: RgbSpace, T: Component + Float> Pixel<T> for Hsl<S, T> {
    const CHANNELS: usize = 3;
}

impl<T> Hsl<Srgb, T>
where
    T: Component + Float,
{
    ///HSL for linear sRGB.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T) -> Hsl<Srgb, T> {
        Hsl {
            hue: hue.into(),
            saturation: saturation,
            lightness: lightness,
            space: PhantomData,
        }
    }
}

impl<S, T> Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    ///Linear HSL.
    pub fn with_wp<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T) -> Hsl<S, T> {
        Hsl {
            hue: hue.into(),
            saturation: saturation,
            lightness: lightness,
            space: PhantomData,
        }
    }

    #[inline]
    fn reinterpret_as<Sp: RgbSpace>(self) -> Hsl<Sp, T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation,
            lightness: self.lightness,
            space: PhantomData,
        }
    }
}

///<span id="Hsla"></span>[`Hsla`](type.Hsla.html) implementations.
impl<T> Alpha<Hsl<Srgb, T>, T>
where
    T: Component + Float,
{
    ///HSL and transparency for linear sRGB.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T, alpha: T) -> Hsla<Srgb, T> {
        Alpha {
            color: Hsl::new(hue, saturation, lightness),
            alpha: alpha,
        }
    }
}

///<span id="Hsla"></span>[`Hsla`](type.Hsla.html) implementations.
impl<S, T> Alpha<Hsl<S, T>, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    ///Linear HSL and transparency.
    pub fn with_wp<H: Into<RgbHue<T>>>(
        hue: H,
        saturation: T,
        lightness: T,
        alpha: T,
    ) -> Hsla<S, T> {
        Alpha {
            color: Hsl::with_wp(hue, saturation, lightness),
            alpha: alpha,
        }
    }
}

impl<S, Wp, T> FromColor<Wp, T> for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace<WhitePoint = Wp>,
    Wp: WhitePoint,
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        let rgb: Rgb<Linear<S>, T> = xyz.into_rgb();
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

        let sum = max + min;
        let l = sum / cast(2.0);
        if max != min {
            let d = max - min;
            s = if sum > T::one() {
                d / (cast::<T, _>(2.0) - sum)
            } else {
                d / sum
            };
            h = ((sep / d) + coeff) * cast(60.0);
        };

        Hsl {
            hue: h.into(),
            saturation: s,
            lightness: l,
            space: PhantomData,
        }
    }

    fn from_hsl<Sp: RgbSpace<WhitePoint = Wp>>(hsl: Hsl<Sp, T>) -> Self {
        if TypeId::of::<Sp::Primaries>() == TypeId::of::<S::Primaries>() {
            hsl.reinterpret_as()
        } else {
            Self::from_rgb(Rgb::<Linear<Sp>, T>::from_hsl(hsl))
        }
    }

    fn from_hsv<Sp: RgbSpace<WhitePoint = Wp>>(hsv: Hsv<Sp, T>) -> Self {
        let hsv = Hsv::<S, T>::from_hsv(hsv);

        let x = (cast::<T, _>(2.0) - hsv.saturation) * hsv.value;
        let saturation = if !hsv.value.is_normal() {
            T::zero()
        } else if x < T::one() {
            if x.is_normal() {
                hsv.saturation * hsv.value / x
            } else {
                T::zero()
            }
        } else {
            let denom = cast::<T, _>(2.0) - x;
            if denom.is_normal() {
                hsv.saturation * hsv.value / denom
            } else {
                T::zero()
            }
        };

        Hsl {
            hue: hsv.hue,
            saturation: saturation,
            lightness: x / cast(2.0),
            space: PhantomData,
        }
    }
}

impl<S, T> Limited for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn is_valid(&self) -> bool {
        self.saturation >= T::zero() && self.saturation <= T::one() &&
        self.lightness >= T::zero() && self.lightness <= T::one()
    }

    fn clamp(&self) -> Hsl<S, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.saturation = clamp(self.saturation, T::zero(), T::one());
        self.lightness = clamp(self.lightness, T::zero(), T::one());
    }
}

impl<S, T> Mix for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Scalar = T;

    fn mix(&self, other: &Hsl<S, T>, factor: T) -> Hsl<S, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();

        Hsl {
            hue: self.hue + factor * hue_diff,
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            lightness: self.lightness + factor * (other.lightness - self.lightness),
            space: PhantomData,
        }
    }
}

impl<S, T> Shade for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Hsl<S, T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation,
            lightness: self.lightness + amount,
            space: PhantomData,
        }
    }
}

impl<S, T> GetHue for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        if self.saturation <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<S, T> Hue for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    fn with_hue<H: Into<Self::Hue>>(&self, hue: H) -> Hsl<S, T> {
        Hsl {
            hue: hue.into(),
            saturation: self.saturation,
            lightness: self.lightness,
            space: PhantomData,
        }
    }

    fn shift_hue<H: Into<Self::Hue>>(&self, amount: H) -> Hsl<S, T> {
        Hsl {
            hue: self.hue + amount.into(),
            saturation: self.saturation,
            lightness: self.lightness,
            space: PhantomData,
        }
    }
}

impl<S, T> Saturate for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Scalar = T;

    fn saturate(&self, factor: T) -> Hsl<S, T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation * (T::one() + factor),
            lightness: self.lightness,
            space: PhantomData,
        }
    }
}

impl<S, T> Default for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    fn default() -> Hsl<S, T> {
        Hsl::with_wp(RgbHue::from(T::zero()), T::zero(), T::zero())
    }
}

impl<S, T> Add<Hsl<S, T>> for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Output = Hsl<S, T>;

    fn add(self, other: Hsl<S, T>) -> Hsl<S, T> {
        Hsl {
            hue: self.hue + other.hue,
            saturation: self.saturation + other.saturation,
            lightness: self.lightness + other.lightness,
            space: PhantomData,
        }
    }
}

impl<S, T> Add<T> for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Output = Hsl<S, T>;

    fn add(self, c: T) -> Hsl<S, T> {
        Hsl {
            hue: self.hue + c,
            saturation: self.saturation + c,
            lightness: self.lightness + c,
            space: PhantomData,
        }
    }
}

impl<S, T> Sub<Hsl<S, T>> for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Output = Hsl<S, T>;

    fn sub(self, other: Hsl<S, T>) -> Hsl<S, T> {
        Hsl {
            hue: self.hue - other.hue,
            saturation: self.saturation - other.saturation,
            lightness: self.lightness - other.lightness,
            space: PhantomData,
        }
    }
}

impl<S, T> Sub<T> for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Output = Hsl<S, T>;

    fn sub(self, c: T) -> Hsl<S, T> {
        Hsl {
            hue: self.hue - c,
            saturation: self.saturation - c,
            lightness: self.lightness - c,
            space: PhantomData,
        }
    }
}

impl<S, T> From<Alpha<Hsl<S, T>, T>> for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    fn from(color: Alpha<Hsl<S, T>, T>) -> Hsl<S, T> {
        color.color
    }
}

impl<S, T> ApproxEq for Hsl<S, T>
where
    T: Component + Float + ApproxEq,
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
        self.lightness.relative_eq(&other.lightness, epsilon, max_relative)
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.hue.ulps_eq(&other.hue, epsilon, max_ulps) &&
        self.saturation.ulps_eq(&other.saturation, epsilon, max_ulps) &&
        self.lightness.ulps_eq(&other.lightness, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod test {
    use super::Hsl;
    use {Hsv, LinSrgb};
    use rgb::standards::Srgb;

    #[test]
    fn red() {
        let a = Hsl::from(LinSrgb::new(1.0, 0.0, 0.0));
        let b = Hsl::new(0.0, 1.0, 0.5);
        let c = Hsl::from(Hsv::new(0.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn orange() {
        let a = Hsl::from(LinSrgb::new(1.0, 0.5, 0.0));
        let b = Hsl::new(30.0, 1.0, 0.5);
        let c = Hsl::from(Hsv::new(30.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn green() {
        let a = Hsl::from(LinSrgb::new(0.0, 1.0, 0.0));
        let b = Hsl::new(120.0, 1.0, 0.5);
        let c = Hsl::from(Hsv::new(120.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn blue() {
        let a = Hsl::from(LinSrgb::new(0.0, 0.0, 1.0));
        let b = Hsl::new(240.0, 1.0, 0.5);
        let c = Hsl::from(Hsv::new(240.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn purple() {
        let a = Hsl::from(LinSrgb::new(0.5, 0.0, 1.0));
        let b = Hsl::new(270.0, 1.0, 0.5);
        let c = Hsl::from(Hsv::new(270.0, 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn ranges() {
        assert_ranges!{
            Hsl<Srgb, f64>;
            limited {
                saturation: 0.0 => 1.0,
                lightness: 0.0 => 1.0
            }
            limited_min {}
            unlimited {
                hue: -360.0 => 360.0
            }
        }
    }

    raw_pixel_conversion_tests!(Hsl<Srgb>: hue, saturation, lightness);
    raw_pixel_conversion_fail_tests!(Hsl<Srgb>: hue, saturation, lightness);
}
