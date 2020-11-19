use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseBuffer, ParseStream, Parser, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parenthesized, Attribute, Fields, Ident, Index, Lit, LitStr, Meta, NestedMeta, Token, Type,
};

pub use self::field_attributes::*;
pub use self::type_item_attributes::*;

mod field_attributes;
mod type_item_attributes;

pub fn parse_namespaced_attributes<T: AttributeArgumentParser>(
    attributes: Vec<Attribute>,
) -> ::std::result::Result<T, Vec<::syn::parse::Error>> {
    let mut result = T::default();
    let mut errors = Vec::new();

    for attribute in attributes {
        let is_palette_attribute = attribute
            .path
            .get_ident()
            .map(|name| name == "palette")
            .unwrap_or(false);

        if !is_palette_attribute {
            continue;
        }

        if attribute.tokens.is_empty() {
            errors.push(::syn::parse::Error::new(
                attribute.path.span(),
                "expected `palette(...)`",
            ));

            continue;
        }

        let parse_result = parse_meta_list.parse2(attribute.tokens);
        match parse_result {
            Ok(meta) => {
                for argument in meta {
                    let argument_result = match argument {
                        NestedMeta::Meta(argument) => result.argument(argument),
                        NestedMeta::Lit(literal) => result.literal(literal),
                    };

                    if let Err(error) = argument_result {
                        errors.push(error);
                    }
                }
            }
            Err(error) => errors.push(error),
        }
    }

    if errors.is_empty() {
        Ok(result)
    } else {
        Err(errors)
    }
}

pub fn parse_field_attributes<T: FieldAttributeArgumentParser>(
    fields: Fields,
) -> ::std::result::Result<T, Vec<::syn::parse::Error>> {
    let mut result = T::default();
    let mut errors = Vec::new();

    let attributes = fields.into_iter().enumerate().flat_map(|(index, field)| {
        let field_name = field
            .ident
            .map(IdentOrIndex::Ident)
            .unwrap_or_else(|| IdentOrIndex::Index(index.into()));
        let ty = field.ty;

        field
            .attrs
            .into_iter()
            .map(move |attribute| (field_name.clone(), ty.clone(), attribute))
    });

    for (field_name, ty, attribute) in attributes {
        let is_palette_attribute = attribute
            .path
            .get_ident()
            .map(|name| name == "palette")
            .unwrap_or(false);

        if !is_palette_attribute {
            continue;
        }

        if attribute.tokens.is_empty() {
            errors.push(::syn::parse::Error::new(
                attribute.path.span(),
                "expected `palette(...)`",
            ));

            continue;
        }

        let parse_result = parse_meta_list.parse2(attribute.tokens);
        match parse_result {
            Ok(meta) => {
                for argument in meta {
                    let argument_result = match argument {
                        NestedMeta::Meta(argument) => result.argument(&field_name, &ty, argument),
                        NestedMeta::Lit(literal) => result.literal(&field_name, &ty, literal),
                    };

                    if let Err(error) = argument_result {
                        errors.push(error);
                    }
                }
            }
            Err(error) => errors.push(error),
        }
    }

    if errors.is_empty() {
        Ok(result)
    } else {
        Err(errors)
    }
}

pub fn assert_path_meta(meta: &Meta) -> Result<()> {
    if !matches!(meta, Meta::Path(_)) {
        return Err(::syn::parse::Error::new(
            meta.span(),
            "expected the attribute to be just an identifier or a path",
        ));
    }

    Ok(())
}

pub fn parse_tuple_attribute<T: Parse>(tts: TokenStream) -> Result<Vec<T>> {
    fn parse_generic_tuple<T: Parse>(input: ParseStream) -> Result<Vec<T>> {
        let content;
        parenthesized!(content in input);

        let mut tuple = Vec::new();
        loop {
            tuple.push(content.parse()?);
            if content.is_empty() {
                break;
            }
            content.parse::<Token![,]>()?;
            if content.is_empty() {
                break;
            }
        }
        Ok(tuple)
    }

    parse_generic_tuple.parse2(tts)
}

fn parse_meta_list(buffer: &ParseBuffer) -> syn::Result<Punctuated<NestedMeta, Token![,]>> {
    let inner;
    parenthesized!(inner in buffer);
    syn::punctuated::Punctuated::parse_terminated(&inner)
}

#[derive(PartialEq)]
pub struct KeyValuePair {
    pub key: Ident,
    pub value: Ident,
}

impl Parse for KeyValuePair {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse::<LitStr>()?.parse::<Ident>()?;
        Ok(KeyValuePair { key, value })
    }
}

impl PartialEq<str> for KeyValuePair {
    fn eq(&self, other: &str) -> bool {
        self.key == other
    }
}

#[derive(Clone)]
pub enum IdentOrIndex {
    Index(Index),
    Ident(Ident),
}

impl PartialEq for IdentOrIndex {
    fn eq(&self, other: &IdentOrIndex) -> bool {
        match (self, other) {
            (&IdentOrIndex::Index(ref this), &IdentOrIndex::Index(ref other)) => {
                this.index == other.index
            }
            (&IdentOrIndex::Ident(ref this), &IdentOrIndex::Ident(ref other)) => this == other,
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

pub trait MetaParser: Default {
    fn internal(&mut self);
    fn parse_attribute(&mut self, attribute_name: Ident, attribute_tts: TokenStream) -> Result<()>;
}

pub trait DataMetaParser: Default {
    fn parse_struct_field_attribute(
        &mut self,
        field_name: IdentOrIndex,
        ty: Type,
        attribute_name: Ident,
        attribute_tts: TokenStream,
    ) -> Result<()>;
}

pub trait AttributeArgumentParser: Default {
    fn argument(&mut self, argument: Meta) -> Result<()>;

    fn literal(&mut self, literal: Lit) -> Result<()> {
        Err(::syn::parse::Error::new(
            literal.span(),
            "unexpected literal",
        ))
    }
}

pub trait FieldAttributeArgumentParser: Default {
    fn argument(&mut self, field_name: &IdentOrIndex, ty: &Type, argument: Meta) -> Result<()>;

    fn literal(&mut self, field_name: &IdentOrIndex, ty: &Type, literal: Lit) -> Result<()> {
        let (_, _) = (field_name, ty);

        Err(::syn::parse::Error::new(
            literal.span(),
            "unexpected literal",
        ))
    }
}
