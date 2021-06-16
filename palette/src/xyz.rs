use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::convert::{FromColorUnclamped, IntoColorUnclamped};
use crate::encoding::pixel::RawPixel;
use crate::luma::LumaStandard;
use crate::matrix::{multiply_rgb_to_xyz, multiply_xyz, rgb_to_xyz_matrix};
use crate::rgb::{Rgb, RgbSpace, RgbStandard};
use crate::white_point::{WhitePoint, D65};
use crate::{
    clamp, contrast_ratio, from_f64, oklab, Alpha, Clamp, Component, ComponentWise, FloatComponent,
    Lab, Luma, Luv, Mix, Oklab, Oklch, Pixel, RelativeContrast, Shade, Yxy,
};

/// CIE 1931 XYZ with an alpha component. See the [`Xyza` implementation in
/// `Alpha`](crate::Alpha#Xyza).
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
#[derive(Debug, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Xyz, Yxy, Luv, Rgb, Lab, Oklab, Oklch, Luma)
)]
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
    /// default D65.
    pub z: T,

    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
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
    /// CIE XYZ with white point D65.
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

    /// Return the `x` value minimum.
    pub fn min_x() -> T {
        T::zero()
    }

    /// Return the `x` value maximum.
    pub fn max_x() -> T {
        let xyz_ref: Xyz<Wp, _> = Wp::get_xyz();
        xyz_ref.x
    }

    /// Return the `y` value minimum.
    pub fn min_y() -> T {
        T::zero()
    }

    /// Return the `y` value maximum.
    pub fn max_y() -> T {
        let xyz_ref: Xyz<Wp, _> = Wp::get_xyz();
        xyz_ref.y
    }

    /// Return the `z` value minimum.
    pub fn min_z() -> T {
        T::zero()
    }

    /// Return the `z` value maximum.
    pub fn max_z() -> T {
        let xyz_ref: Xyz<Wp, _> = Wp::get_xyz();
        xyz_ref.z
    }
}

impl<Wp, T> PartialEq for Xyz<Wp, T>
where
    T: FloatComponent + PartialEq,
    Wp: WhitePoint,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<Wp, T> Eq for Xyz<Wp, T>
where
    T: FloatComponent + Eq,
    Wp: WhitePoint,
{
}

///<span id="Xyza"></span>[`Xyza`](crate::Xyza) implementations.
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

///<span id="Xyza"></span>[`Xyza`](crate::Xyza) implementations.
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

impl<Wp, T> FromColorUnclamped<Xyz<Wp, T>> for Xyz<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Xyz<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T, S> FromColorUnclamped<Rgb<S, T>> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    S: RgbStandard,
    S::Space: RgbSpace<WhitePoint = Wp>,
{
    fn from_color_unclamped(color: Rgb<S, T>) -> Self {
        let transform_matrix = rgb_to_xyz_matrix::<S::Space, T>();
        multiply_rgb_to_xyz(&transform_matrix, &color.into_linear())
    }
}

impl<Wp, T> FromColorUnclamped<Yxy<Wp, T>> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn from_color_unclamped(color: Yxy<Wp, T>) -> Self {
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

impl<Wp, T> FromColorUnclamped<Lab<Wp, T>> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn from_color_unclamped(color: Lab<Wp, T>) -> Self {
        // Recip call shows performance benefits in benchmarks for this function
        let y = (color.l + from_f64(16.0)) * from_f64::<T>(116.0).recip();
        let x = y + (color.a * from_f64::<T>(500.0).recip());
        let z = y - (color.b * from_f64::<T>(200.0).recip());

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

impl<Wp, T> FromColorUnclamped<Luv<Wp, T>> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn from_color_unclamped(color: Luv<Wp, T>) -> Self {
        let from_f64 = T::from_f64;

        let kappa: T = from_f64(29.0 / 3.0).powi(3);

        let w: Xyz<Wp, T> = Wp::get_xyz();
        let ref_denom_recip = (w.x + from_f64(15.0) * w.y + from_f64(3.0) * w.z).recip();
        let u_ref = from_f64(4.0) * w.x * ref_denom_recip;
        let v_ref = from_f64(9.0) * w.y * ref_denom_recip;

        if color.l < from_f64(1e-5) {
            return Xyz::with_wp(T::zero(), T::zero(), T::zero());
        }

        let y = if color.l > from_f64(8.0) {
            ((color.l + from_f64(16.0)) * from_f64(116.0).recip()).powi(3)
        } else {
            color.l * kappa.recip()
        } * w.y;

        let u_prime = color.u / (from_f64(13.0) * color.l) + u_ref;
        let v_prime = color.v / (from_f64(13.0) * color.l) + v_ref;

