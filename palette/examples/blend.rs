//! Demonstrates the output of blending in different color spaces and some of
//! their characteristics.
//!
//! The output image shows the color spaces as groups of three columns. The
//! color spaces are RGB and XYZ. The columns within a group show the foreground
//! image with its transparency multiplied by 0%, 50%, and 100% each.
//!
//! Each row is a different composition function, ordered from top to bottom as
//! `multiply`, `screen`, `overlay`, `darken`, `lighten`, `dodge`, `burn`,
//! `hard_light`, `soft_light`, `difference`, and `exclusion`.

use palette::{blend::Blend, Alpha, FromColor, IntoColor, LinSrgba, Srgba, Xyza};

const ALPHA_STEPS: u32 = 3;
const COLOR_SPACES: u32 = 2;
const COLUMNS: u32 = ALPHA_STEPS * COLOR_SPACES;
const BLEND_FNS: u32 = 11;

fn main() {
    let background = image::open("example-data/input/fruits-128.png")
        .expect("could not open 'example-data/input/fruits-128.png'")
        .to_rgba8();
    let foreground = image::open("example-data/input/blend_fg.png")
        .expect("could not open 'example-data/input/blend_fg.png'")
        .to_rgba8();

    let tile_width = background.width();
    let tile_height = background.height();

    let mut image = image::RgbaImage::new(tile_width * COLUMNS, tile_height * BLEND_FNS);

    for ((x, y, background), foreground) in background.enumerate_pixels().zip(foreground.pixels()) {
        let background = Srgba::from(background.0).into_linear();
        let foreground = Srgba::from(foreground.0).into_linear();

        let bg_rgb = background;
        let bg_xyz = Xyza::from_color(background);

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
                foreground,
                bg_rgb,
            );

            draw_composed_pixels(
                &mut image,
                x,
                y,
                1 * ALPHA_STEPS + alpha_step,
                tile_width,
                tile_height,
                Xyza::from_color(foreground),
                bg_xyz,
            );
        }
    }

    let _ = std::fs::create_dir("example-data/output");
    match image.save("example-data/output/blend.png") {
        Ok(()) => println!("see 'example-data/output/blend.png' for the result"),
        Err(e) => println!("failed to write 'example-data/output/blend.png': {}", e),
    }
}

fn draw_composed_pixels<C>(
    image: &mut image::RgbaImage,
    x: u32,
    y: u32,
    column: u32,
    tile_width: u32,
    tile_height: u32,
    fg: Alpha<C, f32>,
    bg: Alpha<C, f32>,
) where
    Alpha<C, f32>: Blend + IntoColor<LinSrgba> + Copy,
{
    let x = x + tile_width * column;
    let functions = [
        Blend::multiply,
        Blend::screen,
        Blend::overlay,
        Blend::darken,
        Blend::lighten,
        Blend::dodge,
        Blend::burn,
        Blend::hard_light,
        Blend::soft_light,
        Blend::difference,
        Blend::exclusion,
    ];

    for (row, function) in IntoIterator::into_iter(functions).enumerate() {
        let row = row as u32;

        image.put_pixel(x, y + tile_height * row, compose(fg, bg, function));
    }
}

fn compose<C>(
    fg: Alpha<C, f32>,
    bg: Alpha<C, f32>,
    function: fn(Alpha<C, f32>, Alpha<C, f32>) -> Alpha<C, f32>,
) -> image::Rgba<u8>
where
    Alpha<C, f32>: Blend + IntoColor<LinSrgba>,
{
    let composed = function(fg, bg).into_color();
    image::Rgba(Srgba::from_linear(composed).into())
}
