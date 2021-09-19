#[cfg(not(feature = "std"))]
fn main() {
    println!("You can't use gradients without the standard library");
}

#[cfg(feature = "std")]
fn main() {
    use palette::{FromColor, Gradient, IntoColor, Lch, LinSrgb, Srgb};

    use image::{GenericImage, GenericImageView, RgbImage};

    //A gradient of evenly spaced colors
    let grad1 = Gradient::new(vec![
        LinSrgb::new(1.0, 0.1, 0.1),
        LinSrgb::new(0.1, 0.1, 1.0),
        LinSrgb::new(0.1, 1.0, 0.1),
    ]);

    //The same colors as in grad1, but with the blue point shifted down
    let grad2 = Gradient::with_domain(vec![
        (0.0, LinSrgb::new(1.0, 0.1, 0.1)),
        (0.25, LinSrgb::new(0.1, 0.1, 1.0)),
        (1.0, LinSrgb::new(0.1, 1.0, 0.1)),
    ]);

    //The same colors and offsets as in grad1, but in a color space where the hue
    // is a component
    let grad3 = Gradient::new(vec![
        Lch::from_color(LinSrgb::new(1.0, 0.1, 0.1)),
        Lch::from_color(LinSrgb::new(0.1, 0.1, 1.0)),
        Lch::from_color(LinSrgb::new(0.1, 1.0, 0.1)),
    ]);

    //The same colors and color space as in grad3, but with the blue point
    // shifted down
    let grad4 = Gradient::with_domain(vec![
        (0.0, Lch::from_color(LinSrgb::new(1.0, 0.1, 0.1))),
        (0.25, Lch::from_color(LinSrgb::new(0.1, 0.1, 1.0))),
        (1.0, Lch::from_color(LinSrgb::new(0.1, 1.0, 0.1))),
    ]);

    let mut image = RgbImage::new(256, 128);

    for (i, ((c1, c2), (c3, c4))) in grad1
        .take(256)
        .zip(grad2.take(256))
        .zip(grad3.take(256).zip(grad4.take(256)))
        .enumerate()
    {
        let c1 = Srgb::from_linear(c1).into_format().into();
        let c2 = Srgb::from_linear(c2).into_format().into();
        let c3 = Srgb::from_linear(c3.into_color()).into_format().into();
        let c4 = Srgb::from_linear(c4.into_color()).into_format().into();

        {
            let mut sub_image = image.sub_image(i as u32, 0, 1, 31);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgb(c1));
                }
            }
        }

        {
            let mut sub_image = image.sub_image(i as u32, 32, 1, 31);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgb(c2));
                }
            }
        }

        {
            let mut sub_image = image.sub_image(i as u32, 65, 1, 31);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgb(c3));
                }
            }
        }

        {
            let mut sub_image = image.sub_image(i as u32, 97, 1, 31);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgb(c4));
                }
            }
        }
    }

    let _ = std::fs::create_dir("example-data/output");
    match image.save("example-data/output/gradient.png") {
        Ok(()) => println!("see 'example-data/output/gradient.png' for the result"),
        Err(e) => println!("failed to write 'example-data/output/gradient.png': {}", e),
    }
}
