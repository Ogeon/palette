use core::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use crate::encoding::pixel::RawPixel;
use crate::float::Float;
use crate::{clamp, Alpha, Blend, ComponentWise, Mix, Pixel};

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
pub struct PreAlpha<C, T: Float> {
    /// The premultiplied color components (`original.color * original.alpha`).
    #[cfg_attr(feature = "serializing", serde(flatten))]
    pub color: C,

    /// The transparency component. 0.0 is fully transparent and 1.0 is fully
    /// opaque.
    pub alpha: T,
}

impl<C, T> PartialEq for PreAlpha<C, T>
where
    T: Float + PartialEq,
    C: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.alpha == other.alpha
    }
}

impl<C, T> Eq for PreAlpha<C, T>
where
    T: Float + Eq,
    C: Eq,
{
}

impl<C, T> From<Alpha<C, T>> for PreAlpha<C, T>
where
    C: ComponentWise<Scalar = T>,
    T: Float,
{
    fn from(color: Alpha<C, T>) -> PreAlpha<C, T> {
        let alpha = clamp(color.alpha, T::zero(), T::one());

        PreAlpha {
            color: color.color.component_wise_self(|a| a * alpha),
            alpha,
        }
    }
}

impl<C, T> From<PreAlpha<C, T>> for Alpha<C, T>
where
    C: ComponentWise<Scalar = T>,
    T: Float,
{
    fn from(color: PreAlpha<C, T>) -> Alpha<C, T> {
        let alpha = clamp(color.alpha, T::zero(), T::one());

        let color = color.color.component_wise_self(|a| {
            if alpha.is_normal() {
                a / alpha
            } else {
                T::zero()
            }
        });

        Alpha { color, alpha }
    }
}

impl<C, T> Blend for PreAlpha<C, T>
where
    C: Blend<Color = C> + ComponentWise<Scalar = T>,
    T: Float,
{
    type Color = C;

    fn into_premultiplied(self) -> PreAlpha<C, T> {
        self
    }

    fn from_premultiplied(color: PreAlpha<C, T>) -> PreAlpha<C, T> {
        color
    }
}

impl<C: Mix> Mix for PreAlpha<C, C::Scalar> {
    type Scalar = C::Scalar;

    fn mix(&self, other: &PreAlpha<C, C::Scalar>, factor: C::Scalar) -> PreAlpha<C, C::Scalar> {
        PreAlpha {
            color: self.color.mix(&other.color, factor),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl<C: ComponentWise<Scalar = T>, T: Float> ComponentWise for PreAlpha<C, T> {
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(
        &self,
        other: &PreAlpha<C, T>,
        mut f: F,
    ) -> PreAlpha<C, T> {
        PreAlpha {
            alpha: f(self.alpha, other.alpha),
            color: self.color.component_wise(&other.color, f),
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> PreAlpha<C, T> {
        PreAlpha {
            alpha: f(self.alpha),
            color: self.color.component_wise_self(f),
        }
    }
}

unsafe impl<T: Float, C: Pixel<T>> Pixel<T> for PreAlpha<C, T> {
    const CHANNELS: usize = C::CHANNELS + 1;
}

impl<C: Default, T: Float> Default for PreAlpha<C, T> {
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
    T: AbsDiffEq + Float,
    T::Epsilon: Copy,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &PreAlpha<C, T>, epsilon: Self::Epsilon) -> bool {
        self.color.abs_diff_eq(&other.color, epsilon)
            && self.alpha.abs_diff_eq(&other.alpha, epsilon)
    }
}

impl<C, T> RelativeEq for PreAlpha<C, T>
where
    C: RelativeEq<Epsilon = T::Epsilon>,
    T: RelativeEq + Float,
    T::Epsilon: Copy,
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
        self.color.relative_eq(&other.color, epsilon, max_relative)
            && self.alpha.relative_eq(&other.alpha, epsilon, max_relative)
    }
}

impl<C, T> UlpsEq for PreAlpha<C, T>
where
    C: UlpsEq<Epsilon = T::Epsilon>,
    T: UlpsEq + Float,
    T::Epsilon: Copy,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &PreAlpha<C, T>, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.color.ulps_eq(&other.color, epsilon, max_ulps)
            && self.alpha.ulps_eq(&other.alpha, epsilon, max_ulps)
    }
}

