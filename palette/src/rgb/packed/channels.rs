//! Channel orderings for packed RGBA types.
use super::RgbChannels;
use crate::rgb;

/// RGBA color packed in ABGR order.
///
/// See [Packed](crate::Packed) for more details.
pub struct Abgr;

impl RgbChannels for Abgr {
    fn split_rgb<S: rgb::RgbStandard>(rgb: rgb::Rgba<S, u8>) -> (u8, u8, u8, u8) {
        (rgb.alpha, rgb.blue, rgb.green, rgb.red)
    }

    fn combine_rgb<S: rgb::RgbStandard>(channels: (u8, u8, u8, u8)) -> rgb::Rgba<S, u8> {
        rgb::Rgba::new(channels.3, channels.2, channels.1, channels.0)
    }
}

/// RGBA color packed in ARGB order.
///
/// See [Packed](crate::Packed) for more details.
pub struct Argb;

impl RgbChannels for Argb {
    fn split_rgb<S: rgb::RgbStandard>(rgb: rgb::Rgba<S, u8>) -> (u8, u8, u8, u8) {
        (rgb.alpha, rgb.red, rgb.green, rgb.blue)
    }

    fn combine_rgb<S: rgb::RgbStandard>(channels: (u8, u8, u8, u8)) -> rgb::Rgba<S, u8> {
        rgb::Rgba::new(channels.1, channels.2, channels.3, channels.0)
    }
}

/// RGBA color packed in BGRA order.
///
/// See [Packed](crate::Packed) for more details.
pub struct Bgra;

impl RgbChannels for Bgra {
    fn split_rgb<S: rgb::RgbStandard>(rgb: rgb::Rgba<S, u8>) -> (u8, u8, u8, u8) {
        (rgb.blue, rgb.green, rgb.red, rgb.alpha)
    }

    fn combine_rgb<S: rgb::RgbStandard>(channels: (u8, u8, u8, u8)) -> rgb::Rgba<S, u8> {
        rgb::Rgba::new(channels.2, channels.1, channels.0, channels.3)
    }
}

/// RGBA color packed in RGBA order.
///
/// See [Packed](crate::Packed) for more details.
pub struct Rgba;

impl RgbChannels for Rgba {
    fn split_rgb<S: rgb::RgbStandard>(rgb: rgb::Rgba<S, u8>) -> (u8, u8, u8, u8) {
        (rgb.red, rgb.green, rgb.blue, rgb.alpha)
    }

    fn combine_rgb<S: rgb::RgbStandard>(channels: (u8, u8, u8, u8)) -> rgb::Rgba<S, u8> {
        rgb::Rgba::new(channels.0, channels.1, channels.2, channels.3)
    }
}
