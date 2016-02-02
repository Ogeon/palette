//!A collection of named color constants.
//!
//!They are taken from the [SVG keyword
//!colors](https://www.w3.org/TR/SVG/types.html#ColorKeywords) (same as in
//!CSS3) and they can be used as if they were pixel values:
//!
//!```
//!use palette::Rgb;
//!use palette::pixel::Srgb;
//!use palette::named;
//!
//!//From constant
//!let from_const: Rgb = Srgb::from_pixel(&named::OLIVE).into();
//!
//!//From name string
//!let olive = named::from_str("olive").expect("unknown color");
//!let from_str: Rgb = Srgb::from_pixel(&olive).into();
//!
//!assert_eq!(from_const, from_str);
//!```

include!(concat!(env!("OUT_DIR"), "/named.rs"));

///Get a SVG/CSS3 color by name.
///
///The names are the same as the constants, but lower case.
pub fn from_str(name: &str) -> Option<(u8, u8, u8)> {
    COLORS.get(name).cloned()
}
