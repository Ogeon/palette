use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{parse_macro_input, DeriveInput, Generics, Ident, Type};

use meta::{self, DataMetaParser, IdentOrIndex, KeyValuePair, MetaParser};
use util;

use super::shared::{self, ConvertDirection};

use COLOR_TYPES;

pub fn derive(tokens: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        attrs,
        generics: original_generics,
        data,
        ..
    } = parse_macro_input!(tokens);
    let mut generics = original_generics.clone();

    let mut meta: FromColorMeta = meta::parse_attributes(attrs);
    let item_meta: FromColorItemMeta = meta::parse_data_attributes(data);

    let (generic_component, generic_white_point) = shared::find_in_generics(
        meta.component.as_ref(),
        meta.white_point.as_ref(),
        &original_generics,
    );

    let white_point = shared::white_point_type(meta.white_point.clone(), meta.internal);
    let component = shared::component_type(meta.component.clone());

    let (alpha_property, alpha_type) = item_meta
        .alpha_property
        .map(|(property, ty)| (Some(property), ty))
        .unwrap_or_else(|| (None, component.clone()));

    if generic_component {
        shared::add_component_where_clause(&component, &mut generics, meta.internal);
    }

    if generic_white_point {
        shared::add_white_point_where_clause(&white_point, &mut generics, meta.internal);
    }

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    // Assume conversion from Xyz by default
    if meta.manual_implementations.is_empty() {
        meta.manual_implementations.push(KeyValuePair {
            key: Ident::new("Xyz", Span::call_site()),
            value: None,
        });
    }

    let methods = shared::generate_methods(
        &ident,
        ConvertDirection::From,
        &meta.manual_implementations,
        &component,
        &white_point,
        &shared::rgb_space_type(meta.rgb_space.clone(), &white_point, meta.internal),
        &type_generics.as_turbofish(),
        meta.internal,
    );

    let from_impls: Vec<_> = COLOR_TYPES
        .into_iter()
        .map(|&color| {
            let skip_regular_from = (meta.internal && ident == color)
                || meta
                    .manual_implementations
                    .iter()
                    .any(|color_impl| color_impl.key == color && color_impl.value.is_none());

            let regular_from = if skip_regular_from {
                None
            } else {
                Some(impl_from(
                    &ident,
                    color,
                    &meta,
                    &original_generics,
                    generic_component,
                ))
            };

            let self_alpha = if meta.internal && ident != color {
                Some(impl_from_no_alpha_to_alpha(
                    &ident,
                    color,
                    &meta,
                    &original_generics,
                    generic_component,
                ))
            } else {
                None
            };

            let other_alpha = impl_from_alpha_to_no_alpha(
                &ident,
                color,
                &meta,
                &original_generics,
                generic_component,
                alpha_property.as_ref(),
                &alpha_type,
            );

            let both_alpha = if meta.internal && ident != color {
                Some(impl_from_alpha_to_alpha(
                    &ident,
                    color,
                    &meta,
                    &original_generics,
                    generic_component,
                ))
            } else {
                None
            };

            quote!{
                #regular_from
                #self_alpha
                #other_alpha
                #both_alpha
            }
        })
        .collect();

    let trait_path = util::path(&["FromColor"], meta.internal);
    let from_color_impl = quote!{
        #[automatically_derived]
        impl #impl_generics #trait_path<#white_point, #component> for #ident #type_generics #where_clause {
            #(#methods)*
        }
    };

    let result = util::bundle_impl(
        "FromColor",
        ident,
        meta.internal,
        quote! {
            #from_color_impl
            #(#from_impls)*
        },
    );

    result.into()
}

fn impl_from(
    ident: &Ident,
    color: &str,
    meta: &FromColorMeta,
    generics: &Generics,
    generic_component: bool,
) -> TokenStream2 {
    let (_, type_generics, _) = generics.split_for_impl();

    let FromImplParameters {
        generics,
        trait_path,
        color_ty,
        method_call,
        ..
    } = prepare_from_impl(ident, color, meta, generics, generic_component);

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    quote!{
        #[automatically_derived]
        impl #impl_generics From<#color_ty> for #ident #type_generics #where_clause {
            fn from(color: #color_ty) -> Self {
                use #trait_path;
                #method_call
            }
        }
    }
}

fn impl_from_alpha_to_alpha(
    ident: &Ident,
    color: &str,
    meta: &FromColorMeta,
    generics: &Generics,
    generic_component: bool,
) -> TokenStream2 {
    let (_, type_generics, _) = generics.split_for_impl();

    let FromImplParameters {
        generics,
        alpha_path,
        trait_path,
        color_ty,
        component,
        method_call,
    } = prepare_from_impl(ident, color, meta, generics, generic_component);

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    quote!{
           #[automatically_derived]
           impl #impl_generics From<#alpha_path<#color_ty, #component>> for #alpha_path<#ident #type_generics, #component> #where_clause {
               fn from(color: #alpha_path<#color_ty, #component>) -> Self {
                   use #trait_path;
                   let #alpha_path {color, alpha} = color;
                   #alpha_path {
                       color: #method_call,
                       alpha
                   }
               }
           }
    }
}

