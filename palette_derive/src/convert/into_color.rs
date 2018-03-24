use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{self, DeriveInput, Generics, Ident, Type};
use quote::Tokens;

use meta::{self, KeyValuePair, MetaParser};
use util;
use COLOR_TYPES;

use super::shared::{self, ConvertDirection};

pub fn derive(tokens: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        attrs,
        generics: original_generics,
        ..
    } = syn::parse(tokens).expect("could not parse tokens");
    let mut generics = original_generics.clone();
    let mut meta: IntoColorMeta = meta::parse_attributes(attrs);

    let (generic_component, generic_white_point) = shared::find_in_generics(
        meta.component.as_ref(),
        meta.white_point.as_ref(),
        &original_generics,
    );

    let white_point = shared::white_point_type(meta.white_point.clone(), meta.internal);
    let component = shared::component_type(meta.component.clone());

    if generic_component {
        shared::add_component_where_clause(&component, &mut generics, meta.internal);
    }

    if generic_white_point {
        shared::add_white_point_where_clause(&white_point, &mut generics, meta.internal);
    }

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    // Assume conversion into Xyz by default
    if meta.manual_implementations.is_empty() {
        meta.manual_implementations.push(KeyValuePair {
            key: "Xyz".into(),
            value: None,
        });
    }

    let methods = shared::generate_methods(
        &ident,
        ConvertDirection::Into,
        &meta.manual_implementations,
        &component,
        &white_point,
        &shared::rgb_space_type(meta.rgb_space.clone(), &white_point, meta.internal),
        &type_generics.as_turbofish(),
        meta.internal,
    );

    let trait_path = util::path(&["IntoColor"], meta.internal);
    let into_color_impl = quote!{
        #[automatically_derived]
        impl #impl_generics #trait_path<#white_point, #component> for #ident #type_generics #where_clause {
            #(#methods)*
        }
    };

    let into_impls = COLOR_TYPES.into_iter().map(|&color| {
        let skip_regular_into = (meta.internal && color == ident.as_ref())
            || meta.manual_implementations
                .iter()
                .any(|color_impl| color_impl.key == color && color_impl.value.is_none());

        let regular_into = if skip_regular_into {
            None
        } else {
            Some(impl_into(
                &ident,
                color,
                &meta,
                &original_generics,
                generic_component,
                false,
            ))
        };

        let with_alpha = impl_into(
            &ident,
            color,
            &meta,
            &original_generics,
            generic_component,
            true,
        );

        quote! {
            #regular_into
            #with_alpha
        }
    });

    let into_color_ty =
        impl_into_color_type(&ident, &meta, &original_generics, generic_component, false);
    let from_color_ty_other_alpha =
        impl_into_color_type(&ident, &meta, &original_generics, generic_component, true);

    let result = util::bundle_impl(
        "IntoColor",
        ident,
        meta.internal,
        quote! {
            #into_color_impl
            #(#into_impls)*
            #into_color_ty
            #from_color_ty_other_alpha
        },
    );

    result.into()
}

fn impl_into(
    ident: &Ident,
    color: &str,
    meta: &IntoColorMeta,
    generics: &Generics,
    generic_component: bool,
    other_alpha: bool,
) -> Tokens {
    let (_, type_generics, _) = generics.split_for_impl();
    let turbofish_generics = type_generics.as_turbofish();
    let mut generics = generics.clone();

    let method_name = Ident::new(&format!("into_{}", color.to_lowercase()), Span::call_site());

    let trait_path = util::path(&["IntoColor"], meta.internal);
    let alpha_path = util::path(&["Alpha"], meta.internal);

    let white_point = shared::white_point_type(meta.white_point.clone(), meta.internal);
    let component = shared::component_type(meta.component.clone());

    if generic_component {
        shared::add_component_where_clause(&component, &mut generics, meta.internal)
    }

    let color_ty = shared::get_convert_color_type(
        color,
        &white_point,
        &component,
        meta.rgb_space.as_ref(),
        &mut generics,
        meta.internal,
    );

    let method_call = match color {
        "Rgb" | "Luma" => quote! {
            #ident #turbofish_generics::#method_name(color).into_encoding()
        },
        _ => quote! {
            #ident #turbofish_generics::#method_name(color)
        },
    };

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    if other_alpha {
        quote!{
            #[automatically_derived]
            impl #impl_generics Into<#alpha_path<#color_ty, #component>> for #ident #type_generics #where_clause {
                fn into(self) -> #alpha_path<#color_ty, #component> {
                    use #trait_path;
                    let color = self;
                    #method_call.into()
                }
            }
        }
    } else {
        quote!{
            #[automatically_derived]
            impl #impl_generics Into<#color_ty> for #ident #type_generics #where_clause {
                fn into(self) -> #color_ty {
                    use #trait_path;
                    let color = self;
                    #method_call
                }
            }
        }
    }
}

