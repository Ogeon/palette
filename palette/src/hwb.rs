use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use float::Float;

use core::any::TypeId;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use encoding::pixel::RawPixel;
use encoding::Srgb;
use rgb::RgbSpace;
use {
    clamp, Alpha, Component, FromColor, GetHue, Hsv, Hue, IntoColor, Limited, Mix, Pixel, RgbHue,
    Shade, Xyz,
};

/// Linear HWB with an alpha component. See the [`Hwba` implementation in
/// `Alpha`](struct.Alpha.html#Hwba).
pub type Hwba<S = Srgb, T = f32> = Alpha<Hwb<S, T>, T>;

///Linear HWB color space.
///
///HWB is a cylindrical version of [RGB](rgb/struct.LinRgb.html) and it's very
///closely related to [HSV](struct.Hsv.html).  It describes colors with a
/// starting hue, then a degree of whiteness and blackness to mix into that
/// base hue.
///
///It is very intuitive for humans to use and many color-pickers are based on
/// the HWB color system
#[derive(Debug, PartialEq, FromColor, Pixel)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette_internal]
#[palette_rgb_space = "S"]
#[palette_white_point = "S::WhitePoint"]
#[palette_component = "T"]
#[palette_manual_from(Xyz, Hsv, Hwb = "from_hwb_internal")]
#[repr(C)]
pub struct Hwb<S = Srgb, T = f32>
where
    T: Component + Float,
    S: RgbSpace,
{
    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc. Same as the hue for HSL and HSV.
    #[palette_unsafe_same_layout_as = "T"]
    pub hue: RgbHue<T>,

    ///The whiteness of the color. It specifies the amount white to mix into
    /// the hue. It varies from 0 to 1, with 1 being always full white and 0
    ///always being the color shade (a mixture of a pure hue with black)
    /// chosen with the other two controls.
    pub whiteness: T,

    ///The blackness of the color. It specifies the amount black to mix into
    /// the hue. It varies from 0 to 1, with 1 being always full black and
    /// 0 always being the color tint (a mixture of a pure hue with white)
    /// chosen with the other two
    //controls.
    pub blackness: T,

    ///The white point and RGB primaries this color is adapted to. The default
    ///is the sRGB standard.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette_unsafe_zero_sized]
    pub space: PhantomData<S>,
}

impl<S, T> Copy for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
}

impl<S, T> Clone for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    fn clone(&self) -> Hwb<S, T> {
        *self
    }
}

impl<T> Hwb<Srgb, T>
where
    T: Component + Float,
{
    ///HWB for linear sRGB.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, whiteness: T, blackness: T) -> Hwb<Srgb, T> {
        Hwb {
            hue: hue.into(),
            whiteness: whiteness,
            blackness: blackness,
            space: PhantomData,
        }
    }
}

impl<S, T> Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    ///Linear HWB.
    pub fn with_wp<H: Into<RgbHue<T>>>(hue: H, whiteness: T, blackness: T) -> Hwb<S, T> {
        Hwb {
            hue: hue.into(),
            whiteness: whiteness,
            blackness: blackness,
            space: PhantomData,
        }
    }

    /// Convert to a `(hue, whiteness, blackness)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T) {
        (self.hue, self.whiteness, self.blackness)
    }

    /// Convert from a `(hue, whiteness, blackness)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>((hue, whiteness, blackness): (H, T, T)) -> Self {
        Self::with_wp(hue, whiteness, blackness)
    }

    fn from_hwb_internal<Sp: RgbSpace<WhitePoint = S::WhitePoint>>(color: Hwb<Sp, T>) -> Self {
        if TypeId::of::<Sp::Primaries>() == TypeId::of::<S::Primaries>() {
            color.reinterpret_as()
        } else {
            Self::from_hsv(Hsv::<Sp, T>::from_hwb(color))
        }
    }

    #[inline]
    fn reinterpret_as<Sp: RgbSpace>(self) -> Hwb<Sp, T> {
        Hwb {
            hue: self.hue,
            whiteness: self.whiteness,
            blackness: self.blackness,
            space: PhantomData,
        }
    }
}

