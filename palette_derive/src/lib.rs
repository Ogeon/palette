//! Derives traits from the [palette](https://crates.io/crates/palette) crate.

use proc_macro::TokenStream;

macro_rules! syn_try {
    ($e:expr) => {
        match $e {
            Ok(value) => value,
            Err(errors) => {
                trait IntoErrors {
                    fn into_errors(self) -> Vec<::syn::parse::Error>;
                }
                impl IntoErrors for Vec<::syn::parse::Error> {
                    fn into_errors(self) -> Vec<::syn::parse::Error> {
                        self
                    }
                }
                impl IntoErrors for ::syn::parse::Error {
                    fn into_errors(self) -> Vec<::syn::parse::Error> {
                        vec![self]
                    }
                }

                let errors: ::proc_macro2::TokenStream = IntoErrors::into_errors(errors)
                    .iter()
                    .map(::syn::parse::Error::to_compile_error)
                    .collect();
                return ::proc_macro::TokenStream::from(errors);
            }
        }
    };
}

mod alpha;
mod cast;
mod convert;
mod meta;
mod util;

const COLOR_TYPES: &[&str] = &[
    "Rgb",
    "Luma",
    "Hsl",
    "Hsluv",
    "Hsv",
    "Hwb",
    "Lab",
    "Lch",
    "Lchuv",
    "Luv",
    "Oklab",
    "Oklch",
    "Okhwb",
    "Okhsl",
    "Okhsv",
    "Xyz",
    "Yxy",
    #[cfg(feature = "cam16")]
    "Cam16",
    #[cfg(feature = "cam16")]
    "PartialCam16",
    #[cfg(feature = "cam16")]
    "Cam16UcsJmh",
    #[cfg(feature = "cam16")]
    "Cam16UcsJab",
];

const PREFERRED_CONVERSION_SOURCE: &[(&str, &str)] = &[
    ("Rgb", "Xyz"),
    ("Luma", "Xyz"),
    ("Hsl", "Rgb"),
    ("Hsluv", "Lchuv"),
    ("Hsv", "Rgb"),
    ("Hwb", "Hsv"),
    ("Lab", "Xyz"),
    ("Lch", "Lab"),
    ("Lchuv", "Luv"),
    ("Luv", "Xyz"),
    ("Oklab", "Xyz"),
    ("Oklch", "Oklab"),
    ("Okhsl", "Oklab"),
    ("Okhsv", "Oklab"),
    ("Okhwb", "Okhsv"),
    ("Yxy", "Xyz"),
    #[cfg(feature = "cam16")]
    ("Cam16", "Xyz"),
    #[cfg(feature = "cam16")]
    ("PartialCam16", "Xyz"), // Bypasses Cam16 to avoid unnecessary PartialCam16 -> Cam16 -> Xyz
    #[cfg(feature = "cam16")]
    ("Cam16UcsJmh", "PartialCam16"),
    #[cfg(feature = "cam16")]
    ("Cam16UcsJab", "Cam16UcsJmh"),
];

/// Gives better error messages if they are used without having their features
/// activated.
const REQUIRED_COLOR_FEATURES: &[(&str, &str)] = &[
    ("Cam16", "cam16"),
    ("PartialCam16", "cam16"),
    ("Cam16UcsJmh", "cam16"),
    ("Cam16UcsJab", "cam16"),
];

#[proc_macro_derive(WithAlpha, attributes(palette))]
pub fn derive_with_alpha(tokens: TokenStream) -> TokenStream {
    syn_try!(alpha::derive_with_alpha(tokens))
}

#[proc_macro_derive(FromColorUnclamped, attributes(palette))]
pub fn derive_from_color_unclamped(tokens: TokenStream) -> TokenStream {
    syn_try!(convert::derive_from_color_unclamped(tokens))
}

#[proc_macro_derive(ArrayCast, attributes(palette))]
pub fn derive_array_cast(tokens: TokenStream) -> TokenStream {
    syn_try!(cast::derive_array_cast(tokens))
}
