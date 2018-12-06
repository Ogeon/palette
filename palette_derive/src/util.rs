use proc_macro2::{Span, TokenStream};
use syn::{parse_quote, Ident, Type};

pub fn bundle_impl(
    trait_name: &str,
    type_name: Ident,
    internal: bool,
    block: TokenStream,
) -> TokenStream {
    let const_name = Ident::new(
        &format!("_palette_derive_{}_for_{}", trait_name, type_name),
        Span::call_site(),
    );

    if internal {
        quote!{
            #[allow(non_snake_case, unused_attributes, unused_qualifications, unused_imports)]
            mod #const_name {
                use float::Float as _FloatTrait;
                use super::*;
                #block
            }
        }
    } else {
        quote!{
            #[allow(non_snake_case, unused_attributes, unused_qualifications, unused_imports)]
            mod #const_name {
                extern crate palette as _palette;
                use self::_palette::float::Float as _FloatTrait;
                use super::*;
                #block
            }
        }
    }
}

pub fn path(path: &[&str], internal: bool) -> TokenStream {
    let path = path
        .into_iter()
        .map(|&ident| Ident::new(ident, Span::call_site()));

    if internal {
        quote!{::#(#path)::*}
    } else {
        quote!{self::_palette::#(#path)::*}
    }
}

pub fn path_type(path: &[&str], internal: bool) -> Type {
    let path = path
        .into_iter()
        .map(|&ident| Ident::new(ident, Span::call_site()));

    if internal {
        parse_quote!{::#(#path)::*}
    } else {
        parse_quote!{self::_palette::#(#path)::*}
    }
}

pub fn color_path(color: &str, internal: bool) -> TokenStream {
    match color {
        "Luma" => path(&["luma", "Luma"], internal),
        "Rgb" => path(&["rgb", "Rgb"], internal),
        _ => path(&[color], internal),
    }
}
