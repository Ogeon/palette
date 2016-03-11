//!Color blending and blending equations.
//!
//!Palette offers both OpenGL style blending equations, as well as most of the
//!SVG composition operators (also common in photo manipulation software). The
//!composition operators are all implemented in the
//![`Blend`](trait.Blend.html) trait, and ready to use with any appropriate
//!color type:
//!
//!```
//!use palette::{LinSrgba, Blend};
//!
//!let a = LinSrgba::new(0.2, 0.5, 0.1, 0.8);
//!let b = LinSrgba::new(0.6, 0.3, 0.5, 0.1);
//!let c = a.overlay(b);
//!```
//!
//!Blending equations can be defined using the
//![`Equations`](struct.Equations.html) type, which is then passed to the
//!`blend` function, from the `Blend` trait:
//!
//!```
//!use palette::{LinSrgba, Blend};
//!use palette::blend::{Equations, Parameter};
//!
//!let blend_mode = Equations::from_parameters(
//!    Parameter::SourceAlpha,
//!    Parameter::OneMinusSourceAlpha
//!);
//!
//!let a = LinSrgba::new(0.2, 0.5, 0.1, 0.8);
//!let b = LinSrgba::new(0.6, 0.3, 0.5, 0.1);
//!let c = a.blend(b, blend_mode);
//!```
//!
//!Note that blending will use [premultiplied alpha](struct.PreAlpha.html),
//!which may result in loss of some color information in some cases. One such
//!case is that a completely transparent resultant color will become black.

use ComponentWise;

pub use self::equations::{Equations, Equation, Parameters, Parameter};
pub use self::pre_alpha::PreAlpha;
pub use self::blend::Blend;

mod equations;
mod pre_alpha;
mod blend;

#[cfg(test)]
mod test;

///A trait for custom blend functions.
pub trait BlendFunction<C: Blend<Color=C> + ComponentWise> {
    ///Apply this blend function to a pair of colors.
    fn apply_to(self, source: PreAlpha<C, C::Scalar>, destination: PreAlpha<C, C::Scalar>) -> PreAlpha<C, C::Scalar>;
}

impl<C, F> BlendFunction<C> for F where
    C: Blend<Color=C> + ComponentWise,
    F: FnOnce(PreAlpha<C, C::Scalar>, PreAlpha<C, C::Scalar>) -> PreAlpha<C, C::Scalar>,
{
    fn apply_to(self, source: PreAlpha<C, C::Scalar>, destination: PreAlpha<C, C::Scalar>) -> PreAlpha<C, C::Scalar> {
        (self)(source, destination)
    }
}