fn impl_from_no_alpha_to_alpha(
    ident: &Ident,
    color: &str,
    meta: &FromColorMeta,
    generics: &Generics,
    generic_component: bool,
) -> TokenStream2 {
    let (_, type_generics, _) = generics.split_for_impl();

    let FromImplParameters {
        generics,
        alpha_path,
        trait_path,
        color_ty,
        method_call,
        component,
    } = prepare_from_impl(ident, color, meta, generics, generic_component);

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    quote!{
        #[automatically_derived]
        impl #impl_generics From<#color_ty> for #alpha_path<#ident #type_generics, #component> #where_clause {
            fn from(color: #color_ty) -> Self {
                use #trait_path;
                #method_call.into()
            }
        }
    }
}

fn impl_from_alpha_to_no_alpha(
    ident: &Ident,
    color: &str,
    meta: &FromColorMeta,
    generics: &Generics,
    generic_component: bool,
    alpha_property: Option<&IdentOrIndex>,
    alpha_type: &Type,
) -> TokenStream2 {
    let (_, type_generics, _) = generics.split_for_impl();

    let FromImplParameters {
        generics,
        alpha_path,
        trait_path,
        color_ty,
        method_call,
        ..
    } = prepare_from_impl(ident, color, meta, generics, generic_component);

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    if let Some(alpha_property) = alpha_property {
        quote!{
            #[automatically_derived]
            impl #impl_generics From<#alpha_path<#color_ty, #alpha_type>> for #ident #type_generics #where_clause {
                fn from(color: #alpha_path<#color_ty, #alpha_type>) -> Self {
                    use #trait_path;
                    let #alpha_path { color, alpha } = color;
                    let mut result = #method_call;
                    result.#alpha_property = alpha;
                    result
                }
            }
        }
    } else {
        quote!{
            #[automatically_derived]
            impl #impl_generics From<#alpha_path<#color_ty, #alpha_type>> for #ident #type_generics #where_clause {
                fn from(color: #alpha_path<#color_ty, #alpha_type>) -> Self {
                    use #trait_path;
                    let color = color.color;
                    #method_call
                }
            }
        }
    }
}

fn prepare_from_impl(
    ident: &Ident,
    color: &str,
    meta: &FromColorMeta,
    generics: &Generics,
    generic_component: bool,
) -> FromImplParameters {
    let (_, type_generics, _) = generics.split_for_impl();
    let turbofish_generics = type_generics.as_turbofish();
    let mut generics = generics.clone();

    let method_name = Ident::new(&format!("from_{}", color.to_lowercase()), Span::call_site());

    let trait_path = util::path(&["FromColor"], meta.internal);
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
            #ident #turbofish_generics::#method_name(color.into_linear())
        },
        _ => quote! {
            #ident #turbofish_generics::#method_name(color)
        },
    };

    return FromImplParameters {
        generics,
        alpha_path,
        trait_path,
        color_ty,
        component,
        method_call,
    };
}

struct FromImplParameters {
    generics: Generics,
    alpha_path: TokenStream2,
    trait_path: TokenStream2,
    color_ty: Type,
    component: Type,
    method_call: TokenStream2,
}

#[derive(Default)]
struct FromColorMeta {
    manual_implementations: Vec<KeyValuePair>,
    internal: bool,
    component: Option<Type>,
    white_point: Option<Type>,
    rgb_space: Option<Type>,
}

impl MetaParser for FromColorMeta {
    fn internal(&mut self) {
        self.internal = true;
    }

    fn parse_attribute(&mut self, attribute_name: Ident, attribute_tts: TokenStream2) {
        match &*attribute_name.to_string() {
            "palette_manual_from" => {
                let impls =
                    meta::parse_tuple_attribute::<KeyValuePair>(&attribute_name, attribute_tts);
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

#[derive(Default)]
struct FromColorItemMeta {
    alpha_property: Option<(IdentOrIndex, Type)>,
}

impl DataMetaParser for FromColorItemMeta {
    fn parse_struct_field_attribute(
        &mut self,
        field_name: IdentOrIndex,
        ty: Type,
        attribute_name: Ident,
        attribute_tts: TokenStream2,
    ) {
        match &*attribute_name.to_string() {
            "palette_alpha" => {
                meta::assert_empty_attribute(&attribute_name, attribute_tts);
                self.alpha_property = Some((field_name, ty));
            }
            _ => {}
        }
    }
}