fn impl_into_color_type(
    ident: &Ident,
    meta: &IntoColorMeta,
    generics: &Generics,
    generic_component: bool,
    other_alpha: bool,
) -> Tokens {
    let (_, type_generics, _) = generics.split_for_impl();
    let mut generics = generics.clone();

    let color_path = util::path(&["Color"], meta.internal);
    let alpha_path = util::path(&["Alpha"], meta.internal);

    let white_point = shared::white_point_type(meta.white_point.clone(), meta.internal);
    let component = shared::component_type(meta.component.clone());

    if meta.rgb_space.is_none() {
        generics.params.push(parse_quote!(_S));
    }

    if generic_component {
        shared::add_component_where_clause(&component, &mut generics, meta.internal)
    }

    let color_ty = {
        let rgb_space_type = if let Some(ref rgb_space) = meta.rgb_space {
            rgb_space.clone()
        } else {
            let rgb_space_path = util::path(&["rgb", "RgbSpace"], meta.internal);
            util::add_missing_where_clause(&mut generics);
            let where_clause = generics.where_clause.as_mut().unwrap();
            where_clause
                .predicates
                .push(parse_quote!(_S: #rgb_space_path<WhitePoint = #white_point>));

            parse_quote!(_S)
        };

        quote!(#color_path<#rgb_space_type, #component>)
    };

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    if other_alpha {
        quote!{
            #[automatically_derived]
            impl #impl_generics Into<#alpha_path<#color_ty, #component>> for #ident #type_generics #where_clause {
                fn into(self) -> #alpha_path<#color_ty, #component> {
                    #color_path::Rgb(self.into()).into()
                }
            }
        }
    } else {
        quote!{
            #[automatically_derived]
            impl #impl_generics Into<#color_ty> for #ident #type_generics #where_clause {
                fn into(self) -> #color_ty {
                    #color_path::Rgb(self.into())
                }
            }
        }
    }
}

#[derive(Default)]
struct IntoColorMeta {
    manual_implementations: Vec<KeyValuePair>,
    internal: bool,
    component: Option<Type>,
    white_point: Option<Type>,
    rgb_space: Option<Type>,
}

impl MetaParser for IntoColorMeta {
    fn internal(&mut self) {
        self.internal = true;
    }

    fn parse_attribute(&mut self, attribute_name: Ident, attribute_tts: TokenStream2) {
        match attribute_name.as_ref() {
            "palette_manual_into" => {
                let impls = meta::parse_type_tuple_attribute::<KeyValuePair>(
                    &attribute_name,
                    attribute_tts,
                );
                self.manual_implementations.extend(impls)
            }
            "palette_component" => {
                if self.component.is_none() {
                    let component = meta::parse_equal_attribute(&attribute_name, attribute_tts);
                    self.component = Some(component);
                }
            }
            "palette_white_point" => {
                if self.white_point.is_none() {
                    let white_point = meta::parse_equal_attribute(&attribute_name, attribute_tts);
                    self.white_point = Some(white_point);
                }
            }
            "palette_rgb_space" => {
                if self.rgb_space.is_none() {
                    let rgb_space = meta::parse_equal_attribute(&attribute_name, attribute_tts);
                    self.rgb_space = Some(rgb_space);
                }
            }
            _ => {}
        }
    }
}
