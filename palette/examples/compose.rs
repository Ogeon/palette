//! Demonstrates the output of composition in different color spaces and some of
//! their characteristics.
//!
//! The output image shows the color spaces as groups of three columns. The
//! color spaces are sRGB, XYZ, xyY, L\*a\*b\*, Oklab, and L\*u\*v\*. The
//! columns within a group show the foreground image with its transparency
//! multiplied by 0%, 50%, and 100% each.
//!
//! Each row is a different composition function, ordered from top to bottom as
//! `over`, `inside`, `outside`, `atop`, `xor`, and `plus`.
//!
//! The foreground image and the background image are first composed together in
//! the group's color space. The result of that is then composed in RGB over the
//! background color, before being written to the output buffer.

use palette::{
    blend::{Compose, PreAlpha, Premultiply},
    Alpha, FromColor, IntoColor, Laba, LinSrgba, Luva, Oklaba, Srgba, Xyza, Yxya,
};

const ALPHA_STEPS: u32 = 3;
const COLOR_SPACES: u32 = 6;
const COLUMNS: u32 = ALPHA_STEPS * COLOR_SPACES;
const BLEND_FNS: u32 = 6;

fn main() {
    let background = image::open("example-data/input/compose_bg.png")
        .expect("could not open 'example-data/input/compose_bg.png'")
        .to_rgba8();
    let foreground = image::open("example-data/input/compose_fg.png")
        .expect("could not open 'example-data/input/compose_fg.png'")
        .to_rgba8();

    let tile_width = background.width();
    let tile_height = background.height();

    let mut image = image::RgbaImage::new(tile_width * COLUMNS, tile_height * BLEND_FNS);

    for ((x, y, background), foreground) in background.enumerate_pixels().zip(foreground.pixels()) {
        let background = Srgba::from(background.0).into_format();
        let foreground = Srgba::from(foreground.0).into_format();

        let bg_rgb = background.into();
        let bg_xyz = Xyza::from_color(background).into();
        let bg_yxy = Yxya::from_color(background).into();
        let bg_lab = Laba::from_color(background).into();
        let bg_oklab = Oklaba::from_color(background).into();
        let bg_luv = Luva::from_color(background).into();

        for alpha_step in 0..ALPHA_STEPS {
            // Copy the original so we don't stack alpha changes.
            let mut foreground = foreground;
            foreground.alpha *= alpha_step as f32 / (ALPHA_STEPS - 1) as f32;

            draw_composed_pixels(
                &mut image,
                x,
                y,
                0 * ALPHA_STEPS + alpha_step,
                tile_width,
                tile_height,
                foreground.into(),
                bg_rgb,
            );

            draw_composed_pixels(
                &mut image,
                x,
                y,
                1 * ALPHA_STEPS + alpha_step,
                tile_width,
                tile_height,
                Xyza::from_color(foreground).into(),
                bg_xyz,
            );

            draw_composed_pixels(
                &mut image,
                x,
                y,
                2 * ALPHA_STEPS + alpha_step,
                tile_width,
                tile_height,
                Yxya::from_color(foreground).into(),
                bg_yxy,
            );

            draw_composed_pixels(
                &mut image,
                x,
                y,
                3 * ALPHA_STEPS + alpha_step,
                tile_width,
                tile_height,
                Laba::from_color(foreground).into(),
                bg_lab,
            );

            draw_composed_pixels(
                &mut image,
                x,
                y,
                4 * ALPHA_STEPS + alpha_step,
                tile_width,
                tile_height,
                Oklaba::from_color(foreground).into(),
                bg_oklab,
            );

            draw_composed_pixels(
                &mut image,
                x,
                y,
                5 * ALPHA_STEPS + alpha_step,
                tile_width,
                tile_height,
                Luva::from_color(foreground).into(),
                bg_luv,
            );
        }
    }

    let _ = std::fs::create_dir("example-data/output");
    match image.save("example-data/output/compose.png") {
        Ok(()) => println!("see 'example-data/output/compose.png' for the result"),
        Err(e) => println!("failed to write 'example-data/output/compose.png': {}", e),
    }
}

fn draw_composed_pixels<C>(
    image: &mut image::RgbaImage,
    x: u32,
    y: u32,
    column: u32,
    tile_width: u32,
    tile_height: u32,
    fg: PreAlpha<C>,
    bg: PreAlpha<C>,
) where
    PreAlpha<C>: Compose + Copy,
    C: Premultiply<Scalar = f32>,
    Alpha<C, f32>: IntoColor<LinSrgba>,
{
    let x = x + tile_width * column;
    let functions = [
        Compose::over,
        Compose::inside,
        Compose::outside,
        Compose::atop,
        Compose::xor,
        Compose::plus,
    ];

    for (row, function) in IntoIterator::into_iter(functions).enumerate() {
        let row = row as u32;

        image.put_pixel(
            x,
            y + tile_height * row,
            compose(column, row, fg, bg, function),
        );
    }
}

fn compose<C>(
    column: u32,
    row: u32,
    fg: PreAlpha<C>,
    bg: PreAlpha<C>,
    function: fn(PreAlpha<C>, PreAlpha<C>) -> PreAlpha<C>,
) -> image::Rgba<u8>
where
    C: Premultiply<Scalar = f32>,
    PreAlpha<C>: Compose,
    Alpha<C, f32>: IntoColor<LinSrgba>,
{
    let composed = function(fg, bg).unpremultiply().into_color();

    let image_background = if (column + row) % 2 == 0 {
        Srgba::new(0.1, 0.1, 0.1, 1.0).into_linear().into_color()
    } else {
        Srgba::new(0.14, 0.14, 0.14, 1.0).into_linear().into_color()
    };

    let result = composed.over(image_background);

    image::Rgba(Srgba::from_linear(result).into())
}
