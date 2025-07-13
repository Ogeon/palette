fn main() {
    use enterpolation::{
        linear::{ConstEquidistantLinear, Linear},
        Curve, Merge,
    };
    use palette::{FromColor, IntoColor, Lch, LinSrgb, Mix, Srgb};

    use image::{GenericImage, GenericImageView, RgbImage};

    // A gradient of evenly spaced colors
    let grad1 = ConstEquidistantLinear::<f64, _, 3>::equidistant_unchecked([
        LinSrgb::new(1.0, 0.1, 0.1),
        LinSrgb::new(0.1, 0.1, 1.0),
        LinSrgb::new(0.1, 1.0, 0.1),
    ]);

    // The same colors as in grad1, but with the blue point shifted down
    let grad2 = Linear::builder()
        .elements([
            LinSrgb::new(1.0, 0.1, 0.1),
            LinSrgb::new(0.1, 0.1, 1.0),
            LinSrgb::new(0.1, 1.0, 0.1),
        ])
        .knots([0.0, 0.25, 1.0])
        .build()
        .unwrap();

    // Necessary since Lch doesn't implement `Merge` automatically, but also to
    // get the correct circular blending.
    #[derive(Clone, Copy, Debug)]
    struct Adapter<T>(T);

    impl<T: Mix> Merge<T::Scalar> for Adapter<T> {
        fn merge(self, to: Self, factor: T::Scalar) -> Self {
            Adapter(self.0.mix(to.0, factor))
        }
    }

    // The same colors and offsets as in grad1, but in a color space where the hue
    // is a component
    let grad3 = Linear::equidistant_unchecked([
        Adapter(Lch::from_color(LinSrgb::new(1.0, 0.1, 0.1))),
        Adapter(Lch::from_color(LinSrgb::new(0.1, 0.1, 1.0))),
        Adapter(Lch::from_color(LinSrgb::new(0.1, 1.0, 0.1))),
    ]);

    // The same colors and color space as in grad3, but with the blue point
    // shifted down
    let grad4 = Linear::builder()
        .elements([
            Adapter(Lch::from_color(LinSrgb::new(1.0, 0.1, 0.1))),
            Adapter(Lch::from_color(LinSrgb::new(0.1, 0.1, 1.0))),
            Adapter(Lch::from_color(LinSrgb::new(0.1, 1.0, 0.1))),
        ])
        .knots([0.0, 0.25, 1.0])
        .build()
        .unwrap();

    let mut image = RgbImage::new(256, 128);

    for (i, ((c1, c2), (Adapter(c3), Adapter(c4)))) in grad1
        .take(256)
        .zip(grad2.take(256))
        .zip(grad3.take(256).zip(grad4.take(256)))
        .enumerate()
    {
        let c1 = Srgb::from_linear(c1).into();
        let c2 = Srgb::from_linear(c2).into();
        let c3 = Srgb::from_linear(c3.into_color()).into();
        let c4 = Srgb::from_linear(c4.into_color()).into();

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
        Err(e) => println!("failed to write 'example-data/output/gradient.png': {e}"),
    }
}
