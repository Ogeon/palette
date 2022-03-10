use core::{
    fmt,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
#[cfg(feature = "random")]
use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};

use crate::{
    blend::{PreAlpha, Premultiply},
    cast::ArrayCast,
    clamp, clamp_assign,
    convert::{FromColorUnclamped, IntoColorUnclamped},
    num::{Arithmetics, One, Zero},
    stimulus::Stimulus,
    ArrayExt, Clamp, ClampAssign, GetHue, IsWithinBounds, Lighten, LightenAssign, Mix, MixAssign,
    NextArray, Saturate, SaturateAssign, SetHue, ShiftHue, ShiftHueAssign, WithAlpha, WithHue,
};

/// An alpha component wrapper for colors.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Alpha<C, T> {
    /// The color.
    #[cfg_attr(feature = "serializing", serde(flatten))]
    pub color: C,

    /// The transparency component. 0.0 is fully transparent and 1.0 is fully
    /// opaque.
    pub alpha: T,
}

impl<C: Premultiply> Alpha<C, C::Scalar> {
    /// Alpha mask the color by its transparency.
    pub fn premultiply(self) -> PreAlpha<C> {
        PreAlpha::new(self.color, self.alpha)
    }
}

impl<C, T: Stimulus> Alpha<C, T> {
    /// Return the `alpha` value minimum.
    pub fn min_alpha() -> T {
        T::zero()
    }

    /// Return the `alpha` value maximum.
    pub fn max_alpha() -> T {
        T::max_intensity()
    }
}

impl<C, T> PartialEq for Alpha<C, T>
where
    T: PartialEq,
    C: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.alpha == other.alpha
    }
}

impl<C, T> Eq for Alpha<C, T>
where
    T: Eq,
    C: Eq,
{
}

impl<C1: WithAlpha<T>, C2, T> FromColorUnclamped<C1> for Alpha<C2, T>
where
    C1::Color: IntoColorUnclamped<C2>,
{
    fn from_color_unclamped(other: C1) -> Self {
        let (color, alpha) = other.split();

        Alpha {
            color: color.into_color_unclamped(),
            alpha,
        }
    }
}

impl<C, A> WithAlpha<A> for Alpha<C, A> {
    type Color = C;
    type WithAlpha = Self;

    fn with_alpha(mut self, alpha: A) -> Self::WithAlpha {
        self.alpha = alpha;
        self
    }

    fn without_alpha(self) -> Self::Color {
        self.color
    }

    fn split(self) -> (Self::Color, A) {
        (self.color, self.alpha)
    }
}

impl<C, T> Deref for Alpha<C, T> {
    type Target = C;

    fn deref(&self) -> &C {
        &self.color
    }
}

impl<C, T> DerefMut for Alpha<C, T> {
    fn deref_mut(&mut self) -> &mut C {
        &mut self.color
    }
}

impl<C> Mix for Alpha<C, C::Scalar>
where
    C: Mix,
    C::Scalar: Zero + One + PartialOrd + Arithmetics + Clone,
{
    type Scalar = C::Scalar;

    #[inline]
    fn mix(mut self, other: Self, factor: C::Scalar) -> Self {
        let factor = clamp(factor, C::Scalar::zero(), C::Scalar::one());

        self.color = self.color.mix(other.color, factor.clone());
        self.alpha = self.alpha.clone() + factor * (other.alpha - self.alpha);

        self
    }
}

impl<C> MixAssign for Alpha<C, C::Scalar>
where
    C: MixAssign,
    C::Scalar: Zero + One + PartialOrd + Arithmetics + AddAssign + Clone,
{
    type Scalar = C::Scalar;

    #[inline]
    fn mix_assign(&mut self, other: Self, factor: C::Scalar) {
        let factor = clamp(factor, C::Scalar::zero(), C::Scalar::one());

        self.color.mix_assign(other.color, factor.clone());
        self.alpha += factor * (other.alpha - self.alpha.clone());
    }
}

impl<C: Lighten> Lighten for Alpha<C, C::Scalar> {
    type Scalar = C::Scalar;

    #[inline]
    fn lighten(self, factor: C::Scalar) -> Self {
        Alpha {
            color: self.color.lighten(factor),
            alpha: self.alpha,
        }
    }

    #[inline]
    fn lighten_fixed(self, amount: C::Scalar) -> Self {
        Alpha {
            color: self.color.lighten_fixed(amount),
            alpha: self.alpha,
        }
    }
}

