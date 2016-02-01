//!A collection of named color constants.
//!
//!They are taken from the [SVG keyword
//!colors](https://www.w3.org/TR/SVG/types.html#ColorKeywords) (same as in
//!CSS3) and they can be used as if they were pixel values:
//!
//!```
//!use palette::Rgb;
//!use palette::pixel::Srgb;
//!
//!let color: Rgb = Srgb::from_pixel(&palette::named::OLIVE).into();
//!```

include!(concat!(env!("OUT_DIR"), "/named.rs"));