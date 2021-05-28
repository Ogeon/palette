use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(feature = "random")]
use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler};
#[cfg(feature = "random")]
use rand::distributions::{Distribution, Standard};
#[cfg(feature = "random")]
use rand::Rng;

use crate::convert::FromColorUnclamped;
use crate::encoding::pixel::RawPixel;
use crate::white_point::{WhitePoint, D65};
use crate::{
    clamp, contrast_ratio, from_f64, Alpha, Clamp, Component, ComponentWise, FloatComponent,
    GetHue, Lchuv, LuvHue, Mix, Pixel, RelativeContrast, Shade, Xyz,
};

/// CIE L\*u\*v\* (CIELUV) with an alpha component. See the [`Luva`
/// implementation in `Alpha`](crate::Alpha#Luva).
pub type Luva<Wp = D65, T = f32> = Alpha<Luv<Wp, T>, T>;

/// The CIE L\*u\*v\* (CIELUV) color space.
///
/// CIE L\*u\*v\* is a device independent color space. It is a simple
/// transformation of the CIE XYZ color space with the additional
/// property of being more perceptually uniform. In contrast to
/// CIELAB, CIELUV is also linear for a fixed lightness: additive
/// mixtures of colors (at a fixed lightness) will fall on a line in
/// CIELUV-space.
///
/// As a result, CIELUV is used more frequently for additive settings.
#[derive(Debug, Pixel, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Xyz, Luv, Lchuv)
)]
#[repr(C)]
pub struct Luv<Wp = D65, T = f32>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    /// L\* is the lightness of the color. 0.0 gives absolute black and 100
    /// give the brightest white.
    pub l: T,

    /// The range of valid u\* varies depending on the values of L\*
    /// and v\*, but at its limits u\* is within the interval (-84.0,
    /// 176.0).
    pub u: T,

    /// The range of valid v\* varies depending on the values of L\*
    /// and u\*, but at its limits v\* is within the interval (-135.0,
    /// 108.0).
    pub v: T,

    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
}

impl<Wp, T> Clone for Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn clone(&self) -> Luv<Wp, T> {
        *self
    }
}

impl<T> Luv<D65, T>
where
    T: FloatComponent,
{
    /// CIE L\*u\*v\* with white point D65.
    pub fn new(l: T, u: T, v: T) -> Luv<D65, T> {
        Luv {
            l,
            u,
            v,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    /// CIE L\*u\*v\*.
    pub fn with_wp(l: T, u: T, v: T) -> Luv<Wp, T> {
        Luv {
            l,
            u,
            v,
            white_point: PhantomData,
        }
    }

    /// Convert to a `(L\*, u\*, v\*)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.l, self.u, self.v)
    }

    /// Convert from a `(L\*, u\*, v\*)` tuple.
    pub fn from_components((l, u, v): (T, T, T)) -> Self {
        Self::with_wp(l, u, v)
    }

    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::zero()
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        from_f64(100.0)
    }

    /// Return the `u` value minimum.
    pub fn min_u() -> T {
        from_f64(-84.0)
    }

    /// Return the `u` value maximum.
    pub fn max_u() -> T {
        from_f64(176.0)
    }

    /// Return the `v` value minimum.
    pub fn min_v() -> T {
        from_f64(-135.0)
    }

    /// Return the `v` value maximum.
    pub fn max_v() -> T {
        from_f64(108.0)
    }
}

impl<Wp, T> PartialEq for Luv<Wp, T>
where
    T: FloatComponent + PartialEq,
    Wp: WhitePoint,
{
    fn eq(&self, other: &Self) -> bool {
        self.l == other.l && self.u == other.u && self.v == other.v
    }
}

impl<Wp, T> Eq for Luv<Wp, T>
where
    T: FloatComponent + Eq,
    Wp: WhitePoint,
{
}

///<span id="Luva"></span>[`Luva`](crate::Luva) implementations.
impl<T, A> Alpha<Luv<D65, T>, A>
where
    T: FloatComponent,
    A: Component,
{
    /// CIE L\*u\*v\* and transparency and white point D65.
    pub fn new(l: T, u: T, v: T, alpha: A) -> Self {
        Alpha {
            color: Luv::new(l, u, v),
            alpha,
        }
    }
}

