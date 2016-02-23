use num::{Float, Zero, One};

use {ComponentWise, Blend, ColorType};
use blend::{PreAlpha, BlendFunction};

///A pair of blending equations and corresponding parameters.
///
///The `Equations` type is similar to how blending works in OpenGL, where a
///blend function has can be written as `e(sp * S, dp * D)`. `e` is the
///equation (like `s + d`), `sp` and `dp` are the source and destination
///parameters, and `S` and `D` are the source and destination colors.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Equations {
    ///The equation for the color components.
    pub color_equation: Equation,

    ///The equation for the alpha component.
    pub alpha_equation: Equation,

    ///The parameters for the color components.
    pub color_parameters: Parameters,

    ///The parameters for the alpha component.
    pub alpha_parameters: Parameters,
}

impl Equations {
    ///Create a pair of blending equations, where all the parameters are
    ///`One`.
    pub fn from_equations(color: Equation, alpha: Equation) -> Equations {
        Equations {
            color_equation: color,
            alpha_equation: alpha,
            color_parameters: Parameters {
                source: Parameter::One,
                destination: Parameter::One,
            },
            alpha_parameters: Parameters {
                source: Parameter::One,
                destination: Parameter::One,
            },
        }
    }

    ///Create a pair of additive blending equations with the provided
    ///parameters.
    pub fn from_parameters(source: Parameter, destination: Parameter) -> Equations {
        Equations {
            color_equation: Equation::Add,
            alpha_equation: Equation::Add,
            color_parameters: Parameters {
                source: source,
                destination: destination,
            },
            alpha_parameters: Parameters {
                source: source,
                destination: destination,
            },
        }
    }
}

impl<C: Blend<Color=C> + ComponentWise + Clone> BlendFunction<C> for Equations {
    fn apply_to(self, source: PreAlpha<C>, destination: PreAlpha<C>) -> PreAlpha<C> {
        let col_src_param = self.color_parameters.source.apply_to(source.clone(), destination.clone());
        let col_dst_param = self.color_parameters.destination.apply_to(source.clone(), destination.clone());
        let alpha_src_param = self.alpha_parameters.source.apply_to(source.clone(), destination.clone());
        let alpha_dst_param = self.alpha_parameters.destination.apply_to(source.clone(), destination.clone());

        let src_color = col_src_param.mul_color(source.color.clone());
        let dst_color = col_dst_param.mul_color(destination.color.clone());
        let src_alpha = alpha_src_param.mul_constant(source.alpha);
        let dst_alpha = alpha_dst_param.mul_constant(destination.alpha);

        let color = match self.color_equation {
            Equation::Add => src_color.component_wise(&dst_color, |a, b| a + b),
            Equation::Subtract => src_color.component_wise(&dst_color, |a, b| a - b),
            Equation::ReverseSubtract => dst_color.component_wise(&src_color, |a, b| a - b),
            Equation::Min => source.color.component_wise(&destination.color, |a, b| a.min(b)),
            Equation::Max => source.color.component_wise(&destination.color, |a, b| a.max(b)),
        };

        let alpha = match self.alpha_equation {
            Equation::Add => src_alpha + dst_alpha,
            Equation::Subtract => src_alpha - dst_alpha,
            Equation::ReverseSubtract => dst_alpha - src_alpha,
            Equation::Min => source.alpha.min(destination.alpha),
            Equation::Max => source.alpha.max(destination.alpha),
        };

        PreAlpha {
            color: color,
            alpha: alpha,
        }
    }
}

///A blending equation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Equation {
    ///Add the source and destination, according to `sp * S + dp * D`.
    Add,

    ///Subtract the destination from the source, according to `sp * S - dp * D`.
    Subtract,

    ///Subtract the source from the destination, according to `dp * D - sp * S`.
    ReverseSubtract,

    ///Create a color where each component is the smallest of each of the
    ///source and destination components. A.k.a. component wise min. The
    ///parameters are ignored.
    Min,

    ///Create a color where each component is the largest of each of the
    ///source and destination components. A.k.a. component wise max. The
    ///parameters are ignored.
    Max,
}

///A pair of source and destination parameters.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Parameters {
    ///The source parameter.
    pub source: Parameter,

    ///The destination parameter.
    pub destination: Parameter,
}

///A blending parameter.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Parameter {
    ///A simple 1.
    One,

    ///A simple 0.
    Zero,

    ///The source color, or alpha.
    SourceColor,

    ///One minus the source color, or alpha.
    OneMinusSourceColor,

    ///The destination color, or alpha.
    DestinationColor,

    ///One minus the destination color, or alpha.
    OneMinusDestinationColor,

    ///The source alpha.
    SourceAlpha,

    ///One minus the source alpha.
    OneMinusSourceAlpha,

    ///The destination alpha.
    DestinationAlpha,

    ///One minus the destination alpha.
    OneMinusDestinationAlpha,
}

impl Parameter {
    fn apply_to<C: ColorType>(&self, source: PreAlpha<C>, destination: PreAlpha<C>) -> ParamOut<C> where
        PreAlpha<C>: ComponentWise<Scalar=C::Scalar>,
    {
        match *self {
            Parameter::One => ParamOut::Constant(C::Scalar::one()),
            Parameter::Zero => ParamOut::Constant(C::Scalar::zero()),
            Parameter::SourceColor => ParamOut::Color(source),
            Parameter::OneMinusSourceColor => ParamOut::Color(source.component_wise_self(|a| C::Scalar::one() - a)),
            Parameter::DestinationColor => ParamOut::Color(destination),
            Parameter::OneMinusDestinationColor => ParamOut::Color(destination.component_wise_self(|a| C::Scalar::one() - a)),
            Parameter::SourceAlpha => ParamOut::Constant(source.alpha),
            Parameter::OneMinusSourceAlpha => ParamOut::Constant(C::Scalar::one() - source.alpha),
            Parameter::DestinationAlpha => ParamOut::Constant(destination.alpha),
            Parameter::OneMinusDestinationAlpha => ParamOut::Constant(C::Scalar::one() - destination.alpha),
        }
    }
}

enum ParamOut<C: ColorType> {
    Color(PreAlpha<C>),
    Constant(C::Scalar),
}

impl<C: ComponentWise> ParamOut<C> {
    fn mul_constant(self, other: C::Scalar) -> C::Scalar {
        match self {
            ParamOut::Color(c) => c.alpha * other,
            ParamOut::Constant(c) => c * other,
        }
    }

    fn mul_color(self, other: C) -> C {
        match self {
            ParamOut::Color(c) => other.component_wise(&c.color, |a, b| a * b),
            ParamOut::Constant(c) => other.component_wise_self(|a| a * c),
        }
    }
}
