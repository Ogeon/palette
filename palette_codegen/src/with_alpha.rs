//! Code generator for implementing `WithAlpha`.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Generics, Ident, Type};

use crate::{
    color_types::{ColorGenerics, ColorInfo},
    util::{self, IdentOrIndex},
};

/// Describes the struct to implement `WithAlpha` for.
#[non_exhaustive]
pub struct StructInfo {
    /// The struct's type parameters. Empty by default.
    pub generics: Generics,

    /// The name of the struct.
    pub name: Ident,

    /// The name and type of the color's alpha component, if it's internal.
    pub alpha_field: Option<(IdentOrIndex, Type)>,
}

impl StructInfo {
    /// Create a new `StructInfo`.
    ///
    /// See the documentation for each field for default values.
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            generics: Generics::default(),
            alpha_field: None,
        }
    }
}

impl<'a> From<&'a ColorInfo> for StructInfo {
    fn from(value: &'a ColorInfo) -> Self {
        let &ColorInfo {
            name,
            generics: ColorGenerics { component, meta },
            ..
        } = value;

        let mut generics = Generics::default();

        if let Some(meta) = meta {
            let meta = Ident::new(meta, Span::call_site());
            generics.params.push(parse_quote!(#meta));
        }

        let component = Ident::new(component, Span::call_site());
        generics.params.push(parse_quote!(#component));

        Self {
            generics,
            name: Ident::new(name, Span::call_site()),
            alpha_field: None,
        }
    }
}

/// Generate code for implementing `WithAlpha`.
///
/// * `struct_info` describes the struct that implements `WithAlpha`.
/// * `palette_name` is the name of the `palette` crate in the generated code.
///   Usually `palette` unless it's renamed in `Cargo.toml`.
pub fn derive(struct_info: &StructInfo, palette_name: &Ident) -> TokenStream {
    let StructInfo {
        name,
        generics,
        alpha_field,
    } = struct_info;

    if let Some((alpha_property, alpha_type)) = alpha_field {
        implement_for_internal_alpha(name, generics, alpha_property, alpha_type, palette_name)
    } else {
        implement_for_external_alpha(name, generics, palette_name)
    }
}

fn implement_for_internal_alpha(
    ident: &Ident,
    generics: &Generics,
    alpha_property: &IdentOrIndex,
    alpha_type: &Type,
    palette_name: &Ident,
) -> TokenStream {
    let with_alpha_trait_path = util::path(["WithAlpha"], palette_name);
    let stimulus_trait_path = util::path(["stimulus", "Stimulus"], palette_name);

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics #with_alpha_trait_path<#alpha_type> for #ident #type_generics #where_clause {
            type Color = Self;
            type WithAlpha = Self;

            #[inline]
            fn with_alpha(mut self, alpha: #alpha_type) -> Self::WithAlpha {
                self.#alpha_property = alpha;
                self
            }

            #[inline]
            fn without_alpha(mut self) -> Self::Color {
                self.#alpha_property = #stimulus_trait_path::max_intensity();
                self
            }

            #[inline]
            fn split(mut self) -> (Self::Color, #alpha_type) {
                let opaque_alpha = #stimulus_trait_path::max_intensity();
                let alpha = core::mem::replace(&mut self.#alpha_property, opaque_alpha);
                (self, alpha)
            }
        }
    }
}

fn implement_for_external_alpha(
    ident: &Ident,
    generics: &Generics,
    palette_name: &Ident,
) -> TokenStream {
    let with_alpha_trait_path = util::path(["WithAlpha"], palette_name);
    let stimulus_trait_path = util::path(["stimulus", "Stimulus"], palette_name);
    let alpha_path = util::path(["Alpha"], palette_name);

    let (_, type_generics, _) = generics.split_for_impl();

    let alpha_type: Type = parse_quote!(_A);
    let mut impl_generics = generics.clone();
    impl_generics.params.push(parse_quote!(_A));
    impl_generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(_A: #stimulus_trait_path));
    let (impl_generics, _, where_clause) = impl_generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics #with_alpha_trait_path<#alpha_type> for #ident #type_generics #where_clause {
            type Color = Self;
            type WithAlpha = #alpha_path<Self, #alpha_type>;

            #[inline]
            fn with_alpha(self, alpha: #alpha_type) -> Self::WithAlpha {
                #alpha_path {
                    color: self,
                    alpha
                }
            }

            #[inline]
            fn without_alpha(self) -> Self::Color {
                self
            }

            #[inline]
            fn split(self) -> (Self::Color, #alpha_type) {
                (self, #stimulus_trait_path::max_intensity())
            }
        }
    }
}