///<span id="Luva"></span>[`Luva`](crate::Luva) implementations.
impl<Wp, T, A> Alpha<Luv<Wp, T>, A>
where
    T: FloatComponent,
    A: Component,
    Wp: WhitePoint,
{
    /// CIE L\*u\*v\* and transparency.
    pub fn with_wp(l: T, u: T, v: T, alpha: A) -> Self {
        Alpha {
            color: Luv::with_wp(l, u, v),
            alpha,
        }
    }

    /// Convert to u `(L\*, u\*, v\*, alpha)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (self.l, self.u, self.v, self.alpha)
    }

    /// Convert from u `(L\*, u\*, v\*, alpha)` tuple.
    pub fn from_components((l, u, v, alpha): (T, T, T, A)) -> Self {
        Self::with_wp(l, u, v, alpha)
    }
}

impl<Wp, T> FromColorUnclamped<Luv<Wp, T>> for Luv<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Luv<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T> FromColorUnclamped<Lchuv<Wp, T>> for Luv<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Lchuv<Wp, T>) -> Self {
        let (sin_hue, cos_hue) = color.hue.to_radians().sin_cos();
        let chroma = color.chroma.max(T::zero());
        Luv::with_wp(color.l, chroma * cos_hue, chroma * sin_hue)
    }
}

impl<Wp, T> FromColorUnclamped<Xyz<Wp, T>> for Luv<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent,
{
    fn from_color_unclamped(color: Xyz<Wp, T>) -> Self {
        let from_f64 = T::from_f64;
        let w: Xyz<Wp, T> = Wp::get_xyz();

        let kappa = from_f64(29.0 / 3.0).powi(3);
        let epsilon = from_f64(6.0 / 29.0).powi(3);

        let prime_denom = color.x + from_f64(15.0) * color.y + from_f64(3.0) * color.z;
        if prime_denom == from_f64(0.0) {
            return Luv::with_wp(T::zero(), T::zero(), T::zero());
        }
        let prime_denom_recip = prime_denom.recip();
        let prime_ref_denom_recip = (w.x + from_f64(15.0) * w.y + from_f64(3.0) * w.z).recip();

        let u_prime: T = from_f64(4.0) * color.x * prime_denom_recip;
        let u_ref_prime = from_f64(4.0) * w.x * prime_ref_denom_recip;

        let v_prime: T = from_f64(9.0) * color.y * prime_denom_recip;
        let v_ref_prime = from_f64(9.0) * w.y * prime_ref_denom_recip;

        let y_r = color.y / w.y;
        let l = if y_r > epsilon {
            from_f64(116.0) * y_r.powf(from_f64(1.0 / 3.0)) - from_f64(16.0)
        } else {
            kappa * y_r
        };

        Luv {
            l,
            u: from_f64(13.0) * l * (u_prime - u_ref_prime),
            v: from_f64(13.0) * l * (v_prime - v_ref_prime),
            white_point: PhantomData,
        }
    }
}

impl<Wp: WhitePoint, T: FloatComponent> From<(T, T, T)> for Luv<Wp, T> {
    fn from(components: (T, T, T)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent> Into<(T, T, T)> for Luv<Wp, T> {
    fn into(self) -> (T, T, T) {
        self.into_components()
    }
}

impl<Wp: WhitePoint, T: FloatComponent, A: Component> From<(T, T, T, A)> for Alpha<Luv<Wp, T>, A> {
    fn from(components: (T, T, T, A)) -> Self {
        Self::from_components(components)
    }
}

impl<Wp: WhitePoint, T: FloatComponent, A: Component> Into<(T, T, T, A)> for Alpha<Luv<Wp, T>, A> {
    fn into(self) -> (T, T, T, A) {
        self.into_components()
    }
}

impl<Wp, T> Clamp for Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    #[rustfmt::skip]
    fn is_within_bounds(&self) -> bool {
	self.l >= Self::min_l() && self.l <= Self::max_l() &&
	self.u >= Self::min_u() && self.u <= Self::max_u() &&
	self.v >= Self::min_v() && self.v <= Self::max_v()
    }

    fn clamp(&self) -> Luv<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, Self::min_l(), Self::max_l());
        self.u = clamp(self.u, Self::min_u(), Self::max_u());
        self.v = clamp(self.v, Self::min_v(), Self::max_v());
    }
}

