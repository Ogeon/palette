mod codegen;

use core::{iter::FromIterator, marker::PhantomData};

use palette_math::{
    gamma::lut::{GammaLut, GammaLutInput, GammaLutOutput},
    lut::{Lookup, Lut, LutType},
};

pub(crate) use crate::encoding::lut::codegen::*;
use crate::{
    convert::{Convert, ConvertOnce},
    luma::{Luma, LumaStandard},
    num::Real,
    rgb::{Rgb, RgbStandard},
};

use super::{GetLutBuilder, Linear};

/// Lookup table from gamma encoded to linear values.
///
/// `E` and `L` are the encoded and linear types, `F` is the transfer function,
/// and `T` determines the storage type for the table.
///
/// ```
/// use palette::{Srgb, SrgbLuma, encoding, math::lut::VecTable};
///
/// // Accessing a pre-generated lookup table:
/// let lut = encoding::Srgb::get_u8_to_f32_lut();
/// let linear = lut.lookup_rgb(Srgb::new(23, 198, 76));
///
/// // Generating a new lookup table, allocated as a Vec:
/// let lut = encoding::IntoLinearLut::<_, f32, _, VecTable>::new_u8();
/// let linear = lut.lookup_rgb(Srgb::new(23, 198, 76));
///
/// // Looking up a Luma value:
/// let lut = encoding::Srgb::get_u8_to_f32_lut();
/// let linear = lut.lookup_luma(SrgbLuma::new(23));
/// ```
pub struct IntoLinearLut<E, L, F, T>
where
    T: LutType<L>,
    T::Table: Sized,
{
    table: Lut<E, L, T>,
    #[allow(clippy::type_complexity)]
    transfer_fn: PhantomData<fn(Rgb<F, E>) -> Rgb<Linear<F>, L>>,
}

impl<L, F, T> IntoLinearLut<u8, L, F, T>
where
    T: LutType<L>,
    T::Table: Sized,
{
    /// Generate a new lookup table that decodes from `u8` values.
    #[inline]
    pub fn new_u8() -> Self
    where
        L: Real,
        F: GetLutBuilder,
        T::Table: FromIterator<L>,
    {
        let builder = F::get_lut_builder();

        Self::from_table(Lut::new(
            builder.u8_to_linear_entries().map(L::from_f64).collect(),
        ))
    }
}

impl<L, F, T> IntoLinearLut<u16, L, F, T>
where
    T: LutType<L>,
    T::Table: Sized,
{
    /// Generate a new lookup table that decodes from `u16` values.
    #[inline]
    pub fn new_u16() -> Self
    where
        L: Real,
        F: GetLutBuilder,
        T::Table: FromIterator<L>,
    {
        let builder = F::get_lut_builder();

        Self::from_table(Lut::new(
            builder.u16_to_linear_entries().map(L::from_f64).collect(),
        ))
    }
}

impl<E, L, F, T> IntoLinearLut<E, L, F, T>
where
    T: LutType<L>,
    T::Table: Sized,
{
    /// Create a lookup table from a component lookup table.
    ///
    /// The input table will not be verified for correctness and may result in
    /// unintended values if it's not for the intended transfer function. This
    /// function is primarily useful for code generation.
    #[inline]
    pub const fn from_table(table: Lut<E, L, T>) -> Self {
        IntoLinearLut {
            table,
            transfer_fn: PhantomData,
        }
    }

    /// Get an RGB value from the table.
    #[inline]
    pub fn lookup_rgb<S>(&self, encoded: Rgb<S, E>) -> Rgb<Linear<S>, L>
    where
        L: Clone,
        T: Lookup<E, L>,
        S: RgbStandard<TransferFn = F>,
    {
        Rgb {
            red: self.table.lookup(encoded.red).clone(),
            green: self.table.lookup(encoded.green).clone(),
            blue: self.table.lookup(encoded.blue).clone(),
            standard: PhantomData,
        }
    }

    /// Get a luma value from the table.
    #[inline]
    pub fn lookup_luma<S>(&self, encoded: Luma<S, E>) -> Luma<Linear<S>, L>
    where
        L: Clone,
        T: Lookup<E, L>,
        S: LumaStandard<TransferFn = F>,
    {
        Luma {
            luma: self.table.lookup(encoded.luma).clone(),
            standard: PhantomData,
        }
    }
}

