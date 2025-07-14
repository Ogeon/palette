#[cfg(not(feature = "random"))]
fn main() {
    println!("You can't use the `rand` integration without the \"random\" feature");
}

#[cfg(feature = "random")]
fn main() {
    use palette::{Hsl, Hsv, Hwb, IntoColor, RgbHue, Srgb};

    use image::{GenericImage, GenericImageView, RgbImage};
    use rand::Rng;

    let mut image = RgbImage::new(512, 256);
    let mut rng = rand_mt::Mt::default();

    // RGB
    {
        let mut sub_image = image.sub_image(0, 0, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color: Srgb = Srgb::new(rng.random(), rng.random(), rng.random());
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into()));
            }
        }
    }

    {
        let mut sub_image = image.sub_image(0, 128, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color: Srgb = rng.random();
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into()));
            }
        }
    }

    // HSV
    {
        let mut sub_image = image.sub_image(128, 0, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color: Srgb =
                    Hsv::new(rng.random::<RgbHue>(), rng.random(), rng.random()).into_color();
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into()));
            }
        }
    }

    {
        let mut sub_image = image.sub_image(128, 128, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color: Srgb = rng.random::<Hsv>().into_color();
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into()));
            }
        }
    }

    // HSL
    {
        let mut sub_image = image.sub_image(256, 0, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color: Srgb =
                    Hsl::new(rng.random::<RgbHue>(), rng.random(), rng.random()).into_color();
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into()));
            }
        }
    }

    {
        let mut sub_image = image.sub_image(256, 128, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color: Srgb = rng.random::<Hsl>().into_color();
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into()));
            }
        }
    }

    // HWB
    {
        let mut sub_image = image.sub_image(384, 0, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color: Srgb =
                    Hwb::new(rng.random::<RgbHue>(), rng.random(), rng.random()).into_color();
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into()));
            }
        }
    }

    {
        let mut sub_image = image.sub_image(384, 128, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color: Srgb = rng.random::<Hwb>().into_color();
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into()));
            }
        }
    }

    let _ = std::fs::create_dir("example-data/output");
    match image.save("example-data/output/random.png") {
        Ok(()) => println!("see 'example-data/output/random.png' for the result"),
        Err(e) => println!("failed to write 'example-data/output/random.png': {e}"),
    }
}
