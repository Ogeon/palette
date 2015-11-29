extern crate palette;
extern crate image;

use palette::{Range, Rgb};

use image::{RgbImage, GenericImage};

fn main() {
    //A range of evenly spaced colors
    let range1 = Range::new(vec![
        Rgb::rgb(1.0, 0.1, 0.1),
        Rgb::rgb(0.1, 0.1, 1.0),
        Rgb::rgb(0.1, 1.0, 0.1)
    ]);

    //The same colors as in range1, but with the blue point shifted down
    let range2 = Range::with_domain(vec![
        (0.0, Rgb::rgb(1.0, 0.1, 0.1)),
        (0.25, Rgb::rgb(0.1, 0.1, 1.0)),
        (1.0, Rgb::rgb(0.1, 1.0, 0.1))
    ]);

    let mut image = RgbImage::new(256, 64);

    for (i, (c1, c2)) in range1.take(256).zip(range2.take(256)).enumerate() {
        let (r1, g1, b1, _) = c1.to_srgba8();
        let (r2, g2, b2, _) = c2.to_srgba8();

        for (_, _, pixel) in image.sub_image(i as u32, 0, 1, 31).pixels_mut() {
            pixel.data = [r1, g1, b1];
        }

        for (_, _, pixel) in image.sub_image(i as u32, 33, 1, 31).pixels_mut() {
            pixel.data = [r2, g2, b2];
        }
    }

    match image.save("examples/range.png") {
        Ok(()) => println!("see 'examples/range.png' for the result"),
        Err(e) => println!("failed to write 'examples/range.png': {}", e),
    }
}
