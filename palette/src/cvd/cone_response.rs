//! The color vision deficiency cone response types.

use crate::{
    bool_mask::{HasBoolMask, LazySelect},
    convert::{ConvertOnce, Matrix3},
    lms::Lms,
    matrix::matrix_map,
    num::{Arithmetics, PartialCmp, Real},
};

/// A type of color deficient cone response.
pub trait ConeResponse {
    /// Simulates color vision deficiency for this cone response type by the method,
    /// [`Brettel1997`](crate::cvd::simulation::Brettel1997), for given `severity`.
    fn projection_brettel_1997<M, T>(lms: Lms<M, T>, severity: Option<T>) -> Lms<M, T>
    where
        T: Real + Arithmetics + PartialCmp + Clone,
        <[T; 9] as HasBoolMask>::Mask: LazySelect<[T; 9]>;
    /// Simulates color vision deficiency for this cone response type by the method,
    /// [`Vienot1999`](crate::cvd::simulation::Vienot1999), for given `severity`.
    fn projection_vienot_1999<M, T>(lms: Lms<M, T>, severity: Option<T>) -> Lms<M, T>
    where
        T: Real + Arithmetics + Clone;
}

/// The cone response associated with a deficient long (red) cone.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Protan;

impl ConeResponse for Protan {
    #[inline]
    fn projection_brettel_1997<M, T>(lms: Lms<M, T>, severity: Option<T>) -> Lms<M, T>
    where
        T: Real + Arithmetics + PartialCmp + Clone,
        <[T; 9] as HasBoolMask>::Mask: LazySelect<[T; 9]>,
    {
        // Values calculated using information and methods described at
        // https://daltonlens.org/understanding-cvd-simulation/
        let matrix = lazy_select! {
            if (T::from_f64(-0.0480077092304) * &lms.medium
            + T::from_f64(0.998846965183) * &lms.short)
            .gt(&T::from_f64(0.0)) =>
        {
            matrix_map(
                [
                    0.0, 2.27376142579, -5.92721533669,
                    0.0, 1.0,            0.0,
                    0.0, 0.0,            1.0,
                ],
                T::from_f64,
            )
        },
        else => {
            matrix_map(
                [
                    0.0, 2.18595044625, -4.10022271756,
                    0.0, 1.0,            0.0,
                    0.0, 0.0,            1.0,
                ],
                T::from_f64,
            )
        }};
        if let Some(s) = severity {
            let original = lms.clone();
            let clamped: Lms<M, T> = Matrix3::from_array(matrix).convert_once(lms);
            clamped * s.clone() + original * (T::from_f64(1.0) - s)
        } else {
            Matrix3::from_array(matrix).convert_once(lms)
        }
    }

    #[inline]
    fn projection_vienot_1999<M, T>(lms: Lms<M, T>, severity: Option<T>) -> Lms<M, T>
    where
        T: Real + Arithmetics + Clone,
    {
        // Values calculated using information and methods described at
        // https://daltonlens.org/understanding-cvd-simulation/
        let matrix = matrix_map(
            [
                0.0,
                2.02344337265,
                -2.52580325429,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
            T::from_f64,
        );
        if let Some(s) = severity {
            let original = lms.clone();
            let clamped: Lms<M, T> = Matrix3::from_array(matrix).convert_once(lms);
            clamped * s.clone() + original * (T::from_f64(1.0) - s)
        } else {
            Matrix3::from_array(matrix).convert_once(lms)
        }
    }
}

/// The cone response associated with a deficient medium (green) cone.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Deutan;

impl ConeResponse for Deutan {
    #[inline]
    fn projection_brettel_1997<M, T>(lms: Lms<M, T>, severity: Option<T>) -> Lms<M, T>
    where
        T: Real + Arithmetics + PartialCmp + Clone,
        <[T; 9] as HasBoolMask>::Mask: LazySelect<[T; 9]>,
    {
        // Values calculated using information and methods described at
        // https://daltonlens.org/understanding-cvd-simulation/
        let matrix = lazy_select! {
            if (T::from_f64(-0.024158861984) * &lms.long
            + T::from_f64(0.9997081321) * &lms.short)
            .gt(&T::from_f64(0.0)) =>
        {
            matrix_map(
                [
                    1.0,            0.0, 0.0,
                    0.439799879027, 0.0, 2.60678858804,
                    0.0,            0.0, 1.0,
                ],
                T::from_f64,
            )
        },
        else => {
            matrix_map(
                [
                    1.0,            0.0, 0.0,
                    0.457466911802, 0.0, 1.8757162243,
                    0.0,            0.0, 1.0,
                ],
                T::from_f64,
            )
        }};
        if let Some(s) = severity {
            let original = lms.clone();
            let clamped: Lms<M, T> = Matrix3::from_array(matrix).convert_once(lms);
            clamped * s.clone() + original * (T::from_f64(1.0) - s)
        } else {
            Matrix3::from_array(matrix).convert_once(lms)
        }
    }

