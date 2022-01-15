use core::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use crate::{
    cast::ArrayCast,
    clamp,
    num::{Arithmetics, IsValidDivisor, MinMax, One, Real, Sqrt, Zero},
    Alpha, ArrayExt, Blend, ComponentWise, Mix, MixAssign, NextArray,
};

/// Premultiplied alpha wrapper.
///
/// Premultiplied colors are commonly used in composition algorithms to
/// simplify the calculations. It may also be preferred when interpolating
/// between colors, which is one of the reasons why it's offered as a separate
/// type. The other reason is to make it easier to avoid unnecessary
/// computations in composition chains.
///
/// ```
/// use palette::{Blend, LinSrgb, LinSrgba};
/// use palette::blend::PreAlpha;
///
/// let a = PreAlpha::from(LinSrgba::new(0.4, 0.5, 0.5, 0.3));
/// let b = PreAlpha::from(LinSrgba::new(0.3, 0.8, 0.4, 0.4));
/// let c = PreAlpha::from(LinSrgba::new(0.7, 0.1, 0.8, 0.8));
///
/// let res = LinSrgb::from_premultiplied(a.screen(b).overlay(c));
/// ```
///
/// Note that converting to and from premultiplied alpha will cause the alpha
/// component to be clamped to [0.0, 1.0].
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PreAlpha<C, T> {
    /// The premultiplied color components (`original.color * original.alpha`).
    #[cfg_attr(feature = "serializing", serde(flatten))]
    pub color: C,

    /// The transparency component. 0.0 is fully transparent and 1.0 is fully
    /// opaque.
    pub alpha: T,
}

impl<C, T> PartialEq for PreAlpha<C, T>
where
    T: PartialEq,
    C: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.alpha == other.alpha
    }
}

impl<C, T> Eq for PreAlpha<C, T>
where
    T: Eq,
    C: Eq,
{
}

impl<C, T> From<Alpha<C, T>> for PreAlpha<C, T>
where
    C: Mul<T, Output = C>,
    T: Real + Zero + One + PartialOrd + Clone,
{
    #[inline]
    fn from(color: Alpha<C, T>) -> PreAlpha<C, T> {
        let alpha = clamp(color.alpha, T::zero(), T::one());

        PreAlpha {
            color: color.color * alpha.clone(),
            alpha,
        }
    }
}

impl<C, T> From<PreAlpha<C, T>> for Alpha<C, T>
where
    C: Div<T, Output = C> + Default,
    T: Real + Zero + One + IsValidDivisor + PartialOrd + Clone,
{
    #[inline]
    fn from(color: PreAlpha<C, T>) -> Alpha<C, T> {
        let alpha = clamp(color.alpha, T::zero(), T::one());

        Alpha {
            color: if alpha.is_valid_divisor() {
                color.color / alpha.clone()
            } else {
                C::default()
            },
            alpha,
        }
    }
}

impl<C, T> Blend for PreAlpha<C, T>
where
    C: Blend<Color = C> + ComponentWise<Scalar = T>,
    T: Real + One + Zero + MinMax + Sqrt + IsValidDivisor + Arithmetics + PartialOrd + Clone,
{
    type Color = C;

    fn into_premultiplied(self) -> PreAlpha<C, T> {
        self
    }

    fn from_premultiplied(color: PreAlpha<C, T>) -> PreAlpha<C, T> {
        color
    }
}

impl<C> Mix for PreAlpha<C, C::Scalar>
where
    C: Mix,
    C::Scalar: Real + Zero + One + PartialOrd + Arithmetics + Clone,
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

impl<C> MixAssign for PreAlpha<C, C::Scalar>
where
    C: MixAssign,
    C::Scalar: Real + Zero + One + PartialOrd + Arithmetics + AddAssign + Clone,
{
    type Scalar = C::Scalar;

    #[inline]
    fn mix_assign(&mut self, other: Self, factor: C::Scalar) {
        let factor = clamp(factor, C::Scalar::zero(), C::Scalar::one());

        self.color.mix_assign(other.color, factor.clone());
        self.alpha += factor * (other.alpha - self.alpha.clone());
    }
}

