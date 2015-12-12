extern crate palette;
extern crate image;

use palette::{Rgb, Lab, Hsv, Shade};

use image::{RgbImage, GenericImage};

fn main() {
    //The same color in linear RGB, CIE L*a*b*, and HSV
    let rgb = Rgb::rgb(0.5, 0.0, 0.0);
    let lab = Lab::from(rgb.clone());
    let hsv = Hsv::from(rgb.clone());

    let mut image = RgbImage::new(220, 193);

    for i in 0..11 {
        let rgb1= rgb.darken(0.05 * i as f32).to_srgb8_array();
        let rgb2 = rgb.lighten(0.05 * i as f32).to_srgb8_array();

        for (_, _, pixel) in image.sub_image(i as u32 * 20, 0, 20, 31).pixels_mut() {
            pixel.data = rgb1;
        }

        for (_, _, pixel) in image.sub_image(i as u32 * 20, 32, 20, 31).pixels_mut() {
            pixel.data = rgb2;
        }

        let lab1= Rgb::from(lab.darken(0.05 * i as f32)).to_srgb8_array();
        let lab2 = Rgb::from(lab.lighten(0.05 * i as f32)).to_srgb8_array();

        for (_, _, pixel) in image.sub_image(i as u32 * 20, 65, 20, 31).pixels_mut() {
            pixel.data = lab1;
        }

        for (_, _, pixel) in image.sub_image(i as u32 * 20, 97, 20, 31).pixels_mut() {
            pixel.data = lab2;
        }

        let hsv1 = Rgb::from(hsv.darken(0.05 * i as f32)).to_srgb8_array();
        let hsv2 = Rgb::from(hsv.lighten(0.05 * i as f32)).to_srgb8_array();

        for (_, _, pixel) in image.sub_image(i as u32 * 20, 130, 20, 31).pixels_mut() {
            pixel.data = hsv1;
        }

        for (_, _, pixel) in image.sub_image(i as u32 * 20, 162, 20, 31).pixels_mut() {
            pixel.data = hsv2;
        }
    }

    match image.save("examples/shade.png") {
        Ok(()) => println!("see 'examples/shade.png' for the result"),
        Err(e) => println!("failed to write 'examples/shade.png': {}", e),
    }
}
