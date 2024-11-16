//! A collection of named color constants. Can be toggled with the `"named"`
//! Cargo features.
//!
//! They are taken from the [SVG keyword
//! colors](https://www.w3.org/TR/SVG11/types.html#ColorKeywords). These are
//! also part of the CSS3 standard.
//!
//! ```
//! use palette::Srgb;
//! use palette::named;
//!
//! //From constant
//! let from_const = named::OLIVE;
//!
//! //From name string
//! let from_str = named::from_str("olive").expect("unknown color");
//!
//! assert_eq!(from_const, from_str);
//! ```

use core::{fmt, iter::FusedIterator};

pub use codegen::*;

mod codegen;

/// Get an SVG/CSS3 color by name.
///
/// The names are the same as the constants, but lower case.
pub fn from_str(name: &str) -> Option<crate::Srgb<u8>> {
    COLORS.get(name).copied()
}

/// Get an iterator over all SVG/CSS3 names and colors in arbitrary order.
///
/// ```
/// use palette::Srgb;
///
/// let red = Srgb::new(255u8, 0, 0);
///
/// let red_entry = palette::named::entries().find(|(name, color)| *color == red);
/// assert_eq!(red_entry, Some(("red", red)));
/// ```
pub fn entries() -> Entries {
    Entries {
        iter: COLORS.entries(),
    }
}

/// Get an iterator over all SVG/CSS3 color names in arbitrary order.
pub fn names() -> Names {
    Names {
        iter: COLORS.keys(),
    }
}

/// Get an iterator over all SVG/CSS3 color values in arbitrary order.
pub fn colors() -> Colors {
    Colors {
        iter: COLORS.values(),
    }
}

/// An iterator over SVG/CSS3 color entries.
#[derive(Clone)]
pub struct Entries {
    iter: phf::map::Entries<'static, &'static str, crate::Srgb<u8>>,
}

impl fmt::Debug for Entries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter.fmt(f)
    }
}

impl Iterator for Entries {
    type Item = (&'static str, crate::Srgb<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(&name, &color)| (name, color))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl DoubleEndedIterator for Entries {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|(&name, &color)| (name, color))
    }
}

impl ExactSizeIterator for Entries {}

impl FusedIterator for Entries {}

/// An iterator over SVG/CSS3 color names.
#[derive(Clone)]
pub struct Names {
    iter: phf::map::Keys<'static, &'static str, crate::Srgb<u8>>,
}

impl fmt::Debug for Names {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter.fmt(f)
    }
}

impl Iterator for Names {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().copied()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl DoubleEndedIterator for Names {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().copied()
    }
}

impl ExactSizeIterator for Names {}

impl FusedIterator for Names {}

/// An iterator over SVG/CSS3 color values.
#[derive(Clone)]
pub struct Colors {
    iter: phf::map::Values<'static, &'static str, crate::Srgb<u8>>,
}

impl fmt::Debug for Colors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter.fmt(f)
    }
}

impl Iterator for Colors {
    type Item = crate::Srgb<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().copied()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl DoubleEndedIterator for Colors {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().copied()
    }
}

impl ExactSizeIterator for Colors {}

impl FusedIterator for Colors {}
