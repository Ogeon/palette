use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::codegen_file::CodegenFile;

pub fn generate() -> Result<()> {
    let mut file = CodegenFile::create("palette/src/named/codegen.rs")?;

    let colors = parse_colors()?;

    file.append(build_colors(&colors))?;
    file.append(build_from_str(&colors))?;

    Ok(())
}

struct ColorEntry {
    name: String,
    constant: Ident,
    red: u8,
    green: u8,
    blue: u8,
}

fn parse_colors() -> Result<Vec<ColorEntry>> {
    let reader = BufReader::new(
        File::open("codegen/res/svg_colors.txt").expect("could not open svg_colors.txt"),
    );

    // Expected format: "name\t123, 123, 123"
    reader
        .lines()
        .map(|line| {
            let line = line?;
            let mut parts = line.split('\t');

            let name = parts
                .next()
                .context("couldn't get the color name")?
                .to_owned();
            let mut rgb = parts
                .next()
                .with_context(|| format!("couldn't get RGB for {}", name))?
                .split(", ");

            let red: u8 = rgb
                .next()
                .with_context(|| format!("missing red for {name}"))?
                .trim()
                .parse()
                .with_context(|| format!("couldn't parse red for {}", name))?;

            let green: u8 = rgb
                .next()
                .with_context(|| format!("missing green for {name}"))?
                .trim()
                .parse()
                .with_context(|| format!("couldn't parse green for {}", name))?;

            let blue: u8 = rgb
                .next()
                .with_context(|| format!("missing blue for {name}"))?
                .trim()
                .parse()
                .with_context(|| format!("couldn't parse blue for {}", name))?;

            let constant = Ident::new(&name.to_ascii_uppercase(), Span::call_site());

            Ok(ColorEntry {
                name,
                constant,
                red,
                green,
                blue,
            })
        })
        .collect()
}

fn build_colors(colors: &[ColorEntry]) -> TokenStream {
    let constants = colors.iter().map(|ColorEntry { name, constant, red, green, blue }| {
        let swatch_html = format!(
            r#"<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: {name};"></div>"#
        );

        quote! {
            #[doc = #swatch_html]
            pub const #constant: crate::rgb::Srgb<u8> = crate::rgb::Srgb::new(#red, #green, #blue);
        }
    });

    quote! {
        #(#constants)*
    }
}

fn build_from_str(entries: &[ColorEntry]) -> TokenStream {
    let mut map = phf_codegen::Map::new();

    for entry in entries {
        map.entry(&*entry.name, entry.constant.to_string());
    }

    let phf_entries: TokenStream = map
        .build()
        .to_string()
        .parse()
        .expect("phf should generate a valid token stream");

    quote! {
        pub(crate) static COLORS: ::phf::Map<&'static str, crate::rgb::Srgb<u8>> = #phf_entries;
    }
}
