use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use float::Float;

use core::any::TypeId;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use encoding::pixel::RawPixel;
use encoding::{Linear, Srgb};
use rgb::{Rgb, RgbSpace};
use {
    cast, clamp, Alpha, Component, FromColor, GetHue, Hsv, Hue, IntoColor, Limited, Mix, Pixel,
    RgbHue, Saturate, Shade, Xyz,
};

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
///See [HSV](struct.Hsv.html) for a very similar color space, with brightness
/// instead of lightness.
#[derive(Debug, PartialEq, FromColor, Pixel)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette_internal]
#[palette_rgb_space = "S"]
#[palette_white_point = "S::WhitePoint"]
#[palette_component = "T"]
#[palette_manual_from(Xyz, Rgb = "from_rgb_internal", Hsv, Hsl = "from_hsl_internal")]
#[repr(C)]
pub struct Hsl<S = Srgb, T = f32>
where
    T: Component + Float,
    S: RgbSpace,
{
    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc.
    #[palette_unsafe_same_layout_as = "T"]
    pub hue: RgbHue<T>,

    ///The colorfulness of the color. 0.0 gives gray scale colors and 1.0 will
    ///give absolutely clear colors.
    pub saturation: T,

    ///Decides how light the color will look. 0.0 will be black, 0.5 will give
    ///a clear color, and 1.0 will give white.
    pub lightness: T,

    ///The white point and RGB primaries this color is adapted to. The default
    ///is the sRGB standard.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette_unsafe_zero_sized]
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

    /// Convert to a `(hue, saturation, lightness)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T) {
        (self.hue, self.saturation, self.lightness)
    }

    /// Convert from a `(hue, saturation, lightness)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>((hue, saturation, lightness): (H, T, T)) -> Self {
        Self::with_wp(hue, saturation, lightness)
    }

    fn from_hsl_internal<Sp: RgbSpace<WhitePoint = S::WhitePoint>>(hsl: Hsl<Sp, T>) -> Self {
        if TypeId::of::<Sp::Primaries>() == TypeId::of::<S::Primaries>() {
            hsl.reinterpret_as()
        } else {
            Self::from_rgb(Rgb::<Linear<Sp>, T>::from_hsl(hsl))
        }
    }

    fn from_rgb_internal<Sp: RgbSpace<WhitePoint = S::WhitePoint>>(
        color: Rgb<Linear<Sp>, T>,
    ) -> Self {
        let rgb = Rgb::<Linear<S>, T>::from_rgb(color);

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
impl<T, A> Alpha<Hsl<Srgb, T>, A>
where
    T: Component + Float,
    A: Component,
{
    ///HSL and transparency for linear sRGB.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T, alpha: A) -> Self {
        Alpha {
            color: Hsl::new(hue, saturation, lightness),
            alpha: alpha,
        }
    }
}

///<span id="Hsla"></span>[`Hsla`](type.Hsla.html) implementations.
impl<S, T, A> Alpha<Hsl<S, T>, A>
where
    T: Component + Float,
    A: Component,
    S: RgbSpace,
{
    ///Linear HSL and transparency.
    pub fn with_wp<H: Into<RgbHue<T>>>(hue: H, saturation: T, lightness: T, alpha: A) -> Self {
        Alpha {
            color: Hsl::with_wp(hue, saturation, lightness),
            alpha: alpha,
        }
    }

    /// Convert to a `(hue, saturation, lightness, alpha)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T, A) {
        (self.hue, self.saturation, self.lightness, self.alpha)
    }

    /// Convert from a `(hue, saturation, lightness, alpha)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>(
        (hue, saturation, lightness, alpha): (H, T, T, A),
    ) -> Self {
        Self::with_wp(hue, saturation, lightness, alpha)
    }
}

impl<S, T> From<Xyz<S::WhitePoint, T>> for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    fn from(color: Xyz<S::WhitePoint, T>) -> Self {
        let rgb: Rgb<Linear<S>, T> = color.into_rgb();
        Self::from_rgb(rgb)
    }
}