///<span id="Hwba"></span>[`Hwba`](type.Hwba.html) implementations.
impl<T, A> Alpha<Hwb<Srgb, T>, A>
where
    T: Component + Float,
    A: Component,
{
    ///HWB and transparency for linear sRGB.
    pub fn new<H: Into<RgbHue<T>>>(hue: H, whiteness: T, blackness: T, alpha: A) -> Self {
        Alpha {
            color: Hwb::new(hue, whiteness, blackness),
            alpha: alpha,
        }
    }
}

///<span id="Hwba"></span>[`Hwba`](type.Hwba.html) implementations.
impl<S, T, A> Alpha<Hwb<S, T>, A>
where
    T: Component + Float,
    A: Component,
    S: RgbSpace,
{
    ///Linear HWB and transparency.
    pub fn with_wp<H: Into<RgbHue<T>>>(hue: H, whiteness: T, blackness: T, alpha: A) -> Self {
        Alpha {
            color: Hwb::with_wp(hue, whiteness, blackness),
            alpha: alpha,
        }
    }

    /// Convert to a `(hue, whiteness, blackness, alpha)` tuple.
    pub fn into_components(self) -> (RgbHue<T>, T, T, A) {
        (self.hue, self.whiteness, self.blackness, self.alpha)
    }

    /// Convert from a `(hue, whiteness, blackness, alpha)` tuple.
    pub fn from_components<H: Into<RgbHue<T>>>(
        (hue, whiteness, blackness, alpha): (H, T, T, A),
    ) -> Self {
        Self::with_wp(hue, whiteness, blackness, alpha)
    }
}

impl<S, T> From<Xyz<S::WhitePoint, T>> for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    fn from(color: Xyz<S::WhitePoint, T>) -> Self {
        let hsv: Hsv<S, T> = color.into_hsv();
        Self::from_hsv(hsv)
    }
}

impl<S, T, Sp> From<Hsv<Sp, T>> for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
    Sp: RgbSpace<WhitePoint = S::WhitePoint>,
{
    fn from(color: Hsv<Sp, T>) -> Self {
        let color = Hsv::<S, T>::from_hsv(color);

        Hwb {
            hue: color.hue,
            whiteness: (T::one() - color.saturation) * color.value,
            blackness: (T::one() - color.value),
            space: PhantomData,
        }
    }
}

impl<S: RgbSpace, T: Component + Float, H: Into<RgbHue<T>>> From<(H, T, T)> for Hwb<S, T> {
    fn from(components: (H, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<S: RgbSpace, T: Component + Float> Into<(RgbHue<T>, T, T)> for Hwb<S, T> {
    fn into(self) -> (RgbHue<T>, T, T) {
        self.into_components()
    }
}

impl<S: RgbSpace, T: Component + Float, H: Into<RgbHue<T>>, A: Component> From<(H, T, T, A)>
    for Alpha<Hwb<S, T>, A>
{
    fn from(components: (H, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<S: RgbSpace, T: Component + Float, A: Component> Into<(RgbHue<T>, T, T, A)>
    for Alpha<Hwb<S, T>, A>
{
    fn into(self) -> (RgbHue<T>, T, T, A) {
        self.into_components()
    }
}

impl<S, T> Limited for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn is_valid(&self) -> bool {
        self.blackness >= T::zero() && self.blackness <= T::one() &&
        self.whiteness >= T::zero() && self.whiteness <= T::one() &&
        self.whiteness + self.blackness <= T::one()
    }

    fn clamp(&self) -> Hwb<S, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.whiteness = self.whiteness.max(T::zero());
        self.blackness = self.blackness.max(T::zero());
        let sum = self.blackness + self.whiteness;
        if sum > T::one() {
            self.whiteness = self.whiteness / sum;
            self.blackness = self.blackness / sum;
        }
    }
}

impl<S, T> Mix for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Scalar = T;

    fn mix(&self, other: &Hwb<S, T>, factor: T) -> Hwb<S, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();

        Hwb {
            hue: self.hue + factor * hue_diff,
            whiteness: self.whiteness + factor * (other.whiteness - self.whiteness),
            blackness: self.blackness + factor * (other.blackness - self.blackness),
            space: PhantomData,
        }
    }
}

impl<S, T> Shade for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Hwb<S, T> {
        Hwb {
            hue: self.hue,
            whiteness: self.whiteness + amount,
            blackness: self.blackness - amount,
            space: PhantomData,
        }
    }
}

impl<S, T> GetHue for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        if self.whiteness + self.blackness >= T::one() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<S, T> Hue for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    fn with_hue<H: Into<Self::Hue>>(&self, hue: H) -> Hwb<S, T> {
        Hwb {
            hue: hue.into(),
            whiteness: self.whiteness,
            blackness: self.blackness,
            space: PhantomData,
        }
    }

