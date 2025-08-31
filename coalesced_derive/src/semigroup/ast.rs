use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, Data, DataEnum, DataStruct, DataUnion, DeriveInput, FieldValue, Fields, ItemImpl,
    Member,
};

use crate::{
    constant::Constant,
    error::SemigroupError,
    semigroup::attr::{ContainerAttr, FieldAttr},
};

#[derive(Debug, Clone)]
pub struct Semigroup<'a> {
    derive: &'a DeriveInput,
    struct_semigroup: StructSemigroup<'a>,
}
impl ToTokens for Semigroup<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            struct_semigroup, ..
        } = self;
        tokens.extend(quote::quote! {
            #struct_semigroup
        });
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
                Ok(Self {
                    derive,
                    struct_semigroup,
                })
            }
            Data::Union(DataUnion { union_token, .. }) => Err(syn::Error::new_spanned(
                union_token,
                SemigroupError::UnsupportedUnion,
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructSemigroup<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,
    attr: &'a ContainerAttr,
    data_struct: &'a DataStruct,
}
impl ToTokens for StructSemigroup<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_semigroup()
            .as_ref()
            .map(ToTokens::to_token_stream)
            .unwrap_or_else(syn::Error::to_compile_error)
            .to_tokens(tokens)
    }
}
impl<'a> StructSemigroup<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
        data_struct: &'a DataStruct,
    ) -> syn::Result<Self> {
        Ok(Self {
            constant,
            derive,
            data_struct,
            attr,
        })
    }
    pub fn impl_semigroup(&self) -> syn::Result<ItemImpl> {
        let Self {
            constant,
            derive,
            attr,
            data_struct,
            ..
        } = self;
        let Constant {
            path_semigroup,
            ident_semigroup_op,
            ..
        } = constant;
        let DeriveInput { ident, .. } = derive;
        let fields = FieldSemigroupOp::new_fields(constant, derive, attr, &data_struct.fields)?;
        Ok(parse_quote! {
            impl #path_semigroup for #ident {
                fn #ident_semigroup_op(base: Self, other: Self) -> Self {
                    Self {
                        #(#fields),*
                    }
                }
            }
        })
    }
}

pub struct FieldSemigroupOp<'a> {
    constant: &'a Constant,
    container_attr: &'a ContainerAttr,
    member: Member,
    field_attr: FieldAttr,
}
impl ToTokens for FieldSemigroupOp<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_semigroup_field().to_tokens(tokens)
    }
}
impl<'a> FieldSemigroupOp<'a> {
    pub fn new(
        constant: &'a Constant,
        _derive: &'a DeriveInput,
        container_attr: &'a ContainerAttr,
        member: Member,
        field_attr: FieldAttr,
    ) -> Self {
        Self {
            constant,
            container_attr,
            member,
            field_attr,
        }
    }
    pub fn new_fields(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        container_attr: &'a ContainerAttr,
        fields: &'a Fields,
    ) -> syn::Result<Vec<Self>> {
        fields
            .iter()
            .zip(fields.members())
            .map(|(field, member)| {
                Ok(Self::new(
                    constant,
                    derive,
                    container_attr,
                    member,
                    FieldAttr::new(field)?,
                ))
            })
            .collect()
    }

    pub fn impl_semigroup_field(&self) -> FieldValue {
        let Self {
            constant:
                Constant {
                    path_semigroup,
                    ident_semigroup_op,
                    path_construction_trait,
                    ..
                },
            container_attr,
            member,
            field_attr,
            ..
        } = self;
        let with = field_attr.with(container_attr);
        with.map(|path| {
                parse_quote! {
                    #member: <#path<_> as #path_construction_trait<_>>::lift_op(base.#member, other.#member)
                }
            })
            .unwrap_or_else(|| {
                parse_quote! {
                    #member: #path_semigroup::#ident_semigroup_op(base.#member, other.#member)
                }
            })
    }
}
