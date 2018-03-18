use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{self, DeriveInput, GenericParam, Generics, Ident, LitStr, Path, Type, TypePath,
          WhereClause};
use syn::punctuated::Punctuated;
use syn::token::Eq;
use quote::{ToTokens, Tokens};

use meta::{self, MetaOutput, MetaParser};
use util;

use COLOR_TYPES;

pub fn derive(tokens: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        attrs,
        generics: original_generics,
        ..
    } = syn::parse(tokens).expect("could not parse tokens");
    let mut generics = original_generics.clone();
    let mut meta = meta::parse_attributes::<FromColorMetaParser>(attrs);

    let mut generic_component = false;
    let mut generic_white_point = false;

    for param in &generics.params {
        if let GenericParam::Type(ref param) = *param {
            if let Some(&Type::Path(TypePath {
                qself: None,
                path:
                    Path {
                        segments: ref component,
                        leading_colon,
                    },
            })) = meta.component.as_ref()
            {
                let first = component.first().map(|s| s.into_value());
                let is_ident_path = leading_colon.is_none() && component.len() == 1
                    && first.unwrap().arguments.is_empty()
                    && first.unwrap().ident == param.ident;

                if is_ident_path {
                    generic_component = true;
                }
            }

            if let Some(&Type::Path(TypePath {
                qself: None,
                path:
                    Path {
                        segments: ref white_point,
                        leading_colon,
                    },
            })) = meta.white_point.as_ref()
            {
                let first = white_point.first().map(|s| s.into_value());
                let is_ident_path = leading_colon.is_none() && white_point.len() == 1
                    && first.unwrap().arguments.is_empty()
                    && first.unwrap().ident == param.ident;

                if is_ident_path {
                    generic_white_point = true;
                }
            }
        }
    }

    let white_point = meta.white_point
        .as_ref()
        .map(|white_point| white_point.into_tokens())
        .unwrap_or_else(|| util::path(&["white_point", "D65"], meta.internal));
    let component = meta.component.as_ref();

    if generic_component {
        add_missing_where_clause(&mut generics);

        let where_clause = generics.where_clause.as_mut().unwrap();
        let component_trait_path = util::path(&["Component"], meta.internal);

        where_clause
            .predicates
            .push(parse_quote!(#component: #component_trait_path + _num_traits::Float));
    }

    if generic_white_point {
        add_missing_where_clause(&mut generics);

        let where_clause = generics.where_clause.as_mut().unwrap();
        let white_point_trait_path = util::path(&["white_point", "WhitePoint"], meta.internal);

        where_clause
            .predicates
            .push(parse_quote!(#white_point: #white_point_trait_path));
    }

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let turbofish_generics = type_generics.as_turbofish();

    // Assume conversion from Zyz by default
    if meta.manual_implementations.is_empty() {
        meta.manual_implementations.push(ManualImpl {
            ident: "Xyz".into(),
            function: None,
        });
    }

    let mut xyz_included = false;
    let mut methods = vec![];

    let type_params: &[_] = &[&white_point, &component.into_tokens()];

    for color in &meta.manual_implementations {
        if color.ident == "Xyz" {
            xyz_included = true;
        }

        let method_name = Ident::new(
            &format!("from_{}", color.ident.as_ref().to_lowercase()),
            Span::call_site(),
        );
        let color_path = util::color_path(color.ident.as_ref(), meta.internal);
        let convert_function = color
            .function
            .clone()
            .unwrap_or_else(|| Ident::new("from", Span::call_site()));

        let method = match color.ident.as_ref() {
            "Rgb" | "Hsl" | "Hsv" | "Hwb" => {
                let rgb_space_path = util::path(&["rgb", "RgbSpace"], meta.internal);
                quote!(#method_name<_S: #rgb_space_path<WhitePoint = #white_point>>)
            }
            _ => quote!(#method_name),
        };

        let color_ty = match color.ident.as_ref() {
            "Rgb" => {
                let linear_path = util::path(&["encoding", "Linear"], meta.internal);

                quote!(#color_path<#linear_path<_S>, #component>)
            }
            "Luma" => {
                let linear_path = util::path(&["encoding", "Linear"], meta.internal);

                quote!(#color_path<#linear_path<#white_point>, #component>)
            }
            "Hsl" | "Hsv" | "Hwb" => quote!(#color_path<_S, #component>),
            _ => {
                if type_params.is_empty() {
                    quote!(#color_path)
                } else {
                    quote!(#color_path<#(#type_params),*>)
                }
            }
        };

        methods.push(quote! {
            fn #method (color: #color_ty) -> Self {
                #ident #turbofish_generics::#convert_function(color)
            }
        });
    }

    let from_impls: Vec<_> = COLOR_TYPES
        .into_iter()
        .map(|&color| {
            let skip_regular_from = (meta.internal && color == ident.as_ref())
                || meta.manual_implementations
                    .iter()
                    .any(|color_impl| color_impl.ident == color && color_impl.function.is_none());

            let regular_from = if skip_regular_from {
                None
            } else {
                Some(impl_from(
                    &ident,
                    type_params,
                    color,
                    &meta,
                    &original_generics,
                    generic_component,
                    false,
                    false,
                ))
            };

            let self_alpha = if meta.internal && ident != color {
                Some(impl_from(
                    &ident,
                    type_params,
                    color,
                    &meta,
                    &original_generics,
                    generic_component,
                    true,
                    false,
                ))
            } else {
                None
            };

            let other_alpha = impl_from(
                &ident,
                type_params,
                color,
                &meta,
                &original_generics,
                generic_component,
                false,
                true,
            );

            let both_alpha = if meta.internal && ident != color {
                Some(impl_from(
                    &ident,
                    type_params,
                    color,
                    &meta,
                    &original_generics,
                    generic_component,
                    true,
                    true,
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

    let from_color_ty = impl_from_color_type(
        &ident,
        &meta,
        &original_generics,
        generic_component,
        false,
        false,
    );

    let from_color_ty_self_alpha = if meta.internal {
        Some(impl_from_color_type(
            &ident,
            &meta,
            &original_generics,
            generic_component,
            true,
            false,
        ))
    } else {
        None
    };

    let from_color_ty_other_alpha = impl_from_color_type(
        &ident,
        &meta,
        &original_generics,
        generic_component,
        false,
        true,
    );

    let from_color_ty_both_alpha = if meta.internal {
        Some(impl_from_color_type(
            &ident,
            &meta,
            &original_generics,
            generic_component,
            true,
            true,
        ))
    } else {
        None
    };

    if !xyz_included {
        let color_path = util::path(&["Xyz"], meta.internal);
        let color_ty = if type_params.is_empty() {
            quote!(#color_path)
        } else {
            quote!(#color_path<#(#type_params),*>)
        };

        methods.push(quote! {
            fn from_xyz(color: #color_ty) -> Self {
                color.into()
            }
        })
    }

    let trait_path = util::path(&["FromColor"], meta.internal);

    let trait_ty = if type_params.is_empty() {
        quote!(#trait_path)
    } else {
        quote!(#trait_path<#(#type_params),*>)
    };

    let from_color_impl = quote!{
        #[automatically_derived]
        impl #impl_generics #trait_ty for #ident #type_generics #where_clause {
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
            #from_color_ty
            #from_color_ty_self_alpha
            #from_color_ty_other_alpha
            #from_color_ty_both_alpha
        },
    );

    //println!("\n\n{}\n", result);

    result.into()
}

fn impl_from(
    ident: &Ident,
    type_params: &[&Tokens],
    color: &str,
    meta: &FromColorMeta,
    generics: &Generics,
    generic_component: bool,
    self_alpha: bool,
    other_alpha: bool,
) -> Tokens {
    let (_, type_generics, _) = generics.split_for_impl();
    let turbofish_generics = type_generics.as_turbofish();
    let mut generics = generics.clone();

    let color_path = util::color_path(color, meta.internal);
    let method_name = Ident::new(&format!("from_{}", color.to_lowercase()), Span::call_site());

    let trait_path = util::path(&["FromColor"], meta.internal);
    let alpha_path = util::path(&["Alpha"], meta.internal);

    let white_point = meta.white_point
        .as_ref()
        .map(|white_point| white_point.into_tokens())
        .unwrap_or_else(|| util::path(&["white_point", "D65"], meta.internal));
    let component = meta.component
        .as_ref()
        .map(|component| component.into_tokens())
        .unwrap_or_else(|| quote!(f32));

    if generic_component {
        add_missing_where_clause(&mut generics);

        let where_clause = generics.where_clause.as_mut().unwrap();
        let component_trait_path = util::path(&["Component"], meta.internal);

        where_clause
            .predicates
            .push(parse_quote!(#component: #component_trait_path + _num_traits::Float));
    }

    let color_ty = match color {
        "Rgb" => {
            let rgb_standard_path = util::path(&["rgb", "RgbStandard"], meta.internal);
            let rgb_space_path = util::path(&["rgb", "RgbSpace"], meta.internal);
            generics.params.push(GenericParam::Type(
                Ident::new("_S", Span::call_site()).into(),
            ));

            add_missing_where_clause(&mut generics);
            let where_clause = generics.where_clause.as_mut().unwrap();
            if let Some(ref rgb_space) = meta.rgb_space {
                where_clause
                    .predicates
                    .push(parse_quote!(_S: #rgb_standard_path<Space = #rgb_space>));
            } else {
                where_clause
                    .predicates
                    .push(parse_quote!(_S: #rgb_standard_path));
                where_clause
                    .predicates
                    .push(parse_quote!(_S::Space: #rgb_space_path<WhitePoint = #white_point>));
            }

            quote!(#color_path<_S, #component>)
        }
        "Luma" => {
            let luma_standard_path = util::path(&["luma", "LumaStandard"], meta.internal);
            generics.params.push(GenericParam::Type(
                Ident::new("_S", Span::call_site()).into(),
            ));

            add_missing_where_clause(&mut generics);
            let where_clause = generics.where_clause.as_mut().unwrap();
            where_clause
                .predicates
                .push(parse_quote!(_S: #luma_standard_path<WhitePoint = #white_point>));
            quote!(#color_path<_S, #component>)
        }
        "Hsl" | "Hsv" | "Hwb" => {
            let rgb_space_path = util::path(&["rgb", "RgbSpace"], meta.internal);

            add_missing_where_clause(&mut generics);

            if let Some(ref rgb_space) = meta.rgb_space {
                quote!(#color_path<#rgb_space, #component>)
            } else {
                generics.params.push(GenericParam::Type(
                    Ident::new("_S", Span::call_site()).into(),
                ));

                let where_clause = generics.where_clause.as_mut().unwrap();
                where_clause
                    .predicates
                    .push(parse_quote!(_S: #rgb_space_path<WhitePoint = #white_point>));

                quote!(#color_path<_S, #component>)
            }
        }
        _ => {
            if type_params.is_empty() {
                quote!(#color_path)
            } else {
                quote!(#color_path<#(#type_params),*>)
            }
        }
    };

    let method_call = match color {
        "Rgb" | "Luma" => quote! {
            #ident #turbofish_generics::#method_name(color.into_linear())
        },
        _ => quote! {
            #ident #turbofish_generics::#method_name(color)
        },
    };

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    match (self_alpha, other_alpha) {
        (true, true) => quote!{
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
        },
        (true, false) => quote!{
            #[automatically_derived]
            impl #impl_generics From<#color_ty> for #alpha_path<#ident #type_generics, #component> #where_clause {
                fn from(color: #color_ty) -> Self {
                    use #trait_path;
                    #method_call.into()
                }
            }
        },
        (false, true) => quote!{
            #[automatically_derived]
            impl #impl_generics From<#alpha_path<#color_ty, #component>> for #ident #type_generics #where_clause {
                fn from(color: #alpha_path<#color_ty, #component>) -> Self {
                    use #trait_path;
                    let color = color.color;
                    #method_call
                }
            }
        },
        (false, false) => quote!{
            #[automatically_derived]
            impl #impl_generics From<#color_ty> for #ident #type_generics #where_clause {
                fn from(color: #color_ty) -> Self {
                    use #trait_path;
                    #method_call
                }
            }
        },
    }
}

fn impl_from_color_type(
    ident: &Ident,
    meta: &FromColorMeta,
    generics: &Generics,
    generic_component: bool,
    self_alpha: bool,
    other_alpha: bool,
) -> Tokens {
    let (_, type_generics, _) = generics.split_for_impl();
    let turbofish_generics = type_generics.as_turbofish();
    let mut generics = generics.clone();

    let color_path = util::path(&["Color"], meta.internal);
    let alpha_path = util::path(&["Alpha"], meta.internal);

    let white_point = meta.white_point
        .as_ref()
        .map(|white_point| white_point.into_tokens())
        .unwrap_or_else(|| util::path(&["white_point", "D65"], meta.internal));
    let component = meta.component
        .as_ref()
        .map(|component| component.into_tokens())
        .unwrap_or_else(|| quote!(f32));

    if meta.rgb_space.is_none() {
        generics.params.push(GenericParam::Type(
            Ident::new("_S", Span::call_site()).into(),
        ));
    }

    if generic_component {
        add_missing_where_clause(&mut generics);

        let where_clause = generics.where_clause.as_mut().unwrap();
        let component_trait_path = util::path(&["Component"], meta.internal);

        where_clause
            .predicates
            .push(parse_quote!(#component: #component_trait_path + _num_traits::Float));
    }

    let color_ty = {
        let rgb_space_type = if let Some(ref rgb_space) = meta.rgb_space {
            rgb_space.clone()
        } else {
            let rgb_space_path = util::path(&["rgb", "RgbSpace"], meta.internal);
            add_missing_where_clause(&mut generics);
            let where_clause = generics.where_clause.as_mut().unwrap();
            where_clause
                .predicates
                .push(parse_quote!(_S: #rgb_space_path<WhitePoint = #white_point>));

            parse_quote!(_S)
        };

        quote!(#color_path<#rgb_space_type, #component>)
    };

    let match_arms = COLOR_TYPES.into_iter().map(|&color| {
        let color_ident = Ident::new(color, Span::call_site());
        if meta.internal && color == ident.as_ref() {
            let convert_function = meta.manual_implementations
                .iter()
                .find(|color_impl| color_impl.ident == color)
                .and_then(|color_impl| color_impl.function.as_ref());

            if let Some(convert_function) = convert_function {
                quote!(#color_path::#color_ident(c) => #ident #turbofish_generics::#convert_function(c))
            } else {
                quote!(#color_path::#color_ident(c) => c)
            }
        } else {
            quote!(#color_path::#color_ident(c) => Self::from(c))
        }
    });

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    match (self_alpha, other_alpha) {
        (true, true) => quote!{
            #[automatically_derived]
            impl #impl_generics From<#alpha_path<#color_ty, #component>> for #alpha_path<#ident #type_generics, #component> #where_clause {
                fn from(color: #alpha_path<#color_ty, #component>) -> Self {
                    let #alpha_path {color, alpha} = color;
                    #alpha_path {
                        color: #ident #turbofish_generics::from(color),
                        alpha
                    }
                }
            }
        },
        (true, false) => quote!{
            #[automatically_derived]
            impl #impl_generics From<#color_ty> for #alpha_path<#ident #type_generics, #component> #where_clause {
                fn from(color: #color_ty) -> Self {
                    #ident #turbofish_generics::from(color).into()
                }
            }
        },
        (false, true) => quote!{
            #[automatically_derived]
            impl #impl_generics From<#alpha_path<#color_ty, #component>> for #ident #type_generics #where_clause {
                fn from(color: #alpha_path<#color_ty, #component>) -> Self {
                    Self::from(color.color)
                }
            }
        },
        (false, false) => quote!{
            #[automatically_derived]
            impl #impl_generics From<#color_ty> for #ident #type_generics #where_clause {
                fn from(color: #color_ty) -> Self {
                    match color {
                        #(#match_arms),*
                    }
                }
            }
        },
    }
}

fn add_missing_where_clause(generics: &mut Generics) {
    if generics.where_clause.is_none() {
        generics.where_clause = Some(WhereClause {
            where_token: Token![where](Span::call_site()),
            predicates: Punctuated::new(),
        })
    }
}

struct FromColorMetaParser;

impl MetaParser for FromColorMetaParser {
    type Output = FromColorMeta;

    fn parse_attribute(
        output: &mut Self::Output,
        attribute_name: Ident,
        attribute_tts: TokenStream2,
    ) {
        match attribute_name.as_ref() {
            "palette_manual_from" => {
                let impls =
                    meta::parse_type_tuple_attribute::<ManualImpl>(&attribute_name, attribute_tts);
                output.manual_implementations.extend(impls)
            }
            "palette_component" => {
                if output.component.is_none() {
                    let component = meta::parse_equal_attribute(&attribute_name, attribute_tts);
                    output.component = Some(component);
                }
            }
            "palette_white_point" => {
                if output.white_point.is_none() {
                    let white_point = meta::parse_equal_attribute(&attribute_name, attribute_tts);
                    output.white_point = Some(white_point);
                }
            }
            "palette_rgb_space" => {
                if output.rgb_space.is_none() {
                    let rgb_space = meta::parse_equal_attribute(&attribute_name, attribute_tts);
                    output.rgb_space = Some(rgb_space);
                }
            }
            _ => {}
        }
    }
}

#[derive(Default)]
struct FromColorMeta {
    manual_implementations: Vec<ManualImpl>,
    internal: bool,
    component: Option<Type>,
    white_point: Option<Type>,
    rgb_space: Option<Type>,
}

impl MetaOutput for FromColorMeta {
    fn internal(&mut self) {
        self.internal = true;
    }
}

#[derive(PartialEq)]
struct ManualImpl {
    ident: Ident,
    function: Option<Ident>,
}

impl ::syn::synom::Synom for ManualImpl {
    named!(parse -> Self, do_parse!(
        ident: syn!(Ident) >>
        function: option!(do_parse!(
            _eq: syn!(Eq) >>
            function: syn!(LitStr) >>
            (Ident::new(&function.value(), Span::call_site()))
        )) >>
        (ManualImpl {
            ident,
            function
        })
    ));
}

impl PartialEq<str> for ManualImpl {
    fn eq(&self, other: &str) -> bool {
        self.ident == other
    }
}
