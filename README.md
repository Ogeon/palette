# palette

[![Build Status](https://travis-ci.org/Ogeon/palette.svg?branch=master)](https://travis-ci.org/Ogeon/palette)

A Rust library that makes linear color calculations and conversion easy and
accessible for anyone. It provides both precision tools that lets you work in
exactly the color space you want to, as well as a general color type that
abstracts away some of the technical details.

[Online documentation](https://ogeon.github.io/docs/palette/master/palette/index.html).

# Linear?

Colors in, for example, images are often "gamma corrected" or stored in sRGB
format as a compression method and to prevent banding. This is also a bit of a
legacy from the ages of the CRT monitors, where the output from the electron
guns was nonlinear. The problem is that these formats doesn't represent the
actual intensities, and the compression has to be reverted to make sure that
any operations on the colors are accurate. This library uses a completely
linear work flow, and comes with the tools for transitioning between linear
and non-linear RGB.

# What Can It Do?

Palette provides tools for both color manipulation and conversion between
color spaces. These are some highlights.

## Color Spaces

RGB is probably the most widely known color space, but it's not the only one.
You have probably used a color picker with a rainbow wheel and a brightness
slider. That may have been an HSV or an HSL color picker, where the color is
encoded as hue, saturation and brightness/lightness. There's also a group of
color spaces that are designed to be perceptually uniform, meaning that the
perceptual change is equal to the numerical change.

Selecting the proper color space can have a big impact on how the resulting
image looks (as illustrated by some of the programs in `examples`), and
Palette makes the conversion between them as easy as a call to `from` or
`into`.

## Manipulation

Palette comes with a number of color manipulation tools, that are implemented
as traits. These includes lighten/darken, saturate/desaturate and hue shift.
These traits are only implemented on types where they are meaningful, which
means that you can't shift the hue of an RGB color without converting it to a
color space where it makes sense.

This may seem limiting, but the point is to avoid inconsistent behavior due to
arbitrary defaults, such as saturating a gray color to red when there is no
available hue information. The abstract `Color` type does still support every
operation, for when this is less important.

## Gradients

There is also a linear gradient type which makes it easy to interpolate
between a series of colors. This gradient can be used in any color space and
it can be used to make color sequence iterators.

# What It Isn't

This library is only meant for color manipulation and conversion. It's not...

 * ...an image manipulation library. It will only handle colors, and not whole images.
 * ...an optimal pixel format. The colors are represented by linear 32-bit floats with a mandatory alpha component. It's not a compact format.

You will have to look elsewhere for those features.