    fn shift_hue<H: Into<Self::Hue>>(&self, amount: H) -> Hwb<S, T> {
        Hwb {
            hue: self.hue + amount.into(),
            whiteness: self.whiteness,
            blackness: self.blackness,
            space: PhantomData,
        }
    }
}

impl<S, T> Default for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    fn default() -> Hwb<S, T> {
        Hwb::with_wp(RgbHue::from(T::zero()), T::zero(), T::one())
    }
}

impl<S, T> Add<Hwb<S, T>> for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Output = Hwb<S, T>;

    fn add(self, other: Hwb<S, T>) -> Self::Output {
        Hwb {
            hue: self.hue + other.hue,
            whiteness: self.whiteness + other.whiteness,
            blackness: self.blackness + other.blackness,
            space: PhantomData,
        }
    }
}

impl<S, T> Add<T> for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Output = Hwb<S, T>;

    fn add(self, c: T) -> Self::Output {
        Hwb {
            hue: self.hue + c,
            whiteness: self.whiteness + c,
            blackness: self.blackness + c,
            space: PhantomData,
        }
    }
}

impl<S, T> AddAssign<Hwb<S, T>> for Hwb<S, T>
    where
        T: Component + Float + AddAssign,
        S: RgbSpace,
{
    fn add_assign(&mut self, other: Hwb<S, T>) {
        self.hue += other.hue;
        self.whiteness += other.whiteness;
        self.blackness += other.blackness;
    }
}

impl<S, T> AddAssign<T> for Hwb<S, T>
    where
        T: Component + Float + AddAssign,
        S: RgbSpace,
{
    fn add_assign(&mut self, c: T) {
        self.hue += c;
        self.whiteness += c;
        self.blackness += c;
    }
}

impl<S, T> Sub<Hwb<S, T>> for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Output = Hwb<S, T>;

    fn sub(self, other: Hwb<S, T>) -> Self::Output {
        Hwb {
            hue: self.hue - other.hue,
            whiteness: self.whiteness - other.whiteness,
            blackness: self.blackness - other.blackness,
            space: PhantomData,
        }
    }
}

impl<S, T> Sub<T> for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
{
    type Output = Hwb<S, T>;

    fn sub(self, c: T) -> Self::Output {
        Hwb {
            hue: self.hue - c,
            whiteness: self.whiteness - c,
            blackness: self.blackness - c,
            space: PhantomData,
        }
    }
}


impl<S, T> SubAssign<Hwb<S, T>> for Hwb<S, T>
    where
        T: Component + Float + SubAssign,
        S: RgbSpace,
{
    fn sub_assign(&mut self, other: Hwb<S, T>) {
        self.hue -= other.hue;
        self.whiteness -= other.whiteness;
        self.blackness -= other.blackness;
    }
}

impl<S, T> SubAssign<T> for Hwb<S, T>
    where
        T: Component + Float + SubAssign,
        S: RgbSpace,
{
    fn sub_assign(&mut self, c: T) {
        self.hue -= c;
        self.whiteness -= c;
        self.blackness -= c;
    }
}

impl<S, T, P> AsRef<P> for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<S, T, P> AsMut<P> for Hwb<S, T>
where
    T: Component + Float,
    S: RgbSpace,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<S, T> AbsDiffEq for Hwb<S, T>
where
    T: Component + Float + AbsDiffEq,
    T::Epsilon: Copy + Float,
    S: RgbSpace + PartialEq,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let equal_shade = self
            .whiteness
            .abs_diff_eq(&other.whiteness, epsilon)
            && self
            .blackness
            .abs_diff_eq(&other.blackness, epsilon);

        // The hue doesn't matter that much when the color is gray, and may fluctuate
        // due to precision errors. This is a blunt tool, but works for now.
        let is_gray = self.blackness + self.whiteness >= T::one()
            || other.blackness + other.whiteness >= T::one();
        if is_gray {
            equal_shade
        } else {
            self.hue.abs_diff_eq(&other.hue, epsilon) && equal_shade
        }
    }
}

