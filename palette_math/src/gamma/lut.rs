//! Lookup table utilities for gamma functions.

use core::{
    f32,
    fmt::Display,
    iter::FromIterator,
    marker::PhantomData,
    ops::{Add, BitAnd, Mul, Shl, Shr, Sub},
};

use crate::{
    lut::{LutType, SliceTable},
    num::Powf,
};

use super::model::LinearModel;

// The number of mantissa bits used to index into the lookup table
const U8_MAN_INDEX_WIDTH: u32 = 3;
const U16_MAN_INDEX_WIDTH: u32 = 7;

const MAX_FLOAT_BITS: u32 = 0x3f7fffff; // 1.0 - f32::EPSILON
const MANTISSA_BITS: u32 = 23;

/// Models a given transfer function to construct lookup tables.
///
/// It's mainly meant to be used with [`Lut`][crate::lut::Lut] and [`GammaLut`],
/// or to generate constants of those.
#[derive(Clone)]
pub struct GammaLutBuilder {
    /// Varies depending on the type of transfer function.
    pub(crate) into_linear: fn(&Self, f64) -> f64,

    /// The slope of the linear part of a piecewise function.
    pub(crate) linear_slope: Option<f64>,

    /// Preserves continuity in a piecewise linear function.
    pub(crate) alpha: f64,

    /// The input value where the linear function part ends and the power
    /// function part begins.
    pub(crate) beta: f64,

    /// The exponent of the power function.
    pub(crate) gamma: f64,
}

impl GammaLutBuilder {
    /// Create a builder for a simple power function with `gamma` as the exponent.
    pub const fn new_power_fn(gamma: f64) -> Self
    where
        f64: Powf,
    {
        Self {
            into_linear: Self::power_into_linear,
            linear_slope: None,
            alpha: 1.0,
            beta: 0.0,
            gamma,
        }
    }

    /// Create a builder for a piecewise power function with `gamma` as the
    /// exponent.
    ///
    /// The arguments correspond to these values in a linear (`x`) to non-linear
    /// (`x'`) transfer function:
    ///
    /// ```text
    /// S = linear_slope
    /// L = linear_end
    /// G = gamma
    ///
    /// x' = S  * x                 x <= L
    ///      k1 * x ^ (1/G) + k2    x >  L
    /// ```
    ///
    /// where `k1` and `k2` are derived constants.
    pub fn new_piecewise_fn(linear_slope: f64, linear_end: f64, gamma: f64) -> Self
    where
        f64: Powf,
    {
        let alpha =
            (linear_slope * linear_end - 1.0) / (Powf::powf(linear_end, gamma.recip()) - 1.0);
        let beta = linear_end;

        Self {
            into_linear: Self::piecewise_into_linear,
            linear_slope: Some(linear_slope),
            alpha,
            beta,
            gamma,
        }
    }

