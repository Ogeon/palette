use std::ops::{Add, Sub, Mul, Div, Deref, DerefMut};
use approx::ApproxEq;
use num::Float;

use {Alpha, ComponentWise, Mix, Blend, clamp};

///Premultiplied alpha wrapper.
///
///Premultiplied colors are commonly used in composition algorithms to
///simplify the calculations. It may also be preferred when interpolating
///between colors, which is one of the reasons why it's offered as a separate
///type. The other reason is to make it easier to avoid unnecessary
///computations in composition chains.
///
///```
///use palette::{Blend, RgbLinear, RgbaLinear};
///use palette::blend::PreAlpha;
///
///let a = PreAlpha::from(Rgba::new(0.4, 0.5, 0.5, 0.3));
///let b = PreAlpha::from(Rgba::new(0.3, 0.8, 0.4, 0.4));
///let c = PreAlpha::from(Rgba::new(0.7, 0.1, 0.8, 0.8));
///
///let res = Rgb::from_premultiplied(a.screen(b).overlay(c));
///```
///
///Note that converting to and from premultiplied alpha will cause the alpha
///component to be clamped to [0.0, 1.0].
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PreAlpha<C, T: Float> {
    ///The premultiplied color components (`original.color * original.alpha`).
    pub color: C,

    ///The transparency component. 0.0 is fully transparent and 1.0 is fully
    ///opaque.
    pub alpha: T,
}

impl<C, T> From<Alpha<C, T>> for PreAlpha<C, T> where
    C: ComponentWise<Scalar=T>,
    T: Float,
{
    fn from(color: Alpha<C, T>) -> PreAlpha<C, T> {
        let alpha = clamp(color.alpha, T::zero(), T::one());

        PreAlpha{
            color: color.color.component_wise_self(|a| a * alpha),
            alpha: alpha
        }
    }
}

impl<C, T> From<PreAlpha<C, T>> for Alpha<C, T> where
    C: ComponentWise<Scalar=T>,
    T: Float,
{
    fn from(color: PreAlpha<C, T>) -> Alpha<C, T> {
        let alpha = clamp(color.alpha, T::zero(), T::one());

        let color = color.color.component_wise_self(|a| if alpha.is_normal() {
            a / alpha
        } else {
            T::zero()
        });

        Alpha {
            color: color,
            alpha: alpha,
        }
    }
}

impl<C, T> Blend for PreAlpha<C, T> where
    C: Blend<Color=C> + ComponentWise<Scalar=T>,
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

impl<C: ComponentWise<Scalar=T>, T: Float> ComponentWise for PreAlpha<C, T> {
    type Scalar = T;

    fn component_wise<F: FnMut(T, T) -> T>(&self, other: &PreAlpha<C, T>, mut f: F) -> PreAlpha<C, T> {
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
}impl<C, T> ApproxEq for PreAlpha<C, T> where
    C: ApproxEq<Epsilon=T::Epsilon>,
    T: ApproxEq + Float,
    T::Epsilon: Copy,
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

    fn relative_eq(&self, other: &PreAlpha<C, T>, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        self.color.relative_eq(&other.color, epsilon, max_relative) &&
        self.alpha.relative_eq(&other.alpha, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &PreAlpha<C, T>, epsilon: Self::Epsilon, max_ulps: u32) -> bool{
        self.color.ulps_eq(&other.color, epsilon, max_ulps) &&
        self.alpha.ulps_eq(&other.alpha, epsilon, max_ulps)
    }
}

impl<C: Add, T: Float> Add for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn add(self, other: PreAlpha<C, T>) -> PreAlpha<C::Output, T> {
        PreAlpha {
            color: self.color + other.color,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl<T: Float, C: Add<T>> Add<T> for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn add(self, c: T) -> PreAlpha<C::Output, T> {
        PreAlpha {
            color: self.color + c,
            alpha: self.alpha + c,
        }
    }
}

impl<C: Sub, T: Float> Sub for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn sub(self, other: PreAlpha<C, T>) -> PreAlpha<C::Output, T> {
        PreAlpha {
            color: self.color - other.color,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl<T: Float, C: Sub<T>> Sub<T> for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn sub(self, c: T) -> PreAlpha<C::Output, T> {
        PreAlpha {
            color: self.color - c,
            alpha: self.alpha - c,
        }
    }
}

impl<C: Mul, T: Float> Mul for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn mul(self, other: PreAlpha<C, T>) -> PreAlpha<C::Output, T> {
        PreAlpha {
            color: self.color * other.color,
            alpha: self.alpha * other.alpha,
        }
    }
}

impl<T: Float, C: Mul<T>> Mul<T> for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn mul(self, c: T) -> PreAlpha<C::Output, T> {
        PreAlpha {
            color: self.color * c,
            alpha: self.alpha * c,
        }
    }
}

impl<C: Div, T: Float> Div for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn div(self, other: PreAlpha<C, T>) -> PreAlpha<C::Output, T> {
        PreAlpha {
            color: self.color / other.color,
            alpha: self.alpha / other.alpha,
        }
    }
}

impl<T: Float, C: Div<T>> Div<T> for PreAlpha<C, T> {
    type Output = PreAlpha<C::Output, T>;

    fn div(self, c: T) -> PreAlpha<C::Output, T> {
        PreAlpha {
            color: self.color / c,
            alpha: self.alpha / c,
        }
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
