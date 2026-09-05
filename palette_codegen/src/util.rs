//! Various utilities.

use std::{hash::Hash, ops::Deref};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Ident, Index, Type};

/// Construct a token stream containing the described path.
///
/// `palette_name` is the name of the `palette` crate at the call site.
/// Usually `palette` unless it's renamed in `Cargo.toml`.
pub(crate) fn path<'a, P: AsRef<[&'a str]>>(path: P, palette_name: &Ident) -> TokenStream {
    let path = path
        .as_ref()
        .iter()
        .map(|&ident| Ident::new(ident, Span::call_site()));

    quote! {#palette_name::#(#path)::*}
}

/// Construct a `Type` containing the described path.
///
/// `palette_name` is the name of the `palette` crate at the call site.
/// Usually `palette` unless it's renamed in `Cargo.toml`.
pub(crate) fn path_type(path: &[&str], palette_name: &Ident) -> Type {
    let path = path
        .iter()
        .map(|&ident| Ident::new(ident, Span::call_site()));

    parse_quote! {#palette_name::#(#path)::*}
}

/// A helper for reference equality.
#[repr(transparent)]
pub struct Ref<'a, T>(&'a T);

impl<'a, T> PartialEq for Ref<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl<'a, T> Eq for Ref<'a, T> {}

impl<'a, T> Hash for Ref<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const T).hash(state);
    }
}

impl<'a, T> Copy for Ref<'a, T> {}

impl<'a, T> Clone for Ref<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = &'a T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> From<&'a T> for Ref<'a, T> {
    fn from(value: &'a T) -> Self {
        Self(value)
    }
}

/// Either a tuple index or a struct field name.
#[derive(Clone)]
pub enum IdentOrIndex {
    /// A tuple index.
    Index(Index),

    /// A struct field name.
    Ident(Ident),
}

impl PartialEq for IdentOrIndex {
    fn eq(&self, other: &IdentOrIndex) -> bool {
        match (self, other) {
            (IdentOrIndex::Index(this), IdentOrIndex::Index(other)) => this.index == other.index,
            (IdentOrIndex::Ident(this), IdentOrIndex::Ident(other)) => this == other,
            _ => false,
        }
    }
}

impl Eq for IdentOrIndex {}

impl ::std::hash::Hash for IdentOrIndex {
    fn hash<H: ::std::hash::Hasher>(&self, hasher: &mut H) {
        ::std::mem::discriminant(self).hash(hasher);

        match *self {
            IdentOrIndex::Index(ref index) => index.index.hash(hasher),
            IdentOrIndex::Ident(ref ident) => ident.hash(hasher),
        }
    }
}

impl ::quote::ToTokens for IdentOrIndex {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match *self {
            IdentOrIndex::Index(ref index) => index.to_tokens(tokens),
            IdentOrIndex::Ident(ref ident) => ident.to_tokens(tokens),
        }
    }
}