    /// Returns the table entries for an encoded `u8` to linear float table.
    pub fn u8_to_linear_entries(&self) -> impl ExactSizeIterator<Item = f64> + '_ {
        (0u8..=u8::MAX).map(move |encoded| (self.into_linear)(self, (encoded as f64) / 255.0))
    }

    /// Returns the table entries for an encoded `u16` to linear float table.
    pub fn u16_to_linear_entries(&self) -> impl ExactSizeIterator<Item = f64> + '_ {
        (0u16..=u16::MAX).map(move |encoded| (self.into_linear)(self, (encoded as f64) / 65535.0))
    }

    /// Returns the table entries for a linear float to encoded `u8` table.
    ///
    /// The lookup algorithm the table is meant for is based on [this C++
    /// code](<https://gist.github.com/rygorous/2203834>) by Fabian "ryg"
    /// Giesen. This algorithm is implemented in the [`GammaLut`] type, and this
    /// function is mainly meant for use in code generation.
    #[doc(hidden)]
    pub fn linear_to_u8_entries(&self) -> impl ExactSizeIterator<Item = u32> + '_ {
        // This algorithm is an adaptation of [this C++ code](<https://gist.github.com/rygorous/2203834>)
        // by Fabian "ryg" Giesen, which utilizes simple linear regression on
        // sub-intervals of the transfer function's domain and stores the resulting
        // models' scales and biases into a lookup table.
        //
        // The algorithm linked above calculates the transfer function for every
        // potential `f32` input and feeds that into the regression model. In
        // contrast, this algorithm replaces the discrete sums in the model with
        // continuous integrals in order to reduce the time it takes to generate
        // the tables. We are able to do this since transfer functions follow a
        // predictable pattern for which the anti-derivative is known.

        // 1.0 - f32::EPSILON
        const MAX_FLOAT_BITS: u32 = 0x3f7fffff;
        // The number of mantissa bits used to index into the lookup table
        const MAN_INDEX_WIDTH: u32 = U8_MAN_INDEX_WIDTH;
        // The number of bits in the remainder of the mantissa, when removing the index bits.
        const BUCKET_INDEX_WIDTH: u32 = MANTISSA_BITS - MAN_INDEX_WIDTH;
        const BUCKET_SIZE: u32 = 1 << BUCKET_INDEX_WIDTH;
        // Any input less than or equal to this maps to 0
        let min_float_bits = self.linear_to_u8_min_float_bits();

        let exp_table_size = ((MAX_FLOAT_BITS - min_float_bits) >> 23) + 1;
        let table_size = exp_table_size << MAN_INDEX_WIDTH;

        (0..table_size).map(move |i| {
            let start = min_float_bits + (i << BUCKET_INDEX_WIDTH);
            let end = start + BUCKET_SIZE;

            LinearModel::new(self, start, end, MAN_INDEX_WIDTH, 8).into_u8_lookup()
        })
    }

    /// Returns the bit representation of the largest power of 2 float value
    /// that converts to to `0u8`.
    ///
    /// The lookup algorithm the value is meant for is based on [this C++
    /// code](<https://gist.github.com/rygorous/2203834>) by Fabian "ryg"
    /// Giesen. This algorithm is implemented in the [`GammaLut`] type, and this
    /// function is mainly meant for use in code generation.
    #[doc(hidden)]
    pub fn linear_to_u8_min_float_bits(&self) -> u32 {
        (((self.into_linear)(self, 0.5 / 255.0) as f32).to_bits() - 1) & 0xff800000
    }

    /// Returns the slope of the linear part of the gamma function, in `u8`
    /// space.
    ///
    /// The lookup algorithm the value is meant for is based on [this C++
    /// code](<https://gist.github.com/rygorous/2203834>) by Fabian "ryg"
    /// Giesen. This algorithm is implemented in the [`GammaLut`] type, and this
    /// function is mainly meant for use in code generation.
    #[doc(hidden)]
    pub fn linear_to_u8_linear_slope(&self) -> f32 {
        255.0 * (self.linear_slope.unwrap_or_default() as f32) // Unused.
    }

    /// Returns the table entries for a linear float to encoded `u16` table.
    ///
    /// The lookup algorithm the table is meant for is based on [this C++
    /// code](<https://gist.github.com/rygorous/2203834>) by Fabian "ryg"
    /// Giesen. This algorithm is implemented in the [`GammaLut`] type, and this
    /// function is mainly meant for use in code generation.
    #[doc(hidden)]
    pub fn linear_to_u16_entries(&self) -> impl ExactSizeIterator<Item = u64> + '_ {
        // 1.0 - f32::EPSILON
        const MAX_FLOAT_BITS: u32 = 0x3f7fffff;
        // The number of mantissa bits used to index into the lookup table
        const MAN_INDEX_WIDTH: u32 = U16_MAN_INDEX_WIDTH;
        // The number of bits in the remainder of the mantissa, when removing the index bits.
        const BUCKET_INDEX_WIDTH: u32 = MANTISSA_BITS - MAN_INDEX_WIDTH;
        const BUCKET_SIZE: u32 = 1 << BUCKET_INDEX_WIDTH;

        let min_float_bits = self.linear_to_u16_min_float_bits();
        let exp_table_size = ((MAX_FLOAT_BITS - min_float_bits) >> 23) + 1;
        let table_size = exp_table_size << MAN_INDEX_WIDTH;
        (0..table_size).map(move |i| {
            let start = min_float_bits + (i << BUCKET_INDEX_WIDTH);
            let end = start + BUCKET_SIZE;

            LinearModel::new(self, start, end, MAN_INDEX_WIDTH, 16).into_u16_lookup()
        })
    }

    /// Returns the bit representation of the largest power of 2 float value
    /// that either converts to `0u16` or is in the linear part of the transfer
    /// function.
    ///
    /// The lookup algorithm the value is meant for is based on [this C++
    /// code](<https://gist.github.com/rygorous/2203834>) by Fabian "ryg"
    /// Giesen. This algorithm is implemented in the [`GammaLut`] type, and this
    /// function is mainly meant for use in code generation.
    #[doc(hidden)]
    pub fn linear_to_u16_min_float_bits(&self) -> u32 {
        let Self {
            into_linear, beta, ..
        } = self;

        (*beta as f32)
            .to_bits()
            .max((into_linear(self, 0.5 / 65535.0) as f32).to_bits() - 1)
            & 0xff800000
    }

    /// Returns the slope of the linear part of the gamma function, in `u16`
    /// space.
    ///
    /// The lookup algorithm the value is meant for is based on [this C++
    /// code](<https://gist.github.com/rygorous/2203834>) by Fabian "ryg"
    /// Giesen. This algorithm is implemented in the [`GammaLut`] type, and this
    /// function is mainly meant for use in code generation.
    #[doc(hidden)]
    pub fn linear_to_u16_linear_slope(&self) -> f32 {
        65535.0 * (self.linear_slope.unwrap_or_default() as f32)
    }

    fn power_into_linear(&self, encoded: f64) -> f64
    where
        f64: Powf,
    {
        Powf::powf(encoded, self.gamma)
    }

    fn piecewise_into_linear(&self, encoded: f64) -> f64
    where
        f64: Powf,
    {
        let linear_slope = self.linear_slope.unwrap();

        if encoded <= linear_slope * self.beta {
            encoded / linear_slope
        } else {
            Powf::powf((encoded + self.alpha - 1.0) / self.alpha, self.gamma)
        }
    }
}

