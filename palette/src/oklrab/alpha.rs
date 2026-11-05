use crate::alpha::Alpha;
use crate::oklrab::Oklrab;

/// Oklrab with an alpha component.
pub type Oklraba<T = f32> = Alpha<Oklrab<T>, T>;

///<span id="Oklraba"></span>[`Oklraba`](crate::Oklraba) implementations.
impl<T, A> Alpha<Oklrab<T>, A> {
    /// Create an Oklrab color with transparency.
    pub const fn new(l: T, a: T, b: T, alpha: A) -> Self {
        Alpha {
            color: Oklrab::new(l, a, b),
            alpha,
        }
    }

    /// Convert to a `(Lr, a, b, alpha)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (self.color.l, self.color.a, self.color.b, self.alpha)
    }

    /// Convert from a `(Lr, a, b, alpha)` tuple.
    pub fn from_components((l, a, b, alpha): (T, T, T, A)) -> Self {
        Self::new(l, a, b, alpha)
    }
}
