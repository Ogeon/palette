extern crate image;
extern crate palette;

use palette::{Hsl, Hue, Lch, Pixel, Srgb};

fn main() {
    let mut image = image::open("res/fruits.png")
        .expect("could not open 'res/fruits.png'")
        .to_rgb();

    //Shift hue by 180 degrees as HSL in bottom left part, and as LCh in top
    //right part. Notice how LCh manages to preserve the apparent lightness of
    //of the colors, compared to the original.
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let color = Srgb::from_raw(&pixel.0).into_format();

        pixel.0 = if x < y {
            let saturated = Hsl::from(color).shift_hue(180.0);
            Srgb::from_linear(saturated.into()).into_format().into_raw()
        } else {
            let saturated = Lch::from(color).shift_hue(180.0);
            Srgb::from_linear(saturated.into()).into_format().into_raw()
        };
    }

    match image.save("examples/hue.png") {
        Ok(()) => println!("see 'examples/hue.png' for the result"),
        Err(e) => println!("failed to write 'examples/hue.png': {}", e),
    }
}
