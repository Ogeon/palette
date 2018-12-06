use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use float::Float;

use core::any::TypeId;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use encoding::pixel::RawPixel;
use encoding::{Linear, Srgb};
use rgb::{Rgb, RgbSpace};
use {cast, clamp};
use {Alpha, Hsl, Hwb, Xyz};
use {Component, FromColor, GetHue, Hue, Limited, Mix, Pixel, RgbHue, Saturate, Shade};

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
#[derive(Debug, PartialEq, FromColor, Pixel)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette_internal]
#[palette_white_point = "S::WhitePoint"]
#[palette_rgb_space = "S"]
#[palette_component = "T"]
#[palette_manual_from(Xyz, Rgb = "from_rgb_internal", Hsl, Hwb, Hsv = "from_hsv_internal")]
#[repr(C)]
pub struct Hsv<S = Srgb, T = f32>
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

    ///Decides how bright the color will look. 0.0 will be black, and 1.0 will
    ///give a bright an clear color that goes towards white when `saturation`
    ///goes towards 0.0.
    pub value: T,

    ///The white point and RGB primaries this color is adapted to. The default
    ///is the sRGB standard.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette_unsafe_zero_sized]
    pub space: PhantomData<S>,
}

impl<S, T> Copy for Hsv<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
}

impl<S, T> Clone for Hsv<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    fn clone(&self) -> Hsv<S, T> {
        *self
    }
}

impl<T> Hsv<Srgb, T>
where
    T: Component + Float,
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
    T: Component + Float,
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

    /// Convert to a `(hue, saturation, value)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T) {
        (self.hue, self.saturation, self.value)
    }

    /// Convert from a `(hue, saturation, value)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>((hue, saturation, value): (H, T, T)) -> Self {
        Self::with_wp(hue, saturation, value)
    }

    fn from_hsv_internal<Sp: RgbSpace<WhitePoint = S::WhitePoint>>(hsv: Hsv<Sp, T>) -> Self {
        if TypeId::of::<Sp::Primaries>() == TypeId::of::<S::Primaries>() {
            hsv.reinterpret_as()
        } else {
            Self::from_rgb(Rgb::<Linear<Sp>, T>::from_hsv(hsv))
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
impl<T, A> Alpha<Hsv<Srgb, T>, A>
where
    T: Component + Float,
    A: Component,
{
    ///HSV and transparency for linear sRGB.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, saturation: T, value: T, alpha: A) -> Self {
        Alpha {
            color: Hsv::new(hue, saturation, value),
            alpha: alpha,
        }
    }
}

///<span id="Hsva"></span>[`Hsva`](type.Hsva.html) implementations.
impl<S, T, A> Alpha<Hsv<S, T>, A>
where
    T: Component + Float,
    A: Component,
    S: RgbSpace,
{
    ///Linear HSV and transparency.
    pub fn with_wp<H: Into<RgbHue<T>>>(hue: H, saturation: T, value: T, alpha: A) -> Self {
        Alpha {
            color: Hsv::with_wp(hue, saturation, value),
            alpha: alpha,
        }
    }

    /// Convert to a `(hue, saturation, value, alpha)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T, A) {
        (self.hue, self.saturation, self.value, self.alpha)
    }

    /// Convert from a `(hue, saturation, value, alpha)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>(
        (hue, saturation, value, alpha): (H, T, T, A),
    ) -> Self {
        Self::with_wp(hue, saturation, value, alpha)
    }
}

impl<S, T> From<Xyz<S::WhitePoint, T>> for Hsv<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    fn from(color: Xyz<S::WhitePoint, T>) -> Self {
        let rgb: Rgb<Linear<S>, T> = Rgb::from_xyz(color);
        Self::from_rgb(rgb)
    }
}

