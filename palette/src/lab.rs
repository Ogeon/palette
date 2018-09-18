use float::Float;

use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use encoding::pixel::RawPixel;
use white_point::{D65, WhitePoint};
use {cast, clamp};
use {Alpha, LabHue, Lch, Xyz};
use {Component, ComponentWise, GetHue, Limited, Mix, Pixel, Shade};

/// CIE L\*a\*b\* (CIELAB) with an alpha component. See the [`Laba`
/// implementation in `Alpha`](struct.Alpha.html#Laba).
pub type Laba<Wp, T = f32> = Alpha<Lab<Wp, T>, T>;

///The CIE L\*a\*b\* (CIELAB) color space.
///
///CIE L\*a\*b\* is a device independent color space which includes all
///perceivable colors. It's sometimes used to convert between other color
///spaces, because of its ability to represent all of their colors, and
///sometimes in color manipulation, because of its perceptual uniformity. This
///means that the perceptual difference between two colors is equal to their
///numerical difference.
///
///The parameters of L\*a\*b\* are quite different, compared to many other
/// color spaces, so manipulating them manually may be unintuitive.
#[derive(Debug, PartialEq, FromColor, Pixel)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette_internal]
#[palette_white_point = "Wp"]
#[palette_component = "T"]
#[palette_manual_from(Xyz, Lab, Lch)]
#[repr(C)]
pub struct Lab<Wp = D65, T = f32>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    ///L\* is the lightness of the color. 0.0 gives absolute black and 100
    ///give the brightest white.
    pub l: T,

    ///a\* goes from red at -128 to green at 127.
    pub a: T,

    ///b\* goes from yellow at -128 to blue at 127.
    pub b: T,

    ///The white point associated with the color's illuminant and observer.
    ///D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette_unsafe_zero_sized]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
}

impl<Wp, T> Clone for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    fn clone(&self) -> Lab<Wp, T> {
        *self
    }
}

impl<T> Lab<D65, T>
where
    T: Component + Float,
{
    ///CIE L\*a\*b\* with white point D65.
    pub fn new(l: T, a: T, b: T) -> Lab<D65, T> {
        Lab {
            l: l,
            a: a,
            b: b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    ///CIE L\*a\*b\*.
    pub fn with_wp(l: T, a: T, b: T) -> Lab<Wp, T> {
        Lab {
            l: l,
            a: a,
            b: b,
            white_point: PhantomData,
        }
    }

    /// Convert to a `(L\*, a\*, b\*)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.l, self.a, self.b)
    }

    /// Convert from a `(L\*, a\*, b\*)` tuple.
    pub fn from_components((l, a, b): (T, T, T)) -> Self {
        Self::with_wp(l, a, b)
    }
}

///<span id="Laba"></span>[`Laba`](type.Laba.html) implementations.
impl<T, A> Alpha<Lab<D65, T>, A>
where
    T: Component + Float,
    A: Component,
{
    ///CIE L\*a\*b\* and transparency and white point D65.
    pub fn new(l: T, a: T, b: T, alpha: A) -> Self {
        Alpha {
            color: Lab::new(l, a, b),
            alpha: alpha,
        }
    }
}

///<span id="Laba"></span>[`Laba`](type.Laba.html) implementations.
impl<Wp, T, A> Alpha<Lab<Wp, T>, A>
where
    T: Component + Float,
    A: Component,
    Wp: WhitePoint,
{
    ///CIE L\*a\*b\* and transparency.
    pub fn with_wp(l: T, a: T, b: T, alpha: A) -> Self {
        Alpha {
            color: Lab::with_wp(l, a, b),
            alpha: alpha,
        }
    }

    /// Convert to a `(L\*, a\*, b\*, alpha)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (self.l, self.a, self.b, self.alpha)
    }

    /// Convert from a `(L\*, a\*, b\*, alpha)` tuple.
    pub fn from_components((l, a, b, alpha): (T, T, T, A)) -> Self {
        Self::with_wp(l, a, b, alpha)
    }
}

impl<Wp, T> From<Xyz<Wp, T>> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    fn from(color: Xyz<Wp, T>) -> Self {
        let Xyz {
            mut x,
            mut y,
            mut z,
            ..
        } = color / Wp::get_xyz();

        fn convert<T: Component + Float>(c: T) -> T {
            let epsilon: T = (cast::<T, _>(6.0 / 29.0)).powi(3);
            let kappa: T = cast(841.0 / 108.0);
            let delta: T = cast(4.0 / 29.0);
            if c > epsilon {
                c.cbrt()
            } else {
                (kappa * c) + delta
            }
        }

