extern crate palette;
extern crate image;

use palette::{Srgb, Hsl, Lch, Hue};

fn main() {
    let mut image = image::open("res/fruits.png").expect("could not open 'res/fruits.png'").to_rgb();

    //Shift hue by 180 degrees as HSL in bottom left part, and as LCh in top
    //right part. Notice how LCh manages to preserve the apparent lightness of
    //of the colors, compared to the original.
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let color = Srgb::from_pixel(&pixel.data).into_linear();

        pixel.data = if x < y {
            let saturated = Hsl::from(color).shift_hue(180.0.into());
            Srgb::linear_to_pixel(saturated)
        } else {
            let saturated = Lch::from(color).shift_hue(180.0.into());
            Srgb::linear_to_pixel(saturated)
        };
    }

    match image.save("examples/hue.png") {
        Ok(()) => println!("see 'examples/hue.png' for the result"),
        Err(e) => println!("failed to write 'examples/hue.png': {}", e),
    }
}