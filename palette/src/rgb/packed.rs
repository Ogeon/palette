pub mod channels;

use core::marker::PhantomData;

use crate::rgb::{Rgb, RgbStandard, Rgba};
use crate::Pixel;

/// RGBA color packed into a 32-bit unsigned integer. Defaults to ARGB
/// ordering for `Rgb` types and RGBA ordering for `Rgba` types.
///
/// Packed integer type represented in `u32`. Two hexadecimal digits (8-bits)
/// express each value of the Red, Green, Blue, and Alpha components in the
/// RGBA color.
///
/// Note that conversion from float to integer component types in `palette`
/// rounds to nearest even: an `Rgb` component of `0.5` will convert to
/// `0x80`/`128`, not `0x7F`/`127`.
///
/// ```
/// use approx::assert_relative_eq;
/// use palette::{Packed, Srgb, Srgba};
/// use palette::rgb::channels::{Argb, Rgba};
///
/// let packed: Packed = Srgb::new(0.5, 0.0, 0.5).into_format().into();
/// assert_eq!(0xFF80_0080, packed.color);
///
/// let unpacked: Srgba<u8> = Packed::<Rgba>::from(0xFFFF_FF80).into();
/// assert_relative_eq!(
///     Srgba::new(1.0, 1.0, 1.0, 0.5),
///     unpacked.into_format(),
///     epsilon = 0.01
/// );
///
/// // By default, `Packed` uses `Argb` order for creating `Rgb` colors to make
/// // entering 6-digit hex numbers more convenient
/// let rgb = Srgb::from(0xFF8000);
/// assert_eq!(Srgb::new(0xFF, 0x80, 0x00), rgb);
///
/// let rgba = Srgba::from(0xFF80007F);
/// assert_eq!(Srgba::new(0xFF, 0x80, 0x00, 0x7F), rgba);
/// ```
///
/// When an `Rgb` type is packed, the alpha value will be `0xFF` in the
/// corresponding `u32`. Converting from a packed color type back to an `Rgb`
/// type will disregard the alpha value.
///
/// `Packed` implements [Pixel](crate::encoding::pixel::Pixel) and can be
/// constructed from a slice of `&[u32]`.
///
/// ```
/// use palette::{Packed, Pixel};
/// use palette::rgb::channels::Argb;
///
/// let raw = &[0x7F0080u32, 0x60BBCC];
/// let colors = Packed::<Argb>::from_raw_slice(raw);
///
/// assert_eq!(colors.len(), 2);
/// assert_eq!(colors[0].color, 0x7F0080);
/// assert_eq!(colors[1].color, 0x60BBCC);
/// ```
#[derive(Debug, PartialEq, Eq, Pixel)]
#[palette(palette_internal)]
#[repr(C)]
pub struct Packed<C = channels::Argb> {
    /// The sRGB color packed into a `u32`.
    pub color: u32,

    /// The channel ordering for red, green, blue, and alpha components in the
    /// packed integer; can be `Abgr`, `Argb`, `Bgra`, or `Rgba`. See
    /// [RgbChannels](crate::RgbChannels).
    #[palette(unsafe_zero_sized)]
    pub channel_order: PhantomData<C>,
}

impl<C> Copy for Packed<C> {}

impl<C> Clone for Packed<C> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Splits and combines RGB(A) types with some channel ordering. Channels may be
/// ordered as `Abgr`, `Argb`, `Bgra`, or `Rgba`.
pub trait RgbChannels {
    /// Split RGBA components into a `(u8, u8, u8, u8)` tuple.
    fn split_rgb<S: RgbStandard>(rgb: Rgba<S, u8>) -> (u8, u8, u8, u8);
    /// Create an RGBA color from a `(u8, u8, u8, u8)` tuple.
    fn combine_rgb<S: RgbStandard>(channels: (u8, u8, u8, u8)) -> Rgba<S, u8>;
}

impl<S: RgbStandard> From<Rgb<S, u8>> for u32 {
    fn from(color: Rgb<S, u8>) -> Self {
        Rgb::into_u32::<channels::Argb>(color)
    }
}

impl<S: RgbStandard> From<Rgba<S, u8>> for u32 {
    fn from(color: Rgba<S, u8>) -> Self {
        Rgba::into_u32::<channels::Rgba>(color)
    }
}

impl<C: RgbChannels> From<u32> for Packed<C> {
    fn from(color: u32) -> Self {
        Packed {
            color,
            channel_order: PhantomData,
        }
    }
}

impl<S, C> From<Rgb<S, u8>> for Packed<C>
where
    S: RgbStandard,
    C: RgbChannels,
{
    fn from(color: Rgb<S, u8>) -> Self {
        Self::from(Rgba::from(color))
    }
}

impl<S, C> From<Rgba<S, u8>> for Packed<C>
where
    S: RgbStandard,
    C: RgbChannels,
{
    fn from(color: Rgba<S, u8>) -> Self {
        let bytes = C::split_rgb(color);
        Packed {
            color: u32::from_be_bytes([bytes.0, bytes.1, bytes.2, bytes.3]),
            channel_order: PhantomData,
        }
    }
}

