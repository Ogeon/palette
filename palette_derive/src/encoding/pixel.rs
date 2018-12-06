use std::collections::{HashMap, HashSet};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type};

use meta::{self, DataMetaParser, IdentOrIndex, MetaParser};
use util;

pub fn derive(tokens: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        attrs,
        generics,
        data,
        ..
    } = parse_macro_input!(tokens);

    let meta: PixelMeta = meta::parse_attributes(attrs);
    let item_meta: PixelItemMeta = meta::parse_data_attributes(data.clone());

    let mut number_of_channels = 0usize;
    let mut field_type: Option<Type> = None;

    let all_fields = match data {
        Data::Struct(struct_item) => match struct_item.fields {
            Fields::Named(fields) => fields.named,
            Fields::Unnamed(fields) => fields.unnamed,
            Fields::Unit => Default::default(),
        },
        Data::Enum(_) => panic!("`Pixel` cannot be derived for enums, because of the discriminant"),
        Data::Union(_) => panic!("`Pixel` cannot be derived for unions"),
    };

    let fields = all_fields
        .into_iter()
        .enumerate()
        .map(|(index, field)| {
            (
                field
                    .ident
                    .map(IdentOrIndex::Ident)
                    .unwrap_or_else(|| IdentOrIndex::Index(index.into())),
                field.ty,
            )
        })
        .filter(|&(ref field, _)| !item_meta.zero_size_fields.contains(field));

    for (field, ty) in fields {
        let ty = item_meta
            .type_substitutes
            .get(&field)
            .cloned()
            .unwrap_or(ty);
        number_of_channels += 1;

        if let Some(field_type) = field_type.clone() {
            if field_type != ty {
                panic!(
                    "expected fields to be of type `{}`, but `{}` is of type `{}`",
                    field_type.into_token_stream(),
                    field.into_token_stream(),
                    ty.into_token_stream()
                );
            }
        } else {
            field_type = Some(ty);
        }
    }

    if !meta.repr_c {
        panic!(
            "a `#[repr(C)]` attribute is required to give `{}` a fixed memory layout",
            ident
        );
    }

    let pixel_trait_path = util::path(&["Pixel"], meta.internal);

    let implementation = if let Some(field_type) = field_type {
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        quote! {
            #[automatically_derived]
            unsafe impl #impl_generics #pixel_trait_path<#field_type> for #ident #type_generics #where_clause {
                const CHANNELS: usize = #number_of_channels;
            }
        }
    } else {
        panic!("`Pixel` can only be derived for structs with one or more fields");
    };

    let result = util::bundle_impl("Pixel", ident, meta.internal, implementation);
    result.into()
}

#[derive(Default)]
struct PixelMeta {
    internal: bool,
    repr_c: bool,
}

impl MetaParser for PixelMeta {
    fn internal(&mut self) {
        self.internal = true;
    }

    fn parse_attribute(&mut self, attribute_name: Ident, attribute_tts: TokenStream2) {
        match &*attribute_name.to_string() {
            "repr" => {
                let items = meta::parse_tuple_attribute(&attribute_name, attribute_tts);
                let contains_c = items.into_iter().find(|item: &Ident| item == "C").is_some();

                if contains_c {
                    self.repr_c = true;
                }
            }
            _ => {}
        }
    }
}

#[derive(Default)]
struct PixelItemMeta {
    zero_size_fields: HashSet<IdentOrIndex>,
    type_substitutes: HashMap<IdentOrIndex, Type>,
}

impl DataMetaParser for PixelItemMeta {
    fn parse_struct_field_attribute(
        &mut self,
        field_name: IdentOrIndex,
        _ty: Type,
        attribute_name: Ident,
        attribute_tts: TokenStream2,
    ) {
        match &*attribute_name.to_string() {
            "palette_unsafe_same_layout_as" => {
                let substitute = meta::parse_equal_attribute(&attribute_name, attribute_tts);
                self.type_substitutes.insert(field_name, substitute);
            }
            "palette_unsafe_zero_sized" => {
                meta::assert_empty_attribute(&attribute_name, attribute_tts);
                self.zero_size_fields.insert(field_name);
            }
            _ => {}
        }
    }
}
