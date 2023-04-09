use std::collections::{HashMap, HashSet};

use syn::{spanned::Spanned, Expr, ExprLit};
use syn::{Lit, Meta, MetaNameValue, Result, Type};

use super::{assert_path_meta, FieldAttributeArgumentParser, IdentOrIndex};

#[derive(Default)]
pub struct FieldAttributes {
    pub alpha_property: Option<(IdentOrIndex, Type)>,
    pub zero_size_fields: HashSet<IdentOrIndex>,
    pub type_substitutes: HashMap<IdentOrIndex, Type>,
}

impl FieldAttributeArgumentParser for FieldAttributes {
    fn argument(&mut self, field_name: &IdentOrIndex, ty: &Type, argument: Meta) -> Result<()> {
        let argument_name = argument.path().get_ident().map(ToString::to_string);

        match argument_name.as_deref() {
            Some("alpha") => {
                assert_path_meta(&argument)?;
                self.alpha_property = Some((field_name.clone(), ty.clone()));
            }
            Some("unsafe_same_layout_as") => {
                let substitute = if let Meta::NameValue(MetaNameValue {
                    value:
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(string),
                            ..
                        }),
                    ..
                }) = argument
                {
                    string.parse()?
                } else {
                    return Err(::syn::parse::Error::new(
                        argument.span(),
                        "expected `unsafe_same_layout_as = \"SomeType\"`",
                    ));
                };

                self.type_substitutes.insert(field_name.clone(), substitute);
            }
            Some("unsafe_zero_sized") => {
                assert_path_meta(&argument)?;
                self.zero_size_fields.insert(field_name.clone());
            }
            _ => {
                return Err(::syn::parse::Error::new(
                    argument.span(),
                    "unknown field attribute",
                ));
            }
        }

        Ok(())
    }
}
