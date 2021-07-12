#[cfg(not(feature = "random"))]
fn main() {
    println!("You can't use the `rand` integration without the \"random\" feature");
}

#[cfg(feature = "random")]
fn main() {
    use palette::{FromColor, Hsl, Hsv, Hwb, Pixel, RgbHue, Srgb};

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
                let random_color = Srgb::<f32>::new(rng.gen(), rng.gen(), rng.gen());
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into_raw()));
            }
        }
    }

    {
        let mut sub_image = image.sub_image(0, 128, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color = rng.gen::<Srgb>();
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
                let random_color =
                    Srgb::from_color(Hsv::new(rng.gen::<RgbHue>(), rng.gen(), rng.gen()));
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into_raw()));
            }
        }
    }

    {
        let mut sub_image = image.sub_image(128, 128, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color = Srgb::from_color(rng.gen::<Hsv>());
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
                let random_color =
                    Srgb::from_color(Hsl::new(rng.gen::<RgbHue>(), rng.gen(), rng.gen()));
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into_raw()));
            }
        }
    }

    {
        let mut sub_image = image.sub_image(256, 128, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color = Srgb::from_color(rng.gen::<Hsl>());
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
                let random_color =
                    Srgb::from_color(Hwb::new(rng.gen::<RgbHue>(), rng.gen(), rng.gen()));
                sub_image.put_pixel(x, y, image::Rgb(random_color.into_format().into_raw()));
            }
        }
    }

    {
        let mut sub_image = image.sub_image(384, 128, 128, 128);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                let random_color = Srgb::from_color(rng.gen::<Hwb>());
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