impl<E, L, S, T> ConvertOnce<Rgb<S, E>, Rgb<Linear<S>, L>> for IntoLinearLut<E, L, S::TransferFn, T>
where
    L: Clone,
    T: Lookup<E, L>,
    T::Table: Sized,
    S: RgbStandard,
{
    #[inline]
    fn convert_once(self, input: Rgb<S, E>) -> Rgb<Linear<S>, L> {
        self.lookup_rgb(input)
    }
}

impl<E, L, S, T> Convert<Rgb<S, E>, Rgb<Linear<S>, L>> for IntoLinearLut<E, L, S::TransferFn, T>
where
    L: Clone,
    T: Lookup<E, L>,
    T::Table: Sized,
    S: RgbStandard,
{
    #[inline]
    fn convert(&self, input: Rgb<S, E>) -> Rgb<Linear<S>, L> {
        self.lookup_rgb(input)
    }
}

impl<E, L, S, T> ConvertOnce<Luma<S, E>, Luma<Linear<S>, L>>
    for IntoLinearLut<E, L, S::TransferFn, T>
where
    L: Clone,
    T: Lookup<E, L>,
    T::Table: Sized,
    S: LumaStandard,
{
    #[inline]
    fn convert_once(self, input: Luma<S, E>) -> Luma<Linear<S>, L> {
        self.lookup_luma(input)
    }
}

impl<E, L, S, T> Convert<Luma<S, E>, Luma<Linear<S>, L>> for IntoLinearLut<E, L, S::TransferFn, T>
where
    L: Clone,
    T: Lookup<E, L>,
    T::Table: Sized,
    S: LumaStandard,
{
    #[inline]
    fn convert(&self, input: Luma<S, E>) -> Luma<Linear<S>, L> {
        self.lookup_luma(input)
    }
}

impl<E, L, F, T> From<Lut<E, L, T>> for IntoLinearLut<E, L, F, T>
where
    T: LutType<L>,
    T::Table: Sized,
{
    #[inline]
    fn from(table: Lut<E, L, T>) -> Self {
        Self {
            table,
            transfer_fn: PhantomData,
        }
    }
}

/// Lookup table from linear to gamma encoded values.
///
/// `L` and `E` are the linear and encoded types, `F` is the transfer function,
/// and `T` determines the storage type for the table.
///
/// ```
/// use palette::{LinSrgb, SrgbLuma, LinLuma, encoding, math::lut::VecTable};
///
/// // Accessing a pre-generated lookup table:
/// let lut = encoding::Srgb::get_f32_to_u8_lut();
/// let encoded = lut.lookup_rgb(LinSrgb::new(0.3, 0.8, 0.1));
///
/// // Generating a new lookup table, allocated as a Vec:
/// let lut = encoding::FromLinearLut::<f32, _, _, VecTable>::new_u8();
/// let encoded = lut.lookup_rgb(LinSrgb::new(0.3, 0.8, 0.1));
///
/// // Looking up a Luma value:
/// let lut = encoding::Srgb::get_f32_to_u8_lut();
/// let encoded: SrgbLuma<_> = lut.lookup_luma(LinLuma::new(0.3));
/// ```
pub struct FromLinearLut<L, E, F, T>
where
    T: LutType<E::TableValue>,
    T::Table: Sized,
    E: GammaLutOutput,
{
    table: GammaLut<L, E, T>,
    #[allow(clippy::type_complexity)]
    transfer_fn: PhantomData<fn(Rgb<Linear<F>, L>) -> Rgb<F, E>>,
}

impl<L, F, T> FromLinearLut<L, u8, F, T>
where
    T: LutType<u32>,
    T::Table: Sized,
{
    /// Generate a new lookup table that encodes into `u8` values.
    #[inline]
    pub fn new_u8() -> Self
    where
        F: GetLutBuilder,
        T::Table: FromIterator<u32>,
    {
        let builder = F::get_lut_builder();

        Self {
            table: GammaLut::from_builder_u8(&builder),
            transfer_fn: PhantomData,
        }
    }
}

