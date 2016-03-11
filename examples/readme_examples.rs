extern crate image;
extern crate palette;
extern crate num_traits;

use image::{RgbImage, GenericImage};

use palette::{Gradient, Mix, Srgb, LinSrgb};

mod color_spaces {
    use palette::{Srgb, LinSrgb, Lch, Hue};
    use display_colors;

    pub fn run() {
        let lch_color: Lch = Srgb::new(0.8, 0.2, 0.1).into_linear().into();
        let new_color = LinSrgb::from(lch_color.shift_hue(180.0.into()));

        display_colors("examples/readme_color_spaces.png", &[
            ::palette::Srgb::new(0.8, 0.2, 0.1).into_pixel(),
            ::palette::Srgb::linear_to_pixel(new_color),
        ]);
    }
}

mod manipulation {
    use palette::{Color, Srgb, Shade, Saturate};
    use display_colors;

    pub fn run() {
        let color: Color = Srgb::new(0.8, 0.2, 0.1).into();
        let lighter = color.lighten(0.1);
        let desaturated = color.desaturate(0.5);

        display_colors("examples/readme_manipulation.png", &[
            ::palette::Srgb::linear_to_pixel(color),
            ::palette::Srgb::linear_to_pixel(lighter),
            ::palette::Srgb::linear_to_pixel(desaturated),
        ]);
    }
}

mod gradients {
    use palette::{LinSrgb, Hsv, Gradient};
    use display_gradients;

    pub fn run() {
        let grad1 = Gradient::new(vec![
            LinSrgb::new(1.0, 0.1, 0.1),
            LinSrgb::new(0.1, 1.0, 1.0)
        ]);

        let grad2 = Gradient::new(vec![
            Hsv::from(LinSrgb::new(1.0, 0.1, 0.1)),
            Hsv::from(LinSrgb::new(0.1, 1.0, 1.0))
        ]);

        display_gradients("examples/readme_gradients.png", grad1, grad2);
    }
}

fn display_colors(filename: &str, colors: &[[u8; 3]]) {
    let mut image = RgbImage::new(colors.len() as u32 * 64, 64);
    for (i, &color) in colors.iter().enumerate() {
        for (_, _, pixel) in image.sub_image(i as u32 * 64, 0, 64, 64).pixels_mut() {
            pixel.data = color;
        }
    }

    match image.save(filename) {
        Ok(()) => println!("see '{}' for the result", filename),
        Err(e) => println!("failed to write '{}': {}", filename, e),
    }
}

fn display_gradients<A: Mix<Scalar=f32> + Clone, B: Mix<Scalar=f32> + Clone>(filename: &str, grad1: Gradient<A>, grad2: Gradient<B>) where
    LinSrgb: From<A>,
    LinSrgb: From<B>,
{
    let mut image = RgbImage::new(256, 64);

    for (x, _, pixel) in image.sub_image(0, 0, 256, 32).pixels_mut() {
        pixel.data = Srgb::linear_to_pixel(grad1.get(x as f32 / 255.0));
    }

    for (x, _, pixel) in image.sub_image(0, 32, 256, 32).pixels_mut() {
        pixel.data = Srgb::linear_to_pixel(grad2.get(x as f32/ 255.0));
    }

    match image.save(filename) {
        Ok(()) => println!("see '{}' for the result", filename),
        Err(e) => println!("failed to write '{}': {}", filename, e),
    }
}

fn main() {
    color_spaces::run();
    manipulation::run();
    gradients::run();
}