/// A valid input value for a [`GammaLut`].
///
/// Any value that can be extended to or rounded down to `f32` may implement
/// this trait.
pub trait GammaLutInput {
    /// Convert `self` to the nearest `f32` value.
    fn into_f32(self) -> f32;
}

impl GammaLutInput for f32 {
    fn into_f32(self) -> f32 {
        self
    }
}

impl GammaLutInput for f64 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

mod gamma_lut_output_seal {
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
}

/// A possible output type for a [`GammaLut`].
///
/// # Safety
///
/// The constants in this trait need to be valid for the lookup table algorithm.
pub unsafe trait GammaLutOutput: gamma_lut_output_seal::Sealed {
    /// The type of values stored in the lookup table.
    type TableValue;

    /// The maximum value of `Self`.
    const MAX: Self;

    /// The number of bits in `Self`.
    const BITS: u32;

    /// The number of mantissa bits used to index into the lookup table.
    const MAN_INDEX_WIDTH: u32;

    /// Convert a table value to `Self`.
    ///
    /// This is called in the final stage of the lookup, when the table value is
    /// expected to be in range for `Self`.
    fn from_table_value(value: Self::TableValue) -> Self;

    /// Look up a value of this output type, called by [`GammaLut`].
    ///
    /// # Safety
    ///
    /// The lookup algorithm may assume certain properties and values from the
    /// input, such as range and table length. These invariants are upheld by
    /// [`GammaLut`] and [`GammaLutBuilder`].
    unsafe fn lookup(
        linear: f32,
        linear_slope: f32,
        min_float_bits: u32,
        table: &[Self::TableValue],
    ) -> Self;
}

unsafe impl GammaLutOutput for u8 {
    type TableValue = u32;

    const MAX: Self = Self::MAX;
    const BITS: u32 = Self::BITS;
    const MAN_INDEX_WIDTH: u32 = U8_MAN_INDEX_WIDTH;

    #[inline]
    fn from_table_value(value: Self::TableValue) -> Self {
        value as Self
    }

    #[inline]
    unsafe fn lookup(
        linear: f32,
        _linear_slope: f32,
        min_float_bits: u32,
        table: &[Self::TableValue],
    ) -> Self {
        linear_to_encoded_u8(linear, min_float_bits, table)
    }
}

unsafe impl GammaLutOutput for u16 {
    type TableValue = u64;