impl<S, Sp, T> From<Hsl<Sp, T>> for Hsv<S, T>
where
    T: Component + Float,
    S: RgbSpace,
    Sp: RgbSpace<WhitePoint = S::WhitePoint>,
{
    fn from(color: Hsl<Sp, T>) -> Self {
        let hsl = Hsl::<S, T>::from_hsl(color);

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
}

impl<S, Sp, T> From<Hwb<Sp, T>> for Hsv<S, T>
where
    T: Component + Float,
    S: RgbSpace,
    Sp: RgbSpace<WhitePoint = S::WhitePoint>,
{
    fn from(color: Hwb<Sp, T>) -> Self {
        let hwb = Hwb::<S, T>::from_hwb(color);

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

impl<S: RgbSpace, T: Component + Float, H: Into<RgbHue<T>>> From<(H, T, T)> for Hsv<S, T> {
    fn from(components: (H, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<S: RgbSpace, T: Component + Float> Into<(RgbHue<T>, T, T)> for Hsv<S, T> {
    fn into(self) -> (RgbHue<T>, T, T) {
        self.into_components()
    }
}

impl<S: RgbSpace, T: Component + Float, H: Into<RgbHue<T>>, A: Component> From<(H, T, T, A)>
    for Alpha<Hsv<S, T>, A>
{
    fn from(components: (H, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<S: RgbSpace, T: Component + Float, A: Component> Into<(RgbHue<T>, T, T, A)>
    for Alpha<Hsv<S, T>, A>
{
    fn into(self) -> (RgbHue<T>, T, T, A) {
        self.into_components()
    }
}

impl<S, T> Limited for Hsv<S, T>
where
    T: Component + Float,
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
    T: Component + Float,
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
    T: Component + Float,
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
    T: Component + Float,
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
    T: Component + Float,
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
    T: Component + Float,
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
    T: Component + Float,
    S: RgbSpace,
{
    fn default() -> Hsv<S, T> {
        Hsv::with_wp(RgbHue::from(T::zero()), T::zero(), T::zero())
    }
}

impl<S, T> Add<Hsv<S, T>> for Hsv<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Output = Hsv<S, T>;

    fn add(self, other: Hsv<S, T>) -> Self::Output {
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
    T: Component + Float,
    S: RgbSpace,
{
    type Output = Hsv<S, T>;

    fn add(self, c: T) -> Self::Output{
        Hsv {
            hue: self.hue + c,
            saturation: self.saturation + c,
            value: self.value + c,
            space: PhantomData,
        }
    }
}

impl<S, T> AddAssign<Hsv<S, T>> for Hsv<S, T>
    where
        T: Component + Float + AddAssign,
        S: RgbSpace,
{
    fn add_assign(&mut self, other: Hsv<S, T>) {
        self.hue += other.hue;
        self.saturation += other.saturation;
        self.value += other.value;
    }
}

impl<S, T> AddAssign<T> for Hsv<S, T>
    where
        T: Component + Float + AddAssign,
        S: RgbSpace,
{
    fn add_assign(&mut self, c: T) {
        self.hue += c;
        self.saturation += c;
        self.value += c;
    }
}

impl<S, T> Sub<Hsv<S, T>> for Hsv<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Output = Hsv<S, T>;

    fn sub(self, other: Hsv<S, T>) -> Self::Output {
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
    T: Component + Float,
    S: RgbSpace,
{
    type Output = Hsv<S, T>;

    fn sub(self, c: T) -> Self::Output {
        Hsv {
            hue: self.hue - c,
            saturation: self.saturation - c,
            value: self.value - c,
            space: PhantomData,
        }
    }
}

impl<S, T> SubAssign<Hsv<S, T>> for Hsv<S, T>
    where
        T: Component + Float + SubAssign,
        S: RgbSpace,
{
    fn sub_assign(&mut self, other: Hsv<S, T>) {
        self.hue -= other.hue;
        self.saturation -= other.saturation;
        self.value -= other.value;
    }
}

impl<S, T> SubAssign<T> for Hsv<S, T>
    where
        T: Component + Float + SubAssign,
        S: RgbSpace,
{
    fn sub_assign(&mut self, c: T) {
        self.hue -= c;
        self.saturation -= c;
        self.value -= c;
    }
}

impl<S, T, P> AsRef<P> for Hsv<S, T>
where
    T: Component + Float,
    S: RgbSpace,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<S, T, P> AsMut<P> for Hsv<S, T>
where
    T: Component + Float,
    S: RgbSpace,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<S, T> AbsDiffEq for Hsv<S, T>
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
            self.value.abs_diff_eq(&other.value, epsilon)
    }
}

impl<S, T> RelativeEq for Hsv<S, T>
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
            self.value.relative_eq(&other.value, epsilon, max_relative)
    }
}

impl<S, T> UlpsEq for Hsv<S, T>
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
            self.value.ulps_eq(&other.value, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod test {
    use super::Hsv;
    use encoding::Srgb;
    use {Hsl, LinSrgb};

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

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Hsv::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"hue":0.3,"saturation":0.8,"value":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Hsv =
            ::serde_json::from_str(r#"{"hue":0.3,"saturation":0.8,"value":0.1}"#).unwrap();

        assert_eq!(deserialized, Hsv::new(0.3, 0.8, 0.1));
    }
}
