use syn::{Attribute, Ident, LitStr};
use syn::token::{Comma, Eq};
use syn::punctuated::Punctuated;
use syn::synom::Synom;
use proc_macro2::TokenStream;
use quote::ToTokens;

pub fn parse_attributes<P: MetaParser>(attributes: Vec<Attribute>) -> P::Output {
    let mut result = P::Output::default();

    for attribute in attributes {
        let attribute_name = attribute.path.segments.first().unwrap().into_value().ident;
        if !attribute_name.as_ref().starts_with("palette_") {
            continue;
        }

        if attribute.path.segments.len() > 1 {
            panic!(
                "expected `{}`, but found `{}`",
                attribute_name,
                attribute.path.into_tokens()
            );
        }

        if attribute_name == "palette_internal" {
            if attribute.tts.is_empty() {
                result.internal()
            } else {
                panic!(
                    "expected the attribute to be on the form `#[palette_internal]`, but found `{}`",
                    attribute.into_tokens()
                );
            }
        } else {
            P::parse_attribute(&mut result, attribute_name, attribute.tts);
        }
    }

    result
}

pub fn parse_type_tuple_attribute<T: Synom>(
    attribute_name: &Ident,
    tts: TokenStream,
) -> Punctuated<T, Comma> {
    struct GenericTuple<T>(Punctuated<T, Comma>);

    impl<T: Synom> Synom for GenericTuple<T> {
        named!(parse -> Self, do_parse!(
            tuple: parens!(call!(Punctuated::parse_separated_nonempty)) >>
            (GenericTuple(tuple.1))
        ));
    }

    match ::syn::parse2::<GenericTuple<T>>(tts.clone()) {
        Ok(elements) => elements.0,
        Err(_) => panic!(
            "expected the attribute to be on the form `#[{name}(A, B, ...)]`, but found #[{name}{tts}]",
            name = attribute_name,
            tts = tts
        ),
    }
}

pub fn parse_equal_attribute<T: Synom>(attribute_name: &Ident, tts: TokenStream) -> T {
    struct Paren<T>(T);

    impl<T: Synom> Synom for Paren<T> {
        named!(parse -> Self, do_parse!(
            _eq: syn!(Eq) >>
            content: syn!(StringOrValue<T>) >>
            result: switch!(value!(content),
                StringOrValue::Value(value) => value!(value) |
                StringOrValue::String(string) => call!(parse_string, string)
            ) >>
            (Paren(result))
        ));
    }

    enum StringOrValue<T> {
        String(String),
        Value(T),
    }

    impl<T: Synom> Synom for StringOrValue<T> {
        named!(parse -> Self, alt!(
            syn!(T) => {StringOrValue::Value} |
            syn!(LitStr) => {|lit| StringOrValue::String(lit.value())}
        ));
    }

    fn parse_string<T: Synom>(
        cursor: ::syn::buffer::Cursor,
        string: String,
    ) -> ::syn::synom::PResult<T> {
        ::syn::parse2(string.parse().unwrap()).map(|value| (value, cursor))
    }

    match ::syn::parse2::<Paren<T>>(tts.clone()) {
        Ok(assign) => assign.0,
        Err(_) => panic!(
            "expected the attribute to be on the form `#[{name} = A]` or `#[{name} = \"A\"]`, but found #[{name}{tts}]",
            name = attribute_name,
            tts = tts
        ),
    }
}

pub trait MetaParser {
    type Output: MetaOutput;

    fn parse_attribute(
        output: &mut Self::Output,
        attribute_name: Ident,
        attribute_tts: TokenStream,
    );
}

pub trait MetaOutput: Default {
    fn internal(&mut self);
}
