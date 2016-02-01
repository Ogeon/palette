use std::fs::File;
use std::path::Path;
use std::io::{Write, BufRead, BufReader};

pub fn build() {
    let out_dir = ::std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("named.rs");

    let reader = BufReader::new(File::open("build/svg_colors.txt").expect("could not open svg_colors.txt"));
    let mut writer = File::create(dest_path).expect("couldn't create named.rs");

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split('\t');
        let name = parts.next().expect("couldn't get the color name");
        let mut rgb = parts.next().expect(&format!("couldn't get color for {}", name)).split(", ");
        let red: u8 = rgb.next().and_then(|r| r.trim().parse().ok()).expect(&format!("couldn't get red for {}", name));
        let green: u8 = rgb.next().and_then(|r| r.trim().parse().ok()).expect(&format!("couldn't get green for {}", name));
        let blue: u8 = rgb.next().and_then(|r| r.trim().parse().ok()).expect(&format!("couldn't get blue for {}", name));

        writeln!(writer, "\n///<div style=\"display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: {0};\"></div>", name).unwrap();
        writeln!(writer, "pub const {}: (u8, u8, u8) = ({}, {}, {});", name.to_uppercase(), red, green, blue).unwrap();
    }
}