impl<S, T> RelativeEq for Hwb<S, T>
where
    T: Component + Float + RelativeEq,
    T::Epsilon: Copy + Float,
    S: RgbSpace + PartialEq,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        let equal_shade = self
            .whiteness
            .relative_eq(&other.whiteness, epsilon, max_relative)
            && self
            .blackness
            .relative_eq(&other.blackness, epsilon, max_relative);

        // The hue doesn't matter that much when the color is gray, and may fluctuate
        // due to precision errors. This is a blunt tool, but works for now.
        let is_gray = self.blackness + self.whiteness >= T::one()
            || other.blackness + other.whiteness >= T::one();
        if is_gray {
            equal_shade
        } else {
            self.hue.relative_eq(&other.hue, epsilon, max_relative) && equal_shade
        }
    }
}

impl<S, T> UlpsEq for Hwb<S, T>
where
    T: Component + Float + UlpsEq,
    T::Epsilon: Copy + Float,
    S: RgbSpace + PartialEq,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let equal_shade = self
            .whiteness
            .ulps_eq(&other.whiteness, epsilon, max_ulps)
            && self
            .blackness
            .ulps_eq(&other.blackness, epsilon, max_ulps);

        // The hue doesn't matter that much when the color is gray, and may fluctuate
        // due to precision errors. This is a blunt tool, but works for now.
        let is_gray = self.blackness + self.whiteness >= T::one()
            || other.blackness + other.whiteness >= T::one();
        if is_gray {
            equal_shade
        } else {
            self.hue.ulps_eq(&other.hue, epsilon, max_ulps) && equal_shade
        }
    }
}

#[cfg(test)]
mod test {
    use super::Hwb;
    use encoding::Srgb;
    use {Limited, LinSrgb};

    #[test]
    fn red() {
        let a = Hwb::from(LinSrgb::new(1.0, 0.0, 0.0));
        let b = Hwb::new(0.0, 0.0, 0.0);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn orange() {
        let a = Hwb::from(LinSrgb::new(1.0, 0.5, 0.0));
        let b = Hwb::new(30.0, 0.0, 0.0);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn green() {
        let a = Hwb::from(LinSrgb::new(0.0, 1.0, 0.0));
        let b = Hwb::new(120.0, 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn blue() {
        let a = Hwb::from(LinSrgb::new(0.0, 0.0, 1.0));
        let b = Hwb::new(240.0, 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn purple() {
        let a = Hwb::from(LinSrgb::new(0.5, 0.0, 1.0));
        let b = Hwb::new(270.0, 0.0, 0.0);
        assert_relative_eq!(a, b, epsilon = 0.000001);
    }

    #[test]
    fn clamp_invalid() {
        let expected = Hwb::new(240.0, 0.0, 0.0);

        let a = Hwb::new(240.0, -3.0, -4.0);
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);
    }

    #[test]
    fn clamp_none() {
        let expected = Hwb::new(240.0, 0.3, 0.7);

        let a = Hwb::new(240.0, 0.3, 0.7);
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);
    }
    #[test]
    fn clamp_over_one() {
        let expected = Hwb::new(240.0, 0.2, 0.8);

        let a = Hwb::new(240.0, 5.0, 20.0);
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);
    }
    #[test]
    fn clamp_under_one() {
        let expected = Hwb::new(240.0, 0.3, 0.1);

        let a = Hwb::new(240.0, 0.3, 0.1);
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);
    }

    raw_pixel_conversion_tests!(Hwb<Srgb>: hue, whiteness, blackness);
    raw_pixel_conversion_fail_tests!(Hwb<Srgb>: hue, whiteness, blackness);

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Hwb::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"hue":0.3,"whiteness":0.8,"blackness":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Hwb =
            ::serde_json::from_str(r#"{"hue":0.3,"whiteness":0.8,"blackness":0.1}"#).unwrap();

        assert_eq!(deserialized, Hwb::new(0.3, 0.8, 0.1));
    }
}