impl<C: Add, T: Float> Add for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn add(self, other: PreAlpha<C, T>) -> Self::Output {
        PreAlpha {
            color: self.color + other.color,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl<T: Float, C: Add<T>> Add<T> for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn add(self, c: T) -> Self::Output {
        PreAlpha {
            color: self.color + c,
            alpha: self.alpha + c,
        }
    }
}

impl<C: AddAssign, T: Float + AddAssign> AddAssign for PreAlpha<C, T> {
    fn add_assign(&mut self, other: PreAlpha<C, T>) {
        self.color += other.color;
        self.alpha += other.alpha;
    }
}

impl<T: Float + AddAssign, C: AddAssign<T>> AddAssign<T> for PreAlpha<C, T> {
    fn add_assign(&mut self, c: T) {
        self.color += c;
        self.alpha += c;
    }
}

impl<C: Sub, T: Float> Sub for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn sub(self, other: PreAlpha<C, T>) -> Self::Output {
        PreAlpha {
            color: self.color - other.color,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl<T: Float, C: Sub<T>> Sub<T> for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn sub(self, c: T) -> Self::Output {
        PreAlpha {
            color: self.color - c,
            alpha: self.alpha - c,
        }
    }
}

impl<C: SubAssign, T: Float + SubAssign> SubAssign for PreAlpha<C, T> {
    fn sub_assign(&mut self, other: PreAlpha<C, T>) {
        self.color -= other.color;
        self.alpha -= other.alpha;
    }
}

impl<T: Float + SubAssign, C: SubAssign<T>> SubAssign<T> for PreAlpha<C, T> {
    fn sub_assign(&mut self, c: T) {
        self.color -= c;
        self.alpha -= c;
    }
}

impl<C: Mul, T: Float> Mul for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn mul(self, other: PreAlpha<C, T>) -> Self::Output {
        PreAlpha {
            color: self.color * other.color,
            alpha: self.alpha * other.alpha,
        }
    }
}

impl<T: Float, C: Mul<T>> Mul<T> for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn mul(self, c: T) -> Self::Output {
        PreAlpha {
            color: self.color * c,
            alpha: self.alpha * c,
        }
    }
}

impl<C: MulAssign, T: Float + MulAssign> MulAssign for PreAlpha<C, T> {
    fn mul_assign(&mut self, other: PreAlpha<C, T>) {
        self.color *= other.color;
        self.alpha *= other.alpha;
    }
}

impl<T: Float + MulAssign, C: MulAssign<T>> MulAssign<T> for PreAlpha<C, T> {
    fn mul_assign(&mut self, c: T) {
        self.color *= c;
        self.alpha *= c;
    }
}

impl<C: Div, T: Float> Div for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn div(self, other: PreAlpha<C, T>) -> Self::Output {
        PreAlpha {
            color: self.color / other.color,
            alpha: self.alpha / other.alpha,
        }
    }
}

impl<T: Float, C: Div<T>> Div<T> for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn div(self, c: T) -> Self::Output {
        PreAlpha {
            color: self.color / c,
            alpha: self.alpha / c,
        }
    }
}

impl<C: DivAssign, T: Float + DivAssign> DivAssign for PreAlpha<C, T> {
    fn div_assign(&mut self, other: PreAlpha<C, T>) {
        self.color /= other.color;
        self.alpha /= other.alpha;
    }
}

impl<T: Float + DivAssign, C: DivAssign<T>> DivAssign<T> for PreAlpha<C, T> {
    fn div_assign(&mut self, c: T) {
        self.color /= c;
        self.alpha /= c;
    }
}

impl<C, T, P> AsRef<P> for PreAlpha<C, T>
where
    C: Pixel<T>,
    T: Float,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<C, T, P> AsMut<P> for PreAlpha<C, T>
where
    C: Pixel<T>,
    T: Float,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<C, T: Float> Deref for PreAlpha<C, T> {
    type Target = C;

    fn deref(&self) -> &C {
        &self.color
    }
}

impl<C, T: Float> DerefMut for PreAlpha<C, T> {
    fn deref_mut(&mut self) -> &mut C {
        &mut self.color
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<C, T> bytemuck::Zeroable for PreAlpha<C, T>
where
    C: bytemuck::Zeroable,
    T: Float + bytemuck::Zeroable,
{
}

// Safety:
//  See `Alpha<C, T>`'s implementation of `Pod`.
#[cfg(feature = "bytemuck")]
unsafe impl<C, T> bytemuck::Pod for PreAlpha<C, T>
where
    C: bytemuck::Pod + Pixel<T>,
    T: Float + bytemuck::Pod,
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