    const MAX: Self = Self::MAX;
    const BITS: u32 = Self::BITS;
    const MAN_INDEX_WIDTH: u32 = U16_MAN_INDEX_WIDTH;

    #[inline]
    fn from_table_value(value: Self::TableValue) -> Self {
        value as Self
    }

    #[inline]
    unsafe fn lookup(
        linear: f32,
        linear_slope: f32,
        min_float_bits: u32,
        table: &[Self::TableValue],
    ) -> Self {
        linear_to_encoded_u16(linear, linear_slope, min_float_bits, table)
    }
}

/// A lookup table from linear float or real values to integers that follow a
/// gamma curve.
///
/// `L` and `E` are the linear and encoded types, and `T` determines the storage
/// type for the table.
pub struct GammaLut<L, E, T>
where
    T: LutType<E::TableValue>,
    T::Table: Sized,
    E: GammaLutOutput,
{
    table: T::Table,
    min_float_bits: u32,
    linear_slope: f32,
    lookup: PhantomData<fn(L) -> E>,
}

impl<L, T> GammaLut<L, u8, T>
where
    T: LutType<u32>,
    T::Table: Sized,
{
    /// Create a new lookup table where the gamma encoded type is `u8`.
    pub fn from_builder_u8(builder: &GammaLutBuilder) -> Self
    where
        T::Table: FromIterator<u32>,
    {
        // SAFETY: These values need to be correct for the lookup algorithm for
        // `u8`, or calling `lookup` _will_ be UB.
        Self {
            table: builder.linear_to_u8_entries().collect(),
            min_float_bits: builder.linear_to_u8_min_float_bits(),
            linear_slope: builder.linear_to_u8_linear_slope(),
            lookup: PhantomData,
        }
    }
}

impl<L, T> GammaLut<L, u16, T>
where
    T: LutType<u64>,
    T::Table: Sized,
{
    /// Create a new lookup table where the gamma encoded type is `u16`.
    pub fn from_builder_u16(builder: &GammaLutBuilder) -> Self
    where
        T::Table: FromIterator<u64>,
    {
        // SAFETY: These values need to be correct for the lookup algorithm for
        // `u16`, or calling `lookup` _will_ be UB.
        Self {
            table: builder.linear_to_u16_entries().collect(),
            min_float_bits: builder.linear_to_u16_min_float_bits(),
            linear_slope: builder.linear_to_u16_linear_slope(),
            lookup: PhantomData,
        }
    }
}

impl<L, E, T> GammaLut<L, E, T>
where
    T: LutType<E::TableValue>,
    T::Table: Sized,
    E: GammaLutOutput,
{
    /// Get a gamma encoded value for the linear input.
    pub fn lookup(&self, linear: L) -> E
    where
        L: GammaLutInput,
    {
        // SAFETY: The requirements for the lookup is upheld by the
        // constructors, by only taking the values from their corresponding
        // `GammaLutBuilder` methods.
        unsafe {
            E::lookup(
                linear.into_f32(),
                self.linear_slope,
                self.min_float_bits,
                self.table.as_ref(),
            )
        }
    }

    /// Get a lookup table that uses a reference to this table.
    pub fn get_ref(&self) -> GammaLut<L, E, &'_ T> {
        GammaLut {
            table: &self.table,
            min_float_bits: self.min_float_bits,
            linear_slope: self.linear_slope,
            lookup: PhantomData,
        }
    }

    /// Get a lookup table that uses a slice reference to this table.
    pub fn get_slice(&self) -> GammaLut<L, E, &'_ SliceTable> {
        GammaLut {
            table: self.table.as_ref(),
            min_float_bits: self.min_float_bits,
            linear_slope: self.linear_slope,
            lookup: PhantomData,
        }
    }

    /// Create a table from its parts.
    ///
    /// This function is meant for code generation, to initialize constants from
    /// pre-computed values. See [`GammaLut::from_builder_u8`] or
    /// [`GammaLut::from_builder_u16`] for safe runtime alternatives.
    ///
    /// # Safety
    ///
    /// The parts need to come from the corresponding `linear_to_u8_*`,
    /// `linear_to_u16_*`, etc. methods of a [`GammaLutBuilder`] from the same
    /// version of `palette_math`. Anything else results in or is considered
    /// undefined behavior, as the lookup algorithm assumes specific input.
    #[doc(hidden)]
    pub const unsafe fn from_parts(
        min_float_bits: u32,
        linear_slope: f32,
        table: T::Table,
    ) -> Self {
        Self {
            table,
            min_float_bits,
            linear_slope,
            lookup: PhantomData,
        }
    }
}

