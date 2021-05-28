use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::color_difference::ColorDifference;
use crate::color_difference::{get_ciede_difference, LabColorDiff};
use crate::convert::FromColorUnclamped;
use crate::encoding::pixel::RawPixel;
use crate::white_point::{WhitePoint, D65};
use crate::{
    clamp, contrast_ratio, from_f64, Alpha, Clamp, Component, ComponentWise, FloatComponent,
    GetHue, LabHue, Lch, Mix, Pixel, RelativeContrast, Shade, Xyz,
};

/// CIE L\*a\*b\* (CIELAB) with an alpha component. See the [`Laba`
/// implementation in `Alpha`](crate::Alpha#Laba).
pub type Laba<Wp = D65, T = f32> = Alpha<Lab<Wp, T>, T>;

/// The CIE L\*a\*b\* (CIELAB) color space.
///
/// CIE L\*a\*b\* is a device independent color space which includes all
/// perceivable colors. It's sometimes used to convert between other color
/// spaces, because of its ability to represent all of their colors, and
/// sometimes in color manipulation, because of its perceptual uniformity. This
/// means that the perceptual difference between two colors is equal to their
/// numerical difference.
///
/// The parameters of L\*a\*b\* are quite different, compared to many other
/// color spaces, so manipulating them manually may be unintuitive.
#[derive(Debug, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Xyz, Lab, Lch)
)]
#[repr(C)]
pub struct Lab<Wp = D65, T = f32>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    /// L\* is the lightness of the color. 0.0 gives absolute black and 100
    /// give the brightest white.
    pub l: T,

    /// a\* goes from red at -128 to green at 127.
    pub a: T,

    /// b\* goes from yellow at -128 to blue at 127.
    pub b: T,

    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Lab<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
}

impl<Wp, T> Clone for Lab<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn clone(&self) -> Lab<Wp, T> {
        *self
    }
}

