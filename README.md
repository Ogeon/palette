# palette

[![Build Status](https://travis-ci.org/Ogeon/palette.svg?branch=master)](https://travis-ci.org/Ogeon/palette)
[![Crates.io](https://img.shields.io/crates/v/palette.svg)](https://crates.io/crates/palette/)
[![Docs.rs](https://docs.rs/palette/badge.svg)](https://docs.rs/palette)

A Rust library that makes linear color calculations and conversion easy and accessible for anyone. It provides both precision tools that lets you work in exactly the color space you want to, as well as a general color type that abstracts away some of the technical details.

## Online Documentation

[Released](https://docs.rs/palette/0.5.0/palette/)

[Master branch](https://ogeon.github.io/docs/palette/master/palette/index.html)

## Cargo.toml Entries

Add the following lines to your `Cargo.toml` file:

```toml
[dependencies]
palette = "0.5"
```

### Features

These features are enabled by default:

* `"named"` - Enables color constants, located in the `named` module.
* `"named_from_str"` - Enables the `named::from_str`, which maps name string to colors. This requires the standard library.
* `"std"` - Enables use of the standard library.

These features are disabled by default:

* `"serializing"` - Enables color serializing and deserializing using `serde`.
* `"libm"` - Makes it use the `libm` floating point math library. It's only for when the `"std"` feature is disabled.

### Without the standard library

Here is an example `Cargo.toml` entry for using palette on `#![no_std]`:

```toml
[dependencies.palette]
version = "0.4"
default-features = false
features = ["libm"] # Makes it use libm instead of std for float math
```

## It's Never "Just RGB"

Colors in, for example, images, are often "gamma corrected", or converted using some non-linear transfer function into a format like sRGB before being stored or displayed. This is done as a compression method and to prevent banding, and is also a bit of a legacy from the ages of the CRT monitors, where the output from the electron gun was nonlinear. The problem is that these formats are *non-linear color spaces*, which means that many operations that you may want to perform on colors (addition, subtraction, multiplication, linear interpolation, etc.) will work unexpectedly when performed in such a non-linear color space. As such, the compression has to be reverted to restore linearity and make sure that many operations on the colors are accurate. 

But, even when colors *are* 'linear', there is yet more to explore.

The most common way that colors are defined, especially for computer storage, is in terms of so-called *tristimulus values*, meaning that all colors are defined as a vector of three values which may represent any color. The reason colors can generally be stored as only a three dimensional vector, and not an *n* dimensional one, where *n* is some number of possible frequencies of light, is because our eyes contain only three types of cones. Each of these cones have different sensitivity curves to different wavelengths of light, giving us three "dimensions" of sensitivity to color. These cones are often called the S, M, and L (for small, medium, and large) cones, and their sensitivity curves *roughly* position them as most sensitive to "red", "green", and "blue" parts of the spectrum. As such, we can choose only three values to represent any possible color that a human is able to see. An interesting consequence of this is that humans can see two different objects which are emitting *completely different actual light spectra* as the *exact same perceptual color* so long as those wavelengths, when transformed by the sensitivity curves of our cones, end up resulting in the same S, M, and L values sent to our brains.

A **color space** (which simply refers to a set of standards by which we map a set of arbitrary values to real-world colors) which uses tristimulus values is often defined in terms of

1. Its **primaries**
2. Its **reference white** or **white point**

The **primaries** together represent the total *gamut* (i.e. displayable range of colors) of that color space, while the **white point** defines which concrete tristimulus value corresponds to a real, physical white reflecting object being lit by a known light source and observed by the 'standard observer' (i.e. a standardized model of human color perception).

The informal "RGB" color space is such a tristimulus color space, since it is defined by three values, but it is underspecified since we don't know which primaries are being used (i.e. how exactly are the canonical "red", "green", and "blue" defined?), nor its white point. In most cases, when people talk about "RGB" or "Linear RGB" colors, what they are *actually* talking about is the "Linear sRGB" color space, which uses the primaries and white point defined in the sRGB standard, but which *does not* have the (non-linear) sRGB *transfer function* applied.

This library takes these things into account, and attempts to provide an interface which will let those who don't care so much about the intricacies of color still use colors correctly, while also allowing the advanced user a high degree of flexibility in how they use it.

## What It Can Do

Palette provides tools for both color manipulation and conversion between color spaces. These are some highlights.

### Color Spaces

"RGB" (which we now know, from the discussion in the previous section, is usually actually Linear sRGB) and other tristimulus based spaces like CIE Xyz are probably the most widely known color spaces. These spaces are great when you want to perform physically correct math on color (like in a 2d or 3d rendering program) but there are also color spaces that are not defined in terms of tristimulus values.

You have probably used a color picker with a rainbow wheel and a brightness slider. That may have been an HSV or an HSL color picker, where the color is encoded as hue, saturation and brightness/lightness. Even though these spaces are defined using 3 values, they *aren't* based on tristimulus values, since those three values don't have a direct relation to human vision (i.e. our S, M, and L cones, as discussed in the previous section). Such color spaces are excellent when it comes to humans intuitively selecting color values, though, and as such are the go-to choice when this interaction is needed. They can then be converted into other color spaces in order to actually perform modifications to them

There's also a group of color spaces that are designed to be perceptually uniform, meaning that the perceptual change is equal to the numerical change. An example of this is the CIE L\*a\*b\* color space. These color spaces are excellent when you want to "blend" between colors in a *perceptually pleasing* manner (for example, in a data visualization) rather than a *physically correct* one.

Selecting the proper color space can have a big impact on how the resulting image looks (as illustrated by some of the programs in `examples`), and Palette makes the conversion between them as easy as a call to `from_color` or `into_color`.

This example takes an sRGB color, converts it to CIE L\*C\*h°, a color space similar to the colloquial HSL/HSV color spaces, shifts its hue by 180° and converts it back to RGB:

```Rust
use palette::{FromColor, Hue, IntoColor, Lch, Srgb};

let lch_color: Lch = Srgb::new(0.8, 0.2, 0.1).into_color();
let new_color = Srgb::from_color(lch_color.shift_hue(180.0));
```

This results in the following two colors:

![Hue Shift Comparison](gfx/readme_color_spaces.png)

### Manipulation

Palette comes with a number of color manipulation tools, that are implemented as traits. These includes lighten/darken, saturate/desaturate and hue shift. These traits are only implemented on types where they are meaningful, which means that you can't shift the hue of an RGB color without converting it to a color space where it makes sense.

The following example shows how to make a lighter and a desaturated version of the original.

```Rust
use palette::{FromColor, Saturate, Shade, Srgb, Lch};

let color = Srgb::new(0.8, 0.2, 0.1).into_linear();
let lighter = color.lighten(0.1);
let desaturated = Lch::from_color(color).desaturate(0.5);
```

This results in the following three colors:

![Manipulation Comparison](gfx/readme_manipulation.png)

### Gradients

There is also a linear gradient type which makes it easy to interpolate between a series of colors. This gradient can be used in any color space and it can be used to make color sequence iterators.

The following example shows three gradients between the same two endpoints, but the top is in RGB space while the middle and bottom are in HSV space. The bottom gradient is an example of using the color sequence iterator.

```Rust
use palette::{FromColor, LinSrgb, Hsv, Gradient};

let grad1 = Gradient::new(vec![
    LinSrgb::new(1.0, 0.1, 0.1),
    LinSrgb::new(0.1, 1.0, 1.0)
]);

let grad2 = Gradient::new(vec![
    Hsv::from_color(LinSrgb::new(1.0, 0.1, 0.1)),
    Hsv::from_color(LinSrgb::new(0.1, 1.0, 1.0))
]);
```

The RGB gradient goes through gray, while the HSV gradients only change hue:

![Gradient Comparison](gfx/readme_gradients.png)

### Working with Raw Color Types

Palette supports converting from a raw buffer of data into a color type using the `Pixel` trait. This is useful for interoperation with other crates or programs.

Oftentimes, pixel data is stored in a raw buffer such as a `[u8; 3]`. `from_raw` can be used to convert into a Palette color, `into_format` converts from  `Srgb<u8>` to `Srgb<f32>`, and finally `into_raw` to convert from a Palette color back to a `[u8;3]`.

Here's an example of turning a buffer of `[u8; 3]` into a Palette `Srgb` color and back to a raw buffer.

```rust
use approx::assert_relative_eq;
use palette::{Srgb, Pixel};

let buffer = [255, 0, 255];
let raw = Srgb::from_raw(&buffer);
assert_eq!(raw, &Srgb::<u8>::new(255u8, 0, 255));

let raw_float: Srgb<f32> = raw.into_format();
assert_relative_eq!(raw_float, Srgb::new(1.0, 0.0, 1.0));

let raw: [u8; 3] = Srgb::into_raw(raw_float.into_format());
assert_eq!(raw, buffer);
```

## What It Isn't

This library is only meant for color manipulation and conversion. It's not a fully featured image manipulation library. It will only handle colors, and not whole images. There are features that are meant to work as bridges between Palette and other graphical libraries, but the main features are limited to only focus on single pixel operations, to keep the scope at a manageable size.

[pixel_module]: https://ogeon.github.io/docs/palette/master/palette/pixel/index.html

## Using palette in an embedded environment

Palette supports `#![no_std]` environments by disabling the `"std"` feature. However, there are some things that are unavailable without the standard library:

* Gradients are unavailable, because they depend heavily on Vectors
* The `"named_from_str"` feature requires the standard library as well
* Serialization using `serde` is unavailable

It uses [`libm`] to provide the floating-point operations that are typically in `std`.

[`libm`]: https://github.com/japaric/libm

## Contributing

All sorts of contributions are welcome, no matter how huge or tiny, so take a look at [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines, if you are interested.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
