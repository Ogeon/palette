use syn::Ident;
use quote::Tokens;
use proc_macro2::Span;

pub fn bundle_impl(trait_name: &str, type_name: Ident, internal: bool, block: Tokens) -> Tokens {
    let const_name = Ident::new(
        &format!("_palette_derive_{}_for_{}", trait_name, type_name),
        Span::call_site(),
    );

    if internal {
        quote!{
            #[allow(non_snake_case, unused_attributes, unused_qualifications)]
            mod #const_name {
                extern crate num_traits as _num_traits;
                use super::#type_name;
                #block
            }
        }
    } else {
        quote!{
            #[allow(non_snake_case, unused_attributes, unused_qualifications)]
            mod #const_name {
                extern crate palette as _palette;
                extern crate num_traits as _num_traits;
                use super::#type_name;
                #block
            }
        }
    }
}

pub fn path(path: &[&str], internal: bool) -> Tokens {
    let path = path.into_iter()
        .map(|&ident| Ident::new(ident, Span::call_site()));

    if internal {
        quote!{::#(#path)::*}
    } else {
        quote!{self::_palette::#(#path)::*}
    }
}

pub fn color_path(color: &str, internal: bool) -> Tokens {
    match color {
        "Luma" => path(&["luma", "Luma"], internal),
        "Rgb" => path(&["rgb", "Rgb"], internal),
        _ => path(&[color], internal),
    }
}
