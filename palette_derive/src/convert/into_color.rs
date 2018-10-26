use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{parse_macro_input, DeriveInput, Generics, Ident, Type};

use meta::{self, DataMetaParser, IdentOrIndex, KeyValuePair, MetaParser};
use util;
use COLOR_TYPES;

use super::shared::{self, ConvertDirection};

pub fn derive(tokens: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        attrs,
        generics: original_generics,
        data,
        ..
    } = parse_macro_input!(tokens);
    let mut generics = original_generics.clone();

    let mut meta: IntoColorMeta = meta::parse_attributes(attrs);
    let item_meta: IntoColorItemMeta = meta::parse_data_attributes(data);

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

    // Assume conversion into Xyz by default
    if meta.manual_implementations.is_empty() {
        meta.manual_implementations.push(KeyValuePair {
            key: Ident::new("Xyz", Span::call_site()),
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
        let skip_regular_into = (meta.internal && ident == color)
            || meta
                .manual_implementations
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
            ))
        };

        let with_alpha = impl_into_alpha(
            &ident,
            color,
            &meta,
            &original_generics,
            generic_component,
            alpha_property.as_ref(),
            &alpha_type,
        );

        quote! {
            #regular_into
            #with_alpha
        }
    });

    let result = util::bundle_impl(
        "IntoColor",
        ident.clone(),
        meta.internal,
        quote! {
            #into_color_impl
            #(#into_impls)*
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
) -> TokenStream2 {
    let (_, type_generics, _) = generics.split_for_impl();

    let IntoImplParameters {
        generics,
        trait_path,
        color_ty,
        method_call,
        ..
    } = prepare_into_impl(ident, color, meta, generics, generic_component);

    let (impl_generics, _, where_clause) = generics.split_for_impl();

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

fn impl_into_alpha(
    ident: &Ident,
    color: &str,
    meta: &IntoColorMeta,
    generics: &Generics,
    generic_component: bool,
    alpha_property: Option<&IdentOrIndex>,
    alpha_type: &Type,
) -> TokenStream2 {
    let (_, type_generics, _) = generics.split_for_impl();

    let IntoImplParameters {
        generics,
        alpha_path,
        trait_path,
        color_ty,
        method_call,
        ..
    } = prepare_into_impl(ident, color, meta, generics, generic_component);

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    if let Some(alpha_property) = alpha_property {
        quote!{
            #[automatically_derived]
            impl #impl_generics Into<#alpha_path<#color_ty, #alpha_type>> for #ident #type_generics #where_clause {
                fn into(self) -> #alpha_path<#color_ty, #alpha_type> {
                    use #trait_path;
                    let color = self;
                    #alpha_path {
                        alpha: color.#alpha_property.clone(),
                        color: #method_call,
                    }
                }
            }
        }
    } else {
        quote!{
            #[automatically_derived]
            impl #impl_generics Into<#alpha_path<#color_ty, #alpha_type>> for #ident #type_generics #where_clause {
                fn into(self) -> #alpha_path<#color_ty, #alpha_type> {
                    use #trait_path;
                    let color = self;
                    #method_call.into()
                }
            }
        }
    }
}

fn prepare_into_impl(
    ident: &Ident,
    color: &str,
    meta: &IntoColorMeta,
    generics: &Generics,
    generic_component: bool,
) -> IntoImplParameters {
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

    IntoImplParameters {
        generics,
        alpha_path,
        trait_path,
        color_ty,
        method_call,
    }
}

struct IntoImplParameters {
    generics: Generics,
    alpha_path: TokenStream2,
    trait_path: TokenStream2,
    color_ty: Type,
    method_call: TokenStream2,
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
        match &*attribute_name.to_string() {
            "palette_manual_into" => {
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
struct IntoColorItemMeta {
    alpha_property: Option<(IdentOrIndex, Type)>,
}

impl DataMetaParser for IntoColorItemMeta {
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
