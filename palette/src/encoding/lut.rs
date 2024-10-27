mod codegen;

const MAX_FLOAT_BITS: u32 = 0x3f7fffff; // 1.0 - f32::EPSILON

// SAFETY: Only use this macro if `input` is clamped between `min_float` and `max_float`.
macro_rules! linear_float_to_encoded_uint {
    ($enc:ty, $lut:ty, $input:ident, $min_float_bits:ident, $table:ident, $bit_width:expr, $man_index_width:expr) => {{
        let input_bits = $input.to_bits();
        #[cfg(test)]
        {
            debug_assert!(($min_float_bits..=MAX_FLOAT_BITS).contains(&$input.to_bits()));
        }
        let entry = {
            let i = ((input_bits - $min_float_bits) >> (23 - $man_index_width)) as usize;
            #[cfg(test)]
            {
                debug_assert!($table.get(i).is_some());
            }
            unsafe { *$table.get_unchecked(i) }
        };

        let bias = (entry >> (2 * $bit_width)) << ($bit_width + 1);
        let scale = entry & ((1 << (2 * $bit_width)) - 1);
        let t =
            (input_bits as $lut >> (23 - $man_index_width - $bit_width)) & ((1 << $bit_width) - 1);
        let res = (bias + scale * t) >> (2 * $bit_width);
        #[cfg(test)]
        {
            debug_assert!(res < ((<$enc>::MAX as $lut) + 1), "{}", res);
        }
        res as $enc
    }};
}

#[inline]
fn linear_f32_to_encoded_u8(linear: f32, min_float_bits: u32, table: &[u32]) -> u8 {
    let min_float = f32::from_bits(min_float_bits);
    let max_float = f32::from_bits(MAX_FLOAT_BITS);

    let mut input = linear;
    if input.partial_cmp(&min_float) != Some(core::cmp::Ordering::Greater) {
        input = min_float;
    } else if input > max_float {
        input = max_float;
    }

    linear_float_to_encoded_uint!(u8, u32, input, min_float_bits, table, 8, 3)
}

#[cfg(feature = "gamma_lut_u16")]
#[inline]
fn linear_f32_to_encoded_u16_with_linear_scale(
    linear: f32,
    linear_scale: f32,
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
        return ((linear_scale * input + 8388608.0).to_bits() & 65535) as u16;
    }

    linear_float_to_encoded_uint!(u16, u64, input, min_float_bits, table, 16, 7)
}
