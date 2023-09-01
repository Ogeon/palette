use palette::{Darken, IntoColor, Lch, Lighten, LinSrgb, ShiftHue, Srgb};

use image::{GenericImage, GenericImageView, RgbImage, SubImage};

use clap::{Arg, Command};

const SWATCH_SIZE: u32 = 128;

fn main() {
    let matches = Command::new("color_scheme")
        .about("Generates a very simple color scheme from an RGB8 color.")
        .arg_required_else_help(true)
        .arg(
            Arg::new("red")
                .required(true)
                .value_parser(clap::value_parser!(u8))
                .index(1)
                .help("[0-255] The red channel of the primary color."),
        )
        .arg(
            Arg::new("green")
                .required(true)
                .value_parser(clap::value_parser!(u8))
                .index(2)
                .help("[0-255] The green channel of the primary color."),
        )
        .arg(
            Arg::new("blue")
                .required(true)
                .value_parser(clap::value_parser!(u8))
                .index(3)
                .help("[0-255] The blue channel of the primary color."),
        )
        .subcommand(
            Command::new("triad")
                .about("A three point scheme, centered around the complementary.")
                .arg(
                    Arg::new("distance")
                        .help("The distance between the secondary colors.")
                        .long("distance")
                        .short('d')
                        .value_name("degrees")
                        .value_parser(clap::value_parser!(f32)),
                ),
        )
        .subcommand(
            Command::new("analogous")
                .about("Like triad, but centered around the primary.")
                .arg(
                    Arg::new("distance")
                        .help("The distance between the secondary colors.")
                        .long("distance")
                        .short('d')
                        .value_name("degrees")
                        .value_parser(clap::value_parser!(f32)),
                ),
        )
        .subcommand(
            Command::new("rectangle").about("A four point scheme.").arg(
                Arg::new("distance")
                    .help("The distance to the closest secondary colors.")
                    .long("distance")
                    .short('d')
                    .value_name("degrees")
                    .value_parser(clap::value_parser!(f32)),
            ),
        )
        .subcommand(Command::new("complementary").about("A simple two point color scheme."))
        .get_matches();

    //Get the components of the primary color
    let red: u8 = matches
        .get_one::<u8>("red")
        .copied()
        .expect("the red channel must be a number in the range [0-255]");
    let green: u8 = matches
        .get_one::<u8>("green")
        .copied()
        .expect("the green channel must be a number in the range [0-255]");
    let blue: u8 = matches
        .get_one::<u8>("blue")
        .copied()
        .expect("the blue channel must be a number in the range [0-255]");

    let primary: Lch = Srgb::new(red, green, blue).into_linear().into_color();

    //Generate the secondary colors, depending on the input arguments
    let secondary = match matches.subcommand() {
        Some(("triad", matches)) | Some(("", matches)) => {
            //Two secondary colors that are close to the complementary, or evenly spaced
            let distance = matches.get_one::<f32>("distance").copied().unwrap_or(120.0);

            let shift = 180.0 - (distance / 2.0);

            vec![primary.shift_hue(shift), primary.shift_hue(-shift)]
        }
        Some(("analogous", matches)) => {
            //Two secondary colors that are close to the primary
            let distance = matches.get_one::<f32>("distance").copied().unwrap_or(60.0);

            let shift = distance / 2.0;

            vec![primary.shift_hue(shift), primary.shift_hue(-shift)]
        }
        Some(("rectangle", matches)) => {
            //Three secondary colors that forms a rectangle or a square, together with the
            // primary
            let distance = matches.get_one::<f32>("distance").copied().unwrap_or(90.0);

            let shift1 = distance;
            let shift2 = 180.0 + distance;

            vec![
                primary.shift_hue(shift1),
                primary.shift_hue(180.0),
                primary.shift_hue(shift2),
            ]
        }
        Some(("complementary", _)) => vec![primary.shift_hue(180.0)], // Simply the complementary color
        Some((name, _)) => panic!("unknown subcommand: {}", name),
        None => panic!("expected a subcommand"),
    };

    //Create an image for the swatches
    let mut image = RgbImage::new((secondary.len() as u32 + 1) * SWATCH_SIZE, SWATCH_SIZE);

    //Draw the primary swatches
    blit_shades(
        primary.into_color(),
        image.sub_image(0, 0, SWATCH_SIZE, SWATCH_SIZE),
    );

    //Draw the secondary swatches
    for (n, color) in secondary.into_iter().enumerate() {
        blit_shades(
            color.into_color(),
            image.sub_image((n as u32 + 1) * SWATCH_SIZE, 0, SWATCH_SIZE, SWATCH_SIZE),
        );
    }

    let _ = std::fs::create_dir("example-data/output");
    match image.save("example-data/output/color_scheme.png") {
        Ok(()) => println!("see 'example-data/output/color_scheme.png' for the result"),
        Err(e) => println!(
            "failed to write 'example-data/output/color_scheme.png': {}",
            e
        ),
    }
}

fn blit_shades(color: LinSrgb<f32>, mut canvas: SubImage<&mut RgbImage>) {
    let width = canvas.width();
    let height = canvas.height();

    let primary = Srgb::from_linear(color).into();

    //Generate one lighter and two darker versions of the color
    let light = Srgb::from_linear(color.lighten(0.1)).into();
    let dark1 = Srgb::from_linear(color.darken(0.1)).into();
    let dark2 = Srgb::from_linear(color.darken(0.2)).into();

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

            canvas.put_pixel(x, y, image::Rgb(data));
        }
    }
}
