use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Data, DataEnum, DataStruct, DataUnion, DeriveInput, Field, Fields, FieldsUnnamed};

use crate::{
    constant::Constant,
    error::ConstructionError,
    op::{
        ast::{construction_trait::ConstructionTrait, op_trait::OpTrait, trait_impl::TraitImpl},
        attr::ContainerAttr,
    },
};

pub mod construction_trait;
pub mod op_trait;
pub mod trait_impl;

#[derive(Debug, Clone)]
pub struct Op<'a> {
    construction_trait: Option<ConstructionTrait<'a>>,
    op_trait: Option<OpTrait<'a>>,
    trait_impl: TraitImpl<'a>,
}
impl ToTokens for Op<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { construction_trait, op_trait, trait_impl } = self;
        construction_trait.to_tokens(tokens);
        op_trait.to_tokens(tokens);
        trait_impl.to_tokens(tokens);
    }
}
impl<'a> Op<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
    ) -> syn::Result<Self> {
        let construction_trait = attr
            .open_inner()
            .then(|| ConstructionTrait::new(constant, derive, attr, Self::newtype_field(derive)?))
            .transpose()?;
        let op_trait = attr
            .open_inner()
            .then(|| OpTrait::new(constant, derive, attr, Self::newtype_field(derive)?))
            .transpose()?;
        let trait_impl = TraitImpl::new(constant, derive, attr);
        Ok(Self {
            construction_trait,
            op_trait,
            trait_impl,
        })
    }

    pub fn newtype_field(derive: &'a DeriveInput) -> syn::Result<&'a Field> {
        match &derive.data {
            Data::Struct(DataStruct {
                fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }),
                ..
            }) if unnamed.len() == 1 => {
                // if let [field] = unnamed => // match: `if let` guards are experimental see issue #51114 https://github.com/rust-lang/rust/issues/51114
                if let &[field] = unnamed.iter().collect::<Vec<_>>().as_slice() {
                    Ok(field)
                } else {
                    unreachable!()
                }
            }
            Data::Enum(DataEnum { enum_token, .. }) => Err(syn::Error::new_spanned(
                enum_token,
                ConstructionError::ExpectNewType,
            )),
            Data::Struct(DataStruct { struct_token, .. }) => Err(syn::Error::new_spanned(
                struct_token,
                ConstructionError::ExpectNewType,
            )),
            Data::Union(DataUnion { union_token, .. }) => Err(syn::Error::new_spanned(
                union_token,
                ConstructionError::ExpectNewType,
            )),
        }
    }
}
