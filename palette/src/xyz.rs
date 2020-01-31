use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::encoding::pixel::RawPixel;
use crate::luma::LumaStandard;
use crate::matrix::{multiply_rgb_to_xyz, rgb_to_xyz_matrix};
use crate::rgb::{Rgb, RgbSpace, RgbStandard};
use crate::white_point::{WhitePoint, D65};
use crate::{clamp, contrast_ratio, from_f64};
use crate::{Alpha, Lab, Luma, Yxy};
use crate::{
    Component, ComponentWise, FloatComponent, Limited, Mix, Pixel, RelativeContrast, Shade,
};

/// CIE 1931 XYZ with an alpha component. See the [`Xyza` implementation in
/// `Alpha`](struct.Alpha.html#Xyza).
pub type Xyza<Wp = D65, T = f32> = Alpha<Xyz<Wp, T>, T>;

/// The CIE 1931 XYZ color space.
///
/// XYZ links the perceived colors to their wavelengths and simply makes it
/// possible to describe the way we see colors as numbers. It's often used when
/// converting from one color space to an other, and requires a standard
/// illuminant and a standard observer to be defined.
///
/// Conversions and operations on this color space depend on the defined white
/// point
#[derive(Debug, PartialEq, FromColor, Pixel)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette_internal]
#[palette_white_point = "Wp"]
#[palette_component = "T"]
#[palette_manual_from(Xyz, Rgb, Lab, Yxy, Luma)]
#[repr(C)]
pub struct Xyz<Wp = D65, T = f32>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    /// X is the scale of what can be seen as a response curve for the cone
    /// cells in the human eye. Its range depends
    /// on the white point and goes from 0.0 to 0.95047 for the default D65.
    pub x: T,

    /// Y is the luminance of the color, where 0.0 is black and 1.0 is white.
    pub y: T,

    /// Z is the scale of what can be seen as the blue stimulation. Its range
    /// depends on the white point and goes from 0.0 to 1.08883 for the
    /// defautl D65.
    pub z: T,

    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette_unsafe_zero_sized]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
}

impl<Wp, T> Clone for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn clone(&self) -> Xyz<Wp, T> {
        *self
    }
}

impl<T> Xyz<D65, T>
where
    T: FloatComponent,
{
    /// CIE XYZ with whtie point D65.
    pub fn new(x: T, y: T, z: T) -> Xyz<D65, T> {
        Xyz {
            x,
            y,
            z,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    /// CIE XYZ.
    pub fn with_wp(x: T, y: T, z: T) -> Xyz<Wp, T> {
        Xyz {
            x,
            y,
            z,
            white_point: PhantomData,
        }
    }

    /// Convert to a `(X, Y, Z)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    /// Convert from a `(X, Y, Z)` tuple.
    pub fn from_components((x, y, z): (T, T, T)) -> Self {
        Self::with_wp(x, y, z)
    }
}

///<span id="Xyza"></span>[`Xyza`](type.Xyza.html) implementations.
impl<T, A> Alpha<Xyz<D65, T>, A>
where
    T: FloatComponent,
    A: Component,
{
    /// CIE Yxy and transparency with white point D65.
    pub fn new(x: T, y: T, luma: T, alpha: A) -> Self {
        Alpha {
            color: Xyz::new(x, y, luma),
            alpha,
        }
    }
}

///<span id="Xyza"></span>[`Xyza`](type.Xyza.html) implementations.
impl<Wp, T, A> Alpha<Xyz<Wp, T>, A>
where
    T: FloatComponent,
    A: Component,
    Wp: WhitePoint,
{
    /// CIE XYZ and transparency.
    pub fn with_wp(x: T, y: T, z: T, alpha: A) -> Self {
        Alpha {
            color: Xyz::with_wp(x, y, z),
            alpha,
        }
    }

    /// Convert to a `(X, Y, Z, alpha)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (self.x, self.y, self.z, self.alpha)
    }

    /// Convert from a `(X, Y, Z, alpha)` tuple.
    pub fn from_components((x, y, z, alpha): (T, T, T, A)) -> Self {
        Self::with_wp(x, y, z, alpha)
    }
}

impl<Wp, T, S> From<Rgb<S, T>> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    S: RgbStandard,
    S::Space: RgbSpace<WhitePoint = Wp>,
{
    fn from(color: Rgb<S, T>) -> Self {
        let transform_matrix = rgb_to_xyz_matrix::<S::Space, T>();
        multiply_rgb_to_xyz(&transform_matrix, &color.into_linear())
    }
}

impl<Wp, T> From<Yxy<Wp, T>> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn from(color: Yxy<Wp, T>) -> Self {
        let mut xyz = Xyz {
            y: color.luma,
            ..Default::default()
        };
        // If denominator is zero, NAN or INFINITE leave x and z at the default 0
        if color.y.is_normal() {
            xyz.x = color.luma * color.x / color.y;
            xyz.z = color.luma * (T::one() - color.x - color.y) / color.y;
        }
        xyz
    }
}