impl<L, F, T> FromLinearLut<L, u16, F, T>
where
    T: LutType<u64>,
    T::Table: Sized,
{
    /// Generate a new lookup table that encodes into `u16` values.
    #[inline]
    pub fn new_u16() -> Self
    where
        F: GetLutBuilder,
        T::Table: FromIterator<u64>,
    {
        let builder = F::get_lut_builder();

        Self {
            table: GammaLut::from_builder_u16(&builder),
            transfer_fn: PhantomData,
        }
    }
}

impl<L, E, F, T> FromLinearLut<L, E, F, T>
where
    T: LutType<E::TableValue>,
    T::Table: Sized,
    E: GammaLutOutput,
{
    /// Create a lookup table from a component lookup table.
    ///
    /// The input table will not be verified for correctness and may result in
    /// unintended values if it's not for the intended transfer function. This
    /// function is primarily useful for code generation.
    #[inline]
    pub const fn from_table(table: GammaLut<L, E, T>) -> Self {
        Self {
            table,
            transfer_fn: PhantomData,
        }
    }

    /// Get an RGB value from the table.
    #[inline]
    pub fn lookup_rgb<S>(&self, encoded: Rgb<Linear<S>, L>) -> Rgb<S, E>
    where
        L: GammaLutInput,
        S: RgbStandard<TransferFn = F>,
    {
        Rgb {
            red: self.table.lookup(encoded.red),
            green: self.table.lookup(encoded.green),
            blue: self.table.lookup(encoded.blue),
            standard: PhantomData,
        }
    }

    /// Get a luma from in the table.
    #[inline]
    pub fn lookup_luma<S>(&self, encoded: Luma<Linear<S>, L>) -> Luma<S, E>
    where
        L: GammaLutInput,
        S: LumaStandard<TransferFn = F>,
    {
        Luma {
            luma: self.table.lookup(encoded.luma),
            standard: PhantomData,
        }
    }
}

impl<L, E, S, T> ConvertOnce<Rgb<Linear<S>, L>, Rgb<S, E>> for FromLinearLut<L, E, S::TransferFn, T>
where
    L: GammaLutInput,
    E: GammaLutOutput,
    T: LutType<E::TableValue>,
    T::Table: Sized,
    S: RgbStandard,
{
    #[inline]
    fn convert_once(self, input: Rgb<Linear<S>, L>) -> Rgb<S, E> {
        self.lookup_rgb(input)
    }
}

impl<L, E, S, T> Convert<Rgb<Linear<S>, L>, Rgb<S, E>> for FromLinearLut<L, E, S::TransferFn, T>
where
    L: GammaLutInput,
    E: GammaLutOutput,
    T: LutType<E::TableValue>,
    T::Table: Sized,
    S: RgbStandard,
{
    #[inline]
    fn convert(&self, input: Rgb<Linear<S>, L>) -> Rgb<S, E> {
        self.lookup_rgb(input)
    }
}

impl<L, E, S, T> ConvertOnce<Luma<Linear<S>, L>, Luma<S, E>>
    for FromLinearLut<L, E, S::TransferFn, T>
where
    L: GammaLutInput,
    E: GammaLutOutput,
    T: LutType<E::TableValue>,
    T::Table: Sized,
    S: LumaStandard,
{
    #[inline]
    fn convert_once(self, input: Luma<Linear<S>, L>) -> Luma<S, E> {
        self.lookup_luma(input)
    }
}

impl<L, E, S, T> Convert<Luma<Linear<S>, L>, Luma<S, E>> for FromLinearLut<L, E, S::TransferFn, T>
where
    L: GammaLutInput,
    E: GammaLutOutput,
    T: LutType<E::TableValue>,
    T::Table: Sized,
    S: LumaStandard,
{
    #[inline]
    fn convert(&self, input: Luma<Linear<S>, L>) -> Luma<S, E> {
        self.lookup_luma(input)
    }
}
