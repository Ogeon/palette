use core::num::ParseIntError;

#[inline]
pub(crate) fn rgb_from_hex_4bit(hex: &str) -> Result<(u8, u8, u8), ParseIntError> {
    let red = u8::from_str_radix(&hex[..1], 16)?;
    let green = u8::from_str_radix(&hex[1..2], 16)?;
    let blue = u8::from_str_radix(&hex[2..3], 16)?;

    Ok((red * 17, green * 17, blue * 17))
}

#[inline]
pub(crate) fn rgba_from_hex_4bit(hex: &str) -> Result<(u8, u8, u8, u8), ParseIntError> {
    let (red, green, blue) = rgb_from_hex_4bit(hex)?;
    let alpha = u8::from_str_radix(&hex[3..4], 16)?;

    Ok((red, green, blue, alpha * 17))
}

#[inline]
pub(crate) fn rgb_from_hex_8bit(hex: &str) -> Result<(u8, u8, u8), ParseIntError> {
    let red = u8::from_str_radix(&hex[..2], 16)?;
    let green = u8::from_str_radix(&hex[2..4], 16)?;
    let blue = u8::from_str_radix(&hex[4..6], 16)?;

    Ok((red, green, blue))
}

#[inline]
pub(crate) fn rgba_from_hex_8bit(hex: &str) -> Result<(u8, u8, u8, u8), ParseIntError> {
    let (red, green, blue) = rgb_from_hex_8bit(hex)?;
    let alpha = u8::from_str_radix(&hex[6..8], 16)?;

    Ok((red, green, blue, alpha))
}

#[inline]
pub(crate) fn rgb_from_hex_16bit(hex: &str) -> Result<(u16, u16, u16), ParseIntError> {
    let red = u16::from_str_radix(&hex[..4], 16)?;
    let green = u16::from_str_radix(&hex[4..8], 16)?;
    let blue = u16::from_str_radix(&hex[8..12], 16)?;

    Ok((red, green, blue))
}

#[inline]
pub(crate) fn rgba_from_hex_16bit(hex: &str) -> Result<(u16, u16, u16, u16), ParseIntError> {
    let (red, green, blue) = rgb_from_hex_16bit(hex)?;
    let alpha = u16::from_str_radix(&hex[12..16], 16)?;

    Ok((red, green, blue, alpha))
}

#[inline]
pub(crate) fn rgb_from_hex_32bit(hex: &str) -> Result<(u32, u32, u32), ParseIntError> {
    let red = u32::from_str_radix(&hex[..8], 16)?;
    let green = u32::from_str_radix(&hex[8..16], 16)?;
    let blue = u32::from_str_radix(&hex[16..24], 16)?;

    Ok((red, green, blue))
}

#[inline]
pub(crate) fn rgba_from_hex_32bit(hex: &str) -> Result<(u32, u32, u32, u32), ParseIntError> {
    let (red, green, blue) = rgb_from_hex_32bit(hex)?;
    let alpha = u32::from_str_radix(&hex[24..32], 16)?;

    Ok((red, green, blue, alpha))
}
