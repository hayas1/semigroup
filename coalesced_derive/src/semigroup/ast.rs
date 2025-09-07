use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Data, DataEnum, DataUnion, DeriveInput};

use crate::{
    constant::Constant,
    error::SemigroupError,
    semigroup::{
        ast::struct_semigroup::{StructAnnotate, StructSemigroup},
        attr::ContainerAttr,
    },
};

pub mod field_semigroup;
pub mod struct_semigroup;

#[derive(Debug, Clone)]
pub enum Semigroup<'a> {
    Struct {
        struct_semigroup: StructSemigroup<'a>,
        struct_annotate: Option<StructAnnotate<'a>>,
    },
}
impl ToTokens for Semigroup<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Struct {
                struct_semigroup,
                struct_annotate,
            } => {
                struct_semigroup.to_tokens(tokens);
                struct_annotate.iter().for_each(|s| s.to_tokens(tokens));
            }
        }
    }
}
impl<'a> Semigroup<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
    ) -> syn::Result<Self> {
        match &derive.data {
            Data::Enum(DataEnum { enum_token, .. }) => Err(syn::Error::new_spanned(
                enum_token,
                SemigroupError::UnsupportedEnum,
            )),
            Data::Struct(data_struct) => {
                let struct_semigroup = StructSemigroup::new(constant, derive, attr, data_struct)?;
                let struct_annotate = attr
                    .is_annotated()
                    .then(|| StructAnnotate::new(constant, derive, attr, data_struct));
                Ok(Self::Struct {
                    struct_semigroup,
                    struct_annotate,
                })
            }
            Data::Union(DataUnion { union_token, .. }) => Err(syn::Error::new_spanned(
                union_token,
                SemigroupError::UnsupportedUnion,
            )),
        }
    }
}
