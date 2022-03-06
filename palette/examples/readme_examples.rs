use image::{GenericImage, GenericImageView, RgbImage, SubImage};

#[allow(unused_variables)]
fn converting() {
    use palette::{FromColor, Hsl, IntoColor, Lch, Srgb};

    let my_rgb = Srgb::new(0.8, 0.3, 0.3);

    let mut my_lch = Lch::from_color(my_rgb);
    my_lch.hue += 180.0;

    let mut my_hsl: Hsl = my_lch.into_color();
    my_hsl.lightness *= 0.6;

    let my_new_rgb = Srgb::from_color(my_hsl);

    // Write example image
    display_colors(
        "example-data/output/readme_converting.png",
        &[DisplayType::Discrete(&[
            my_rgb.into_format(),
            Srgb::from_linear(my_lch.into_color()).into_format(),
            Srgb::from_color(my_hsl).into_format(), // my_new_rgb is the same as my_hsl
        ])],
    );
}

fn pixels_and_buffers() {
    use palette::{cast, Srgb};

    // The input to this function could be data from an image file or
    // maybe a texture in a game.
    fn swap_red_and_blue(my_rgb_image: &mut [u8]) {
        // Convert `my_rgb_image` into `&mut [Srgb<u8>]` without copying.
        let my_rgb_image: &mut [Srgb<u8>] = cast::from_component_slice_mut(my_rgb_image);

        for color in my_rgb_image {
            std::mem::swap(&mut color.red, &mut color.blue);
        }
    }

    // Write example image
    let mut image = image::open("example-data/input/fruits-128.png")
        .expect("could not open 'example-data/input/fruits-128.png'")
        .to_rgb8();
    swap_red_and_blue(&mut image);
    let filename = "example-data/output/readme_pixels_and_buffers.png";
    match image.save(filename) {
        Ok(()) => println!("see '{}' for the result", filename),
        Err(e) => println!("could not write '{}': {}", filename, e),
    }
}

fn color_operations_1() {
    use palette::{Hsl, Hsv, Lighten, Mix, ShiftHue};

    fn transform_color<C>(color: C, amount: f32) -> C
    where
        C: ShiftHue<Scalar = f32> + Lighten<Scalar = f32> + Mix<Scalar = f32> + Copy,
    {
        let new_color = color.shift_hue(170.0).lighten(1.0);

        // Interpolate between the old and new color.
        color.mix(new_color, amount)
    }

    // Write example image
    let hsl_color = Hsl::new_srgb(0.00, 0.70, 0.20);
    let hsl_color_at = |amount| {
        use palette::FromColor;

        let color = transform_color(hsl_color, amount);
        palette::Srgb::from_color(color).into_format()
    };

    let hsv_color = Hsv::new_srgb(0.00, 0.82, 0.34);
    let hsv_color_at = |amount| {
        use palette::FromColor;

        let color = transform_color(hsv_color, amount);
        palette::Srgb::from_color(color).into_format()
    };

    display_colors(
        "example-data/output/readme_color_operations_1.png",
        &[
            DisplayType::Continuous(&hsl_color_at),
            DisplayType::Continuous(&hsv_color_at),
        ],
    );
}

fn color_operations_2() {
    use palette::{blend::Compose, cast, Srgb, WithAlpha};

    // The input to this function could be data from image files.
    fn alpha_blend_images(image1: &mut [u8], image2: &[u8]) {
        // Convert the images into `&mut [Srgb<u8>]` and `&[Srgb<u8>]` without copying.
        let image1: &mut [Srgb<u8>] = cast::from_component_slice_mut(image1);
        let image2: &[Srgb<u8>] = cast::from_component_slice(image2);

        for (color1, color2) in image1.iter_mut().zip(image2) {
            // Convert the colors to linear floating point format and give them transparency values.
            let color1_alpha = color1.into_format().into_linear().opaque();
            let color2_alpha = color2.into_format().into_linear().with_alpha(0.5);

            // Alpha blend `color2_alpha` over `color1_alpha`.
            let blended = color2_alpha.over(color1_alpha);

            // Convert the color part back to `Srgb<u8>` and overwrite the value in image1.
            *color1 = blended.color.into_encoding().into_format();
        }
    }

    // Write example image
    let mut image1 = image::open("example-data/input/fruits-128.png")
        .expect("could not open 'example-data/input/fruits-128.png'")
        .to_rgb8();
    let image2 = image::open("example-data/input/cat-128.png")
        .expect("could not open 'example-data/input/fruits-128.png'")
        .to_rgb8();
    alpha_blend_images(&mut image1, &image2);
    let filename = "example-data/output/readme_color_operations_2.png";
    match image1.save(filename) {
        Ok(()) => println!("see '{}' for the result", filename),
        Err(e) => println!("could not write '{}': {}", filename, e),
    }
}

