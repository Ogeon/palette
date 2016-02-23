use std::ops::{Deref, DerefMut, Add, Sub, Mul, Div};

use num::{Float, One, Zero};

use approx::ApproxEq;

use {Mix, Shade, GetHue, Hue, Saturate, Limited, Blend, ComponentWise, ColorType, clamp};
use blend::PreAlpha;

///An alpha component wrapper for colors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Alpha<C: ColorType> {
    ///The color.
    pub color: C,

    ///The transparency component. 0.0 is fully transparent and 1.0 is fully
    ///opaque.
    pub alpha: C::Scalar,
}

impl<C: ColorType> ColorType for Alpha<C> {
    type Scalar = C::Scalar;
}

impl<C: ColorType> Deref for Alpha<C> {
    type Target = C;

    fn deref(&self) -> &C {
        &self.color
    }
}

impl<C: ColorType> DerefMut for Alpha<C> {
    fn deref_mut(&mut self) -> &mut C {
        &mut self.color
    }
}

impl<C: Mix> Mix for Alpha<C> {
    fn mix(&self, other: &Alpha<C>, factor: C::Scalar) -> Alpha<C> {
        Alpha {
            color: self.color.mix(&other.color, factor),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl<C: Shade> Shade for Alpha<C> {
    fn lighten(&self, amount: C::Scalar) -> Alpha<C> {
        Alpha {
            color: self.color.lighten(amount),
            alpha: self.alpha,
        }
    }
}

impl<C: GetHue + ColorType> GetHue for Alpha<C> {
    type Hue = C::Hue;

    fn get_hue(&self) -> Option<C::Hue> {
        self.color.get_hue()
    }
}

impl<C: Hue + ColorType> Hue for Alpha<C> {
    fn with_hue(&self, hue: C::Hue) -> Alpha<C> {
        Alpha {
            color: self.color.with_hue(hue),
            alpha: self.alpha,
        }
    }

    fn shift_hue(&self, amount: C::Hue) -> Alpha<C> {
        Alpha {
            color: self.color.shift_hue(amount),
            alpha: self.alpha,
        }
    }
}

impl<C: Saturate> Saturate for Alpha<C> {
    fn saturate(&self, factor: C::Scalar) -> Alpha<C> {
        Alpha {
            color: self.color.saturate(factor),
            alpha: self.alpha,
        }
    }
}

impl<C: Limited + ColorType> Limited for Alpha<C> {
    fn is_valid(&self) -> bool {
        self.color.is_valid() && self.alpha >= C::Scalar::zero() && self.alpha <= C::Scalar::one()
    }

    fn clamp(&self) -> Alpha<C> {
        Alpha {
            color: self.color.clamp(),
            alpha: clamp(self.alpha, C::Scalar::zero(), C::Scalar::one()),
        }
    }

    fn clamp_self(&mut self) {
        self.color.clamp_self();
        self.alpha = clamp(self.alpha, C::Scalar::zero(), C::Scalar::one());
    }
}

impl<C, T> Blend for Alpha<C> where
    C: Blend + ColorType<Scalar=T>,
    C::Color: ComponentWise<Scalar=T>,
    Alpha<C>: Into<Alpha<C::Color>> + From<Alpha<C::Color>>,
    T: Float,
{
    type Color = C::Color;

    fn into_premultiplied(self) -> PreAlpha<C::Color> {
        PreAlpha::<C::Color>::from(self.into())
    }

    fn from_premultiplied(color: PreAlpha<C::Color>) -> Alpha<C> {
        Alpha::<C::Color>::from(color).into()
    }
}

impl<C: ComponentWise> ComponentWise for Alpha<C> {
    fn component_wise<F: FnMut(C::Scalar, C::Scalar) -> C::Scalar>(&self, other: &Alpha<C>, mut f: F) -> Alpha<C> {
        Alpha {
            alpha: f(self.alpha, other.alpha),
            color: self.color.component_wise(&other.color, f),
        }
    }

    fn component_wise_self<F: FnMut(C::Scalar) -> C::Scalar>(&self, mut f: F) -> Alpha<C> {
        Alpha {
            alpha: f(self.alpha),
            color: self.color.component_wise_self(f),
        }
    }
}

impl<C: Default + ColorType> Default for Alpha<C> {
    fn default() -> Alpha<C> {
        Alpha {
            color: C::default(),
            alpha: C::Scalar::one(),
        }
    }
}

impl<C, T> ApproxEq for Alpha<C> where
    C: ApproxEq<Epsilon=T> + ColorType<Scalar=T>,
    T: ApproxEq<Epsilon=T> + Float,
{
    type Epsilon = T;

    fn default_epsilon() -> T {
        C::Scalar::default_epsilon()
    }

    fn default_max_relative() -> T {
        C::Scalar::default_max_relative()
    }

    fn default_max_ulps() -> u32 {
        C::Scalar::default_max_ulps()
    }

    fn relative_eq(&self, other: &Alpha<C>, epsilon: T, max_relative: T) -> bool {
        self.color.relative_eq(&other.color, epsilon, max_relative) &&
        self.alpha.relative_eq(&other.alpha, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Alpha<C>, epsilon: T, max_ulps: u32) -> bool{
        self.color.ulps_eq(&other.color, epsilon, max_ulps) &&
        self.alpha.ulps_eq(&other.alpha, epsilon, max_ulps)
    }
}

impl<C, T: Float> Add for Alpha<C> where
    C: Add + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
{
    type Output = Alpha<C::Output>;

    fn add(self, other: Alpha<C>) -> Alpha<C::Output> {
        Alpha {
            color: self.color + other.color,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl<C, T: Float> Add<T> for Alpha<C> where
    C: Add<T> + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
{
    type Output = Alpha<C::Output>;

    fn add(self, c: C::Scalar) -> Alpha<C::Output> {
        Alpha {
            color: self.color + c,
            alpha: self.alpha + c,
        }
    }
}

impl<C, T: Float> Sub for Alpha<C> where
    C: Sub + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
{
    type Output = Alpha<C::Output>;

    fn sub(self, other: Alpha<C>) -> Alpha<C::Output> {
        Alpha {
            color: self.color - other.color,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl<C, T: Float> Sub<T> for Alpha<C> where
    C: Sub<T> + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
{
    type Output = Alpha<C::Output>;

    fn sub(self, c: C::Scalar) -> Alpha<C::Output> {
        Alpha {
            color: self.color - c,
            alpha: self.alpha - c,
        }
    }
}

impl<C, T: Float> Mul for Alpha<C> where
    C: Mul + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
{
    type Output = Alpha<C::Output>;

    fn mul(self, other: Alpha<C>) -> Alpha<C::Output> {
        Alpha {
            color: self.color * other.color,
            alpha: self.alpha * other.alpha,
        }
    }
}

impl<C, T: Float> Mul<T> for Alpha<C> where
    C: Mul<T> + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
{
    type Output = Alpha<C::Output>;

    fn mul(self, c: C::Scalar) -> Alpha<C::Output> {
        Alpha {
            color: self.color * c,
            alpha: self.alpha * c,
        }
    }
}

impl<C, T: Float> Div for Alpha<C> where
    C: Div + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
{
    type Output = Alpha<C::Output>;

    fn div(self, other: Alpha<C>) -> Alpha<C::Output> {
        Alpha {
            color: self.color / other.color,
            alpha: self.alpha / other.alpha,
        }
    }
}

impl<C, T: Float> Div<T> for Alpha<C> where
    C: Div<T> + ColorType<Scalar=T>,
    C::Output: ColorType<Scalar=T>,
{
    type Output = Alpha<C::Output>;

    fn div(self, c: C::Scalar) -> Alpha<C::Output> {
        Alpha {
            color: self.color / c,
            alpha: self.alpha / c,
        }
    }
}

impl<C: ColorType> From<C> for Alpha<C> {
    fn from(color: C) -> Alpha<C> {
        Alpha {
            color: color,
            alpha: C::Scalar::one(),
        }
    }
}
