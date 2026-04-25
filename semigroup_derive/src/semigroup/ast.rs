use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Data, DataEnum, DataUnion, DeriveInput};

use crate::{
    constant::Constant,
    error::SemigroupError,
    semigroup::{
        ast::struct_semigroup::{StructAnnotated, StructSemigroup},
        attr::ContainerAttr,
    },
};

pub mod field_semigroup;
pub mod struct_semigroup;

#[derive(Debug, Clone)]
pub struct Semigroup<'a> {
    semigroup: StructSemigroup<'a>,
    annotated: Option<StructAnnotated<'a>>,
}
impl ToTokens for Semigroup<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.semigroup.to_tokens(tokens);
        self.annotated.to_tokens(tokens);
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
                let semigroup = StructSemigroup::new(constant, derive, attr, data_struct)?;
                let annotated = attr
                    .is_annotated()
                    .then(|| StructAnnotated::new(constant, derive, attr, data_struct))
                    .transpose()?;
                Ok(Self {
                    semigroup,
                    annotated,
                })
            }
            Data::Union(DataUnion { union_token, .. }) => Err(syn::Error::new_spanned(
                union_token,
                SemigroupError::UnsupportedUnion,
            )),
        }
    }
}