impl<S, Sp, T> From<Hsv<Sp, T>> for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
    Sp: RgbSpace<WhitePoint = S::WhitePoint>,
{
    fn from(color: Hsv<Sp, T>) -> Self {
        let hsv = Hsv::<S, T>::from_hsv(color);

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

impl<S: RgbSpace, T: Component + Float, H: Into<RgbHue<T>>> From<(H, T, T)> for Hsl<S, T> {
    fn from(components: (H, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<S: RgbSpace, T: Component + Float> Into<(RgbHue<T>, T, T)> for Hsl<S, T> {
    fn into(self) -> (RgbHue<T>, T, T) {
        self.into_components()
    }
}

impl<S: RgbSpace, T: Component + Float, H: Into<RgbHue<T>>, A: Component> From<(H, T, T, A)>
    for Alpha<Hsl<S, T>, A>
{
    fn from(components: (H, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<S: RgbSpace, T: Component + Float, A: Component> Into<(RgbHue<T>, T, T, A)>
    for Alpha<Hsl<S, T>, A>
{
    fn into(self) -> (RgbHue<T>, T, T, A) {
        self.into_components()
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

    fn add(self, other: Hsl<S, T>) -> Self::Output {
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

    fn add(self, c: T) -> Self::Output {
        Hsl {
            hue: self.hue + c,
            saturation: self.saturation + c,
            lightness: self.lightness + c,
            space: PhantomData,
        }
    }
}

impl<S, T> AddAssign<Hsl<S, T>> for Hsl<S, T>
    where
        T: Component + Float + AddAssign,
        S: RgbSpace,
{
    fn add_assign(&mut self, other: Hsl<S, T>) {
        self.hue += other.hue;
        self.saturation += other.saturation;
        self.lightness += other.lightness;
    }
}

impl<S, T> AddAssign<T> for Hsl<S, T>
    where
        T: Component + Float + AddAssign,
        S: RgbSpace,
{
    fn add_assign(&mut self, c: T) {
        self.hue += c;
        self.saturation += c;
        self.lightness += c;
    }
}

impl<S, T> Sub<Hsl<S, T>> for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Output = Hsl<S, T>;

    fn sub(self, other: Hsl<S, T>) -> Self::Output {
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

    fn sub(self, c: T) -> Self::Output {
        Hsl {
            hue: self.hue - c,
            saturation: self.saturation - c,
            lightness: self.lightness - c,
            space: PhantomData,
        }
    }
}

impl<S, T> SubAssign<Hsl<S, T>> for Hsl<S, T>
    where
        T: Component + Float + SubAssign,
        S: RgbSpace,
{
    fn sub_assign(&mut self, other: Hsl<S, T>) {
        self.hue -= other.hue;
        self.saturation -= other.saturation;
        self.lightness -= other.lightness;
    }
}

impl<S, T> SubAssign<T> for Hsl<S, T>
    where
        T: Component + Float + SubAssign,
        S: RgbSpace,
{
    fn sub_assign(&mut self, c: T) {
        self.hue -= c;
        self.saturation -= c;
        self.lightness -= c;
    }
}

impl<S, T, P> AsRef<P> for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<S, T, P> AsMut<P> for Hsl<S, T>
where
    T: Component + Float,
    S: RgbSpace,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<S, T> AbsDiffEq for Hsl<S, T>
where
    T: Component + Float + AbsDiffEq,
    T::Epsilon: Copy + Float,
    S: RgbSpace + PartialEq,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        self.hue.abs_diff_eq(&other.hue, epsilon) &&
            self.saturation.abs_diff_eq(&other.saturation, epsilon) &&
            self.lightness.abs_diff_eq(&other.lightness, epsilon)
    }
}

impl<S, T> RelativeEq for Hsl<S, T>
where
    T: Component + Float + RelativeEq,
    T::Epsilon: Copy + Float,
    S: RgbSpace + PartialEq,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
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
}

impl<S, T> UlpsEq for Hsl<S, T>
where
    T: Component + Float + UlpsEq,
    T::Epsilon: Copy + Float,
    S: RgbSpace + PartialEq,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
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
    use encoding::Srgb;
    use {Hsv, LinSrgb};

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

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Hsl::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(
            serialized,
            r#"{"hue":0.3,"saturation":0.8,"lightness":0.1}"#
        );
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Hsl =
            ::serde_json::from_str(r#"{"hue":0.3,"saturation":0.8,"lightness":0.1}"#).unwrap();

        assert_eq!(deserialized, Hsl::new(0.3, 0.8, 0.1));
    }
}
