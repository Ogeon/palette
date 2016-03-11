extern crate palette;
extern crate image;

use palette::{Srgb, Hsl, Lch, Saturate};

use image::GenericImage;

fn main() {
    let mut image = image::open("res/cat.png").expect("could not open 'res/cat.png'").to_rgb();

    let width = image.width();
    let height = image.height();

    //Increase the saturation by 80% (!) as HSL in the left half, and as LCh
    //in the right half. Notice the strong yellow tone in the HSL part.
    for (_, _, pixel) in image.sub_image(0, 0, width / 2, height).pixels_mut() {
        let color: Hsl = Srgb::pixel_to_linear(&pixel.data);

        let saturated = color.saturate(0.8);
        pixel.data = Srgb::linear_to_pixel(saturated);
    }

    for (_, _, pixel) in image.sub_image(width / 2, 0, width / 2, height).pixels_mut() {
        let color: Lch = Srgb::pixel_to_linear(&pixel.data);

        let saturated = color.saturate(0.8);
        pixel.data = Srgb::linear_to_pixel(saturated);
    }

    match image.save("examples/saturate.png") {
        Ok(()) => println!("see 'examples/saturate.png' for the result"),
        Err(e) => println!("failed to write 'examples/saturate.png': {}", e),
    }
}