use palette::{FromColor, Hsv, IntoColor, Lab, LinSrgb, Pixel, Shade, Srgb};

use image::{GenericImage, GenericImageView, RgbImage};

fn main() {
    //The same color in linear RGB, CIE L*a*b*, and HSV
    let rgb = LinSrgb::new(0.5, 0.0, 0.0);
    let lab = Lab::from_color(rgb);
    let hsv = Hsv::from_color(rgb);

    let mut image = RgbImage::new(220, 193);

    for i in 0..11 {
        let rgb1 = Srgb::from_linear(rgb.darken(0.05 * i as f32))
            .into_format()
            .into_raw();
        let rgb2 = Srgb::from_linear(rgb.lighten(0.05 * i as f32))
            .into_format()
            .into_raw();

        {
            let mut sub_image = image.sub_image(i as u32 * 20, 0, 20, 31);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgb(rgb1));
                }
            }
        }

        {
            let mut sub_image = image.sub_image(i as u32 * 20, 32, 20, 31);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgb(rgb2));
                }
            }
        }

        let lab1 = Srgb::from_linear(lab.darken(0.05 * i as f32).into_color())
            .into_format()
            .into_raw();
        let lab2 = Srgb::from_linear(lab.lighten(0.05 * i as f32).into_color())
            .into_format()
            .into_raw();

        {
            let mut sub_image = image.sub_image(i as u32 * 20, 65, 20, 31);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgb(lab1));
                }
            }
        }

        {
            let mut sub_image = image.sub_image(i as u32 * 20, 97, 20, 31);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgb(lab2));
                }
            }
        }

        let hsv1 = Srgb::from_linear(hsv.darken(0.05 * i as f32).into_color())
            .into_format()
            .into_raw();
        let hsv2 = Srgb::from_linear(hsv.lighten(0.05 * i as f32).into_color())
            .into_format()
            .into_raw();

        {
            let mut sub_image = image.sub_image(i as u32 * 20, 130, 20, 31);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgb(hsv1));
                }
            }
        }

        {
            let mut sub_image = image.sub_image(i as u32 * 20, 162, 20, 31);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgb(hsv2));
                }
            }
        }
    }

    match image.save("examples/shade.png") {
        Ok(()) => println!("see 'examples/shade.png' for the result"),
        Err(e) => println!("failed to write 'examples/shade.png': {}", e),
    }
}