impl<S: RgbStandard> From<u32> for Rgb<S, u8> {
    fn from(color: u32) -> Self {
        Self::from_u32::<channels::Argb>(color)
    }
}

impl<S, C> From<Packed<C>> for Rgb<S, u8>
where
    S: RgbStandard,
    C: RgbChannels,
{
    fn from(packed: Packed<C>) -> Self {
        Rgba::from(packed).color
    }
}

impl<S: RgbStandard> From<u32> for Rgba<S, u8> {
    fn from(color: u32) -> Self {
        Self::from_u32::<channels::Rgba>(color)
    }
}

impl<S, C> From<Packed<C>> for Rgba<S, u8>
where
    S: RgbStandard,
    C: RgbChannels,
{
    fn from(packed: Packed<C>) -> Self {
        let bytes = packed.color.to_be_bytes();
        C::combine_rgb((bytes[0], bytes[1], bytes[2], bytes[3]))
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<C> bytemuck::Zeroable for Packed<C> {}
#[cfg(feature = "bytemuck")]
unsafe impl<C: 'static> bytemuck::Pod for Packed<C> {}

#[cfg(test)]
mod test {
    use crate::rgb::packed::channels::{Abgr, Argb, Bgra, Rgba};
    use crate::{Packed, Srgb, Srgba};

    #[test]
    fn rgba() {
        let a1: Packed<Rgba> = Srgb::new(0.5, 0.0, 0.0).into_format().into();
        let a2: Packed<Rgba> = Srgb::new(0.0, 1.0, 0.0).into_format().into();
        let a3: Packed<Rgba> = Srgb::new(0.0, 0.0, 0.5).into_format().into();
        let x1: u32 = 0x8000_00FF;
        let x2: u32 = 0x00FF_00FF;
        let x3: u32 = 0x0000_80FF;
        assert_eq!(a1.color, x1);
        assert_eq!(a2.color, x2);
        assert_eq!(a3.color, x3);

        let unpacked: Srgb<u8> = Packed::<Rgba>::from(0x80FF_80FF).into();
        assert_relative_eq!(
            Srgb::new(0.5, 1.0, 0.5),
            unpacked.into_format(),
            epsilon = 0.01
        );

        let b1: Packed<Rgba> = Srgba::new(0.5, 0.0, 0.0, 0.0).into_format().into();
        let b2: Packed<Rgba> = Srgba::new(0.0, 1.0, 0.0, 0.0).into_format().into();
        let b3: Packed<Rgba> = Srgba::new(0.0, 0.0, 0.5, 0.0).into_format().into();
        let b4: Packed<Rgba> = Srgba::new(0.0, 0.0, 0.0, 1.0).into_format().into();
        let y1: u32 = 0x8000_0000;
        let y2: u32 = 0x00FF_0000;
        let y3: u32 = 0x0000_8000;
        let y4: u32 = 0x0000_00FF;
        assert_eq!(b1.color, y1);
        assert_eq!(b2.color, y2);
        assert_eq!(b3.color, y3);
        assert_eq!(b4.color, y4);

        let unpacked: Srgba<u8> = Packed::<Rgba>::from(0x80FF_80FF).into();
        assert_relative_eq!(
            Srgba::new(0.5, 1.0, 0.5, 1.0),
            unpacked.into_format(),
            epsilon = 0.01
        );
    }

    #[test]
    fn argb() {
        let a1: Packed<Argb> = Srgb::new(0.5, 0.0, 0.0).into_format().into();
        let a2: Packed<Argb> = Srgb::new(0.0, 1.0, 0.0).into_format().into();
        let a3: Packed<Argb> = Srgb::new(0.0, 0.0, 0.5).into_format().into();
        let x1: u32 = 0xFF80_0000;
        let x2: u32 = 0xFF00_FF00;
        let x3: u32 = 0xFF00_0080;
        assert_eq!(a1.color, x1);
        assert_eq!(a2.color, x2);
        assert_eq!(a3.color, x3);

        let unpacked: Srgb<u8> = Packed::<Argb>::from(0x80FF_80FF).into();
        assert_relative_eq!(
            Srgb::new(1.0, 0.5, 1.0),
            unpacked.into_format(),
            epsilon = 0.01
        );

        let b1: Packed<Argb> = Srgba::new(0.5, 0.0, 0.0, 0.0).into_format().into();
        let b2: Packed<Argb> = Srgba::new(0.0, 1.0, 0.0, 0.0).into_format().into();
        let b3: Packed<Argb> = Srgba::new(0.0, 0.0, 0.5, 0.0).into_format().into();
        let b4: Packed<Argb> = Srgba::new(0.0, 0.0, 0.0, 1.0).into_format().into();
        let y1: u32 = 0x0080_0000;
        let y2: u32 = 0x0000_FF00;
        let y3: u32 = 0x0000_0080;
        let y4: u32 = 0xFF00_0000;
        assert_eq!(b1.color, y1);
        assert_eq!(b2.color, y2);
        assert_eq!(b3.color, y3);
        assert_eq!(b4.color, y4);

        let unpacked: Srgba<u8> = Packed::<Argb>::from(0x80FF_80FF).into();
        assert_relative_eq!(
            Srgba::new(1.0, 0.5, 1.0, 0.5),
            unpacked.into_format(),
            epsilon = 0.01
        );
    }

    #[test]
    fn bgra() {
        let a1: Packed<Bgra> = Srgb::new(0.5, 0.0, 0.0).into_format().into();
        let a2: Packed<Bgra> = Srgb::new(0.0, 1.0, 0.0).into_format().into();
        let a3: Packed<Bgra> = Srgb::new(0.0, 0.0, 0.5).into_format().into();
        let x1: u32 = 0x0000_80FF;
        let x2: u32 = 0x00FF_00FF;
        let x3: u32 = 0x8000_00FF;
        assert_eq!(a1.color, x1);
        assert_eq!(a2.color, x2);
        assert_eq!(a3.color, x3);

        let unpacked: Srgb<u8> = Packed::<Bgra>::from(0x80FF_FF80).into();
        assert_relative_eq!(
            Srgb::new(1.0, 1.0, 0.5),
            unpacked.into_format(),
            epsilon = 0.01
        );

        let b1: Packed<Bgra> = Srgba::new(0.5, 0.0, 0.0, 0.0).into_format().into();
        let b2: Packed<Bgra> = Srgba::new(0.0, 1.0, 0.0, 0.0).into_format().into();
        let b3: Packed<Bgra> = Srgba::new(0.0, 0.0, 0.5, 0.0).into_format().into();
        let b4: Packed<Bgra> = Srgba::new(0.0, 0.0, 0.0, 1.0).into_format().into();
        let y1: u32 = 0x0000_8000;
        let y2: u32 = 0x00FF_0000;
        let y3: u32 = 0x8000_0000;
        let y4: u32 = 0x0000_00FF;
        assert_eq!(b1.color, y1);
        assert_eq!(b2.color, y2);
        assert_eq!(b3.color, y3);
        assert_eq!(b4.color, y4);

        let unpacked: Srgba<u8> = Packed::<Bgra>::from(0x80FF_FF80).into();
        assert_relative_eq!(
            Srgba::new(1.0, 1.0, 0.5, 0.5),
            unpacked.into_format(),
            epsilon = 0.01
        );
    }

    #[test]
    fn abgr() {
        let a1: Packed<Abgr> = Srgb::new(0.5, 0.0, 0.0).into_format().into();
        let a2: Packed<Abgr> = Srgb::new(0.0, 1.0, 0.0).into_format().into();
        let a3: Packed<Abgr> = Srgb::new(0.0, 0.0, 0.5).into_format().into();
        let x1: u32 = 0xFF00_0080;
        let x2: u32 = 0xFF00_FF00;
        let x3: u32 = 0xFF80_0000;
        assert_eq!(a1.color, x1);
        assert_eq!(a2.color, x2);
        assert_eq!(a3.color, x3);

        let unpacked: Srgb<u8> = Packed::<Abgr>::from(0x80FF_FF80).into();
        assert_relative_eq!(
            Srgb::new(0.5, 1.0, 1.0),
            unpacked.into_format(),
            epsilon = 0.01
        );

        let b1: Packed<Abgr> = Srgba::new(0.5, 0.0, 0.0, 0.0).into_format().into();
        let b2: Packed<Abgr> = Srgba::new(0.0, 1.0, 0.0, 0.0).into_format().into();
        let b3: Packed<Abgr> = Srgba::new(0.0, 0.0, 0.5, 0.0).into_format().into();
        let b4: Packed<Abgr> = Srgba::new(0.0, 0.0, 0.0, 1.0).into_format().into();
        let y1: u32 = 0x0000_0080;
        let y2: u32 = 0x0000_FF00;
        let y3: u32 = 0x0080_0000;
        let y4: u32 = 0xFF00_0000;
        assert_eq!(b1.color, y1);
        assert_eq!(b2.color, y2);
        assert_eq!(b3.color, y3);
        assert_eq!(b4.color, y4);

        let unpacked: Srgba<u8> = Packed::<Abgr>::from(0x80FF_FF80).into();
        assert_relative_eq!(
            Srgba::new(0.5, 1.0, 1.0, 0.5),
            unpacked.into_format(),
            epsilon = 0.01
        );
    }

    #[test]
    fn u32_to_color() {
        assert_eq!(0xFFFF_FF80, u32::from(Srgb::new(255u8, 255, 128)));
        assert_eq!(0x7FFF_FF80, u32::from(Srgba::new(127u8, 255u8, 255, 128)));
    }
}