impl<C: ComponentWise<Scalar = T>, T: Clone> ComponentWise for PreAlpha<C, T> {
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(
        &self,
        other: &PreAlpha<C, T>,
        mut f: F,
    ) -> PreAlpha<C, T> {
        PreAlpha {
            alpha: f(self.alpha.clone(), other.alpha.clone()),
            color: self.color.component_wise(&other.color, f),
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> PreAlpha<C, T> {
        PreAlpha {
            alpha: f(self.alpha.clone()),
            color: self.color.component_wise_self(f),
        }
    }
}

unsafe impl<C> ArrayCast for PreAlpha<C, <<C as ArrayCast>::Array as ArrayExt>::Item>
where
    C: ArrayCast,
    C::Array: NextArray,
{
    type Array = <C::Array as NextArray>::Next;
}

impl<C: Default, T: One> Default for PreAlpha<C, T> {
    fn default() -> PreAlpha<C, T> {
        PreAlpha {
            color: C::default(),
            alpha: T::one(),
        }
    }
}

impl<C, T> AbsDiffEq for PreAlpha<C, T>
where
    C: AbsDiffEq<Epsilon = T::Epsilon>,
    T: AbsDiffEq,
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &PreAlpha<C, T>, epsilon: Self::Epsilon) -> bool {
        self.color.abs_diff_eq(&other.color, epsilon.clone())
            && self.alpha.abs_diff_eq(&other.alpha, epsilon)
    }
}

impl<C, T> RelativeEq for PreAlpha<C, T>
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
        other: &PreAlpha<C, T>,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.color
            .relative_eq(&other.color, epsilon.clone(), max_relative.clone())
            && self.alpha.relative_eq(&other.alpha, epsilon, max_relative)
    }
}

impl<C, T> UlpsEq for PreAlpha<C, T>
where
    C: UlpsEq<Epsilon = T::Epsilon>,
    T: UlpsEq,
    T::Epsilon: Clone,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &PreAlpha<C, T>, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.color.ulps_eq(&other.color, epsilon.clone(), max_ulps)
            && self.alpha.ulps_eq(&other.alpha, epsilon, max_ulps)
    }
}

impl<C: Add, T: Add> Add for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T::Output>;

    fn add(self, other: PreAlpha<C, T>) -> Self::Output {
        PreAlpha {
            color: self.color + other.color,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl<T: Add + Clone, C: Add<T>> Add<T> for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T::Output>;

    fn add(self, c: T) -> Self::Output {
        PreAlpha {
            color: self.color + c.clone(),
            alpha: self.alpha + c,
        }
    }
}

impl<C: AddAssign, T: AddAssign> AddAssign for PreAlpha<C, T> {
    fn add_assign(&mut self, other: PreAlpha<C, T>) {
        self.color += other.color;
        self.alpha += other.alpha;
    }
}

impl<T: AddAssign + Clone, C: AddAssign<T>> AddAssign<T> for PreAlpha<C, T> {
    fn add_assign(&mut self, c: T) {
        self.color += c.clone();
        self.alpha += c;
    }
}

impl<C: Sub, T: Sub> Sub for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T::Output>;

    fn sub(self, other: PreAlpha<C, T>) -> Self::Output {
        PreAlpha {
            color: self.color - other.color,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl<T: Sub + Clone, C: Sub<T>> Sub<T> for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T::Output>;

    fn sub(self, c: T) -> Self::Output {
        PreAlpha {
            color: self.color - c.clone(),
            alpha: self.alpha - c,
        }
    }
}

impl<C: SubAssign, T: SubAssign> SubAssign for PreAlpha<C, T> {
    fn sub_assign(&mut self, other: PreAlpha<C, T>) {
        self.color -= other.color;
        self.alpha -= other.alpha;
    }
}

impl<T: SubAssign + Clone, C: SubAssign<T>> SubAssign<T> for PreAlpha<C, T> {
    fn sub_assign(&mut self, c: T) {
        self.color -= c.clone();
        self.alpha -= c;
    }
}

impl<C: Mul, T: Mul> Mul for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T::Output>;

    fn mul(self, other: PreAlpha<C, T>) -> Self::Output {
        PreAlpha {
            color: self.color * other.color,
            alpha: self.alpha * other.alpha,
        }
    }
}

impl<T: Mul + Clone, C: Mul<T>> Mul<T> for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T::Output>;

    fn mul(self, c: T) -> Self::Output {
        PreAlpha {
            color: self.color * c.clone(),
            alpha: self.alpha * c,
        }
    }
}

