use std::collections::HashSet;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_quote, DeriveInput, Generics, Ident, Result, Type};

use crate::meta::{
    parse_field_attributes, parse_namespaced_attributes, FieldAttributes, IdentOrIndex,
    TypeItemAttributes,
};
use crate::util;

use crate::COLOR_TYPES;

use super::util::{
    add_float_component_where_clause, add_white_point_where_clause, component_type,
    find_in_generics, find_nearest_color, get_convert_color_type, white_point_type,
};

pub fn derive(item: TokenStream) -> ::std::result::Result<TokenStream, Vec<::syn::parse::Error>> {
    let DeriveInput {
        ident,
        generics: original_generics,
        data,
        attrs,
        ..
    } = syn::parse(item).map_err(|error| vec![error])?;
    let mut generics = original_generics.clone();

    let mut item_meta: TypeItemAttributes = parse_namespaced_attributes(attrs)?;

    let fields_meta: FieldAttributes = if let syn::Data::Struct(struct_data) = data {
        parse_field_attributes(struct_data.fields)?
    } else {
        return Err(vec![syn::Error::new(
            Span::call_site(),
            "only structs are supported",
        )]);
    };

    let (generic_component, generic_white_point) = find_in_generics(
        item_meta.component.as_ref(),
        item_meta.white_point.as_ref(),
        &original_generics,
    );

    let white_point = white_point_type(item_meta.white_point.clone(), item_meta.internal);
    let component = component_type(item_meta.component.clone());

    let alpha_field = fields_meta.alpha_property;

    if generic_component {
        add_float_component_where_clause(&component, &mut generics, item_meta.internal);
    }

    if generic_white_point {
        add_white_point_where_clause(&white_point, &mut generics, item_meta.internal);
    }

    // Assume conversion from Xyz by default
    if item_meta.skip_derives.is_empty() {
        item_meta.skip_derives.insert("Xyz".into());
    }

    let all_from_impl_params = prepare_from_impl(
        &item_meta.skip_derives,
        &component,
        &white_point,
        item_meta.rgb_standard.as_ref(),
        &item_meta,
        &generics,
        generic_component,
    )
    .map_err(|error| vec![error])?;

    let mut implementations =
        generate_from_implementations(&ident, &generics, &item_meta, &all_from_impl_params);

    if let Some((alpha_property, alpha_type)) = alpha_field {
        implementations.push(generate_from_alpha_implementation_with_internal(
            &ident,
            &generics,
            &item_meta,
            &alpha_property,
            &alpha_type,
        ));
    } else {
        implementations.push(generate_from_alpha_implementation(
            &ident, &generics, &item_meta,
        ));
    }

    Ok(TokenStream::from(quote! {
        #(#implementations)*
    }))
}

