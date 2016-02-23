use std::ops::{Add, Sub, Mul, Div, Deref, DerefMut};
use approx::ApproxEq;
use num::{Float, Zero, One};

use {Alpha, ComponentWise, Mix, Blend, ColorType, clamp};

///Premultiplied alpha wrapper.
///
///Premultiplied colors are commonly used in composition algorithms to
///simplify the calculations. It may also be preferred when interpolating
///between colors, which is one of the reasons why it's offered as a separate
///type. The other reason is to make it easier to avoid unnecessary
///computations in composition chains.
///
///```
///use palette::{Blend, Rgb, Rgba};
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
pub struct PreAlpha<C: ColorType> {
    ///The premultiplied color components (`original.color * original.alpha`).
    pub color: C,

    ///The transparency component. 0.0 is fully transparent and 1.0 is fully
    ///opaque.
    pub alpha: C::Scalar,
}

impl<C> From<Alpha<C>> for PreAlpha<C> where
    C: ComponentWise,
{
    fn from(color: Alpha<C>) -> PreAlpha<C> {
        let alpha = clamp(color.alpha, C::Scalar::zero(), C::Scalar::one());

        PreAlpha{
            color: color.color.component_wise_self(|a| a * alpha),
            alpha: alpha
        }
    }
}

impl<C: ComponentWise> From<PreAlpha<C>> for Alpha<C> {
    fn from(color: PreAlpha<C>) -> Alpha<C> {
        let alpha = clamp(color.alpha, C::Scalar::zero(), C::Scalar::one());

        let color = color.color.component_wise_self(|a| if alpha.is_normal() {
            a / alpha
        } else {
            C::Scalar::zero()
        });

        Alpha {
            color: color,
            alpha: alpha,
        }
    }
}

impl<C: Blend<Color=C> + ComponentWise> Blend for PreAlpha<C> {
    type Color = C;

    fn into_premultiplied(self) -> PreAlpha<C> {
        self
    }

    fn from_premultiplied(color: PreAlpha<C>) -> PreAlpha<C> {
        color
    }
}

impl<C: Mix> Mix for PreAlpha<C> {
    fn mix(&self, other: &PreAlpha<C>, factor: C::Scalar) -> PreAlpha<C> {
        PreAlpha {
            color: self.color.mix(&other.color, factor),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl<C: ComponentWise> ComponentWise for PreAlpha<C> {
    fn component_wise<F: FnMut(C::Scalar, C::Scalar) -> C::Scalar>(&self, other: &PreAlpha<C>, mut f: F) -> PreAlpha<C> {
        PreAlpha {
            alpha: f(self.alpha, other.alpha),
            color: self.color.component_wise(&other.color, f),
        }
    }

    fn component_wise_self<F: FnMut(C::Scalar) -> C::Scalar>(&self, mut f: F) -> PreAlpha<C> {
        PreAlpha {
            alpha: f(self.alpha),
            color: self.color.component_wise_self(f),
        }
    }
}

impl<C: ColorType> ColorType for PreAlpha<C> {
    type Scalar = C::Scalar;
}

impl<C, T> ApproxEq for PreAlpha<C> where
    C: ApproxEq<Epsilon=T> + ColorType<Scalar=T>,
    T: ApproxEq<Epsilon=T> + Float,
{
    type Epsilon = T;

    fn default_epsilon() -> T {
        T::default_epsilon()
    }

    fn default_max_relative() -> T {
        T::default_max_relative()
    }

    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn relative_eq(&self, other: &PreAlpha<C>, epsilon: T, max_relative: T) -> bool {
        self.color.relative_eq(&other.color, epsilon, max_relative) &&
        self.alpha.relative_eq(&other.alpha, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &PreAlpha<C>, epsilon: T, max_ulps: u32) -> bool{
        self.color.ulps_eq(&other.color, epsilon, max_ulps) &&
        self.alpha.ulps_eq(&other.alpha, epsilon, max_ulps)
    }
}

impl<C, T> Add for PreAlpha<C> where
    C: Add + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
    T: Float,
{
    type Output = PreAlpha<C::Output>;

    fn add(self, other: PreAlpha<C>) -> PreAlpha<C::Output> {
        PreAlpha {
            color: self.color + other.color,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl<C, T> Add<T> for PreAlpha<C> where
    C: Add<T> + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
    T: Float,
{
    type Output = PreAlpha<C::Output>;

    fn add(self, c: T) -> PreAlpha<C::Output> {
        PreAlpha {
            color: self.color + c,
            alpha: self.alpha + c,
        }
    }
}

impl<C, T> Sub for PreAlpha<C> where
    C: Sub + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
    T: Float,
{
    type Output = PreAlpha<C::Output>;

    fn sub(self, other: PreAlpha<C>) -> PreAlpha<C::Output> {
        PreAlpha {
            color: self.color - other.color,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl<C, T> Sub<T> for PreAlpha<C> where
    C: Sub<T> + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
    T: Float,
{
    type Output = PreAlpha<C::Output>;

    fn sub(self, c: T) -> PreAlpha<C::Output> {
        PreAlpha {
            color: self.color - c,
            alpha: self.alpha - c,
        }
    }
}

impl<C, T> Mul for PreAlpha<C> where
    C: Mul + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
    T: Float,
{
    type Output = PreAlpha<C::Output>;

    fn mul(self, other: PreAlpha<C>) -> PreAlpha<C::Output> {
        PreAlpha {
            color: self.color * other.color,
            alpha: self.alpha * other.alpha,
        }
    }
}

impl<C, T> Mul<T> for PreAlpha<C> where
    C: Mul<T> + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
    T: Float,
{
    type Output = PreAlpha<C::Output>;

    fn mul(self, c: T) -> PreAlpha<C::Output> {
        PreAlpha {
            color: self.color * c,
            alpha: self.alpha * c,
        }
    }
}

impl<C, T> Div for PreAlpha<C> where
    C: Div + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
    T: Float,
{
    type Output = PreAlpha<C::Output>;

    fn div(self, other: PreAlpha<C>) -> PreAlpha<C::Output> {
        PreAlpha {
            color: self.color / other.color,
            alpha: self.alpha / other.alpha,
        }
    }
}

impl<C, T> Div<T> for PreAlpha<C> where
    C: Div<T> + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
    T: Float,
{
    type Output = PreAlpha<C::Output>;

    fn div(self, c: T) -> PreAlpha<C::Output> {
        PreAlpha {
            color: self.color / c,
            alpha: self.alpha / c,
        }
    }
}

impl<C: ColorType> Deref for PreAlpha<C> {
    type Target = C;

    fn deref(&self) -> &C {
        &self.color
    }
}

impl<C: ColorType> DerefMut for PreAlpha<C> {
    fn deref_mut(&mut self) -> &mut C {
        &mut self.color
    }
}
