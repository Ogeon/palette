//! Code generator for implementing `ArrayCast`.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Generics, Ident, Type};

use crate::{
    color_types::{ColorGenerics, ColorInfo},
    util,
};

/// Describes the struct to implement `ArrayCast` for.
#[non_exhaustive]
pub struct StructInfo {
    /// The struct's type parameters. Empty by default.
    pub generics: syn::Generics,

    /// The name of the struct.
    pub name: Ident,

    /// The type of the color component fields. This will be the array's item
    /// type.
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

    /// The number of color component fields. This will be the array length.
    pub component_count: usize,
}

impl StructInfo {
    /// Create a new `StructInfo`.
    ///
    /// See the documentation for each field for default values.
    pub fn new(name: Ident, component: Type, component_count: usize) -> Self {
        Self {
            name,
            component,
            component_count,
            generics: Generics::default(),
        }
    }
}

impl<'a> TryFrom<&'a ColorInfo> for StructInfo {
    type Error = StructInfoError;

    fn try_from(value: &'a ColorInfo) -> Result<Self, Self::Error> {
        let &ColorInfo {
            name,
            generics: ColorGenerics { component, meta },
            ref array_cast,
            ..
        } = value;

        let Some(array_cast) = array_cast else {
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
            component_count: array_cast.component_count,
            generics,
        })
    }
}

/// An error that may occur when creating `StructInfo` from `ColorInfo`.
#[non_exhaustive]
pub enum StructInfoError {
    /// `ArrayCast` is not to be implemented for this type.
    CannotImplement,
}

/// Generate code for implementing `ArrayCast`.
///
/// * `struct_info` describes the struct that implements `ArrayCast`.
/// * `palette_name` is the name of the `palette` crate in the generated code.
///   Usually `palette` unless it's renamed in `Cargo.toml`.
///
/// # Safety
///
/// The struct described by `StructInfo` must be valid for implementing `ArrayCast`.
pub unsafe fn derive(struct_info: &StructInfo, palette_name: &Ident) -> TokenStream {
    let StructInfo {
        generics,
        name,
        component,
        component_count,
    } = struct_info;

    let array_cast_trait_path = util::path(["cast", "ArrayCast"], palette_name);

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote! {
        #[automatically_derived]
        unsafe impl #impl_generics #array_cast_trait_path for #name #type_generics #where_clause {
            type Array = [#component; #component_count];
        }
    }
}
