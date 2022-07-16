# Palette

A color management and conversion library that focuses on maintaining correctness, flexibility and ease of use. It makes use of the type system to prevent mistakes, support a wide range of color spaces (including user defined variants) and offer different ways of integrating with other libraries.

[The announcement post for 0.6.0](https://ogeon.github.io/2021/07/12/palette-0.6.0.html).

## Feature Summary

* Type system representations of color spaces, including RGB, HSL, HSV, HWB, L\*a\*b\*, L\*C\*h°, XYZ and xyY.
* Copy free conversion to and from color buffers allows simple integration with other crates and systems.
* Color operations implemented as traits, such as arithmetic, lighten/darken, hue shifting, mixing/interpolating, and SVG blend functions.
* Provides types for creating gradients.
* Color spaces can be customized, using type parameters, to support different levels of precision, linearity, white points, RGB standards, etc.
* Supports `#[no_std]`, with only gradients disabled.
* Optional `serde` and `rand` integration.

## Minimum Supported Rust Version (MSRV)

This version of Palette has been automatically tested with Rust version `1.60.0` and the `stable`, `beta`, and `nightly` channels. Future versions of the library may advance the minimum supported version to make use of new language features, but this will normally be considered a breaking change. Exceptions may be made for security patches and similar changes.

## Getting Started

Add the following lines to your `Cargo.toml` file:

```toml
[dependencies]
palette = "0.6"
```

or these lines if you want to opt out of `std`:

```toml
[dependencies.palette]
version = "0.6"
default-features = false
features = ["libm"] # Uses libm instead of std for floating point math
```

### Cargo Features

These features are enabled by default:

* `"named"` - Enables color constants, located in the `named` module.
* `"named_from_str"` - Enables `named::from_str`, which maps name strings to colors.
* `"named_gradients"`- Enables gradient constants, located in `gradient::named`. This requires the standard library.
* `"std"` - Enables use of the standard library.

These features are disabled by default:

* `"serializing"` - Enables color serializing and deserializing using [`serde`].
* `"random"` - Enables generating random colors using [`rand`].
* `"libm"` - Uses the [`libm`] floating point math library (for when the `std` feature is disabled).
* `"bytemuck"` - Enables casting between plain data types using [`bytemuck`].

### Using palette in an embedded environment

Palette supports `#![no_std]` environments by disabling the `"std"` feature. However, there are some things that are unavailable without the standard library:

* Gradients are unavailable, because they depend heavily on Vectors.
* Serialization using `serde` is unavailable.

It uses [`libm`] to provide the floating-point operations that are typically in `std`.

## Examples

These are examples of some of the features listed in the feature summary.

### Converting

It's possible to convert from one color space to another with the `FromColor` and `IntoColor` traits. They are similar to `From` and `Into`, but tailored for colors:

```rust
use palette::{FromColor, Hsl, IntoColor, Lch, Srgb};

let my_rgb = Srgb::new(0.8, 0.3, 0.3);

let mut my_lch = Lch::from_color(my_rgb);
my_lch.hue += 180.0;

let mut my_hsl: Hsl = my_lch.into_color();
my_hsl.lightness *= 0.6;

let my_new_rgb = Srgb::from_color(my_hsl);
```

This image shows the starting color and the results of the two changes:

![The result of each step in the "converting" example.](https://raw.githubusercontent.com/Ogeon/palette/05e60121f3ab39aba972c477f258c70d0495551d/gfx/readme_converting.png)

Most of the common color spaces are already implemented in Palette, but some situations may require something more customized. The conversion traits make it possible to integrate custom color types into the system. For example, this can be used for adding new color spaces or making a simpler user-facing API.

A longer and more advanced example that shows how to implement the conversion traits for a custom color type can be found further down.

### Pixels And Buffers

When working with image or pixel buffers, or any color type that can be converted to a slice of components (ex. `&[u8]`), the `Pixel` trait provides methods for turning them into slices of Palette colors without cloning the whole buffer:

```rust
use palette::{Srgb, Pixel};

// The input to this function could be data from an image file or
// maybe a texture in a game.
fn swap_red_and_blue(my_rgb_image: &mut [u8]) {
    // Convert `my_rgb_image` into `&mut [Srgb<u8>]` without copying.
    let my_rgb_image = Srgb::from_raw_slice_mut(my_rgb_image);

    for color in my_rgb_image {
        std::mem::swap(&mut color.red, &mut color.blue);
    }
}
```

| Before | After |
|--------|-------|
| ![The fruit image before swapping the red and blue color channels.](https://raw.githubusercontent.com/Ogeon/palette/05e60121f3ab39aba972c477f258c70d0495551d/example-data/input/fruits-128.png) | ![The fruit image with the red and blue color channels swapped.](https://raw.githubusercontent.com/Ogeon/palette/05e60121f3ab39aba972c477f258c70d0495551d/gfx/readme_pixels_and_buffers.png) |

It's also possible to create a single color from a slice or array. Let's say we are using something that implements `AsMut<[u8; 3]>`:

```rust
use palette::{Srgb, Pixel};

fn swap_red_and_blue(mut my_rgb: impl AsMut<[u8; 3]>) {
    let my_rgb = Srgb::from_raw_mut(my_rgb.as_mut());

    std::mem::swap(&mut my_rgb.red, &mut my_rgb.blue);
}
```

This makes it possible to use Palette with any other crate that can convert their color types to slices and arrays, with minimal glue code and little to no overhead. It's also possible to go the opposite direction and convert Palette types to slices and arrays.

### Color Operations

Palette comes with a number of color operations built in, such as saturate/desaturate, hue shift, etc., in the form of operator traits. That means it's possible to write generic functions that perform these operation on any color space that supports them. The output will vary depending on the color space's characteristics.

```rust
use palette::{Hue, Shade, Mix, Hsl, Hsv};

fn transform_color<C>(color: &C, amount: f32) -> C
where
    C: Hue + Shade<Scalar=f32> + Mix<Scalar=f32> + Clone,
    f32: Into<C::Hue>,
{
    let new_color = color.shift_hue(170.0).lighten(1.0);

    // Interpolate between the old and new color.
    color.mix(&new_color, amount)
}

let new_hsl = transform_color(&Hsl::new(0.00, 0.70, 0.20), 0.8);
let new_hsv = transform_color(&Hsv::new(0.00, 0.82, 0.34), 0.8);
```

This image shows the transition from the color to `new_color` in HSL and HSV:

![Gradients showing the transition from the starting color to the modified color in HSL and HSV.](https://raw.githubusercontent.com/Ogeon/palette/05e60121f3ab39aba972c477f258c70d0495551d/gfx/readme_color_operations_1.png)

In addition to the operator traits, the SVG blend functions have also been implemented.

```rust
use palette::{blend::Blend, Pixel, Srgb, WithAlpha};

// The input to this function could be data from image files.
fn alpha_blend_images(image1: &mut [u8], image2: &[u8]) {
    // Convert the images into `&mut [Srgb<u8>]` and `&[Srgb<u8>]` without copying.
    let image1 = Srgb::from_raw_slice_mut(image1);
    let image2 = Srgb::from_raw_slice(image2);

    for (color1, color2) in image1.iter_mut().zip(image2) {
        // Convert the colors to linear floating point format and give them transparency values.
        let color1_alpha = color1.into_format().into_linear().opaque();
        let color2_alpha = color2.into_format().into_linear().with_alpha(0.5);

        // Alpha blend `color2_alpha` over `color1_alpha`.
        let blended = color2_alpha.over(color1_alpha);

        // Convert the color part back to `Srgb<u8>` and overwrite the value in image1.
        *color1 = blended.color.into_encoding().into_format();
    }
}
```

| Image 1 | Image 2 | Result |
|---------|---------|--------|
| ![A photo of various fruit.](https://raw.githubusercontent.com/Ogeon/palette/05e60121f3ab39aba972c477f258c70d0495551d/example-data/input/fruits-128.png) | ![A photo of kitten in a strawhat.](https://raw.githubusercontent.com/Ogeon/palette/05e60121f3ab39aba972c477f258c70d0495551d/example-data/input/cat-128.png) |![Image 2 blended over Image 1 with 50% transparency.](https://raw.githubusercontent.com/Ogeon/palette/05e60121f3ab39aba972c477f258c70d0495551d/gfx/readme_color_operations_2.png)

There's also the option to explicitly convert to and from premultiplied alpha, to avoid converting back and forth more than necessary, using `Blend::into_premultiplied` and `Blend::from_premultiplied`.

### Gradients

The `Gradient` type provides basic support for linear gradients in any color space that implements the `Mix` trait. As an example, the `Gradient::take` method returns an iterator over a number of points along the gradient:

```rust
use palette::{Gradient, LinSrgb};

let gradient = Gradient::new(vec![
    LinSrgb::new(0.00, 0.05, 0.20),
    LinSrgb::new(0.70, 0.10, 0.20),
    LinSrgb::new(0.95, 0.90, 0.30),
]);

let taken_colors: Vec<_> = gradient.take(10).collect();
```

Here's the gradient as both its continuous form and as the 10 colors from `.take(10)`:

![An illustration of the gradient with the continuous form above a row of discrete color swatches.](https://raw.githubusercontent.com/Ogeon/palette/05e60121f3ab39aba972c477f258c70d0495551d/gfx/readme_gradients_1.png)

There's also support for arbitrary spacing between the input points:

```rust
use palette::{Gradient, LinSrgb};

let gradient = Gradient::from([
    (0.0, LinSrgb::new(0.00, 0.05, 0.20)), // A pair of position and color.
    (0.2, LinSrgb::new(0.70, 0.10, 0.20)),
    (1.0, LinSrgb::new(0.95, 0.90, 0.30)),
]);

let taken_colors: Vec<_> = gradient.take(10).collect();
```

Here's the gradient after the middle point has been moved more towards the beginning:

![An illustration of the gradient with the continuous form above a row of discrete color swatches.](https://raw.githubusercontent.com/Ogeon/palette/05e60121f3ab39aba972c477f258c70d0495551d/gfx/readme_gradients_2.png)

### Customizing Color Spaces

The built-in color spaces have been made customizable to account for as much variation as possible. The more common variants have been exposed as type aliases (like `Srgb`, `Srgba` and `LinSrgb` from above), but it's entirely possible to make custom compositions, including with entirely new parameters. For example, making up your own RGB standard:

```rust
use palette::{
    encoding,
    white_point,
    rgb::Rgb,
    chromatic_adaptation::AdaptFrom,
    Srgb
};

// RgbStandard and RgbSpace are implemented for 2 and 3 element tuples,
// allowing mixing and matching of existing types. In this case we are
// combining sRGB primaries, the CIE equal energy white point and the
// sRGB transfer function (a.k.a. encoding or gamma).
type EqualEnergyStandard = (encoding::Srgb, white_point::E, encoding::Srgb);
type EqualEnergySrgb<T> = Rgb<EqualEnergyStandard, T>;

let ee_rgb = EqualEnergySrgb::new(1.0, 0.5, 0.3);

// We need to use chromatic adaptation when going between white points.
let srgb = Srgb::adapt_from(ee_rgb);
```

It's also possible to implement the traits for a custom type, for when the built-in options are not enough.

### Converting Custom Color Types

The following example shows how it's possible for Palette users to convert from and into a custom made `Color` type. It's not exactly a one-liner, but it can still save a lot of repetitive manual work.

```rust
use palette::{
    convert::FromColorUnclamped,
    encoding,
    rgb::{Rgb, RgbStandard},
    IntoColor, WithAlpha, Clamp, Srgb, Lcha
};

// This implements conversion to and from all Palette colors.
#[derive(FromColorUnclamped, WithAlpha)]
// We have to tell Palette that we will take care of converting to/from sRGB.
#[palette(skip_derives(Rgb), rgb_standard = "encoding::Srgb")]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    // Let Palette know this is our alpha channel.
    #[palette(alpha)]
    a: f32,
}

// There's no blanket implementation for Self -> Self, unlike the From trait.
// This is to better allow cases like Self<A> -> Self<B>.
impl FromColorUnclamped<Color> for Color {
    fn from_color_unclamped(color: Color) -> Color {
        color
    }
}

// Convert from any kind of f32 sRGB.
impl<S> FromColorUnclamped<Rgb<S, f32>> for Color
where
    S: RgbStandard<Space = encoding::Srgb>,
{
    fn from_color_unclamped(color: Rgb<S, f32>) -> Color {
        let srgb = Srgb::from_color_unclamped(color);
        Color { r: srgb.red, g: srgb.green, b: srgb.blue, a: 1.0 }
    }
}

// Convert into any kind of f32 sRGB.
impl<S> FromColorUnclamped<Color> for Rgb<S, f32>
where
    S: RgbStandard<Space = encoding::Srgb>,
{
    fn from_color_unclamped(color: Color) -> Self {
        let srgb = Srgb::new(color.r, color.g, color.b);
        Self::from_color_unclamped(srgb)
    }
}

// Add the required clamping and validation.
impl Clamp for Color {
    fn is_within_bounds(&self) -> bool {
        let zero_to_one = 0.0..=1.0;

        zero_to_one.contains(&self.r)
            && zero_to_one.contains(&self.g)
            && zero_to_one.contains(&self.b)
            && zero_to_one.contains(&self.a)
    }

    fn clamp(&self) -> Self {
        Color {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
            a: self.a.min(1.0).max(0.0),
        }
    }

    fn clamp_self(&mut self) {
        *self = self.clamp();
    }
}


// This function uses only our `Color`, but Palette users can convert to it.
fn do_something(color: Color) {
    // ...
}

do_something(Color { r: 1.0, g: 0.0, b: 1.0, a: 0.5 });
do_something(Lcha::new(60.0, 116.0, 328.0, 0.5).into_color());


// This function has the conversion built in and takes any compatible
// color type as input.
fn generic_do_something(color: impl IntoColor<Color>) {
    let color = color.into_color();
    // ...
}

generic_do_something(Color { r: 1.0, g: 0.0, b: 1.0, a: 0.5 });
generic_do_something(Lcha::new(60.0, 116.0, 328.0, 0.5));
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

[`serde`]: https://crates.io/crates/serde
[`rand`]: https://crates.io/crates/rand
[`libm`]: https://crates.io/crates/libm
[`bytemuck`]: https://crates.io/crates/bytemuck