impl<T> Lab<D65, T>
where
    T: FloatComponent,
{
    /// CIE L\*a\*b\* with white point D65.
    pub fn new(l: T, a: T, b: T) -> Lab<D65, T> {
        Lab {
            l,
            a,
            b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Lab<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    /// CIE L\*a\*b\*.
    pub fn with_wp(l: T, a: T, b: T) -> Lab<Wp, T> {
        Lab {
            l,
            a,
            b,
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

    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::zero()
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        from_f64(100.0)
    }

    /// Return the `a` value minimum.
    pub fn min_a() -> T {
        from_f64(-128.0)
    }

    /// Return the `a` value maximum.
    pub fn max_a() -> T {
        from_f64(127.0)
    }

    /// Return the `b` value minimum.
    pub fn min_b() -> T {
        from_f64(-128.0)
    }

    /// Return the `b` value maximum.
    pub fn max_b() -> T {
        from_f64(127.0)
    }
}

impl<Wp, T> PartialEq for Lab<Wp, T>
where
    T: FloatComponent + PartialEq,
    Wp: WhitePoint,
{
    fn eq(&self, other: &Self) -> bool {
        self.l == other.l && self.a == other.a && self.b == other.b
    }
}

impl<Wp, T> Eq for Lab<Wp, T>
where
    T: FloatComponent + Eq,
    Wp: WhitePoint,
{
}

///<span id="Laba"></span>[`Laba`](crate::Laba) implementations.
impl<T, A> Alpha<Lab<D65, T>, A>
where
    T: FloatComponent,
    A: Component,
{
    /// CIE L\*a\*b\* and transparency and white point D65.
    pub fn new(l: T, a: T, b: T, alpha: A) -> Self {
        Alpha {
            color: Lab::new(l, a, b),
            alpha,
        }
    }
}

///<span id="Laba"></span>[`Laba`](crate::Laba) implementations.
impl<Wp, T, A> Alpha<Lab<Wp, T>, A>
where
    T: FloatComponent,
    A: Component,
    Wp: WhitePoint,
{
    /// CIE L\*a\*b\* and transparency.
    pub fn with_wp(l: T, a: T, b: T, alpha: A) -> Self {
        Alpha {
            color: Lab::with_wp(l, a, b),
            alpha,
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

impl<Wp, T> FromColorUnclamped<Lab<Wp, T>> for Lab<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Lab<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T> FromColorUnclamped<Xyz<Wp, T>> for Lab<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Xyz<Wp, T>) -> Self {
        let Xyz {
            mut x,
            mut y,
            mut z,
            ..
        } = color / Wp::get_xyz();

        fn convert<T: FloatComponent>(c: T) -> T {
            let epsilon = from_f64::<T>(6.0 / 29.0).powi(3);
            let kappa: T = from_f64(841.0 / 108.0);
            let delta: T = from_f64(4.0 / 29.0);
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
            l: ((y * from_f64(116.0)) - from_f64(16.0)),
            a: ((x - y) * from_f64(500.0)),
            b: ((y - z) * from_f64(200.0)),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> FromColorUnclamped<Lch<Wp, T>> for Lab<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Lch<Wp, T>) -> Self {
        Lab {
            l: color.l,
            a: color.chroma.max(T::zero()) * color.hue.to_radians().cos(),
            b: color.chroma.max(T::zero()) * color.hue.to_radians().sin(),
            white_point: PhantomData,
        }
    }
}

impl<Wp: WhitePoint, T: FloatComponent> From<(T, T, T)> for Lab<Wp, T> {
    fn from(components: (T, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent> Into<(T, T, T)> for Lab<Wp, T> {
    fn into(self) -> (T, T, T) {
        self.into_components()
    }
}

impl<Wp: WhitePoint, T: FloatComponent, A: Component> From<(T, T, T, A)> for Alpha<Lab<Wp, T>, A> {
    fn from(components: (T, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent, A: Component> Into<(T, T, T, A)> for Alpha<Lab<Wp, T>, A> {
    fn into(self) -> (T, T, T, A) {
        self.into_components()
    }
}

impl<Wp, T> Clamp for Lab<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    #[rustfmt::skip]
    fn is_within_bounds(&self) -> bool {
        self.l >= T::zero() && self.l <= from_f64(100.0) &&
        self.a >= from_f64(-128.0) && self.a <= from_f64(127.0) &&
        self.b >= from_f64(-128.0) && self.b <= from_f64(127.0)
    }

    fn clamp(&self) -> Lab<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, T::zero(), from_f64(100.0));
        self.a = clamp(self.a, from_f64(-128.0), from_f64(127.0));
        self.b = clamp(self.b, from_f64(-128.0), from_f64(127.0));
    }
}

impl<Wp, T> Mix for Lab<Wp, T>
where
    T: FloatComponent,
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
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn lighten(&self, factor: T) -> Lab<Wp, T> {
        let difference = if factor >= T::zero() {
            T::from_f64(100.0) - self.l
        } else {
            self.l
        };

        let delta = difference.max(T::zero()) * factor;

        Lab {
            l: (self.l + delta).max(T::zero()),
            a: self.a,
            b: self.b,
            white_point: PhantomData,
        }
    }

    fn lighten_fixed(&self, amount: T) -> Lab<Wp, T> {
        Lab {
            l: (self.l + T::from_f64(100.0) * amount).max(T::zero()),
            a: self.a,
            b: self.b,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> GetHue for Lab<Wp, T>
where
    T: FloatComponent,
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

impl<Wp, T> ColorDifference for Lab<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn get_color_difference(&self, other: &Lab<Wp, T>) -> Self::Scalar {
        // Color difference calculation requires Lab and chroma components. This
        // function handles the conversion into those components which are then
        // passed to `get_ciede_difference()` where calculation is completed.
        let self_params = LabColorDiff {
            l: self.l,
            a: self.a,
            b: self.b,
            chroma: (self.a * self.a + self.b * self.b).sqrt(),
        };
        let other_params = LabColorDiff {
            l: other.l,
            a: other.a,
            b: other.b,
            chroma: (other.a * other.a + other.b * other.b).sqrt(),
        };

        get_ciede_difference(&self_params, &other_params)
    }
}

impl<Wp, T> ComponentWise for Lab<Wp, T>
where
    T: FloatComponent,
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
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn default() -> Lab<Wp, T> {
        Lab::with_wp(T::zero(), T::zero(), T::zero())
    }
}

impl<Wp, T> Add<Lab<Wp, T>> for Lab<Wp, T>
where
    T: FloatComponent,
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
    T: FloatComponent,
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
    T: FloatComponent + AddAssign,
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
    T: FloatComponent + AddAssign,
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
    T: FloatComponent,
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
    T: FloatComponent,
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
    T: FloatComponent + SubAssign,
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
    T: FloatComponent + SubAssign,
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
    T: FloatComponent,
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
    T: FloatComponent,
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
    T: FloatComponent + MulAssign,
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
    T: FloatComponent + MulAssign,
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
    T: FloatComponent,
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
    T: FloatComponent,
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
    T: FloatComponent + DivAssign,
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
    T: FloatComponent + DivAssign,
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
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<Wp, T, P> AsMut<P> for Lab<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<Wp, T> RelativeContrast for Lab<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    type Scalar = T;

    fn get_contrast_ratio(&self, other: &Self) -> T {
        use crate::FromColor;

        let xyz1 = Xyz::from_color(*self);
        let xyz2 = Xyz::from_color(*other);

        contrast_ratio(xyz1.y, xyz2.y)
    }
}

#[cfg(feature = "random")]
impl<Wp, T> Distribution<Lab<Wp, T>> for Standard
where
    T: FloatComponent,
    Wp: WhitePoint,
    Standard: Distribution<T>,
{
    // `a` and `b` both range from (-128.0, 127.0)
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Lab<Wp, T> {
        Lab {
            l: rng.gen() * from_f64(100.0),
            a: rng.gen() * from_f64(255.0) - from_f64(128.0),
            b: rng.gen() * from_f64(255.0) - from_f64(128.0),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformLab<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    l: Uniform<T>,
    a: Uniform<T>,
    b: Uniform<T>,
    white_point: PhantomData<Wp>,
}

#[cfg(feature = "random")]
impl<Wp, T> SampleUniform for Lab<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    type Sampler = UniformLab<Wp, T>;
}

#[cfg(feature = "random")]
impl<Wp, T> UniformSampler for UniformLab<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    type X = Lab<Wp, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();

        UniformLab {
            l: Uniform::new::<_, T>(low.l, high.l),
            a: Uniform::new::<_, T>(low.a, high.a),
            b: Uniform::new::<_, T>(low.b, high.b),
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

        UniformLab {
            l: Uniform::new_inclusive::<_, T>(low.l, high.l),
            a: Uniform::new_inclusive::<_, T>(low.a, high.a),
            b: Uniform::new_inclusive::<_, T>(low.b, high.b),
            white_point: PhantomData,
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Lab<Wp, T> {
        Lab {
            l: self.l.sample(rng),
            a: self.a.sample(rng),
            b: self.b.sample(rng),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Zeroable for Lab<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent + bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Pod for Lab<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent + bytemuck::Pod,
{
}

#[cfg(test)]
mod test {
    use super::Lab;
    use crate::white_point::D65;
    use crate::{FromColor, LinSrgb};

    #[test]
    fn red() {
        let a = Lab::from_color(LinSrgb::new(1.0, 0.0, 0.0));
        let b = Lab::new(53.23288, 80.09246, 67.2031);
        assert_relative_eq!(a, b, epsilon = 0.01);
    }

    #[test]
    fn green() {
        let a = Lab::from_color(LinSrgb::new(0.0, 1.0, 0.0));
        let b = Lab::new(87.73704, -86.184654, 83.18117);
        assert_relative_eq!(a, b, epsilon = 0.01);
    }

    #[test]
    fn blue() {
        let a = Lab::from_color(LinSrgb::new(0.0, 0.0, 1.0));
        let b = Lab::new(32.302586, 79.19668, -107.863686);
        assert_relative_eq!(a, b, epsilon = 0.01);
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Lab<D65, f64>;
            clamped {
                l: 0.0 => 100.0,
                a: -128.0 => 127.0,
                b: -128.0 => 127.0
            }
            clamped_min {}
            unclamped {}
        }
    }

    raw_pixel_conversion_tests!(Lab<D65>: l, a, b);
    raw_pixel_conversion_fail_tests!(Lab<D65>: l, a, b);

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Lab::<D65, f32>::min_l(), 0.0);
        assert_relative_eq!(Lab::<D65, f32>::min_a(), -128.0);
        assert_relative_eq!(Lab::<D65, f32>::min_b(), -128.0);
        assert_relative_eq!(Lab::<D65, f32>::max_l(), 100.0);
        assert_relative_eq!(Lab::<D65, f32>::max_a(), 127.0);
        assert_relative_eq!(Lab::<D65, f32>::max_b(), 127.0);
    }

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

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Lab<D65, f32> {
            l: (0.0, 100.0),
            a: (-128.0, 127.0),
            b: (-128.0, 127.0)
        },
        min: Lab::new(0.0f32, -128.0, -128.0),
        max: Lab::new(100.0, 127.0, 127.0)
    }
}
