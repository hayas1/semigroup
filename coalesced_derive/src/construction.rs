use std::fmt::{Display, Formatter};

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, Attribute, Data, DataStruct, DeriveInput, Field, Fields, FieldsUnnamed, Generics,
    Ident, ItemImpl,
};

use crate::error::ConstructionError;

pub struct Construction<'a> {
    construction: ConstructionAttr,
    ident: &'a Ident,
    generics: &'a Generics,
    field: &'a Field,
}
impl ToTokens for Construction<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let into_inner = self.impl_into_inner();
        tokens.extend(quote::quote! {
            #into_inner
        });
    }
}
impl<'a> Construction<'a> {
    pub fn new(derive: &'a DeriveInput) -> syn::Result<Self> {
        let DeriveInput {
            attrs,
            ident,
            generics,
            data,
            ..
        } = derive;
        match &data {
            Data::Struct(DataStruct {
                fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }),
                ..
            }) if unnamed.len() == 1 => {
                // if let [field] = unnamed => // match: `if let` guards are experimental see issue #51114 https://github.com/rust-lang/rust/issues/51114
                let &[field] = unnamed.iter().collect::<Vec<_>>().as_slice() else {
                    unreachable!()
                };
                Ok(Self {
                    construction: ConstructionAttr::new(attrs, ident)?,
                    ident,
                    generics,
                    field,
                })
            }
            Data::Enum(_) | Data::Struct(_) | Data::Union(_) => Err(syn::Error::new_spanned(
                ident,
                ConstructionError::OnlyNewType,
            )),
        }
    }

    pub fn impl_into_inner(&self) -> ItemImpl {
        let Self {
            ident,
            generics,
            field,
            ..
        } = self;
        let field_type = &field.ty;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        parse_quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                pub fn into_inner(self) -> #field_type {
                    self.0
                }
            }
        }
    }
}
pub enum ConstructionAttr {
    Annotated,
    Semigroup,
}
impl Display for ConstructionAttr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
impl AsRef<str> for ConstructionAttr {
    fn as_ref(&self) -> &str {
        match self {
            Self::Annotated => "annotated",
            Self::Semigroup => "semigroup",
        }
    }
}
impl ConstructionAttr {
    pub fn new(attrs: &[Attribute], ident: &Ident) -> syn::Result<Self> {
        let (annotated, semigroup) =
            attrs
                .iter()
                .fold((false, false), |(annotated, semigroup), attr| {
                    if attr.path().is_ident(&Self::Annotated) {
                        (true, semigroup)
                    } else if attr.path().is_ident(&Self::Semigroup) {
                        (annotated, true)
                    } else {
                        (annotated, semigroup)
                    }
                });
        match (annotated, semigroup) {
            (true, false) => Ok(Self::Annotated),
            (false, true) => Ok(Self::Semigroup),
            (false, false) => Err(syn::Error::new_spanned(
                ident,
                ConstructionError::ConstructionTypeNotFound,
            )),
            (true, true) => Err(syn::Error::new_spanned(
                ident,
                ConstructionError::DuplicateConstructionType,
            )),
        }
    }
}
