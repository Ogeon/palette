//! Code generator for implementing `FromColorUnclamped`.

pub(crate) mod util;

use std::collections::HashSet;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Generics, Result, Type};

use crate::{
    color_types::{ColorGenerics, ColorGroup, ColorInfo, ColorMeta, MetaTypeSource, XYZ_COLORS},
    util::{IdentOrIndex, Ref},
};

use self::util::{
    find_nearest_color, get_convert_color_type, white_point_type, InputUser, WhitePointSource,
};

/// Describes the struct to implement `FromColorUnclamped` for.
#[non_exhaustive]
pub struct StructInfo {
    /// The struct's type parameters. Empty by default.
    pub generics: Generics,

    /// The name of the struct.
    pub name: Ident,

    /// The type of the color component fields.
    ///
    /// The component type would be `f32` in this example:
    ///
    ///```
    /// struct MyRgb {
    ///     red: f32,
    ///     green: f32,
    ///     blue: f32,
    /// }
    /// ```
    pub component: Type,

    /// Color meta types that may not be possible to infer.
    pub color_meta: ColorMeta,

    /// The name and type of the color's alpha component, if it's internal.
    pub alpha_field: Option<(IdentOrIndex, Type)>,

    /// Base type derives to skip.
    ///
    /// Add the names of color types with manually implemented conversions.
    /// This avoids adding duplicate implementations for those types. The
    /// default is only `Xyz`.
    pub skip_derives: HashSet<String>,

    /// Color group.
    ///
    /// Specifies color groups this color type can convert directly to and
    /// from without any additional input. The default is the `XYZ_COLORS`.
    pub color_groups: HashSet<Ref<'static, ColorGroup>>,

    is_base_type: bool,
}

impl StructInfo {
    /// Create a new `StructInfo`.
    ///
    /// See the documentation for each field for default values.
    pub fn new(name: Ident, component: Type, color_meta: ColorMeta) -> Self {
        Self {
            name,
            component,
            color_meta,
            generics: Generics::default(),
            alpha_field: None,
            skip_derives: HashSet::from_iter([XYZ_COLORS.root_type.name.into()]),
            color_groups: HashSet::from_iter([(&XYZ_COLORS).into()]),
            is_base_type: false,
        }
    }
}

impl<'a> TryFrom<&'a ColorInfo> for StructInfo {
    type Error = StructInfoError;