impl<Wp, T> From<Lab<Wp, T>> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn from(color: Lab<Wp, T>) -> Self {
        let y = (color.l + from_f64(16.0)) / from_f64(116.0);
        let x = y + (color.a / from_f64(500.0));
        let z = y - (color.b / from_f64(200.0));

        fn convert<T: FloatComponent>(c: T) -> T {
            let epsilon: T = from_f64(6.0 / 29.0);
            let kappa: T = from_f64(108.0 / 841.0);
            let delta: T = from_f64(4.0 / 29.0);

            if c > epsilon {
                c.powi(3)
            } else {
                (c - delta) * kappa
            }
        }

        Xyz::with_wp(convert(x), convert(y), convert(z)) * Wp::get_xyz()
    }
}

impl<Wp, T, S> From<Luma<S, T>> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    S: LumaStandard<WhitePoint = Wp>,
{
    fn from(color: Luma<S, T>) -> Self {
        Wp::get_xyz() * color.luma
    }
}

impl<Wp: WhitePoint, T: FloatComponent> From<(T, T, T)> for Xyz<Wp, T> {
    fn from(components: (T, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent> Into<(T, T, T)> for Xyz<Wp, T> {
    fn into(self) -> (T, T, T) {
        self.into_components()
    }
}

impl<Wp: WhitePoint, T: FloatComponent, A: Component> From<(T, T, T, A)> for Alpha<Xyz<Wp, T>, A> {
    fn from(components: (T, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent, A: Component> Into<(T, T, T, A)> for Alpha<Xyz<Wp, T>, A> {
    fn into(self) -> (T, T, T, A) {
        self.into_components()
    }
}

impl<Wp, T> Limited for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    #[rustfmt::skip]
    fn is_valid(&self) -> bool {
        let xyz_ref: Self = Wp::get_xyz();
        self.x >= T::zero() && self.x <= xyz_ref.x &&
        self.y >= T::zero() && self.y <= xyz_ref.y &&
        self.z >= T::zero() && self.z <= xyz_ref.z
    }

    fn clamp(&self) -> Xyz<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        let xyz_ref: Self = Wp::get_xyz();
        self.x = clamp(self.x, T::zero(), xyz_ref.x);
        self.y = clamp(self.y, T::zero(), xyz_ref.y);
        self.z = clamp(self.z, T::zero(), xyz_ref.z);
    }
}

impl<Wp, T> Mix for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn mix(&self, other: &Xyz<Wp, T>, factor: T) -> Xyz<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());

        Xyz {
            x: self.x + factor * (other.x - self.x),
            y: self.y + factor * (other.y - self.y),
            z: self.z + factor * (other.z - self.z),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Xyz<Wp, T> {
        Xyz {
            x: self.x,
            y: self.y + amount,
            z: self.z,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> ComponentWise for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Xyz<Wp, T>, mut f: F) -> Xyz<Wp, T> {
        Xyz {
            x: f(self.x, other.x),
            y: f(self.y, other.y),
            z: f(self.z, other.z),
            white_point: PhantomData,
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Xyz<Wp, T> {
        Xyz {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn default() -> Xyz<Wp, T> {
        Xyz::with_wp(T::zero(), T::zero(), T::zero())
    }
}

impl<Wp, T> Add<Xyz<Wp, T>> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Xyz<Wp, T>;

    fn add(self, other: Xyz<Wp, T>) -> Self::Output {
        Xyz {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Add<T> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Xyz<Wp, T>;

    fn add(self, c: T) -> Self::Output {
        Xyz {
            x: self.x + c,
            y: self.y + c,
            z: self.z + c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> AddAssign<Xyz<Wp, T>> for Xyz<Wp, T>
where
    T: FloatComponent + AddAssign,
    Wp: WhitePoint,
{
    fn add_assign(&mut self, other: Xyz<Wp, T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<Wp, T> AddAssign<T> for Xyz<Wp, T>
where
    T: FloatComponent + AddAssign,
    Wp: WhitePoint,
{
    fn add_assign(&mut self, c: T) {
        self.x += c;
        self.y += c;
        self.z += c;
    }
}

impl<Wp, T> Sub<Xyz<Wp, T>> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Xyz<Wp, T>;

    fn sub(self, other: Xyz<Wp, T>) -> Self::Output {
        Xyz {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<T> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Xyz<Wp, T>;

    fn sub(self, c: T) -> Self::Output {
        Xyz {
            x: self.x - c,
            y: self.y - c,
            z: self.z - c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> SubAssign<Xyz<Wp, T>> for Xyz<Wp, T>
where
    T: FloatComponent + SubAssign,
    Wp: WhitePoint,
{
    fn sub_assign(&mut self, other: Xyz<Wp, T>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<Wp, T> SubAssign<T> for Xyz<Wp, T>
where
    T: FloatComponent + SubAssign,
    Wp: WhitePoint,
{
    fn sub_assign(&mut self, c: T) {
        self.x -= c;
        self.y -= c;
        self.z -= c;
    }
}

impl<Wp, T> Mul<Xyz<Wp, T>> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Xyz<Wp, T>;

    fn mul(self, other: Xyz<Wp, T>) -> Self::Output {
        Xyz {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Mul<T> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Xyz<Wp, T>;

    fn mul(self, c: T) -> Self::Output {
        Xyz {
            x: self.x * c,
            y: self.y * c,
            z: self.z * c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> MulAssign<Xyz<Wp, T>> for Xyz<Wp, T>
where
    T: FloatComponent + MulAssign,
    Wp: WhitePoint,
{
    fn mul_assign(&mut self, other: Xyz<Wp, T>) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<Wp, T> MulAssign<T> for Xyz<Wp, T>
where
    T: FloatComponent + MulAssign,
    Wp: WhitePoint,
{
    fn mul_assign(&mut self, c: T) {
        self.x *= c;
        self.y *= c;
        self.z *= c;
    }
}

impl<Wp, T> Div<Xyz<Wp, T>> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Xyz<Wp, T>;

    fn div(self, other: Xyz<Wp, T>) -> Self::Output {
        Xyz {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Div<T> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Xyz<Wp, T>;

    fn div(self, c: T) -> Self::Output {
        Xyz {
            x: self.x / c,
            y: self.y / c,
            z: self.z / c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> DivAssign<Xyz<Wp, T>> for Xyz<Wp, T>
where
    T: FloatComponent + DivAssign,
    Wp: WhitePoint,
{
    fn div_assign(&mut self, other: Xyz<Wp, T>) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl<Wp, T> DivAssign<T> for Xyz<Wp, T>
where
    T: FloatComponent + DivAssign,
    Wp: WhitePoint,
{
    fn div_assign(&mut self, c: T) {
        self.x /= c;
        self.y /= c;
        self.z /= c;
    }
}

impl<Wp, T, P> AsRef<P> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<Wp, T, P> AsMut<P> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<Wp, T> RelativeContrast for Xyz<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    type Scalar = T;

    fn get_contrast_ratio(&self, other: &Self) -> T {
        contrast_ratio(self.y, other.y)
    }
}

#[cfg(test)]
mod test {
    use super::Xyz;
    use crate::white_point::D65;
    use crate::LinLuma;
    use crate::LinSrgb;
    const X_N: f64 = 0.95047;
    const Y_N: f64 = 1.0;
    const Z_N: f64 = 1.08883;

    #[test]
    fn luma() {
        let a = Xyz::from(LinLuma::new(0.5));
        let b = Xyz::new(0.475235, 0.5, 0.544415);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn red() {
        let a = Xyz::from(LinSrgb::new(1.0, 0.0, 0.0));
        let b = Xyz::new(0.41240, 0.21260, 0.01930);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn green() {
        let a = Xyz::from(LinSrgb::new(0.0, 1.0, 0.0));
        let b = Xyz::new(0.35760, 0.71520, 0.11920);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn blue() {
        let a = Xyz::from(LinSrgb::new(0.0, 0.0, 1.0));
        let b = Xyz::new(0.18050, 0.07220, 0.95030);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Xyz<D65, f64>;
            limited {
                x: 0.0 => X_N,
                y: 0.0 => Y_N,
                z: 0.0 => Z_N
            }
            limited_min {}
            unlimited {}
        }
    }

    raw_pixel_conversion_tests!(Xyz<D65>: x, y, z);
    raw_pixel_conversion_fail_tests!(Xyz<D65>: x, y, z);

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Xyz::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"x":0.3,"y":0.8,"z":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Xyz = ::serde_json::from_str(r#"{"x":0.3,"y":0.8,"z":0.1}"#).unwrap();

        assert_eq!(deserialized, Xyz::new(0.3, 0.8, 0.1));
    }
}
