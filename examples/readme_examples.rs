extern crate image;
extern crate palette;
extern crate num;

use num::traits::Float;
use image::{RgbImage, GenericImage};

use palette::{Rgb, Gradient, Mix};

mod color_spaces {
    use palette::{Rgb, Lch, Hue};
    use display_colors;

    pub fn run() {
        let lch_color = Lch::from(Rgb::srgb(0.8, 0.2, 0.1));
        let new_color: Rgb<f32> = lch_color.shift_hue(180.0.into()).into();

        display_colors("examples/readme_color_spaces.png",
                       &[Rgb::srgb(0.8, 0.2, 0.1).to_srgb(), new_color.to_srgb()]);
    }
}

mod manipulation {
    use palette::{Color, Shade, Saturate};
    use display_colors;

    pub fn run() {
        let color = Color::srgb(0.8, 0.2, 0.1);
        let lighter = color.lighten(0.1);
        let desaturated = color.desaturate(0.5);

        display_colors("examples/readme_manipulation.png",
                       &[::palette::Rgb::from(color).to_srgb(),
                         ::palette::Rgb::from(lighter).to_srgb(),
                         ::palette::Rgb::from(desaturated).to_srgb()]);
    }
}

mod gradients {
    use palette::{Rgb, Hsv, Gradient};
    use display_gradients;

    pub fn run() {
        let grad1 = Gradient::new(vec![Rgb::linear_rgb(1.0, 0.1, 0.1),
                                       Rgb::linear_rgb(0.1, 1.0, 1.0)]);

        let grad2 = Gradient::new(vec![Hsv::from(Rgb::linear_rgb(1.0, 0.1, 0.1)),
                                       Hsv::from(Rgb::linear_rgb(0.1, 1.0, 1.0))]);

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

fn display_gradients<T: Float, A: Mix<T> + Clone, B: Mix<T> + Clone>(filename: &str,
                                                                     grad1: Gradient<T, A>,
                                                                     grad2: Gradient<T, B>)
    where Rgb<T>: From<A>,
          Rgb<T>: From<B>
{
    let mut image = RgbImage::new(256, 64);

    for (x, _, pixel) in image.sub_image(0, 0, 256, 32).pixels_mut() {
        pixel.data = Rgb::from(grad1.get(T::from(x).unwrap() / T::from(255.0).unwrap())).to_srgb();
    }

    for (x, _, pixel) in image.sub_image(0, 32, 256, 32).pixels_mut() {
        pixel.data = Rgb::from(grad2.get(T::from(x).unwrap() / T::from(255.0).unwrap())).to_srgb();
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