    fn try_from(value: &'a ColorInfo) -> std::result::Result<Self, Self::Error> {
        let &ColorInfo {
            name,
            generics: ColorGenerics { component, meta },
            ref from_color_unclamped,
            ..
        } = value;

        let Some(from_color_unclamped) = from_color_unclamped else {
            return Err(StructInfoError::CannotImplement);
        };

        let mut generics = Generics::default();

        if let Some(meta) = meta {
            let meta = Ident::new(meta, Span::call_site());
            generics.params.push(parse_quote!(#meta));
        }

        let component = Ident::new(component, Span::call_site());
        generics.params.push(parse_quote!(#component));

        Ok(Self {
            name: Ident::new(name, Span::call_site()),
            component: parse_quote!(#component),
            color_meta: from_color_unclamped
                .color_meta
                .try_into()
                .expect("color_meta should contain valid syntax for types"),
            generics,
            alpha_field: None,
            skip_derives: HashSet::from_iter(
                from_color_unclamped
                    .skip_derives
                    .iter()
                    .map(|&color| String::from(color)),
            ),
            color_groups: HashSet::from_iter(
                from_color_unclamped
                    .color_groups
                    .iter()
                    .map(|&group| Ref::from(group)),
            ),
            is_base_type: true,
        })
    }
}

/// An error that may occur when creating `StructInfo` from `ColorInfo`.
#[non_exhaustive]
pub enum StructInfoError {
    /// `FromColorUnclamped` is not to be implemented for this type.
    CannotImplement,
}

/// Generate code for implementing `FromColorUnclamped` from the `palette`
/// base types.
///
/// * `struct_info` describes the struct that implements `FromColorUnclamped`.
/// * `palette_name` is the name of the `palette` crate in the generated code.
///   Usually `palette` unless it's renamed in `Cargo.toml`.
pub fn derive(struct_info: &StructInfo, palette_name: &Ident) -> TokenStream {
    let (all_from_impl_params, impl_params_errors) = prepare_from_impl(struct_info, palette_name);

    let mut implementations =
        generate_from_implementations(struct_info, &all_from_impl_params, palette_name);

    if let Some((alpha_property, alpha_type)) = struct_info.alpha_field.as_ref() {
        implementations.push(generate_from_alpha_implementation_with_internal(
            &struct_info.name,
            &struct_info.generics,
            alpha_property,
            alpha_type,
            palette_name,
        ));
    } else {
        implementations.push(generate_from_alpha_implementation(
            &struct_info.name,
            &struct_info.generics,
            palette_name,
        ));
    }

    let impl_params_errors = impl_params_errors
        .into_iter()
        .map(|error| error.into_compile_error());

    quote! {
        #(#impl_params_errors)*

        #(#implementations)*
    }
}

fn prepare_from_impl(
    struct_info: &StructInfo,
    palette_name: &Ident,
) -> (Vec<FromImplParameters>, Vec<syn::Error>) {
    let white_point = white_point_type(
        struct_info.color_meta.white_point.as_ref(),
        struct_info.color_meta.rgb_standard.as_ref(),
        struct_info.color_meta.luma_standard.as_ref(),
        palette_name,
    );

    let included_colors = struct_info
        .color_groups
        .iter()
        .flat_map(|group| group.color_names())
        .filter(|&color| !struct_info.skip_derives.contains(color.name));

    let mut parameters = Vec::new();
    let mut errors = Vec::new();

    for color in included_colors {
        let impl_params =
            prepare_from_impl_for_pair(color, struct_info, white_point.clone(), palette_name);

        match impl_params {
            Ok(Some(impl_params)) => parameters.push(impl_params),
            Ok(None) => {}
            Err(error) => errors.push(error),
        }
    }

    (parameters, errors)
}

fn prepare_from_impl_for_pair(
    color: &ColorInfo,
    struct_info: &StructInfo,
    white_point: Option<(Type, WhitePointSource)>,
    palette_name: &Ident,
) -> Result<Option<FromImplParameters>> {
    let mut generics = struct_info.generics.clone();
    let component = &struct_info.component;

    let nearest_color =
        find_nearest_color(color, &struct_info.skip_derives, &struct_info.color_groups)?;

    // Figures out which white point the target type prefers, unless it's specified in `white_point`.
    let (white_point, white_point_source) = if let Some((white_point, source)) = white_point {
        (white_point, source)
    } else {
        color.get_default_white_point(palette_name)
    };

    let (color_ty, mut used_input) = get_convert_color_type(
        color,
        &white_point,
        component,
        &struct_info.color_meta,
        &mut generics,
        palette_name,
    )?;

    let nearest_color_ty = nearest_color.get_type(
        MetaTypeSource::OtherColor(color),
        component,
        &white_point,
        &mut used_input,
        InputUser::Nearest,
        &struct_info.color_meta,
        palette_name,
    )?;

    // Skip implementing the trait where it wouldn't be able to constrain the
    // white point. This is only happening when certain optional features are
    // enabled.
    if used_input.white_point.is_unconstrained()
        && matches!(white_point_source, WhitePointSource::GeneratedGeneric)
    {
        return Ok(None);
    }

    if used_input.white_point.is_used() {
        match white_point_source {
            WhitePointSource::WhitePoint => {
                let white_point_path =
                    crate::util::path(["white_point", "WhitePoint"], palette_name);
                generics
                    .make_where_clause()
                    .predicates
                    .push(parse_quote!(#white_point: #white_point_path<#component>))
            }
            WhitePointSource::RgbStandard => {
                let rgb_standard_path = crate::util::path(["rgb", "RgbStandard"], palette_name);
                let rgb_standard = struct_info.color_meta.rgb_standard.as_ref();
                generics
                    .make_where_clause()
                    .predicates
                    .push(parse_quote!(#rgb_standard: #rgb_standard_path));
            }
            WhitePointSource::LumaStandard => {
                let luma_standard_path = crate::util::path(["luma", "LumaStandard"], palette_name);
                let luma_standard = struct_info.color_meta.luma_standard.as_ref();
                generics
                    .make_where_clause()
                    .predicates
                    .push(parse_quote!(#luma_standard: #luma_standard_path));
            }
            WhitePointSource::ConcreteType => {}
            WhitePointSource::GeneratedGeneric => {
                generics.params.push(parse_quote!(_Wp));
            }
        }
    }

    Ok(Some(FromImplParameters {
        generics,
        color_ty,
        nearest_color_ty,
    }))
}

struct FromImplParameters {
    generics: Generics,
    color_ty: Type,
    nearest_color_ty: Type,
}

fn generate_from_implementations(
    struct_info: &StructInfo,
    all_parameters: &[FromImplParameters],
    palette_name: &Ident,
) -> Vec<TokenStream> {
    let from_trait_path = crate::util::path(["convert", "FromColorUnclamped"], palette_name);
    let into_trait_path = crate::util::path(["convert", "IntoColorUnclamped"], palette_name);

    let (_, type_generics, _) = struct_info.generics.split_for_impl();
    let ident = &struct_info.name;

    let mut implementations = Vec::with_capacity(all_parameters.len());

    for parameters in all_parameters {
        let FromImplParameters {
            color_ty,
            generics,
            nearest_color_ty,
        } = parameters;

        {
            let mut generics = generics.clone();

            {
                let where_clause = generics.make_where_clause();
                where_clause
                    .predicates
                    .push(parse_quote!(#nearest_color_ty: #from_trait_path<#color_ty>));
                where_clause
                    .predicates
                    .push(parse_quote!(#nearest_color_ty: #into_trait_path<Self>));
            }

            let (impl_generics, _, where_clause) = generics.split_for_impl();

            implementations.push(quote! {
                #[automatically_derived]
                impl #impl_generics #from_trait_path<#color_ty> for #ident #type_generics #where_clause {
                    #[inline]
                    fn from_color_unclamped(color: #color_ty) -> Self {
                        use #into_trait_path;
                        #nearest_color_ty::from_color_unclamped(color).into_color_unclamped()
                    }
                }
            });
        }

        if !struct_info.is_base_type {
            let mut generics = generics.clone();

            {
                let where_clause = generics.make_where_clause();
                where_clause
                    .predicates
                    .push(parse_quote!(#nearest_color_ty: #from_trait_path<#ident #type_generics>));
                where_clause
                    .predicates
                    .push(parse_quote!(#nearest_color_ty: #into_trait_path<Self>));
            }

            let (impl_generics, _, where_clause) = generics.split_for_impl();

            implementations.push(quote! {
                #[automatically_derived]
                impl #impl_generics #from_trait_path<#ident #type_generics> for #color_ty #where_clause {
                    #[inline]
                    fn from_color_unclamped(color: #ident #type_generics) -> Self {
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
    palette_name: &Ident,
) -> TokenStream {
    let from_trait_path = crate::util::path(["convert", "FromColorUnclamped"], palette_name);
    let into_trait_path = crate::util::path(["convert", "IntoColorUnclamped"], palette_name);
    let alpha_path = crate::util::path(["Alpha"], palette_name);

    let mut impl_generics = generics.clone();
    impl_generics.params.push(parse_quote!(_C));
    impl_generics.params.push(parse_quote!(_A));
    {
        let where_clause = impl_generics.make_where_clause();
        where_clause
            .predicates
            .push(parse_quote!(_C: #into_trait_path<Self>));
    }

    let (_, type_generics, _) = generics.split_for_impl();
    let (self_impl_generics, _, self_where_clause) = impl_generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #self_impl_generics #from_trait_path<#alpha_path<_C, _A>> for #ident #type_generics #self_where_clause {
            #[inline]
            fn from_color_unclamped(color: #alpha_path<_C, _A>) -> Self {
                color.color.into_color_unclamped()
            }
        }
    }
}

fn generate_from_alpha_implementation_with_internal(
    ident: &Ident,
    generics: &Generics,
    alpha_property: &IdentOrIndex,
    alpha_type: &Type,
    palette_name: &Ident,
) -> TokenStream {
    let from_trait_path = crate::util::path(["convert", "FromColorUnclamped"], palette_name);
    let into_trait_path = crate::util::path(["convert", "IntoColorUnclamped"], palette_name);
    let alpha_path = crate::util::path(["Alpha"], palette_name);

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
            #[inline]
            fn from_color_unclamped(color: #alpha_path<_C, #alpha_type>) -> Self {
                use #into_trait_path;

                let #alpha_path { color, alpha } = color;

                let mut result: Self = color.into_color_unclamped();
                result.#alpha_property = alpha;

                result
            }
        }
    }
}
