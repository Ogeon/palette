use std::fs::File;

pub fn build() {
    use std::path::Path;

    let out_dir = ::std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("named.rs");
    let mut writer = File::create(dest_path).expect("couldn't create named.rs");
    build_colors(&mut writer);
    let dest_path = Path::new(&out_dir).join("named_gradients.rs");
    let mut writer = File::create(dest_path).expect("couldn't create named_gradients.rs");
    build_gradients(&mut writer);
}

#[cfg(feature = "named")]
pub fn build_colors(writer: &mut File) {
    use std::io::{BufRead, BufReader, Write};

    let reader =
        BufReader::new(File::open("build/svg_colors.txt").expect("could not open svg_colors.txt"));
    let mut entries = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split('\t');
        let name = parts.next().expect("couldn't get the color name");
        let mut rgb = parts
            .next()
            .unwrap_or_else(|| panic!("couldn't get color for {}", name))
            .split(", ");
        let red: u8 = rgb
            .next()
            .and_then(|r| r.trim().parse().ok())
            .unwrap_or_else(|| panic!("couldn't get red for {}", name));
        let green: u8 = rgb
            .next()
            .and_then(|r| r.trim().parse().ok())
            .unwrap_or_else(|| panic!("couldn't get green for {}", name));
        let blue: u8 = rgb
            .next()
            .and_then(|r| r.trim().parse().ok())
            .unwrap_or_else(|| panic!("couldn't get blue for {}", name));

        writeln!(writer, "\n///<div style=\"display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: {0};\"></div>", name).unwrap();
        writeln!(writer, "pub const {}: crate::rgb::Srgb<u8> = crate::rgb::Srgb {{ red: {}, green: {}, blue: {}, standard: ::core::marker::PhantomData }};", name.to_uppercase(), red, green, blue).unwrap();

        entries.push((name.to_owned(), name.to_uppercase()));
    }

    gen_from_str(writer, &entries)
}

#[cfg(feature = "named_gradients")]
pub fn build_gradients(writer: &mut File) {
    use std::io::{BufRead, BufReader, Write};

    let reader = BufReader::new(
        File::open("build/svg_gradients_mpl.txt").expect("could not open svg_gradients_mpl.txt"),
    );

    let mut line_iter = reader.lines();
    while let Some(Ok(line)) = line_iter.next() {
        //empty lines are allowed
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        //every line should have the same info: name type number_of_colors [\n red green blue]^number_of_colors
        let name = parts.next().expect("couldn't get the color name");
        let color_type = parts.next().expect("couldn't get the type of the colors");
        //we assume that color_type is a rgb type
        let color_type = format!("crate::rgb::{}", color_type);
        let number_of_colors: usize = parts
            .next()
            .expect("couldn't get the number of colors")
            .parse()
            .unwrap_or_else(|_| panic!("couldn't parse the number of colors for color {}", name));
        writeln!(writer, "/// New matplotlib colormap by Nathaniel J. Smith, Stefan van der Walt, and (in the case of viridis) Eric Firing.").unwrap();
        writeln!(writer, "///").unwrap();
        writeln!(writer, "/// This gradient is perfectly perceptually-uniform, both in regular form and also when converted to black-and-white.").unwrap();
        writeln!(
            writer,
            "/// The colormap is released under the CC0 license public domain dedication."
        )
        .unwrap();
        write!(writer,
            "pub const {0}: crate::gradient::Gradient<{1}, [(f32,{1});{2}]> = crate::gradient::Gradient([",
            name.to_uppercase(), color_type, number_of_colors).unwrap();
        for i in 0..number_of_colors {
            let color = line_iter
                .next()
                .unwrap_or_else(|| panic!("less lines than stated colors in gradient {}", name))
                .unwrap_or_else(|_| panic!("couldn't read the {}th line of color {}", i, name));
            let mut rgb = color.split(',');
            let red: f32 = rgb
                .next()
                .and_then(|r| r.trim().parse().ok())
                .unwrap_or_else(|| panic!("couldn't get the {}th red-value for {}", i, name));
            let green: f32 = rgb
                .next()
                .and_then(|r| r.trim().parse().ok())
                .unwrap_or_else(|| panic!("couldn't get the {}th green-value for {}", i, name));
            let blue: f32 = rgb
                .next()
                .and_then(|r| r.trim().parse().ok())
                .unwrap_or_else(|| panic!("couldn't get the {}th blue-value for {}", i, name));
            write!(writer, "({:.10},{}{{red: {}, green: {}, blue: {}, standard: ::core::marker::PhantomData}}),", (i as f32/number_of_colors as f32), color_type, red, green, blue).unwrap();
        }
        writeln!(writer, "], ::core::marker::PhantomData);").unwrap();
    }
}

#[cfg(feature = "named_from_str")]
fn gen_from_str(writer: &mut File, entries: &[(String, String)]) {
    use std::io::Write;

    writer
        .write_all(
            "static COLORS: ::phf::Map<&'static str, crate::rgb::Srgb<u8>> = phf::phf_map! {\n"
                .as_bytes(),
        )
        .unwrap();

    for (key, value) in entries {
        writeln!(writer, "    \"{}\" => {},", key, value).unwrap();
    }

    writer.write_all("};\n".as_bytes()).unwrap();
}

#[cfg(not(feature = "named"))]
pub fn build_colors(_writer: &mut File) {}

#[allow(unused)]
#[cfg(not(feature = "named_from_str"))]
fn gen_from_str(_writer: &mut File, _entries: &[(String, String)]) {}

#[allow(unused)]
#[cfg(not(feature = "named_gradients"))]
pub fn build_gradients(_writer: &mut File) {}
