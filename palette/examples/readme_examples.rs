use image::{GenericImage, GenericImageView, RgbImage};

#[cfg(feature = "std")]
use palette::{FromColor, Gradient, IntoColor, LinSrgb, Mix};
use palette::{Pixel, Srgb};

mod color_spaces {
    use crate::display_colors;
    use palette::{FromColor, Hue, IntoColor, Lch, Srgb};

    pub fn run() {
        let lch_color: Lch = Srgb::new(0.8, 0.2, 0.1).into_color();
        let new_color = Srgb::from_color(lch_color.shift_hue(180.0));

        display_colors(
            "examples/readme_color_spaces.png",
            &[
                ::palette::Srgb::new(0.8, 0.2, 0.1).into_format(),
                new_color.into_format(),
            ],
        );
    }
}

mod manipulation {
    use crate::display_colors;
    use palette::{FromColor, IntoColor, Lch, Saturate, Shade, Srgb};

    pub fn run() {
        let color = Srgb::new(0.8, 0.2, 0.1).into_linear();
        let lighter = color.lighten(0.1);
        let desaturated = Lch::from_color(color).desaturate(0.5);

        display_colors(
            "examples/readme_manipulation.png",
            &[
                Srgb::from_linear(color.into()).into_format(),
                Srgb::from_linear(lighter.into()).into_format(),
                Srgb::from_linear(desaturated.into_color()).into_format(),
            ],
        );
    }
}

#[cfg(feature = "std")]
mod gradients {
    use crate::display_gradients;
    use palette::{FromColor, Gradient, Hsv, LinSrgb};

    pub fn run() {
        let grad1 = Gradient::new(vec![
            LinSrgb::new(1.0, 0.1, 0.1),
            LinSrgb::new(0.1, 1.0, 1.0),
        ]);

        let grad2 = Gradient::new(vec![
            Hsv::from_color(LinSrgb::new(1.0, 0.1, 0.1)),
            Hsv::from_color(LinSrgb::new(0.1, 1.0, 1.0)),
        ]);

        display_gradients("examples/readme_gradients.png", grad1, grad2);
    }
}

fn display_colors(filename: &str, colors: &[Srgb<u8>]) {
    let mut image = RgbImage::new(colors.len() as u32 * 64, 64);
    for (i, &color) in colors.iter().enumerate() {
        let mut sub_image = image.sub_image(i as u32 * 64, 0, 64, 64);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                sub_image.put_pixel(x, y, image::Rgb(*color.as_raw()));
            }
        }
    }

    match image.save(filename) {
        Ok(()) => println!("see '{}' for the result", filename),
        Err(e) => println!("failed to write '{}': {}", filename, e),
    }
}

#[cfg(feature = "std")]
fn display_gradients<A: Mix<Scalar = f32> + Clone, B: Mix<Scalar = f32> + Clone>(
    filename: &str,
    grad1: Gradient<A>,
    grad2: Gradient<B>,
) where
    LinSrgb: FromColor<A>,
    LinSrgb: FromColor<B>,
{
    let mut image = RgbImage::new(256, 96);
    {
        let mut sub_image = image.sub_image(0, 0, 256, 32);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                sub_image.put_pixel(
                    x,
                    y,
                    image::Rgb(
                        Srgb::from_linear(grad1.get(x as f32 / 255.0).into_color())
                            .into_format()
                            .into_raw(),
                    ),
                );
            }
        }
    }

    {
        let mut sub_image = image.sub_image(0, 32, 256, 32);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                sub_image.put_pixel(
                    x,
                    y,
                    image::Rgb(
                        Srgb::from_linear(grad2.get(x as f32 / 255.0).into_color())
                            .into_format()
                            .into_raw(),
                    ),
                );
            }
        }
    }

    {
        let mut sub_image = image.sub_image(0, 64, 256, 32);
        let swatch_size = 32;
        let mut v1 = Vec::new();
        for color in grad2.take(8) {
            let pix: [u8; 3] = Srgb::from_linear(LinSrgb::from_color(color))
                .into_format()
                .into_raw();
            v1.push(pix);
        }
        for (s, color) in v1.into_iter().enumerate() {
            for x in (s * swatch_size)..((s + 1) * swatch_size) {
                for y in 0..swatch_size {
                    let pixel = sub_image.get_pixel_mut(x as u32, y as u32);
                    *pixel = image::Rgb(color);
                }
            }
        }
    }

    match image.save(filename) {
        Ok(()) => println!("see '{}' for the result", filename),
        Err(e) => println!("failed to write '{}': {}", filename, e),
    }
}

fn main() {
    color_spaces::run();
    manipulation::run();
    #[cfg(feature = "std")]
    gradients::run();
}
