use core::ops::{Add, Div};

use crate::component::Component;
use crate::{from_f64, FromF64};

/// A trait for calculating relative contrast between two colors.
///
/// W3C's Web Content Accessibility Guidelines (WCAG) 2.1 suggest a method
/// to calculate accessible contrast ratios of text and background colors for
/// those with low vision or color deficiencies, and for contrast of colors used
/// in user interface graphics objects.
///
/// These criteria are recommendations, not hard and fast rules. Most
/// importantly, look at the colors in action and make sure they're clear and
/// comfortable to read. A pair of colors may pass contrast guidelines but still
/// be uncomfortable to look at. Favor readability over only satisfying the
/// contrast ratio metric. It is recommended to verify the contrast ratio
/// in the output format of the colors and not to assume the contrast ratio
/// remains exactly the same across color formats. The following example checks
/// the contrast ratio of two colors in RGB format.
///
/// ```rust
/// use std::str::FromStr;
/// use palette::{Srgb, RelativeContrast};
/// # fn main() -> Result<(), palette::rgb::FromHexError> {
///
/// // the rustdoc "DARK" theme background and text colors
/// let my_background_rgb: Srgb<f32> = Srgb::from(0x353535).into_format();
/// let my_foreground_rgb = Srgb::from_str("#ddd")?.into_format();
///
/// assert!(my_background_rgb.has_enhanced_contrast_text(&my_foreground_rgb));
/// # Ok(())
/// # }
/// ```
///
/// The possible range of contrast ratios is from 1:1 to 21:1. There is a
/// Success Criterion for Contrast (Minimum) and a Success Criterion for
/// Contrast (Enhanced), SC 1.4.3 and SC 1.4.6 respectively, which are concerned
/// with text and images of text. SC 1.4.11 is a Success Criterion for "non-text
/// contrast" such as user interface components and other graphics. The relative
/// contrast is calculated by `(L1 + 0.05) / (L2 + 0.05)`, where `L1` is the
/// luminance of the brighter color and `L2` is the luminance of the darker
/// color both in sRGB linear space. A higher contrast ratio is generally
/// desireable.
///
/// For more details, visit the following links:
///
/// [Success Criterion 1.4.3 Contrast (Minimum) (Level AA)](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum)
///
/// [Success Criterion 1.4.6 Contrast (Enhanced) (Level AAA)](https://www.w3.org/WAI/WCAG21/Understanding/contrast-enhanced)
///
/// [Success Criterion 1.4.11 Non-text Contrast (Level AA)](https://www.w3.org/WAI/WCAG21/Understanding/non-text-contrast.html)
#[doc(alias = "wcag")]
pub trait RelativeContrast {
    /// The type of the contrast ratio.
    type Scalar: FromF64 + PartialOrd;

    /// Calculate the contrast ratio between two colors.
    fn get_contrast_ratio(&self, other: &Self) -> Self::Scalar;
    /// Verify the contrast between two colors satisfies SC 1.4.3. Contrast
    /// is at least 4.5:1 (Level AA).
    fn has_min_contrast_text(&self, other: &Self) -> bool {
        self.get_contrast_ratio(other) >= from_f64(4.5)
    }
    /// Verify the contrast between two colors satisfies SC 1.4.3 for large
    /// text. Contrast is at least 3:1 (Level AA).
    fn has_min_contrast_large_text(&self, other: &Self) -> bool {
        self.get_contrast_ratio(other) >= from_f64(3.0)
    }
    /// Verify the contrast between two colors satisfies SC 1.4.6. Contrast
    /// is at least 7:1 (Level AAA).
    fn has_enhanced_contrast_text(&self, other: &Self) -> bool {
        self.get_contrast_ratio(other) >= from_f64(7.0)
    }
    /// Verify the contrast between two colors satisfies SC 1.4.6 for large
    /// text. Contrast is at least 4.5:1 (Level AAA).
    fn has_enhanced_contrast_large_text(&self, other: &Self) -> bool {
        self.has_min_contrast_text(other)
    }
    /// Verify the contrast between two colors satisfies SC 1.4.11 for graphical
    /// objects. Contrast is at least 3:1 (Level AA).
    fn has_min_contrast_graphics(&self, other: &Self) -> bool {
        self.has_min_contrast_large_text(other)
    }
}

/// Calculate the ratio between two `luma` values.
pub fn contrast_ratio<T>(luma1: T, luma2: T) -> T
where
    T: Component + FromF64 + Add<Output = T> + Div<Output = T>,
{
    if luma1 > luma2 {
        (luma1 + from_f64(0.05)) / (luma2 + from_f64(0.05))
    } else {
        (luma2 + from_f64(0.05)) / (luma1 + from_f64(0.05))
    }
}

#[cfg(test)]
mod test {
    use core::str::FromStr;

    use crate::RelativeContrast;
    use crate::Srgb;

    #[test]
    fn relative_contrast() {
        let white = Srgb::new(1.0, 1.0, 1.0);
        let black = Srgb::new(0.0, 0.0, 0.0);

        assert_relative_eq!(white.get_contrast_ratio(&white), 1.0);
        assert_relative_eq!(white.get_contrast_ratio(&black), 21.0);
        assert_relative_eq!(
            white.get_contrast_ratio(&black),
            black.get_contrast_ratio(&white)
        );

        let c1 = Srgb::from_str("#600").unwrap().into_format();

        assert_relative_eq!(c1.get_contrast_ratio(&white), 13.41, epsilon = 0.01);
        assert_relative_eq!(c1.get_contrast_ratio(&black), 1.56, epsilon = 0.01);

        assert!(c1.has_min_contrast_text(&white));
        assert!(c1.has_min_contrast_large_text(&white));
        assert!(c1.has_enhanced_contrast_text(&white));
        assert!(c1.has_enhanced_contrast_large_text(&white));
        assert!(c1.has_min_contrast_graphics(&white));
        assert!(c1.has_min_contrast_text(&black) == false);
        assert!(c1.has_min_contrast_large_text(&black) == false);
        assert!(c1.has_enhanced_contrast_text(&black) == false);
        assert!(c1.has_enhanced_contrast_large_text(&black) == false);
        assert!(c1.has_min_contrast_graphics(&black) == false);

        let c1 = Srgb::from_str("#066").unwrap().into_format();

        assert_relative_eq!(c1.get_contrast_ratio(&white), 6.79, epsilon = 0.01);
        assert_relative_eq!(c1.get_contrast_ratio(&black), 3.09, epsilon = 0.01);

        let c1 = Srgb::from_str("#9f9").unwrap().into_format();

        assert_relative_eq!(c1.get_contrast_ratio(&white), 1.22, epsilon = 0.01);
        assert_relative_eq!(c1.get_contrast_ratio(&black), 17.11, epsilon = 0.01);
    }
}
