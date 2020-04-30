use palette::{FromColor, Hsl, Hue, IntoColor, Lch, Pixel, Srgb};

fn main() {
    let mut image = image::open("example-data/input/fruits.png")
        .expect("could not open 'example-data/input/fruits.png'")
        .to_rgb();

    //Shift hue by 180 degrees as HSL in bottom left part, and as LCh in top
    //right part. Notice how LCh manages to preserve the apparent lightness of
    //of the colors, compared to the original.
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let color = Srgb::from_raw(&pixel.0).into_format();

        pixel.0 = if x < y {
            let saturated = Hsl::from_color(color).shift_hue(180.0);
            Srgb::from_linear(saturated.into_color())
                .into_format()
                .into_raw()
        } else {
            let saturated = Lch::from_color(color).shift_hue(180.0);
            Srgb::from_linear(saturated.into_color())
                .into_format()
                .into_raw()
        };
    }

    let _ = std::fs::create_dir("example-data/output");
    match image.save("example-data/output/hue.png") {
        Ok(()) => println!("see 'example-data/output/hue.png' for the result"),
        Err(e) => println!("failed to write 'example-data/output/hue.png': {}", e),
    }
}
