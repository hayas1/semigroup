use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Field, FieldValue, Fields,
    FieldsNamed, Ident, ItemImpl,
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
            ..
        } = self;
        match &self.data_struct.fields {
            Fields::Named(fields_named) => {
                StructNamedSemigroup::new(constant, derive, attr, fields_named).impl_semigroup()
            }
            Fields::Unnamed(fields_unnamed) => todo!(),
            Fields::Unit => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructNamedSemigroup<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,
    attr: &'a ContainerAttr,
    fields_named: &'a FieldsNamed,
}
impl ToTokens for StructNamedSemigroup<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_semigroup()
            .as_ref()
            .map(ToTokens::to_token_stream)
            .unwrap_or_else(syn::Error::to_compile_error)
            .to_tokens(tokens)
    }
}
impl<'a> StructNamedSemigroup<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
        fields_named: &'a FieldsNamed,
    ) -> Self {
        Self {
            constant,
            derive,
            attr,
            fields_named,
        }
    }

    pub fn impl_semigroup(&self) -> syn::Result<ItemImpl> {
        let Self {
            constant,
            derive,
            attr,
            fields_named,
            ..
        } = self;
        let Constant {
            path_semigroup,
            ident_semigroup_op,
            ..
        } = constant;
        let DeriveInput { ident, .. } = derive;
        let fields = FieldSemigroupOp::new_fields(constant, derive, attr, &fields_named.named)?;
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
    index: usize,
    field: &'a Field,
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
        index: usize,
        field: &'a Field,
    ) -> syn::Result<Self> {
        let field_attr = FieldAttr::new(field)?;
        Ok(Self {
            constant,
            container_attr,
            index,
            field,
            field_attr,
        })
    }
    pub fn new_fields(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        container_attr: &'a ContainerAttr,
        fields: impl 'a + IntoIterator<Item = &'a Field>,
    ) -> syn::Result<Vec<Self>> {
        fields
            .into_iter()
            .enumerate()
            .map(move |(index, field)| Self::new(constant, derive, container_attr, index, field))
            .collect()
    }
    pub fn impl_semigroup_field(&self) -> FieldValue {
        match &self.field.ident {
            Some(ident) => self.impl_semigroup_field_named(ident),
            None => todo!(),
        }
    }

    pub fn impl_semigroup_field_named(&self, ident: &Ident) -> FieldValue {
        let Self {
            constant:
                Constant {
                    path_semigroup,
                    ident_semigroup_op,
                    path_construction_trait,
                    ..
                },
            container_attr,
            field_attr,
            ..
        } = self;
        field_attr
            .with(container_attr)
            .map(|path| {
                parse_quote! {
                    #ident: <#path<_> as #path_construction_trait<_>>::lift_op(base.#ident, other.#ident)
                }
            })
            .unwrap_or_else(|| {
                parse_quote! {
                    #ident: #path_semigroup::#ident_semigroup_op(base.#ident, other.#ident)
                }
            })
    }
}
