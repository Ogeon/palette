use std::ops::{Deref, DerefMut, Add, Sub, Mul, Div};

use num::Float;

use {Mix, Shade, GetHue, Hue, Saturate, Limited, clamp};

///An alpha component wrapper for colors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Alpha<C, T: Float> {
    ///The color.
    pub color: C,

    ///The transparency component. 0.0 is fully transparent and 1.0 is fully
    ///opaque.
    pub alpha: T,
}

impl<C, T: Float> Deref for Alpha<C, T> {
    type Target = C;

    fn deref(&self) -> &C {
        &self.color
    }
}

impl<C, T: Float> DerefMut for Alpha<C, T> {
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

impl<C: GetHue, T: Float> GetHue for Alpha<C, T> {
    type Hue = C::Hue;

    fn get_hue(&self) -> Option<C::Hue> {
        self.color.get_hue()
    }
}

impl<C: Hue, T: Float> Hue for Alpha<C, T> {
    fn with_hue(&self, hue: C::Hue) -> Alpha<C, T> {
        Alpha {
            color: self.color.with_hue(hue),
            alpha: self.alpha,
        }
    }

    fn shift_hue(&self, amount: C::Hue) -> Alpha<C, T> {
        Alpha {
            color: self.color.shift_hue(amount),
            alpha: self.alpha,
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

impl<C: Limited, T: Float> Limited for Alpha<C, T> {
    fn is_valid(&self) -> bool {
        self.color.is_valid() && self.alpha >= T::zero() && self.alpha <= T::one()
    }

    fn clamp(&self) -> Alpha<C, T> {
        Alpha {
            color: self.color.clamp(),
            alpha: clamp(self.alpha, T::zero(), T::one()),
        }
    }

    fn clamp_self(&mut self) {
        self.color.clamp_self();
        self.alpha = clamp(self.alpha, T::zero(), T::one());
    }
}

impl<C: Default, T: Float> Default for Alpha<C, T> {
    fn default() -> Alpha<C, T> {
        Alpha {
            color: C::default(),
            alpha: T::one(),
        }
    }
}

impl<C: Add, T: Float> Add for Alpha<C, T> {
    type Output = Alpha<C::Output, T>;

    fn add(self, other: Alpha<C, T>) -> Alpha<C::Output, T> {
        Alpha {
            color: self.color + other.color,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl<T: Float + Clone, C: Add<T>> Add<T> for Alpha<C, T> {
    type Output = Alpha<C::Output, T>;

    fn add(self, c: T) -> Alpha<C::Output, T> {
        Alpha {
            color: self.color + c.clone(),
            alpha: self.alpha + c,
        }
    }
}

impl<C: Sub, T: Float> Sub for Alpha<C, T> {
    type Output = Alpha<C::Output, T>;

    fn sub(self, other: Alpha<C, T>) -> Alpha<C::Output, T> {
        Alpha {
            color: self.color - other.color,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl<T: Float + Clone, C: Sub<T>> Sub<T> for Alpha<C, T> {
    type Output = Alpha<C::Output, T>;

    fn sub(self, c: T) -> Alpha<C::Output, T> {
        Alpha {
            color: self.color - c.clone(),
            alpha: self.alpha - c,
        }
    }
}

impl<C: Mul, T: Float> Mul for Alpha<C, T> {
    type Output = Alpha<C::Output, T>;

    fn mul(self, other: Alpha<C, T>) -> Alpha<C::Output, T> {
        Alpha {
            color: self.color * other.color,
            alpha: self.alpha * other.alpha,
        }
    }
}

impl<T: Float + Clone, C: Mul<T>> Mul<T> for Alpha<C, T> {
    type Output = Alpha<C::Output, T>;

    fn mul(self, c: T) -> Alpha<C::Output, T> {
        Alpha {
            color: self.color * c.clone(),
            alpha: self.alpha * c,
        }
    }
}

impl<C: Div, T: Float> Div for Alpha<C, T> {
    type Output = Alpha<C::Output, T>;

    fn div(self, other: Alpha<C, T>) -> Alpha<C::Output, T> {
        Alpha {
            color: self.color / other.color,
            alpha: self.alpha / other.alpha,
        }
    }
}

impl<T: Float + Clone, C: Div<T>> Div<T> for Alpha<C, T> {
    type Output = Alpha<C::Output, T>;

    fn div(self, c: T) -> Alpha<C::Output, T> {
        Alpha {
            color: self.color / c.clone(),
            alpha: self.alpha / c,
        }
    }
}

impl<C, T: Float> From<C> for Alpha<C, T> {
    fn from(color: C) -> Alpha<C, T> {
        Alpha {
            color: color,
            alpha: T::one(),
        }
    }
}
