use crate::{Alpha, OklabHue};

use super::Oklrch;

/// Oklrch with an alpha component. See the [`Oklrcha` implementation in
/// `Alpha`](crate::Alpha#Oklrcha).
pub type Oklrcha<T = f32> = Alpha<Oklrch<T>, T>;

///<span id="Oklrcha"></span>[`Oklrcha`](crate::Oklrcha) implementations.
impl<T, A> Alpha<Oklrch<T>, A> {
    /// Create an Oklrch color with transparency.
    pub fn new<H: Into<OklabHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Alpha {
            color: Oklrch::new(l, chroma, hue),
            alpha,
        }
    }

    /// Create an `Oklrcha` color. This is the same as `Oklrcha::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(l: T, chroma: T, hue: OklabHue<T>, alpha: A) -> Self {
        Alpha {
            color: Oklrch::new_const(l, chroma, hue),
            alpha,
        }
    }

    /// Convert to a `(Lr, C, h, alpha)` tuple.
    pub fn into_components(self) -> (T, T, OklabHue<T>, A) {
        (self.color.l, self.color.chroma, self.color.hue, self.alpha)
    }

    /// Convert from a `(Lr, C, h, alpha)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((l, chroma, hue, alpha): (T, T, H, A)) -> Self {
        Self::new(l, chroma, hue, alpha)
    }
}