impl<C: LightenAssign> LightenAssign for Alpha<C, C::Scalar> {
    type Scalar = C::Scalar;

    #[inline]
    fn lighten_assign(&mut self, factor: C::Scalar) {
        self.color.lighten_assign(factor);
    }

    #[inline]
    fn lighten_fixed_assign(&mut self, amount: C::Scalar) {
        self.color.lighten_fixed_assign(amount);
    }
}

impl<C: GetHue, T> GetHue for Alpha<C, T> {
    type Hue = C::Hue;

    #[inline]
    fn get_hue(&self) -> Option<C::Hue> {
        self.color.get_hue()
    }
}

impl<C, T, H> WithHue<H> for Alpha<C, T>
where
    C: WithHue<H>,
{
    #[inline]
    fn with_hue(mut self, hue: H) -> Self {
        self.color = self.color.with_hue(hue);
        self
    }
}

impl<C, T, H> SetHue<H> for Alpha<C, T>
where
    C: SetHue<H>,
{
    #[inline]
    fn set_hue(&mut self, hue: H) {
        self.color.set_hue(hue);
    }
}

impl<C, T> ShiftHue for Alpha<C, T>
where
    C: ShiftHue,
{
    type Scalar = C::Scalar;

    #[inline]
    fn shift_hue(mut self, amount: Self::Scalar) -> Self {
        self.color = self.color.shift_hue(amount);
        self
    }
}

impl<C, T> ShiftHueAssign for Alpha<C, T>
where
    C: ShiftHueAssign,
{
    type Scalar = C::Scalar;

    #[inline]
    fn shift_hue_assign(&mut self, amount: Self::Scalar) {
        self.color.shift_hue_assign(amount);
    }
}

impl<C: Saturate> Saturate for Alpha<C, C::Scalar> {
    type Scalar = C::Scalar;

    #[inline]
    fn saturate(self, factor: C::Scalar) -> Self {
        Alpha {
            color: self.color.saturate(factor),
            alpha: self.alpha,
        }
    }

    #[inline]
    fn saturate_fixed(self, amount: C::Scalar) -> Self {
        Alpha {
            color: self.color.saturate_fixed(amount),
            alpha: self.alpha,
        }
    }
}

impl<C: SaturateAssign> SaturateAssign for Alpha<C, C::Scalar> {
    type Scalar = C::Scalar;

    #[inline]
    fn saturate_assign(&mut self, factor: C::Scalar) {
        self.color.saturate_assign(factor);
    }

    #[inline]
    fn saturate_fixed_assign(&mut self, amount: C::Scalar) {
        self.color.saturate_fixed_assign(amount);
    }
}

impl<C, T> IsWithinBounds for Alpha<C, T>
where
    C: IsWithinBounds,
    T: Stimulus + PartialOrd,
{
    #[inline]
    fn is_within_bounds(&self) -> bool {
        self.color.is_within_bounds()
            && self.alpha >= Self::min_alpha()
            && self.alpha <= Self::max_alpha()
    }
}

impl<C, T> Clamp for Alpha<C, T>
where
    C: Clamp,
    T: Stimulus + PartialOrd,
{
    #[inline]
    fn clamp(self) -> Self {
        Alpha {
            color: self.color.clamp(),
            alpha: clamp(self.alpha, Self::min_alpha(), Self::max_alpha()),
        }
    }
}

impl<C, T> ClampAssign for Alpha<C, T>
where
    C: ClampAssign,
    T: Stimulus + PartialOrd,
{
    #[inline]
    fn clamp_assign(&mut self) {
        self.color.clamp_assign();
        clamp_assign(&mut self.alpha, Self::min_alpha(), Self::max_alpha());
    }
}

unsafe impl<C> ArrayCast for Alpha<C, <<C as ArrayCast>::Array as ArrayExt>::Item>
where
    C: ArrayCast,
    C::Array: NextArray,
{
    type Array = <C::Array as NextArray>::Next;
}

impl<C: Default, T: Stimulus> Default for Alpha<C, T> {
    fn default() -> Alpha<C, T> {
        Alpha {
            color: C::default(),
            alpha: Self::max_alpha(),
        }
    }
}

impl<C, T> AbsDiffEq for Alpha<C, T>
where
    C: AbsDiffEq<Epsilon = T::Epsilon>,
    T: AbsDiffEq,
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        self.color.abs_diff_eq(&other.color, epsilon.clone())
            && self.alpha.abs_diff_eq(&other.alpha, epsilon)
    }
}

