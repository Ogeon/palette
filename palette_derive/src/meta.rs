use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Parser, Result};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Attribute, Data, Field, Fields, Ident, Index, LitStr, Token, Type};

pub fn parse_attributes<T: MetaParser>(attributes: Vec<Attribute>) -> T {
    let mut result = T::default();

    for attribute in attributes {
        let attribute_name = attribute.path.segments.first().unwrap().ident.clone();
        let is_palette_attribute = attribute_name.to_string().starts_with("palette_");

        if attribute.path.segments.len() > 1 {
            if is_palette_attribute {
                panic!(
                    "expected `{}`, but found `{}`",
                    attribute_name,
                    attribute.path.into_token_stream()
                );
            } else {
                continue;
            }
        }

        if attribute_name == "palette_internal" {
            assert_empty_attribute(&attribute_name, attribute.tokens);
            result.internal();
        } else {
            result.parse_attribute(attribute_name, attribute.tokens);
        }
    }

    result
}

pub fn parse_data_attributes<T: DataMetaParser>(data: Data) -> T {
    let mut result = T::default();

    match data {
        Data::Struct(struct_item) => {
            let fields = match struct_item.fields {
                Fields::Named(fields) => fields.named,
                Fields::Unnamed(fields) => fields.unnamed,
                Fields::Unit => Default::default(),
            };

            parse_struct_field_attributes(&mut result, fields)
        }
        Data::Enum(_) => {}
        Data::Union(_) => {}
    }

    result
}

pub fn parse_struct_field_attributes<T: DataMetaParser>(
    parser: &mut T,
    fields: Punctuated<Field, Token![,]>,
) {
    for (index, field) in fields.into_iter().enumerate() {
        let identifier = field
            .ident
            .map(IdentOrIndex::Ident)
            .unwrap_or_else(|| IdentOrIndex::Index(index.into()));

        for attribute in field.attrs {
            let attribute_name = attribute.path.segments.first().unwrap().ident.clone();
            if !attribute_name.to_string().starts_with("palette_") {
                continue;
            }

            if attribute.path.segments.len() > 1 {
                panic!(
                    "expected `{}`, but found `{}`",
                    attribute_name,
                    attribute.path.into_token_stream()
                );
            }

            parser.parse_struct_field_attribute(
                identifier.clone(),
                field.ty.clone(),
                attribute_name,
                attribute.tokens,
            );
        }
    }
}

pub fn assert_empty_attribute(attribute_name: &Ident, tts: TokenStream) {
    if !tts.is_empty() {
        panic!(
            "expected the attribute to be on the form `#[{name}]`, but found `#[{name}{tts}]`",
            name = attribute_name,
            tts = tts
        );
    }
}

pub fn parse_tuple_attribute<T: Parse>(attribute_name: &Ident, tts: TokenStream) -> Vec<T> {
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

    match parse_generic_tuple.parse2(tts.clone()) {
        Ok(elements) => elements,
        Err(_) => panic!(
            "expected the attribute to be on the form `#[{name}(A, B, ...)]`, but found #[{name}{tts}]",
            name = attribute_name,
            tts = tts
        ),
    }
}

pub fn parse_equal_attribute<T: Parse>(attribute_name: &Ident, tts: TokenStream) -> T {
    fn parse_paren<T: Parse>(input: ParseStream) -> Result<T> {
        input.parse::<Token![=]>()?;
        if input.peek(LitStr) {
            input.parse::<LitStr>()?.parse()
        } else {
            input.parse()
        }
    }

    match parse_paren::<T>.parse2(tts.clone()) {
        Ok(assign) => assign,
        Err(_) => panic!(
            "expected the attribute to be on the form `#[{name} = A]` or `#[{name} = \"A\"]`, but found #[{name}{tts}]",
            name = attribute_name,
            tts = tts
        ),
    }
}

#[derive(PartialEq)]
pub struct KeyValuePair {
    pub key: Ident,
    pub value: Option<Ident>,
}

impl Parse for KeyValuePair {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        let option_eq: Option<Token![=]> = input.parse()?;
        let value = match option_eq {
            None => None,
            Some(_) => Some(input.parse::<LitStr>()?.parse::<Ident>()?),
        };
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
    fn parse_attribute(&mut self, attribute_name: Ident, attribute_tts: TokenStream);
}

pub trait DataMetaParser: Default {
    fn parse_struct_field_attribute(
        &mut self,
        field_name: IdentOrIndex,
        ty: Type,
        attribute_name: Ident,
        attribute_tts: TokenStream,
    );
}
