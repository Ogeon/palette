extern crate palette;
extern crate image;

use palette::{Range, Rgb, Lch};

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

    //The same colors and offsets as in range1, but in a color space where the hue is a component
    let range3 = Range::new(vec![
        Lch::from(Rgb::rgb(1.0, 0.1, 0.1)),
        Lch::from(Rgb::rgb(0.1, 0.1, 1.0)),
        Lch::from(Rgb::rgb(0.1, 1.0, 0.1))
    ]);

    //The same colors and and color space as in range3, but with the blue point shifted down
    let range4 = Range::with_domain(vec![
        (0.0, Lch::from(Rgb::rgb(1.0, 0.1, 0.1))),
        (0.25, Lch::from(Rgb::rgb(0.1, 0.1, 1.0))),
        (1.0, Lch::from(Rgb::rgb(0.1, 1.0, 0.1)))
    ]);

    let mut image = RgbImage::new(256, 128);

    for (i, ((c1, c2), (c3, c4))) in range1.take(256).zip(range2.take(256)).zip(range3.take(256).zip(range4.take(256))).enumerate() {
        let (r1, g1, b1, _) = c1.to_srgba8();
        let (r2, g2, b2, _) = c2.to_srgba8();
        let (r3, g3, b3, _) = Rgb::from(c3).to_srgba8();
        let (r4, g4, b4, _) = Rgb::from(c4).to_srgba8();

        for (_, _, pixel) in image.sub_image(i as u32, 0, 1, 31).pixels_mut() {
            pixel.data = [r1, g1, b1];
        }

        for (_, _, pixel) in image.sub_image(i as u32, 32, 1, 31).pixels_mut() {
            pixel.data = [r2, g2, b2];
        }

        for (_, _, pixel) in image.sub_image(i as u32, 65, 1, 31).pixels_mut() {
            pixel.data = [r3, g3, b3];
        }

        for (_, _, pixel) in image.sub_image(i as u32, 97, 1, 31).pixels_mut() {
            pixel.data = [r4, g4, b4];
        }
    }

    match image.save("examples/range.png") {
        Ok(()) => println!("see 'examples/range.png' for the result"),
        Err(e) => println!("failed to write 'examples/range.png': {}", e),
    }
}
