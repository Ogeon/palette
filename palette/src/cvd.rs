//! Simulate various types of color vision deficiency.

use crate::{
    cvd::{cone_response::*, simulation::*},
    lms::matrix::SmithPokorny,
};

pub mod cone_response;
pub mod simulation;

/// Simulator for protanopia, a form of dichromacy correlated to a missing or
/// non-functional long (red) cone.
///
/// By default this uses the [`Vienot1999`] simulation method for the sake of
/// efficiency and accuracy with extreme values.
pub type ProtanopiaSimul<S = Vienot1999, M = SmithPokorny> = DichromacySimul<Protan, S, M>;

/// Simulator for deuteranopia, a form of dichromacy correlated to a missing or
/// non-functional medium (green) cone.
///
/// By default this uses the [`Vienot1999`] simulation method for the sake of
/// efficiency and accuracy with extreme values.
pub type DeuteranopiaSimul<S = Vienot1999, M = SmithPokorny> = DichromacySimul<Deutan, S, M>;

/// Simulator for tritanopia, a form of dichromacy correlated to a missing or
/// non-functional short (blue) cone.
///
/// By default this uses the [`Brettel1997`] since other methods are much less
/// accurate for tritanopia.
pub type TritanopiaSimul<S = Brettel1997, M = SmithPokorny> = DichromacySimul<Tritan, S, M>;

/// Simulator for protanomaly, a form of anomalous trichromacy correlated to an
/// anomalous long (red) cone.
///
/// The current default implementation uses linear interpolation, which is not
/// ideal, so this default implementation may change in the future.
pub type ProtanomalySimul<S = Vienot1999, M = SmithPokorny> =
    AnomalousTrichromacySimul<Protan, S, M>;

/// Simulator for deuteranomaly, a form of anomalous trichromacy correlated to an
/// anomalous medium (green) cone.
///
/// The current default implementation uses linear interpolation, which is not
/// ideal, so this default implementation may change in the future.
pub type DeuteranomalySimul<S = Vienot1999, M = SmithPokorny> =
    AnomalousTrichromacySimul<Deutan, S, M>;

/// Simulator for tritanomaly, a form of anomalous trichromacy correlated to an
/// anomalous short (blue) cone.
///
/// The current default implementation uses linear interpolation, which is not
/// ideal, so this default implementation may change in the future.
pub type TritanomalySimul<S = Brettel1997, M = SmithPokorny> =
    AnomalousTrichromacySimul<Tritan, S, M>;