fn prepare_from_impl(
    skip: &HashSet<String>,
    component: &Type,
    white_point: &Type,
    rgb_standard: Option<&Type>,
    meta: &TypeItemAttributes,
    generics: &Generics,
    generic_component: bool,
) -> Result<Vec<FromImplParameters>> {
    let included_colors = COLOR_TYPES.iter().filter(|&&color| !skip.contains(color));
    let linear_path = util::path(&["encoding", "Linear"], meta.internal);
    let from_trait_path = util::path(&["convert", "FromColorUnclamped"], meta.internal);
    let into_trait_path = util::path(&["convert", "IntoColorUnclamped"], meta.internal);

    let mut parameters = Vec::new();

    for &color_name in included_colors {
        let nearest_color_name = find_nearest_color(color_name, skip)?;

        let mut generics = generics.clone();
        if generic_component {
            add_float_component_where_clause(&component, &mut generics, meta.internal)
        }

        let color_ty = get_convert_color_type(
            color_name,
            white_point,
            component,
            rgb_standard,
            &mut generics,
            meta.internal,
        );

        let nearest_color_path = util::color_path(nearest_color_name, meta.internal);
        let target_color_rgb_standard = match color_name {
            "Rgb" | "Hsl" | "Hsv" | "Hwb" => Some(parse_quote!(_S)),
            _ => None,
        };

        let rgb_standard = rgb_standard.cloned().or(target_color_rgb_standard);

        let nearest_color_ty: Type = match nearest_color_name {
            "Rgb" | "Hsl" | "Hsv" | "Hwb" => {
                let rgb_standard = rgb_standard
                    .ok_or_else(|| {
                        syn::parse::Error::new(
                            Span::call_site(),
                            format!(
                                "could not determine which RGB standard to use when converting to and from `{}` via `{}`",
                                color_name,
                                nearest_color_name
                            ),
                        )
                    })?;

                parse_quote!(#nearest_color_path::<#rgb_standard, #component>)
            }
            "Luma" => parse_quote!(#nearest_color_path::<#linear_path<#white_point>, #component>),
            _ => parse_quote!(#nearest_color_path::<#white_point, #component>),
        };

        let mut into_generics = generics.clone();

        if color_name == "Oklab" {
            generics.make_where_clause().predicates.push(parse_quote!(
                #nearest_color_ty: #from_trait_path<#color_ty>
            ));
            into_generics
                .make_where_clause()
                .predicates
                .push(parse_quote!(
                    #nearest_color_ty: #into_trait_path<#color_ty>
                ));
        }

        parameters.push(FromImplParameters {
            generics,
            into_generics,
            color_ty,
            nearest_color_ty,
        });
    }

    Ok(parameters)
}

struct FromImplParameters {
    generics: Generics,
    into_generics: Generics,
    color_ty: Type,
    nearest_color_ty: Type,
}

fn generate_from_implementations(
    ident: &Ident,
    generics: &Generics,
    meta: &TypeItemAttributes,
    all_parameters: &[FromImplParameters],
) -> Vec<TokenStream2> {
    let from_trait_path = util::path(&["convert", "FromColorUnclamped"], meta.internal);
    let into_trait_path = util::path(&["convert", "IntoColorUnclamped"], meta.internal);

    let (_, type_generics, _) = generics.split_for_impl();

    let mut implementations = Vec::with_capacity(all_parameters.len());

    for parameters in all_parameters {
        let FromImplParameters {
            color_ty,
            generics,
            into_generics,
            nearest_color_ty,
        } = parameters;

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        implementations.push(quote! {
            #[automatically_derived]
            impl #impl_generics #from_trait_path<#color_ty> for #ident #type_generics #where_clause {
                fn from_color_unclamped(color: #color_ty) -> Self {
                    use #from_trait_path;
                    use #into_trait_path;
                    #nearest_color_ty::from_color_unclamped(color).into_color_unclamped()
                }
            }
        });

        if !meta.internal || meta.internal_not_base_type {
            let (impl_generics, _, where_clause) = into_generics.split_for_impl();

            implementations.push(quote! {
                #[automatically_derived]
                impl #impl_generics #from_trait_path<#ident #type_generics> for #color_ty #where_clause {
                    fn from_color_unclamped(color: #ident #type_generics) -> Self {
                        use #from_trait_path;
                        use #into_trait_path;
                        #nearest_color_ty::from_color_unclamped(color).into_color_unclamped()
                    }
                }
            });
        }
    }

    implementations
}

fn generate_from_alpha_implementation(
    ident: &Ident,
    generics: &Generics,
    meta: &TypeItemAttributes,
) -> TokenStream2 {
    let from_trait_path = util::path(&["convert", "FromColorUnclamped"], meta.internal);
    let into_trait_path = util::path(&["convert", "IntoColorUnclamped"], meta.internal);
    let component_trait_path = util::path(&["Component"], meta.internal);
    let alpha_path = util::path(&["Alpha"], meta.internal);

    let mut impl_generics = generics.clone();
    impl_generics.params.push(parse_quote!(_C));
    impl_generics.params.push(parse_quote!(_A));
    {
        let where_clause = impl_generics.make_where_clause();
        where_clause
            .predicates
            .push(parse_quote!(_C: #into_trait_path<Self>));
        where_clause
            .predicates
            .push(parse_quote!(_A: #component_trait_path));
    }

    let (_, type_generics, _) = generics.split_for_impl();
    let (self_impl_generics, _, self_where_clause) = impl_generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #self_impl_generics #from_trait_path<#alpha_path<_C, _A>> for #ident #type_generics #self_where_clause {
            fn from_color_unclamped(color: #alpha_path<_C, _A>) -> Self {
                color.color.into_color_unclamped()
            }
        }
    }
}

fn generate_from_alpha_implementation_with_internal(
    ident: &Ident,
    generics: &Generics,
    meta: &TypeItemAttributes,
    alpha_property: &IdentOrIndex,
    alpha_type: &Type,
) -> TokenStream2 {
    let from_trait_path = util::path(&["convert", "FromColorUnclamped"], meta.internal);
    let into_trait_path = util::path(&["convert", "IntoColorUnclamped"], meta.internal);
    let alpha_path = util::path(&["Alpha"], meta.internal);

    let (_, type_generics, _) = generics.split_for_impl();
    let mut impl_generics = generics.clone();
    impl_generics.params.push(parse_quote!(_C));
    {
        let where_clause = impl_generics.make_where_clause();
        where_clause
            .predicates
            .push(parse_quote!(_C: #into_trait_path<Self>));
    }
    let (impl_generics, _, where_clause) = impl_generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics #from_trait_path<#alpha_path<_C, #alpha_type>> for #ident #type_generics #where_clause {
            fn from_color_unclamped(color: #alpha_path<_C, #alpha_type>) -> Self {
                use #from_trait_path;
                use #into_trait_path;

                let #alpha_path { color, alpha } = color;

                let mut result: Self = color.into_color_unclamped();
                result.#alpha_property = alpha;

                result
            }
        }
    }
}