        let x = y * from_f64(2.25) * u_prime / v_prime;
        let z = y * (from_f64(3.0) - from_f64(0.75) * u_prime - from_f64(5.0) * v_prime) / v_prime;
        Xyz::with_wp(x, y, z)
    }
}

impl<T> FromColorUnclamped<Oklab<T>> for Xyz<D65, T>
where
    T: FloatComponent,
{
    fn from_color_unclamped(color: Oklab<T>) -> Self {
        let m1_inv = oklab::m1_inv();
        let m2_inv = oklab::m2_inv();

        let Xyz {
            x: l_,
            y: m_,
            z: s_,
            ..
        } = multiply_xyz::<_, D65, _>(&m2_inv, &Xyz::new(color.l, color.a, color.b));

        let lms = Xyz::new(l_.powi(3), m_.powi(3), s_.powi(3));
        let Xyz { x, y, z, .. } = multiply_xyz::<_, D65, _>(&m1_inv, &lms);

        Self::with_wp(x, y, z)
    }
}

impl<T> FromColorUnclamped<Oklch<T>> for Xyz<D65, T>
where
    T: FloatComponent,
{
    fn from_color_unclamped(color: Oklch<T>) -> Self {
        let oklab: Oklab<T> = color.into_color_unclamped();
        Self::from_color_unclamped(oklab)
    }
}

impl<Wp, T, S> FromColorUnclamped<Luma<S, T>> for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    S: LumaStandard<WhitePoint = Wp>,
{
    fn from_color_unclamped(color: Luma<S, T>) -> Self {
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

impl<Wp, T> Clamp for Xyz<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    #[rustfmt::skip]
    fn is_within_bounds(&self) -> bool {
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

    fn lighten(&self, factor: T) -> Xyz<Wp, T> {
        let difference = if factor >= T::zero() {
            T::max_intensity() - self.y
        } else {
            self.y
        };

        let delta = difference.max(T::zero()) * factor;

        Xyz {
            x: self.x,
            y: (self.y + delta).max(T::zero()),
            z: self.z,
            white_point: PhantomData,
        }
    }

    fn lighten_fixed(&self, amount: T) -> Xyz<Wp, T> {
        Xyz {
            x: self.x,
            y: (self.y + T::max_intensity() * amount).max(T::zero()),
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

#[cfg(feature = "random")]
impl<Wp, T> Distribution<Xyz<Wp, T>> for Standard
where
    T: FloatComponent,
    Wp: WhitePoint,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Xyz<Wp, T> {
        let xyz_ref: Xyz<Wp, T> = Wp::get_xyz();
        Xyz {
            x: rng.gen() * xyz_ref.x,
            y: rng.gen() * xyz_ref.y,
            z: rng.gen() * xyz_ref.z,
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformXyz<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    x: Uniform<T>,
    y: Uniform<T>,
    z: Uniform<T>,
    white_point: PhantomData<Wp>,
}

#[cfg(feature = "random")]
impl<Wp, T> SampleUniform for Xyz<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    type Sampler = UniformXyz<Wp, T>;
}

#[cfg(feature = "random")]
impl<Wp, T> UniformSampler for UniformXyz<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    type X = Xyz<Wp, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        UniformXyz {
            x: Uniform::new::<_, T>(low.x, high.x),
            y: Uniform::new::<_, T>(low.y, high.y),
            z: Uniform::new::<_, T>(low.z, high.z),
            white_point: PhantomData,
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        UniformXyz {
            x: Uniform::new_inclusive::<_, T>(low.x, high.x),
            y: Uniform::new_inclusive::<_, T>(low.y, high.y),
            z: Uniform::new_inclusive::<_, T>(low.z, high.z),
            white_point: PhantomData,
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Xyz<Wp, T> {
        Xyz {
            x: self.x.sample(rng),
            y: self.y.sample(rng),
            z: self.z.sample(rng),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Zeroable for Xyz<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent + bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Pod for Xyz<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent + bytemuck::Pod,
{
}

#[cfg(test)]
mod test {
    use super::Xyz;
    use crate::white_point::D65;
    use crate::{FromColor, LinLuma, LinSrgb};

    #[cfg(feature = "random")]
    use crate::white_point::WhitePoint;

    const X_N: f64 = 0.95047;
    const Y_N: f64 = 1.0;
    const Z_N: f64 = 1.08883;

    #[test]
    fn luma() {
        let a = Xyz::from_color(LinLuma::new(0.5));
        let b = Xyz::new(0.475235, 0.5, 0.544415);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn red() {
        let a = Xyz::from_color(LinSrgb::new(1.0, 0.0, 0.0));
        let b = Xyz::new(0.41240, 0.21260, 0.01930);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn green() {
        let a = Xyz::from_color(LinSrgb::new(0.0, 1.0, 0.0));
        let b = Xyz::new(0.35760, 0.71520, 0.11920);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn blue() {
        let a = Xyz::from_color(LinSrgb::new(0.0, 0.0, 1.0));
        let b = Xyz::new(0.18050, 0.07220, 0.95030);
        assert_relative_eq!(a, b, epsilon = 0.0001);
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Xyz<D65, f64>;
            clamped {
                x: 0.0 => X_N,
                y: 0.0 => Y_N,
                z: 0.0 => Z_N
            }
            clamped_min {}
            unclamped {}
        }
    }

    raw_pixel_conversion_tests!(Xyz<D65>: x, y, z);
    raw_pixel_conversion_fail_tests!(Xyz<D65>: x, y, z);

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Xyz::<D65>::min_x(), 0.0);
        assert_relative_eq!(Xyz::<D65>::min_y(), 0.0);
        assert_relative_eq!(Xyz::<D65>::min_z(), 0.0);
        assert_relative_eq!(Xyz::<D65, f64>::max_x(), X_N);
        assert_relative_eq!(Xyz::<D65, f64>::max_y(), Y_N);
        assert_relative_eq!(Xyz::<D65, f64>::max_z(), Z_N);
    }

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

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Xyz<D65, f32> {
            x: (0.0, D65::get_xyz::<D65, f32>().x),
            y: (0.0, D65::get_xyz::<D65, f32>().y),
            z: (0.0, D65::get_xyz::<D65, f32>().z)
        },
        min: Xyz::new(0.0f32, 0.0, 0.0),
        max: D65::get_xyz::<D65, f32>()
    }
}