    #[inline]
    fn projection_vienot_1999<M, T>(lms: Lms<M, T>, severity: Option<T>) -> Lms<M, T>
    where
        T: Real + Arithmetics + Clone,
    {
        // Values calculated using information and methods described at
        // https://daltonlens.org/understanding-cvd-simulation/
        let matrix = matrix_map(
            [
                1.0,
                0.0,
                0.0,
                0.494207059864,
                0.0,
                1.2482698001,
                0.0,
                0.0,
                1.0,
            ],
            T::from_f64,
        );
        if let Some(s) = severity {
            let original = lms.clone();
            let clamped: Lms<M, T> = Matrix3::from_array(matrix).convert_once(lms);
            clamped * s.clone() + original * (T::from_f64(1.0) - s)
        } else {
            Matrix3::from_array(matrix).convert_once(lms)
        }
    }
}

/// The cone response associated with a deficient short (blue) cone.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Tritan;

impl ConeResponse for Tritan {
    #[inline]
    fn projection_brettel_1997<M, T>(lms: Lms<M, T>, severity: Option<T>) -> Lms<M, T>
    where
        T: Real + Arithmetics + PartialCmp + Clone,
        <[T; 9] as HasBoolMask>::Mask: LazySelect<[T; 9]>,
    {
        // Values calculated using information and methods described at
        // https://daltonlens.org/understanding-cvd-simulation/
        let matrix = lazy_select! {
            if (T::from_f64(-0.449210402667) * &lms.long
            + T::from_f64(0.893425998131) * &lms.short)
            .gt(&T::from_f64(0.0)) =>
        {
            matrix_map(
                [
                     1.0,             0.0,            0.0,
                     0.0,             1.0,            0.0,
                    -0.0557429252223, 0.158929167991, 0.0,
                ],
                T::from_f64,
            )
        },
        else => {
            matrix_map(
                [
                     1.0,              0.0,             0.0,
                     0.0,              1.0,             0.0,
                    -0.00254865354166, 0.0531320960863, 0.0,
                ],
                T::from_f64,
            )
        }};
        if let Some(s) = severity {
            let original = lms.clone();
            let clamped: Lms<M, T> = Matrix3::from_array(matrix).convert_once(lms);
            clamped * s.clone() + original * (T::from_f64(1.0) - s)
        } else {
            Matrix3::from_array(matrix).convert_once(lms)
        }
    }

    #[inline]
    fn projection_vienot_1999<M, T>(lms: Lms<M, T>, severity: Option<T>) -> Lms<M, T>
    where
        T: Real + Arithmetics + Clone,
    {
        // Values calculated using information and methods described at
        // https://daltonlens.org/understanding-cvd-simulation/
        let matrix = matrix_map(
            [
                1.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                -0.012244922724,
                0.0720343802523,
                0.0,
            ],
            T::from_f64,
        );
        if let Some(s) = severity {
            let original = lms.clone();
            let clamped: Lms<M, T> = Matrix3::from_array(matrix).convert_once(lms);
            clamped * s.clone() + original * (T::from_f64(1.0) - s)
        } else {
            Matrix3::from_array(matrix).convert_once(lms)
        }
    }
}
