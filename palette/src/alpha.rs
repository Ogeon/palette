use core::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use core::fmt;

use float::Float;

use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use {clamp, Blend, Component, ComponentWise, GetHue, Hue, Limited, Mix, Pixel, Saturate, Shade};
use blend::PreAlpha;
use encoding::pixel::RawPixel;

///An alpha component wrapper for colors.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Alpha<C, T> {
    ///The color.
    #[cfg_attr(feature = "serializing", serde(flatten))]
    pub color: C,

    ///The transparency component. 0.0 is fully transparent and 1.0 is fully
    ///opaque.
    pub alpha: T,
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

impl<C: Mix> Mix for Alpha<C, C::Scalar> {
    type Scalar = C::Scalar;

    fn mix(&self, other: &Alpha<C, C::Scalar>, factor: C::Scalar) -> Alpha<C, C::Scalar> {
        Alpha {
            color: self.color.mix(&other.color, factor),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl<C: Shade> Shade for Alpha<C, C::Scalar> {
    type Scalar = C::Scalar;

    fn lighten(&self, amount: C::Scalar) -> Alpha<C, C::Scalar> {
        Alpha {
            color: self.color.lighten(amount),
            alpha: self.alpha,
        }
    }
}

impl<C: GetHue, T> GetHue for Alpha<C, T> {
    type Hue = C::Hue;

    fn get_hue(&self) -> Option<C::Hue> {
        self.color.get_hue()
    }
}

impl<C: Hue, T: Clone> Hue for Alpha<C, T> {
    fn with_hue<H: Into<C::Hue>>(&self, hue: H) -> Alpha<C, T> {
        Alpha {
            color: self.color.with_hue(hue),
            alpha: self.alpha.clone(),
        }
    }

    fn shift_hue<H: Into<C::Hue>>(&self, amount: H) -> Alpha<C, T> {
        Alpha {
            color: self.color.shift_hue(amount),
            alpha: self.alpha.clone(),
        }
    }
}

impl<C: Saturate> Saturate for Alpha<C, C::Scalar> {
    type Scalar = C::Scalar;

    fn saturate(&self, factor: C::Scalar) -> Alpha<C, C::Scalar> {
        Alpha {
            color: self.color.saturate(factor),
            alpha: self.alpha,
        }
    }
}

impl<C: Limited, T: Component> Limited for Alpha<C, T> {
    fn is_valid(&self) -> bool {
        self.color.is_valid() && self.alpha >= T::zero() && self.alpha <= T::max_intensity()
    }

    fn clamp(&self) -> Alpha<C, T> {
        Alpha {
            color: self.color.clamp(),
            alpha: clamp(self.alpha, T::zero(), T::max_intensity()),
        }
    }

    fn clamp_self(&mut self) {
        self.color.clamp_self();
        self.alpha = clamp(self.alpha, T::zero(), T::max_intensity());
    }
}

impl<C: Blend, T: Float> Blend for Alpha<C, T>
where
    C::Color: ComponentWise<Scalar = T>,
    Alpha<C, T>: Into<Alpha<C::Color, T>> + From<Alpha<C::Color, T>>,
{
    type Color = C::Color;

    fn into_premultiplied(self) -> PreAlpha<C::Color, T> {
        PreAlpha::<C::Color, T>::from(self.into())
    }

    fn from_premultiplied(color: PreAlpha<C::Color, T>) -> Alpha<C, T> {
        Alpha::<C::Color, T>::from(color).into()
    }
}

impl<C: ComponentWise<Scalar = T>, T: Clone> ComponentWise for Alpha<C, T> {
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &Alpha<C, T>, mut f: F) -> Alpha<C, T> {
        Alpha {
            color: self.color.component_wise(&other.color, &mut f),
            alpha: f(self.alpha.clone(), other.alpha.clone()),
        }
    }

    fn component_wise_self<F: FnMut(T) -> T>(&self, mut f: F) -> Alpha<C, T> {
        Alpha {
            color: self.color.component_wise_self(&mut f),
            alpha: f(self.alpha.clone()),
        }
    }
}

unsafe impl<T, C: Pixel<T>> Pixel<T> for Alpha<C, T> {
    const CHANNELS: usize = C::CHANNELS + 1;
}

impl<C: Default, T: Component> Default for Alpha<C, T> {
    fn default() -> Alpha<C, T> {
        Alpha {
            color: C::default(),
            alpha: T::max_intensity(),
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

impl<C: Add, T: Float> Add for Alpha<C, T> {
    type Output = Alpha<C::Output, <T as Add>::Output>;

    fn add(self, other: Alpha<C, T>) -> Self::Output {
        Alpha {
            color: self.color + other.color,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl<T: Add + Clone, C: Add<T>> Add<T> for Alpha<C, T> {
    type Output = Alpha<C::Output, <T as Add>::Output>;

    fn add(self, c: T) -> Self::Output {
        Alpha {
            color: self.color + c.clone(),
            alpha: self.alpha + c,
        }
    }
}

impl<C: AddAssign, T: Float + AddAssign> AddAssign for Alpha<C, T> {
    fn add_assign(&mut self, other: Alpha<C, T>) {
        self.color += other.color;
        self.alpha += other.alpha;
    }
}

impl<T: AddAssign + Copy, C: AddAssign<T>> AddAssign<T> for Alpha<C, T> {
    fn add_assign(&mut self, c: T) {
        self.color += c;
        self.alpha += c;
    }
}

impl<C: Sub, T: Float> Sub for Alpha<C, T> {
    type Output = Alpha<C::Output, <T as Sub>::Output>;

    fn sub(self, other: Alpha<C, T>) -> Self::Output {
        Alpha {
            color: self.color - other.color,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl<T: Sub + Clone, C: Sub<T>> Sub<T> for Alpha<C, T> {
    type Output = Alpha<C::Output, <T as Sub>::Output>;

    fn sub(self, c: T) -> Self::Output {
        Alpha {
            color: self.color - c.clone(),
            alpha: self.alpha - c,
        }
    }
}

impl<C: SubAssign, T: Float + SubAssign> SubAssign for Alpha<C, T> {
    fn sub_assign(&mut self, other: Alpha<C, T>) {
        self.color -= other.color;
        self.alpha -= other.alpha;
    }
}

impl<T: SubAssign + Copy, C: SubAssign<T>> SubAssign<T> for Alpha<C, T> {
    fn sub_assign(&mut self, c: T) {
        self.color -= c;
        self.alpha -= c;
    }
}

impl<C: Mul, T: Float> Mul for Alpha<C, T> {
    type Output = Alpha<C::Output, <T as Mul>::Output>;

    fn mul(self, other: Alpha<C, T>) -> Self::Output {
        Alpha {
            color: self.color * other.color,
            alpha: self.alpha * other.alpha,
        }
    }
}

impl<T: Mul + Clone, C: Mul<T>> Mul<T> for Alpha<C, T> {
    type Output = Alpha<C::Output, <T as Mul>::Output>;

    fn mul(self, c: T) -> Self::Output {
        Alpha {
            color: self.color * c.clone(),
            alpha: self.alpha * c,
        }
    }
}

impl<C: MulAssign, T: Float + MulAssign> MulAssign for Alpha<C, T> {
    fn mul_assign(&mut self, other: Alpha<C, T>) {
        self.color *= other.color;
        self.alpha *= other.alpha;
    }
}

impl<T: MulAssign + Copy, C: MulAssign<T>> MulAssign<T> for Alpha<C, T> {
    fn mul_assign(&mut self, c: T) {
        self.color *= c;
        self.alpha *= c;
    }
}

impl<C: Div, T: Float> Div for Alpha<C, T> {
    type Output = Alpha<C::Output, <T as Div>::Output>;

    fn div(self, other: Alpha<C, T>) -> Self::Output {
        Alpha {
            color: self.color / other.color,
            alpha: self.alpha / other.alpha,
        }
    }
}

impl<T: Div + Clone, C: Div<T>> Div<T> for Alpha<C, T> {
    type Output = Alpha<C::Output, <T as Div>::Output>;

    fn div(self, c: T) -> Self::Output {
        Alpha {
            color: self.color / c.clone(),
            alpha: self.alpha / c,
        }
    }
}

impl<C: DivAssign, T: Float + DivAssign> DivAssign for Alpha<C, T> {
    fn div_assign(&mut self, other: Alpha<C, T>) {
        self.color /= other.color;
        self.alpha /= other.alpha;
    }
}

impl<T: DivAssign + Copy, C: DivAssign<T>> DivAssign<T> for Alpha<C, T> {
    fn div_assign(&mut self, c: T) {
        self.color /= c;
        self.alpha /= c;
    }
}

impl<C, T, P> AsRef<P> for Alpha<C, T>
where
    C: Pixel<T>,
    P: RawPixel<T> + ?Sized,
{
    fn as_ref(&self) -> &P {
        self.as_raw()
    }
}

impl<C, T, P> AsMut<P> for Alpha<C, T>
where
    C: Pixel<T>,
    P: RawPixel<T> + ?Sized,
{
    fn as_mut(&mut self) -> &mut P {
        self.as_raw_mut()
    }
}

impl<C, T: Component> From<C> for Alpha<C, T> {
    fn from(color: C) -> Alpha<C, T> {
        Alpha {
            color: color,
            alpha: T::max_intensity(),
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

#[cfg(test)]
mod test {
    use rgb::Rgba;
    use encoding::Srgb;

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
}