// SAFETY: `input` needs to be clamped between `min_float` and `max_float`.
// `table` needs to be long enough to be indexed with any input value.
#[inline]
unsafe fn linear_float_to_encoded_uint<E: GammaLutOutput>(
    input: f32,
    min_float_bits: u32,
    table: &[E::TableValue],
) -> E
where
    E::TableValue: Copy
        + From<u32>
        + From<E>
        + Shl<E::TableValue, Output = E::TableValue>
        + Shr<E::TableValue, Output = E::TableValue>
        + BitAnd<E::TableValue, Output = E::TableValue>
        + Add<E::TableValue, Output = E::TableValue>
        + Sub<E::TableValue, Output = E::TableValue>
        + Mul<E::TableValue, Output = E::TableValue>
        + PartialOrd
        + Display,
{
    let bit_width = E::BITS;
    let man_index_width = E::MAN_INDEX_WIDTH;

    // Converts to table values.
    let tv = E::TableValue::from;

    let input_bits = input.to_bits();
    #[cfg(test)]
    {
        debug_assert!((min_float_bits..=MAX_FLOAT_BITS).contains(&input_bits));
    }
    let entry = {
        let i = ((input_bits - min_float_bits) >> (23 - man_index_width)) as usize;
        #[cfg(test)]
        {
            debug_assert!(table.get(i).is_some());
        }
        *table.get_unchecked(i)
    };

    let bit_width_mask = (tv(1) << tv(bit_width)) - tv(1);
    let bit_width_2 = tv(2 * bit_width);
    let bit_width_2_mask = (tv(1) << bit_width_2) - tv(1);

    let bias = (entry >> bit_width_2) << tv(bit_width + 1);
    let scale = entry & bit_width_2_mask;
    let t = (tv(input_bits) >> tv(23 - man_index_width - bit_width)) & bit_width_mask;
    let res = (bias + scale * t) >> bit_width_2;
    #[cfg(test)]
    {
        debug_assert!(res < E::TableValue::from(E::MAX) + tv(1), "{}", res);
    }
    E::from_table_value(res)
}

/// Look up a `u8` value from `table`, indexed with `linear`.
///
/// # Safety
///
/// `table` needs to be long enough to be indexed with any value from the float
/// representation of `min_float_bits` to the float representation of
/// `MAX_FLOAT_BITS`.
#[inline]
unsafe fn linear_to_encoded_u8(linear: f32, min_float_bits: u32, table: &[u32]) -> u8 {
    let min_float = f32::from_bits(min_float_bits);
    let max_float = f32::from_bits(MAX_FLOAT_BITS);

    let mut input = linear;
    if input.partial_cmp(&min_float) != Some(core::cmp::Ordering::Greater) {
        input = min_float;
    } else if input > max_float {
        input = max_float;
    }

    linear_float_to_encoded_uint(input, min_float_bits, table)
}

/// Look up a `u16` value from `table`, indexed with `linear`.
///
/// # Safety
///
/// `table` needs to be long enough to be indexed with any value from the float
/// representation of `min_float_bits` to the float representation of
/// `MAX_FLOAT_BITS`.
#[inline]
unsafe fn linear_to_encoded_u16(
    linear: f32,
    linear_slope: f32,
    min_float_bits: u32,
    table: &[u64],
) -> u16 {
    let min_float = f32::from_bits(min_float_bits);
    let max_float = f32::from_bits(MAX_FLOAT_BITS);

    let mut input = linear;
    if input.partial_cmp(&0.0) != Some(core::cmp::Ordering::Greater) {
        input = 0.0;
    } else if input > max_float {
        input = max_float;
    }

    if input < min_float {
        return ((linear_slope * input + 8388608.0).to_bits() & 65535) as u16;
    }

    linear_float_to_encoded_uint(input, min_float_bits, table)
}
