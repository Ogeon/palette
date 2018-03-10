use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

use num_traits::Float;

use approx::ApproxEq;

use {clamp, Blend, Component, ComponentWise, GetHue, Hue, Limited, Mix, Pixel, Saturate, Shade};
use blend::PreAlpha;
use encoding::pixel::RawPixel;

///An alpha component wrapper for colors.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Alpha<C, T> {
    ///The color.
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

impl<C, T> ApproxEq for Alpha<C, T>
where
    C: ApproxEq<Epsilon = T::Epsilon>,
    T: ApproxEq,
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
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
