use palette::{FromColor, Hsl, IntoColor, Lch, Saturate, Srgb};

use image::{GenericImage, GenericImageView};

fn main() {
    let mut image = image::open("example-data/input/cat.png")
        .expect("could not open 'example-data/input/cat.png'")
        .to_rgb8();

    let width = image.width();
    let height = image.height();

    //Increase the saturation by 30% towards full saturation as HSL in the left half, and as LCh
    //in the right half. Notice the strong yellow tone in the HSL part.
    {
        let mut sub_image = image.sub_image(0, 0, width / 2, height);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let color: Hsl = Srgb::from(sub_image.get_pixel(x, y).0)
                    .into_format()
                    .into_color();

                let saturated = color.saturate(0.3);
                sub_image.put_pixel(
                    x,
                    y,
                    image::Rgb(Srgb::from_color(saturated).into_format().into()),
                );
            }
        }
    }

    {
        let mut sub_image = image.sub_image(width / 2, 0, width / 2, height);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let color: Lch = Srgb::from(sub_image.get_pixel(x, y).0)
                    .into_format()
                    .into_color();

                let saturated = color.saturate(0.3);
                sub_image.put_pixel(
                    x,
                    y,
                    image::Rgb(Srgb::from_color(saturated).into_format().into()),
                );
            }
        }
    }

    let _ = std::fs::create_dir("example-data/output");
    match image.save("example-data/output/saturate.png") {
        Ok(()) => println!("see 'example-data/output/saturate.png' for the result"),
        Err(e) => println!("failed to write 'example-data/output/saturate.png': {e}"),
    }
}