        x = convert(x);
        y = convert(y);
        z = convert(z);

        Lab {
            l: ((y * cast(116.0)) - cast(16.0)),
            a: ((x - y) * cast(500.0)),
            b: ((y - z) * cast(200.0)),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> From<Lch<Wp, T>> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    fn from(color: Lch<Wp, T>) -> Self {
        Lab {
            l: color.l,
            a: color.chroma.max(T::zero()) * color.hue.to_radians().cos(),
            b: color.chroma.max(T::zero()) * color.hue.to_radians().sin(),
            white_point: PhantomData,
        }
    }
}

impl<Wp: WhitePoint, T: Component + Float> From<(T, T, T)> for Lab<Wp, T> {
    fn from(components: (T, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: Component + Float> Into<(T, T, T)> for Lab<Wp, T> {
    fn into(self) -> (T, T, T) {
        self.into_components()
    }
}

impl<Wp: WhitePoint, T: Component + Float, A: Component> From<(T, T, T, A)>
    for Alpha<Lab<Wp, T>, A>
{
    fn from(components: (T, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: Component + Float, A: Component> Into<(T, T, T, A)>
    for Alpha<Lab<Wp, T>, A>
{
    fn into(self) -> (T, T, T, A) {
        self.into_components()
    }
}

impl<Wp, T> Limited for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn is_valid(&self) -> bool {
        self.l >= T::zero() && self.l <= cast(100.0) &&
        self.a >= cast(-128) && self.a <= cast(127.0) &&
        self.b >= cast(-128) && self.b <= cast(127.0)
    }

    fn clamp(&self) -> Lab<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, T::zero(), cast(100.0));
        self.a = clamp(self.a, cast(-128.0), cast(127.0));
        self.b = clamp(self.b, cast(-128.0), cast(127.0));
    }
}

impl<Wp, T> Mix for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn mix(&self, other: &Lab<Wp, T>, factor: T) -> Lab<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());

        Lab {
            l: self.l + factor * (other.l - self.l),
            a: self.a + factor * (other.a - self.a),
            b: self.b + factor * (other.b - self.b),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Lab<Wp, T> {
        Lab {
            l: self.l + amount * cast(100.0),
            a: self.a,
            b: self.b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> GetHue for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Hue = LabHue<T>;

    fn get_hue(&self) -> Option<LabHue<T>> {
        if self.a == T::zero() && self.b == T::zero() {
            None
        } else {
            Some(LabHue::from_radians(self.b.atan2(self.a)))
        }
    }
}

impl<Wp, T> ComponentWise for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Lab<Wp, T>, mut f: F) -> Lab<Wp, T> {
        Lab {
            l: f(self.l, other.l),
            a: f(self.a, other.a),
            b: f(self.b, other.b),
            white_point: PhantomData,
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Lab<Wp, T> {
        Lab {
            l: f(self.l),
            a: f(self.a),
            b: f(self.b),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    fn default() -> Lab<Wp, T> {
        Lab::with_wp(T::zero(), T::zero(), T::zero())
    }
}

impl<Wp, T> Add<Lab<Wp, T>> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Output = Lab<Wp, T>;

    fn add(self, other: Lab<Wp, T>) -> Self::Output {
        Lab {
            l: self.l + other.l,
            a: self.a + other.a,
            b: self.b + other.b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Add<T> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Output = Lab<Wp, T>;

    fn add(self, c: T) -> Self::Output {
        Lab {
            l: self.l + c,
            a: self.a + c,
            b: self.b + c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> AddAssign<Lab<Wp, T>> for Lab<Wp, T>
    where
        T: Component + Float + AddAssign,
        Wp: WhitePoint,
{
    fn add_assign(&mut self, other: Lab<Wp, T>) {
        self.l += other.l;
        self.a += other.a;
        self.b += other.b;
    }
}

impl<Wp, T> AddAssign<T> for Lab<Wp, T>
    where
        T: Component + Float + AddAssign,
        Wp: WhitePoint,
{
    fn add_assign(&mut self, c: T) {
        self.l += c;
        self.a += c;
        self.b += c;
    }
}

impl<Wp, T> Sub<Lab<Wp, T>> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Output = Lab<Wp, T>;

    fn sub(self, other: Lab<Wp, T>) -> Self::Output {
        Lab {
            l: self.l - other.l,
            a: self.a - other.a,
            b: self.b - other.b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<T> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Output = Lab<Wp, T>;

    fn sub(self, c: T) -> Self::Output {
        Lab {
            l: self.l - c,
            a: self.a - c,
            b: self.b - c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> SubAssign<Lab<Wp, T>> for Lab<Wp, T>
    where
        T: Component + Float + SubAssign,
        Wp: WhitePoint,
{
    fn sub_assign(&mut self, other: Lab<Wp, T>) {
         self.l -= other.l;
         self.a -= other.a;
         self.b -= other.b;
    }
}

impl<Wp, T> SubAssign<T> for Lab<Wp, T>
    where
        T: Component + Float + SubAssign,
        Wp: WhitePoint,
{
    fn sub_assign(&mut self, c: T) {
        self.l -= c;
        self.a -= c;
        self.b -= c;
    }
}

impl<Wp, T> Mul<Lab<Wp, T>> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Output = Lab<Wp, T>;

    fn mul(self, other: Lab<Wp, T>) -> Self::Output {
        Lab {
            l: self.l * other.l,
            a: self.a * other.a,
            b: self.b * other.b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Mul<T> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Output = Lab<Wp, T>;

    fn mul(self, c: T) -> Self::Output {
        Lab {
            l: self.l * c,
            a: self.a * c,
            b: self.b * c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> MulAssign<Lab<Wp, T>> for Lab<Wp, T>
    where
        T: Component + Float + MulAssign,
        Wp: WhitePoint,
{
    fn mul_assign(&mut self, other: Lab<Wp, T>) {
        self.l *= other.l;
        self.a *= other.a;
        self.b *= other.b;
    }
}

impl<Wp, T> MulAssign<T> for Lab<Wp, T>
    where
        T: Component + Float + MulAssign,
        Wp: WhitePoint,
{
    fn mul_assign(&mut self, c: T) {
        self.l *= c;
        self.a *= c;
        self.b *= c;
    }
}

impl<Wp, T> Div<Lab<Wp, T>> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Output = Lab<Wp, T>;

    fn div(self, other: Lab<Wp, T>) -> Self::Output {
        Lab {
            l: self.l / other.l,
            a: self.a / other.a,
            b: self.b / other.b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Div<T> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
{
    type Output = Lab<Wp, T>;

    fn div(self, c: T) -> Self::Output {
        Lab {
            l: self.l / c,
            a: self.a / c,
            b: self.b / c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> DivAssign<Lab<Wp, T>> for Lab<Wp, T>
    where
        T: Component + Float + DivAssign,
        Wp: WhitePoint,
{
    fn div_assign(&mut self, other: Lab<Wp, T>) {
        self.l /= other.l;
        self.a /= other.a;
        self.b /= other.b;
    }
}

impl<Wp, T> DivAssign<T> for Lab<Wp, T>
    where
        T: Component + Float + DivAssign,
        Wp: WhitePoint,
{
    fn div_assign(&mut self, c: T) {
        self.l /= c;
        self.a /= c;
        self.b /= c;
    }
}

impl<Wp, T, P> AsRef<P> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<Wp, T, P> AsMut<P> for Lab<Wp, T>
where
    T: Component + Float,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

#[cfg(test)]
mod test {
    use super::Lab;
    use white_point::D65;
    use LinSrgb;

    #[test]
    fn red() {
        let a = Lab::from(LinSrgb::new(1.0, 0.0, 0.0));
        let b = Lab::new(53.23288, 80.09246, 67.2031);
        assert_relative_eq!(a, b, epsilon = 0.01);
    }

    #[test]
    fn green() {
        let a = Lab::from(LinSrgb::new(0.0, 1.0, 0.0));
        let b = Lab::new(87.73704, -86.184654, 83.18117);
        assert_relative_eq!(a, b, epsilon = 0.01);
    }

    #[test]
    fn blue() {
        let a = Lab::from(LinSrgb::new(0.0, 0.0, 1.0));
        let b = Lab::new(32.302586, 79.19668, -107.863686);
        assert_relative_eq!(a, b, epsilon = 0.01);
    }

    #[test]
    fn ranges() {
        assert_ranges!{
            Lab<D65, f64>;
            limited {
                l: 0.0 => 100.0,
                a: -128.0 => 127.0,
                b: -128.0 => 127.0
            }
            limited_min {}
            unlimited {}
        }
    }

    raw_pixel_conversion_tests!(Lab<D65>: l, a, b);
    raw_pixel_conversion_fail_tests!(Lab<D65>: l, a, b);

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Lab::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"l":0.3,"a":0.8,"b":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Lab = ::serde_json::from_str(r#"{"l":0.3,"a":0.8,"b":0.1}"#).unwrap();

        assert_eq!(deserialized, Lab::new(0.3, 0.8, 0.1));
    }
}