#[cfg(feature = "std")]
fn gradients_1() {
    use palette::{Gradient, LinSrgb};

    let gradient = Gradient::new(vec![
        LinSrgb::new(0.00, 0.05, 0.20),
        LinSrgb::new(0.70, 0.10, 0.20),
        LinSrgb::new(0.95, 0.90, 0.30),
    ]);

    let taken_colors: Vec<_> = gradient.take(10).collect();

    // Write example image
    let taken_srgb8_colors: Vec<_> = taken_colors
        .into_iter()
        .map(|color| color.into_encoding().into_format())
        .collect();
    display_colors(
        "example-data/output/readme_gradients_1.png",
        &[
            DisplayType::Continuous(&|i| gradient.get(i).into_encoding().into_format()),
            DisplayType::Discrete(&taken_srgb8_colors),
        ],
    );
}

#[cfg(feature = "std")]
fn gradients_2() {
    use palette::{Gradient, LinSrgb};

    let gradient = Gradient::from([
        (0.0, LinSrgb::new(0.00, 0.05, 0.20)), // A pair of position and color.
        (0.2, LinSrgb::new(0.70, 0.10, 0.20)),
        (1.0, LinSrgb::new(0.95, 0.90, 0.30)),
    ]);

    let taken_colors: Vec<_> = gradient.take(10).collect();

    // Write example image
    let taken_srgb8_colors: Vec<_> = taken_colors
        .into_iter()
        .map(|color| color.into_encoding().into_format())
        .collect();
    display_colors(
        "example-data/output/readme_gradients_2.png",
        &[
            DisplayType::Continuous(&|i| gradient.get(i).into_encoding().into_format()),
            DisplayType::Discrete(&taken_srgb8_colors),
        ],
    );
}

enum DisplayType<'a> {
    Discrete(&'a [palette::Srgb<u8>]),
    Continuous(&'a dyn Fn(f32) -> palette::Srgb<u8>),
}

fn display_colors(filename: &str, displays: &[DisplayType]) {
    const WIDTH: u32 = 500;
    const ROW_HEIGHT: u32 = 50;

    let row_height = if displays.len() > 1 {
        ROW_HEIGHT
    } else {
        ROW_HEIGHT * 2
    };

    let mut image = RgbImage::new(WIDTH, displays.len() as u32 * row_height);

    for (i, display) in displays.into_iter().enumerate() {
        let image = image.sub_image(0, i as u32 * row_height, WIDTH, row_height);
        match *display {
            DisplayType::Discrete(colors) => {
                display_discrete(image, colors);
            }
            DisplayType::Continuous(color_at) => {
                display_continuous(image, color_at);
            }
        }
    }

    let _ = std::fs::create_dir("example-data/output");
    match image.save(filename) {
        Ok(()) => println!("see '{}' for the result", filename),
        Err(e) => println!("could not write '{}': {}", filename, e),
    }
}

fn display_discrete(mut image: SubImage<&mut RgbImage>, colors: &[palette::Srgb<u8>]) {
    let (width, height) = image.dimensions();
    let swatch_size = width as f32 / colors.len() as f32;
    for (i, &color) in colors.iter().enumerate() {
        let swatch_begin = (i as f32 * swatch_size) as u32;
        let swatch_end = ((i + 1) as f32 * swatch_size) as u32;
        let mut sub_image = image.sub_image(swatch_begin, 0, swatch_end - swatch_begin, height);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                sub_image.put_pixel(x, y, image::Rgb(color.into()));
            }
        }
    }
}

fn display_continuous(
    mut image: SubImage<&mut RgbImage>,
    color_at: &dyn Fn(f32) -> palette::Srgb<u8>,
) {
    let (width, height) = image.dimensions();
    for x in 0..width {
        for y in 0..height {
            image.put_pixel(x, y, image::Rgb(color_at(x as f32 / width as f32).into()));
        }
    }
}

fn main() {
    converting();
    pixels_and_buffers();
    color_operations_1();
    color_operations_2();
    #[cfg(feature = "std")]
    gradients_1();
    #[cfg(feature = "std")]
    gradients_2();
}
