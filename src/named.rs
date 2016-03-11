//!A collection of named color constants. Can be toggled with the `"named"`
//!Cargo feature.
//!
//!They are taken from the [SVG keyword
//!colors](https://www.w3.org/TR/SVG/types.html#ColorKeywords) (same as in
//!CSS3) and they can be used as if they were pixel values:
//!
//!```
//!use palette::LinRgb;
//!use palette::pixel::Srgb;
//!use palette::named;
//!
//!//From constant
//!let from_const: LinRgb = Srgb::from_pixel(&named::OLIVE).into();
//!
//!//From name string
//!let olive = named::from_str("olive").expect("unknown color");
//!let from_str: LinRgb = Srgb::from_pixel(&olive).into();
//!
//!assert_eq!(from_const, from_str);
//!```

include!(concat!(env!("OUT_DIR"), "/named.rs"));

///Get a SVG/CSS3 color by name. Can be toggled with the `"named_from_str"`
///Cargo feature.
///
///The names are the same as the constants, but lower case.
#[cfg(feature = "named_from_str")]
pub fn from_str(name: &str) -> Option<(u8, u8, u8)> {
    COLORS.get(name).cloned()
}
