#[cfg(not(feature = "random"))]
fn main() {
    println!("You can't use the `rand` integration without the \"random\" feature");
}

#[cfg(feature = "random")]
fn main() {
    use palette::{Hsl, Hsv, Hwb, IntoColor, Pixel, RgbHue, Srgb};

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
                let random_color: Srgb = Srgb::new(rng.gen(), rng.gen(), rng.gen());
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into_raw()));
            }
        }
    }

    {
        let mut sub_image = image.sub_image(0, 128, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color: Srgb = rng.gen();
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into_raw()));
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
                    Hsv::new(rng.gen::<RgbHue>(), rng.gen(), rng.gen()).into_color();
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into_raw()));
            }
        }
    }

    {
        let mut sub_image = image.sub_image(128, 128, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color: Srgb = rng.gen::<Hsv>().into_color();
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into_raw()));
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
                    Hsl::new(rng.gen::<RgbHue>(), rng.gen(), rng.gen()).into_color();
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into_raw()));
            }
        }
    }

    {
        let mut sub_image = image.sub_image(256, 128, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color: Srgb = rng.gen::<Hsl>().into_color();
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into_raw()));
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
                    Hwb::new(rng.gen::<RgbHue>(), rng.gen(), rng.gen()).into_color();
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into_raw()));
            }
        }
    }

    {
        let mut sub_image = image.sub_image(384, 128, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color: Srgb = rng.gen::<Hwb>().into_color();
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into_raw()));
            }
        }
    }

    let _ = std::fs::create_dir("example-data/output");
    match image.save("example-data/output/random.png") {
        Ok(()) => println!("see 'example-data/output/random.png' for the result"),
        Err(e) => println!("failed to write 'example-data/output/random.png': {}", e),
    }
}