impl<C: MulAssign, T: MulAssign> MulAssign for PreAlpha<C, T> {
    fn mul_assign(&mut self, other: PreAlpha<C, T>) {
        self.color *= other.color;
        self.alpha *= other.alpha;
    }
}

impl<T: MulAssign + Clone, C: MulAssign<T>> MulAssign<T> for PreAlpha<C, T> {
    fn mul_assign(&mut self, c: T) {
        self.color *= c.clone();
        self.alpha *= c;
    }
}

impl<C: Div, T: Div> Div for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T::Output>;

    fn div(self, other: PreAlpha<C, T>) -> Self::Output {
        PreAlpha {
            color: self.color / other.color,
            alpha: self.alpha / other.alpha,
        }
    }
}

impl<T: Div + Clone, C: Div<T>> Div<T> for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T::Output>;

    fn div(self, c: T) -> Self::Output {
        PreAlpha {
            color: self.color / c.clone(),
            alpha: self.alpha / c,
        }
    }
}

impl<C: DivAssign, T: DivAssign> DivAssign for PreAlpha<C, T> {
    fn div_assign(&mut self, other: PreAlpha<C, T>) {
        self.color /= other.color;
        self.alpha /= other.alpha;
    }
}

impl<T: DivAssign + Clone, C: DivAssign<T>> DivAssign<T> for PreAlpha<C, T> {
    fn div_assign(&mut self, c: T) {
        self.color /= c.clone();
        self.alpha /= c;
    }
}

impl_array_casts!([C, T, const N: usize] PreAlpha<C, T>, [T; N], where PreAlpha<C, T>: ArrayCast<Array = [T; N]>);

impl<C, T> Deref for PreAlpha<C, T> {
    type Target = C;

    fn deref(&self) -> &C {
        &self.color
    }
}

impl<C, T> DerefMut for PreAlpha<C, T> {
    fn deref_mut(&mut self) -> &mut C {
        &mut self.color
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<C, T> bytemuck::Zeroable for PreAlpha<C, T>
where
    C: bytemuck::Zeroable,
    T: bytemuck::Zeroable,
{
}

// Safety:
//
// See `Alpha<C, T>`'s implementation of `Pod`.
#[cfg(feature = "bytemuck")]
unsafe impl<C, T> bytemuck::Pod for PreAlpha<C, T>
where
    C: bytemuck::Pod + ArrayCast,
    T: bytemuck::Pod,
{
}

#[cfg(test)]
#[cfg(feature = "serializing")]
mod test {
    use super::PreAlpha;
    use crate::encoding::Srgb;
    use crate::rgb::Rgb;

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let color = PreAlpha {
            color: Rgb::<Srgb>::new(0.3, 0.8, 0.1),
            alpha: 0.5,
        };

        let serialized = ::serde_json::to_string(&color).unwrap();

        assert_eq!(
            serialized,
            r#"{"red":0.3,"green":0.8,"blue":0.1,"alpha":0.5}"#
        );
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let expected = PreAlpha {
            color: Rgb::<Srgb>::new(0.3, 0.8, 0.1),
            alpha: 0.5,
        };

        let deserialized: PreAlpha<_, _> =
            ::serde_json::from_str(r#"{"red":0.3,"green":0.8,"blue":0.1,"alpha":0.5}"#).unwrap();

        assert_eq!(deserialized, expected);
    }
}
