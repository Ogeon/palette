use syn::{Generics, Ident, Type, WhereClause};
use syn::punctuated::Punctuated;
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

pub fn path_type(path: &[&str], internal: bool) -> Type {
    let path = path.into_iter()
        .map(|&ident| Ident::new(ident, Span::call_site()));

    if internal {
        parse_quote!{::#(#path)::*}
    } else {
        parse_quote!{self::_palette::#(#path)::*}
    }
}

pub fn color_path(color: &str, internal: bool) -> Tokens {
    match color {
        "Luma" => path(&["luma", "Luma"], internal),
        "Rgb" => path(&["rgb", "Rgb"], internal),
        _ => path(&[color], internal),
    }
}

pub fn add_missing_where_clause(generics: &mut Generics) {
    if generics.where_clause.is_none() {
        generics.where_clause = Some(WhereClause {
            where_token: Token![where](Span::call_site()),
            predicates: Punctuated::new(),
        })
    }
}