impl<C, T> RelativeEq for Alpha<C, T>
where
    C: RelativeEq<Epsilon = T::Epsilon>,
    T: RelativeEq,
    T::Epsilon: Clone,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Alpha<C, T>,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.color
            .relative_eq(&other.color, epsilon.clone(), max_relative.clone())
            && self.alpha.relative_eq(&other.alpha, epsilon, max_relative)
    }
}

impl<C, T> UlpsEq for Alpha<C, T>
where
    C: UlpsEq<Epsilon = T::Epsilon>,
    T: UlpsEq,
    T::Epsilon: Clone,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Alpha<C, T>, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.color.ulps_eq(&other.color, epsilon.clone(), max_ulps)
            && self.alpha.ulps_eq(&other.alpha, epsilon, max_ulps)
    }
}

impl<C, T> Add for Alpha<C, T>
where
    C: Add,
    T: Add,
{
    type Output = Alpha<C::Output, <T as Add>::Output>;

    fn add(self, other: Alpha<C, T>) -> Self::Output {
        Alpha {
            color: self.color + other.color,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl<T, C> Add<T> for Alpha<C, T>
where
    T: Add + Clone,
    C: Add<T>,
{
    type Output = Alpha<C::Output, <T as Add>::Output>;

    fn add(self, c: T) -> Self::Output {
        Alpha {
            color: self.color + c.clone(),
            alpha: self.alpha + c,
        }
    }
}

impl<C, T> AddAssign for Alpha<C, T>
where
    C: AddAssign,
    T: AddAssign,
{
    fn add_assign(&mut self, other: Alpha<C, T>) {
        self.color += other.color;
        self.alpha += other.alpha;
    }
}

impl<T, C> AddAssign<T> for Alpha<C, T>
where
    T: AddAssign + Clone,
    C: AddAssign<T>,
{
    fn add_assign(&mut self, c: T) {
        self.color += c.clone();
        self.alpha += c;
    }
}

impl<C, T> Sub for Alpha<C, T>
where
    C: Sub,
    T: Sub,
{
    type Output = Alpha<C::Output, <T as Sub>::Output>;

    fn sub(self, other: Alpha<C, T>) -> Self::Output {
        Alpha {
            color: self.color - other.color,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl<T, C> Sub<T> for Alpha<C, T>
where
    T: Sub + Clone,
    C: Sub<T>,
{
    type Output = Alpha<C::Output, <T as Sub>::Output>;

    fn sub(self, c: T) -> Self::Output {
        Alpha {
            color: self.color - c.clone(),
            alpha: self.alpha - c,
        }
    }
}

impl<C, T> SubAssign for Alpha<C, T>
where
    C: SubAssign,
    T: SubAssign,
{
    fn sub_assign(&mut self, other: Alpha<C, T>) {
        self.color -= other.color;
        self.alpha -= other.alpha;
    }
}

impl<T, C> SubAssign<T> for Alpha<C, T>
where
    T: SubAssign + Clone,
    C: SubAssign<T>,
{
    fn sub_assign(&mut self, c: T) {
        self.color -= c.clone();
        self.alpha -= c;
    }
}

impl<C, T> Mul for Alpha<C, T>
where
    C: Mul,
    T: Mul,
{
    type Output = Alpha<C::Output, <T as Mul>::Output>;

    fn mul(self, other: Alpha<C, T>) -> Self::Output {
        Alpha {
            color: self.color * other.color,
            alpha: self.alpha * other.alpha,
        }
    }
}

impl<T, C> Mul<T> for Alpha<C, T>
where
    T: Mul + Clone,
    C: Mul<T>,
{
    type Output = Alpha<C::Output, <T as Mul>::Output>;

    fn mul(self, c: T) -> Self::Output {
        Alpha {
            color: self.color * c.clone(),
            alpha: self.alpha * c,
        }
    }
}

impl<C, T> MulAssign for Alpha<C, T>
where
    C: MulAssign,
    T: MulAssign,
{
    fn mul_assign(&mut self, other: Alpha<C, T>) {
        self.color *= other.color;
        self.alpha *= other.alpha;
    }
}

impl<T, C> MulAssign<T> for Alpha<C, T>
where
    T: MulAssign + Clone,
    C: MulAssign<T>,
{
    fn mul_assign(&mut self, c: T) {
        self.color *= c.clone();
        self.alpha *= c;
    }
}

impl<C, T> Div for Alpha<C, T>
where
    C: Div,
    T: Div,
{
    type Output = Alpha<C::Output, <T as Div>::Output>;

    fn div(self, other: Alpha<C, T>) -> Self::Output {
        Alpha {
            color: self.color / other.color,
            alpha: self.alpha / other.alpha,
        }
    }
}

impl<T, C> Div<T> for Alpha<C, T>
where
    T: Div + Clone,
    C: Div<T>,
{
    type Output = Alpha<C::Output, <T as Div>::Output>;

    fn div(self, c: T) -> Self::Output {
        Alpha {
            color: self.color / c.clone(),
            alpha: self.alpha / c,
        }
    }
}

impl<C, T> DivAssign for Alpha<C, T>
where
    C: DivAssign,
    T: DivAssign,
{
    fn div_assign(&mut self, other: Alpha<C, T>) {
        self.color /= other.color;
        self.alpha /= other.alpha;
    }
}

impl<T, C> DivAssign<T> for Alpha<C, T>
where
    T: DivAssign + Clone,
    C: DivAssign<T>,
{
    fn div_assign(&mut self, c: T) {
        self.color /= c.clone();
        self.alpha /= c;
    }
}

impl_array_casts!([C, T, const N: usize] Alpha<C, T>, [T; N], where Alpha<C, T>: ArrayCast<Array = [T; N]>);

impl<C, T: Stimulus> From<C> for Alpha<C, T> {
    fn from(color: C) -> Alpha<C, T> {
        Alpha {
            color,
            alpha: Self::max_alpha(),
        }
    }
}

impl<C, T> fmt::LowerHex for Alpha<C, T>
where
    T: fmt::LowerHex,
    C: fmt::LowerHex,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = f.width().unwrap_or(::core::mem::size_of::<T>() * 2);
        write!(
            f,
            "{:0width$x}{:0width$x}",
            self.color,
            self.alpha,
            width = size
        )
    }
}

impl<C, T> fmt::UpperHex for Alpha<C, T>
where
    T: fmt::UpperHex,
    C: fmt::UpperHex,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = f.width().unwrap_or(::core::mem::size_of::<T>() * 2);
        write!(
            f,
            "{:0width$X}{:0width$X}",
            self.color,
            self.alpha,
            width = size
        )
    }
}

#[cfg(feature = "random")]
impl<C, T> Distribution<Alpha<C, T>> for Standard
where
    Standard: Distribution<C> + Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Alpha<C, T> {
        Alpha {
            color: rng.gen(),
            alpha: rng.gen(),
        }
    }
}

#[cfg(feature = "random")]
pub struct UniformAlpha<C, T>
where
    T: SampleUniform,
    C: SampleUniform,
{
    color: Uniform<C>,
    alpha: Uniform<T>,
}

#[cfg(feature = "random")]
impl<C, T> SampleUniform for Alpha<C, T>
where
    T: Clone + SampleUniform,
    C: Clone + SampleUniform,
{
    type Sampler = UniformAlpha<C, T>;
}

#[cfg(feature = "random")]
impl<C, T> UniformSampler for UniformAlpha<C, T>
where
    T: Clone + SampleUniform,
    C: Clone + SampleUniform,
{
    type X = Alpha<C, T>;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

        UniformAlpha {
            color: Uniform::new::<C, _>(low.color, high.color),
            alpha: Uniform::new::<_, T>(low.alpha, high.alpha),
        }
    }

    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow().clone();
        let high = high_b.borrow().clone();

        UniformAlpha {
            color: Uniform::new_inclusive::<C, _>(low.color, high.color),
            alpha: Uniform::new_inclusive::<_, T>(low.alpha, high.alpha),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Alpha<C, T> {
        Alpha {
            color: self.color.sample(rng),
            alpha: self.alpha.sample(rng),
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<C, T> bytemuck::Zeroable for Alpha<C, T>
where
    C: bytemuck::Zeroable,
    T: bytemuck::Zeroable,
{
}

// Safety:
//
// It is a requirement of `ArrayCast` that the in-memory representation of `C`
// is made of `T`s. Because `T` is `Pod`, `Alpha<C, T>` is `Pod` as well because
// no internal padding can be introduced during monomorphization.
#[cfg(feature = "bytemuck")]
unsafe impl<C, T> bytemuck::Pod for Alpha<C, T>
where
    T: bytemuck::Pod,
    C: bytemuck::Pod + ArrayCast,
{
}

#[cfg(test)]
mod test {
    use crate::encoding::Srgb;
    use crate::rgb::Rgba;

    #[test]
    fn lower_hex() {
        assert_eq!(
            format!("{:x}", Rgba::<Srgb, u8>::new(171, 193, 35, 161)),
            "abc123a1"
        );
    }

    #[test]
    fn lower_hex_small_numbers() {
        assert_eq!(
            format!("{:x}", Rgba::<Srgb, u8>::new(1, 2, 3, 4)),
            "01020304"
        );
        assert_eq!(
            format!("{:x}", Rgba::<Srgb, u16>::new(1, 2, 3, 4)),
            "0001000200030004"
        );
        assert_eq!(
            format!("{:x}", Rgba::<Srgb, u32>::new(1, 2, 3, 4)),
            "00000001000000020000000300000004"
        );
        assert_eq!(
            format!("{:x}", Rgba::<Srgb, u64>::new(1, 2, 3, 4)),
            "0000000000000001000000000000000200000000000000030000000000000004"
        );
    }

    #[test]
    fn lower_hex_custom_width() {
        assert_eq!(
            format!("{:03x}", Rgba::<Srgb, u8>::new(1, 2, 3, 4)),
            "001002003004"
        );
        assert_eq!(
            format!("{:03x}", Rgba::<Srgb, u16>::new(1, 2, 3, 4)),
            "001002003004"
        );
        assert_eq!(
            format!("{:03x}", Rgba::<Srgb, u32>::new(1, 2, 3, 4)),
            "001002003004"
        );
        assert_eq!(
            format!("{:03x}", Rgba::<Srgb, u64>::new(1, 2, 3, 4)),
            "001002003004"
        );
    }

    #[test]
    fn upper_hex() {
        assert_eq!(
            format!("{:X}", Rgba::<Srgb, u8>::new(171, 193, 35, 161)),
            "ABC123A1"
        );
    }

    #[test]
    fn upper_hex_small_numbers() {
        assert_eq!(
            format!("{:X}", Rgba::<Srgb, u8>::new(1, 2, 3, 4)),
            "01020304"
        );
        assert_eq!(
            format!("{:X}", Rgba::<Srgb, u16>::new(1, 2, 3, 4)),
            "0001000200030004"
        );
        assert_eq!(
            format!("{:X}", Rgba::<Srgb, u32>::new(1, 2, 3, 4)),
            "00000001000000020000000300000004"
        );
        assert_eq!(
            format!("{:X}", Rgba::<Srgb, u64>::new(1, 2, 3, 4)),
            "0000000000000001000000000000000200000000000000030000000000000004"
        );
    }

    #[test]
    fn upper_hex_custom_width() {
        assert_eq!(
            format!("{:03X}", Rgba::<Srgb, u8>::new(1, 2, 3, 4)),
            "001002003004"
        );
        assert_eq!(
            format!("{:03X}", Rgba::<Srgb, u16>::new(1, 2, 3, 4)),
            "001002003004"
        );
        assert_eq!(
            format!("{:03X}", Rgba::<Srgb, u32>::new(1, 2, 3, 4)),
            "001002003004"
        );
        assert_eq!(
            format!("{:03X}", Rgba::<Srgb, u64>::new(1, 2, 3, 4)),
            "001002003004"
        );
    }

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Rgba::<Srgb>::min_alpha(), 0.0);
        assert_relative_eq!(Rgba::<Srgb>::max_alpha(), 1.0);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Rgba::<Srgb>::new(0.3, 0.8, 0.1, 0.5)).unwrap();

        assert_eq!(
            serialized,
            r#"{"red":0.3,"green":0.8,"blue":0.1,"alpha":0.5}"#
        );
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Rgba<Srgb> =
            ::serde_json::from_str(r#"{"red":0.3,"green":0.8,"blue":0.1,"alpha":0.5}"#).unwrap();

        assert_eq!(deserialized, Rgba::<Srgb>::new(0.3, 0.8, 0.1, 0.5));
    }

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Rgba<Srgb, f32> {
            red: (0.0, 1.0),
            green: (0.0, 1.0),
            blue: (0.0, 1.0),
            alpha: (0.0, 1.0)
        },
        min: Rgba::new(0.0f32, 0.0, 0.0, 0.0),
        max: Rgba::new(1.0, 1.0, 1.0, 1.0)
    }
}
