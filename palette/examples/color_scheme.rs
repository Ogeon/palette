extern crate clap;
extern crate image;
extern crate palette;

use palette::{Hue, Lch, LinSrgb, Pixel, Shade, Srgb};

use image::{GenericImage, GenericImageView, RgbImage, SubImage};

use clap::{App, AppSettings, Arg, SubCommand};

const SWATCH_SIZE: u32 = 128;

fn main() {
    let matches = App::new("color_scheme")
        .about("Generates a very simple color scheme from an RGB8 color.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("red")
                .required(true)
                .empty_values(false)
                .index(1)
                .help("[0-255] The red channel of the primary color."),
        )
        .arg(
            Arg::with_name("green")
                .required(true)
                .empty_values(false)
                .index(2)
                .help("[0-255] The green channel of the primary color."),
        )
        .arg(
            Arg::with_name("blue")
                .required(true)
                .empty_values(false)
                .index(3)
                .help("[0-255] The blue channel of the primary color."),
        )
        .subcommand(
            SubCommand::with_name("triad")
                .about("A three point scheme, centered around the complementary.")
                .arg(
                    Arg::with_name("distance")
                        .help("The distance between the secondary colors.")
                        .long("distance")
                        .short("d")
                        .value_name("degrees")
                        .takes_value(true)
                        .empty_values(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("analogous")
                .about("Like triad, but centered around the primary.")
                .arg(
                    Arg::with_name("distance")
                        .help("The distance between the secondary colors.")
                        .long("distance")
                        .short("d")
                        .value_name("degrees")
                        .takes_value(true)
                        .empty_values(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("rectangle")
                .about("A four point scheme.")
                .arg(
                    Arg::with_name("distance")
                        .help("The distance to the closest secondary colors.")
                        .long("distance")
                        .short("d")
                        .value_name("degrees")
                        .takes_value(true)
                        .empty_values(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("complementary").about("A simple two point color scheme."),
        )
        .get_matches();

    //Get the components of the primary color
    let red: u8 = matches
        .value_of("red")
        .and_then(|r| r.parse().ok())
        .expect("the red channel must be a number in the range [0-255]");
    let green: u8 = matches
        .value_of("green")
        .and_then(|r| r.parse().ok())
        .expect("the green channel must be a number in the range [0-255]");
    let blue: u8 = matches
        .value_of("blue")
        .and_then(|r| r.parse().ok())
        .expect("the blue channel must be a number in the range [0-255]");

    let primary: Lch = Srgb::new(red, green, blue)
        .into_format::<f32>()
        .into_linear()
        .into();

    //Generate the secondary colors, depending on the input arguments
    let secondary = match matches.subcommand() {
        ("triad", matches) | ("", matches) => {
            //Two secondary colors that are close to the complementary, or evenly spaced
            let distance: f32 = matches
                .and_then(|m| m.value_of("distance"))
                .and_then(|d| d.parse().ok())
                .unwrap_or(120.0);

            let shift = 180.0 - (distance / 2.0);

            vec![primary.shift_hue(shift), primary.shift_hue(-shift)]
        }
        ("analogous", matches) => {
            //Two secondary colors that are close to the primary
            let distance: f32 = matches
                .and_then(|m| m.value_of("distance"))
                .and_then(|d| d.parse().ok())
                .unwrap_or(60.0);

            let shift = distance / 2.0;

            vec![primary.shift_hue(shift), primary.shift_hue(-shift)]
        }
        ("rectangle", matches) => {
            //Three secondary colors that forms a rectangle or a square, together with the
            // primary
            let distance: f32 = matches
                .and_then(|m| m.value_of("distance"))
                .and_then(|d| d.parse().ok())
                .unwrap_or(90.0);

            let shift1 = distance;
            let shift2 = 180.0 + distance;

            vec![
                primary.shift_hue(shift1),
                primary.shift_hue(180.0),
                primary.shift_hue(shift2),
            ]
        }
        ("complementary", _) => vec![primary.shift_hue(180.0)], // Simply the complementary color
        (name, _) => panic!("unknown subcommand: {}", name),
    };

    //Create an image for the swatches
    let mut image = RgbImage::new((secondary.len() as u32 + 1) * SWATCH_SIZE, SWATCH_SIZE);

    //Draw the primary swatches
    blit_shades(
        primary.into(),
        image.sub_image(0, 0, SWATCH_SIZE, SWATCH_SIZE),
    );

    //Draw the secondary swatches
    for (n, color) in secondary.into_iter().enumerate() {
        blit_shades(
            color.into(),
            image.sub_image((n as u32 + 1) * SWATCH_SIZE, 0, SWATCH_SIZE, SWATCH_SIZE),
        );
    }

    match image.save("examples/color_scheme.png") {
        Ok(()) => println!("see 'examples/color_scheme.png' for the result"),
        Err(e) => println!("failed to write 'examples/color_scheme.png': {}", e),
    }
}

fn blit_shades(color: LinSrgb<f32>, mut canvas: SubImage<&mut RgbImage>) {
    let width = canvas.width();
    let height = canvas.height();

    let primary = Srgb::from_linear(color).into_format().into_raw();

    //Generate one lighter and two darker versions of the color
    let light = Srgb::from_linear(color.lighten(0.1).into())
        .into_format()
        .into_raw();
    let dark1 = Srgb::from_linear(color.darken(0.1).into())
        .into_format()
        .into_raw();
    let dark2 = Srgb::from_linear(color.darken(0.2).into())
        .into_format()
        .into_raw();

    for x in 0..width {
        for y in 0..height {
            let data = if y < height / 2 {
                primary
            } else if x < width / 3 {
                light
            } else if x < (width / 3) * 2 {
                dark1
            } else {
                dark2
            };

            canvas.put_pixel(x, y, image::Rgb { data });
        }
    }
}
