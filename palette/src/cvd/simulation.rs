//! Simulators and simulation methods for color vision deficiency.

use core::marker::PhantomData;

use crate::{
    bool_mask::{HasBoolMask, LazySelect},
    cvd::cone_response::ConeResponse,
    lms::{matrix::WithLmsMatrix, Lms},
    num::{Arithmetics, PartialCmp, Real},
    white_point::WhitePoint,
    xyz::meta::HasXyzMeta,
    FromColor, IntoColor, Xyz,
};

/// A color deficiency simulator that converts colors to dichromatic vision
/// based on the [`ConeResponse`]: `Cn`, [`SimulationMethod`]: `S`, and LMS
/// matrix: `M`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DichromacySimul<Cn, S, M>(PhantomData<Cn>, PhantomData<S>, PhantomData<M>);

impl<Cn, S, M> DichromacySimul<Cn, S, M>
where
    Cn: ConeResponse,
    S: SimulationMethod,
    M: HasXyzMeta,
{
    /// Converts a color into the percieved color of an individual with
    /// dichromacy using the generic settings of `Self`.
    pub fn simulate_deficiency<C, Wp, T>(color: C) -> C
    where
        C: FromColor<Xyz<Wp, T>> + IntoColor<Xyz<Wp, T>>,
        Wp: WhitePoint<T>,
        T: Real + Arithmetics + PartialCmp + Clone,
        Lms<WithLmsMatrix<Wp, M>, T>: FromColor<Xyz<Wp, T>> + IntoColor<Xyz<Wp, T>>,
        <[T; 9] as HasBoolMask>::Mask: LazySelect<[T; 9]>,
    {
        let xyz: Xyz<Wp, T> = color.into_color();
        let lms = Lms::<WithLmsMatrix<Wp, M>, T>::from_color(xyz);
        let clamped_xyz: Xyz<Wp, T> =
            S::clamp_by_deficiency::<Cn, WithLmsMatrix<Wp, M>, T>(lms, None).into_color();
        clamped_xyz.into_color()
    }
}

/// A color deficiency simulator that converts colors to anomalous trichromatic
/// vision based on the [`ConeResponse`]: `Cn`, [`SimulationMethod`]: `S`, and
/// LMS matrix: `M`.
///
/// Currently, all implemented simulation methods use linear interpolation
/// between the original and clamped color, which is not ideal, but is an alright
/// approximation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AnomalousTrichromacySimul<Cn, S, M>(PhantomData<Cn>, PhantomData<S>, PhantomData<M>);

impl<Cn, S, M> AnomalousTrichromacySimul<Cn, S, M>
where
    Cn: ConeResponse,
    S: SimulationMethod,
{
    /// Converts a color into the percieved color of an individual with
    /// anomalous trichromacy using the generic settings of `Self`.
    #[inline]
    pub fn simulate_deficiency_by_severity<C, T>(color: C, severity: T) -> C
    where
        C: IntoColor<Lms<M, T>> + FromColor<Lms<M, T>>,
        T: Real + Arithmetics + PartialCmp + Clone,
        <[T; 9] as HasBoolMask>::Mask: LazySelect<[T; 9]>,
    {
        let lms = color.into_color();
        S::clamp_by_deficiency::<Cn, M, T>(lms, Some(severity)).into_color()
    }
}

/// Represents a method for projecting typical color vision onto that of someone
/// with a color vision deficiency.
///
/// The generic `Cn` must implement [`ConeResponse`] which contains transformation
/// functions for each method.
pub trait SimulationMethod {
    /// Clamps a color in LMS space according to the color deficient cone response.
    fn clamp_by_deficiency<Cn, _M, T>(lms: Lms<_M, T>, severity: Option<T>) -> Lms<_M, T>
    where
        Cn: ConeResponse,
        T: Real + Arithmetics + PartialCmp + Clone,
        <[T; 9] as HasBoolMask>::Mask: LazySelect<[T; 9]>;
}

/// Color vision deficiency simulation described in
/// "Computerized simulation of color appearance for dichromats"
/// by Brettel, H., Viénot, F., & Mollon, J. D. (1997).
///
/// This method projects colors in the LMS space onto two half-planes and is the
/// best method for simulating tritan color deficiency.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Brettel1997;

impl SimulationMethod for Brettel1997 {
    #[inline]
    fn clamp_by_deficiency<Cn, _M, T>(lms: Lms<_M, T>, severity: Option<T>) -> Lms<_M, T>
    where
        Cn: ConeResponse,
        T: Real + Arithmetics + PartialCmp + Clone,
        <[T; 9] as HasBoolMask>::Mask: LazySelect<[T; 9]>,
    {
        Cn::projection_brettel_1997(lms, severity)
    }
}

/// Color vision deficiency simulation described in
/// "Digital video colourmaps for checking the legibility of displays by dichromats"
/// Viénot, F., Brettel, H., & Mollon, J. D. (1999).
///
/// This method simplifies the method used by [`Brettel1997`] by projecting to a
/// single plane in LMS space, but as a result has a poor approximation of tritan
/// color vision.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vienot1999;

impl SimulationMethod for Vienot1999 {
    #[inline]
    fn clamp_by_deficiency<Cn, _M, T>(lms: Lms<_M, T>, severity: Option<T>) -> Lms<_M, T>
    where
        Cn: ConeResponse,
        T: Real + Arithmetics + PartialCmp + Clone,
    {
        Cn::projection_vienot_1999(lms, severity)
    }
}