impl<Wp, T> Mix for Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn mix(&self, other: &Luv<Wp, T>, factor: T) -> Luv<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());

        Luv {
            l: self.l + factor * (other.l - self.l),
            u: self.u + factor * (other.u - self.u),
            v: self.v + factor * (other.v - self.v),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn lighten(&self, factor: T) -> Luv<Wp, T> {
        let difference = if factor >= T::zero() {
            T::from_f64(100.0) - self.l
        } else {
            self.l
        };

        let delta = difference.max(T::zero()) * factor;

        Luv {
            l: (self.l + delta).max(T::zero()),
            u: self.u,
            v: self.v,
            white_point: PhantomData,
        }
    }

    fn lighten_fixed(&self, amount: T) -> Luv<Wp, T> {
        Luv {
            l: (self.l + T::from_f64(100.0) * amount).max(T::zero()),
            u: self.u,
            v: self.v,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> GetHue for Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Hue = LuvHue<T>;

    fn get_hue(&self) -> Option<LuvHue<T>> {
        if self.u == T::zero() && self.v == T::zero() {
            None
        } else {
            Some(LuvHue::from_radians(self.v.atan2(self.u)))
        }
    }
}

impl<Wp, T> ComponentWise for Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Luv<Wp, T>, mut f: F) -> Luv<Wp, T> {
        Luv {
            l: f(self.l, other.l),
            u: f(self.u, other.u),
            v: f(self.v, other.v),
            white_point: PhantomData,
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Luv<Wp, T> {
        Luv {
            l: f(self.l),
            u: f(self.u),
            v: f(self.v),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    fn default() -> Luv<Wp, T> {
        Luv::with_wp(T::zero(), T::zero(), T::zero())
    }
}

impl_color_add!(Luv, [l, u, v], white_point);
impl_color_sub!(Luv, [l, u, v], white_point);

impl<Wp, T> Mul<Luv<Wp, T>> for Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Luv<Wp, T>;

    fn mul(self, other: Luv<Wp, T>) -> Self::Output {
        Luv {
            l: self.l * other.l,
            u: self.u * other.u,
            v: self.v * other.v,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Mul<T> for Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Luv<Wp, T>;

    fn mul(self, c: T) -> Self::Output {
        Luv {
            l: self.l * c,
            u: self.u * c,
            v: self.v * c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> MulAssign<Luv<Wp, T>> for Luv<Wp, T>
where
    T: FloatComponent + MulAssign,
    Wp: WhitePoint,
{
    fn mul_assign(&mut self, other: Luv<Wp, T>) {
        self.l *= other.l;
        self.u *= other.u;
        self.v *= other.v;
    }
}

impl<Wp, T> MulAssign<T> for Luv<Wp, T>
where
    T: FloatComponent + MulAssign,
    Wp: WhitePoint,
{
    fn mul_assign(&mut self, c: T) {
        self.l *= c;
        self.u *= c;
        self.v *= c;
    }
}

impl<Wp, T> Div<Luv<Wp, T>> for Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Luv<Wp, T>;

    fn div(self, other: Luv<Wp, T>) -> Self::Output {
        Luv {
            l: self.l / other.l,
            u: self.u / other.u,
            v: self.v / other.v,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Div<T> for Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
{
    type Output = Luv<Wp, T>;

    fn div(self, c: T) -> Self::Output {
        Luv {
            l: self.l / c,
            u: self.u / c,
            v: self.v / c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> DivAssign<Luv<Wp, T>> for Luv<Wp, T>
where
    T: FloatComponent + DivAssign,
    Wp: WhitePoint,
{
    fn div_assign(&mut self, other: Luv<Wp, T>) {
        self.l /= other.l;
        self.u /= other.u;
        self.v /= other.v;
    }
}

impl<Wp, T> DivAssign<T> for Luv<Wp, T>
where
    T: FloatComponent + DivAssign,
    Wp: WhitePoint,
{
    fn div_assign(&mut self, c: T) {
        self.l /= c;
        self.u /= c;
        self.v /= c;
    }
}

impl<Wp, T, P> AsRef<P> for Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<Wp, T, P> AsMut<P> for Luv<Wp, T>
where
    T: FloatComponent,
    Wp: WhitePoint,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<Wp, T> RelativeContrast for Luv<Wp, T>
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
impl<Wp, T> Distribution<Luv<Wp, T>> for Standard
where
    T: FloatComponent,
    Wp: WhitePoint,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Luv<Wp, T> {
        Luv {
            l: rng.gen() * from_f64(100.0),
            u: rng.gen() * from_f64(260.0) - from_f64(84.0),
            v: rng.gen() * from_f64(243.0) - from_f64(135.0),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformLuv<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    l: Uniform<T>,
    u: Uniform<T>,
    v: Uniform<T>,
    white_point: PhantomData<Wp>,
}

#[cfg(feature = "random")]
impl<Wp, T> SampleUniform for Luv<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    type Sampler = UniformLuv<Wp, T>;
}

#[cfg(feature = "random")]
impl<Wp, T> UniformSampler for UniformLuv<Wp, T>
where
    T: FloatComponent + SampleUniform,
    Wp: WhitePoint,
{
    type X = Luv<Wp, T>;

    fn new<B1, B2>(low_v: B1, high_v: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_v.borrow();
        let high = *high_v.borrow();

        UniformLuv {
            l: Uniform::new::<_, T>(low.l, high.l),
            u: Uniform::new::<_, T>(low.u, high.u),
            v: Uniform::new::<_, T>(low.v, high.v),
            white_point: PhantomData,
        }
    }

    fn new_inclusive<B1, B2>(low_v: B1, high_v: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_v.borrow();
        let high = *high_v.borrow();

        UniformLuv {
            l: Uniform::new_inclusive::<_, T>(low.l, high.l),
            u: Uniform::new_inclusive::<_, T>(low.u, high.u),
            v: Uniform::new_inclusive::<_, T>(low.v, high.v),
            white_point: PhantomData,
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Luv<Wp, T> {
        Luv {
            l: self.l.sample(rng),
            u: self.u.sample(rng),
            v: self.v.sample(rng),
            white_point: PhantomData,
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Zeroable for Luv<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent + bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Pod for Luv<Wp, T>
where
    Wp: WhitePoint,
    T: FloatComponent + bytemuck::Pod,
{
}

#[cfg(test)]
mod test {
    use super::Luv;
    use crate::white_point::D65;
    use crate::{FromColor, LinSrgb};

    #[test]
    fn red() {
        let u = Luv::from_color(LinSrgb::new(1.0, 0.0, 0.0));
        let v = Luv::new(53.237116, 175.0098, 37.7650);
        assert_relative_eq!(u, v, epsilon = 0.01);
    }

    #[test]
    fn green() {
        let u = Luv::from_color(LinSrgb::new(0.0, 1.0, 0.0));
        let v = Luv::new(87.73703, -83.07975, 107.40136);
        assert_relative_eq!(u, v, epsilon = 0.01);
    }

    #[test]
    fn blue() {
        let u = Luv::from_color(LinSrgb::new(0.0, 0.0, 1.0));
        let v = Luv::new(32.30087, -9.40241, -130.35109);
        assert_relative_eq!(u, v, epsilon = 0.01);
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Luv<D65, f64>;
            clamped {
            l: 0.0 => 100.0,
            u: -84.0 => 176.0,
            v: -135.0 => 108.0
            }
            clamped_min {}
            unclamped {}
        }
    }
    /// Check that the arithmetic operations (add/sub) are all
    /// implemented.
    #[test]
    fn test_arithmetic() {
        let luv = Luv::new(120.0, 40.0, 30.0);
        let luv2 = Luv::new(200.0, 30.0, 40.0);
        let mut _luv3 = luv + luv2;
        _luv3 += luv2;
        let mut _luv4 = luv2 + 0.3;
        _luv4 += 0.1;

        _luv3 = luv2 - luv;
        _luv3 = _luv4 - 0.1;
        _luv4 -= _luv3;
        _luv3 -= 0.1;
    }

    raw_pixel_conversion_tests!(Luv<D65>: l, u, v);
    raw_pixel_conversion_fail_tests!(Luv<D65>: l, u, v);

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Luv::<D65, f32>::min_l(), 0.0);
        assert_relative_eq!(Luv::<D65, f32>::min_u(), -84.0);
        assert_relative_eq!(Luv::<D65, f32>::min_v(), -135.0);
        assert_relative_eq!(Luv::<D65, f32>::max_l(), 100.0);
        assert_relative_eq!(Luv::<D65, f32>::max_u(), 176.0);
        assert_relative_eq!(Luv::<D65, f32>::max_v(), 108.0);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Luv::new(80.0, 20.0, 30.0)).unwrap();

        assert_eq!(serialized, r#"{"l":80.0,"u":20.0,"v":30.0}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Luv = ::serde_json::from_str(r#"{"l":80.0,"u":20.0,"v":30.0}"#).unwrap();

        assert_eq!(deserialized, Luv::new(80.0, 20.0, 30.0));
    }

    #[cfg(feature = "random")]
    test_uniform_distribution! {
    Luv<D65, f32> {
    l: (0.0, 100.0),
    u: (-84.0, 176.0),
    v: (-135.0, 108.0)
    },
    min: Luv::new(0.0f32, -84.0, -135.0),
    max: Luv::new(100.0, 176.0, 108.0)
    }
}
