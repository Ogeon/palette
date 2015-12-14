extern crate palette;
extern crate image;

use palette::{Rgb, Hsl, Lch, Saturate};

use image::GenericImage;

fn main() {
    let mut image = image::open("res/cat.png").expect("could not open 'res/cat.png'").to_rgb();

    let width = image.width();
    let height = image.height();

    //Increase the saturation by 80% (!) as HSL in the left half, and as LCh
    //in the right half. Notice the strong yellow tone in the HSL part.
    for (_, _, pixel) in image.sub_image(0, 0, width / 2, height).pixels_mut() {
        let color = Rgb::srgb8(pixel.data[0], pixel.data[1], pixel.data[2]);

        let saturated = Hsl::from(color).saturate(0.8);
        pixel.data = Rgb::from(saturated).to_srgb();
    }

    for (_, _, pixel) in image.sub_image(width / 2, 0, width / 2, height).pixels_mut() {
        let color = Rgb::srgb8(pixel.data[0], pixel.data[1], pixel.data[2]);

        let saturated = Lch::from(color).saturate(0.8);
        pixel.data = Rgb::from(saturated).to_srgb();
    }

    match image.save("examples/saturate.png") {
        Ok(()) => println!("see 'examples/saturate.png' for the result"),
        Err(e) => println!("failed to write 'examples/saturate.png': {}", e),
    }
}