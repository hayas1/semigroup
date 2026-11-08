use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Data, DataStruct, DeriveInput, Expr, FieldValue, ItemImpl, parse_quote};

use crate::{
    attr::Property,
    constant::Constant,
    derive::SemigroupDerive,
    error::SemigroupError,
    semigroup::{
        ast::struct_semigroup::{StructAnnotated, StructSemigroup},
        attr::{ContainerAttr, FieldAttr},
    },
};

pub mod field_semigroup;
pub mod struct_semigroup;

#[derive(Debug, Clone)]
enum SemigroupImpl<'a> {
    Semigroup(&'a DataStruct),
    FieldsMonoid(&'a DataStruct, Vec<FieldValue>),
    GivenMonoid(Expr),
    FieldsCommutative(&'a DataStruct),
    GivenCommutative,
    AnnotateFields(&'a DataStruct),
}

#[derive(Debug, Clone)]
pub struct Semigroup<'a> {
    constant: Constant,
    derive: &'a DeriveInput,
    attr: ContainerAttr,
    field_attrs: Vec<FieldAttr>,
    semigroup_impl: SemigroupImpl<'a>,
}
impl ToTokens for Semigroup<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.semigroup_impl {
            SemigroupImpl::Semigroup(data_struct) => {
                self.fields(data_struct).impl_semigroup().to_tokens(tokens)
            }
            SemigroupImpl::FieldsMonoid(data_struct, fields_identity) => self
                .fields(data_struct)
                .impl_monoid(fields_identity)
                .to_tokens(tokens),
            SemigroupImpl::GivenMonoid(identity) => self.impl_monoid(identity).to_tokens(tokens),
            SemigroupImpl::FieldsCommutative(data_struct) => self
                .fields(data_struct)
                .impl_commutative()
                .to_tokens(tokens),
            SemigroupImpl::GivenCommutative => self.impl_commutative().to_tokens(tokens),
            SemigroupImpl::AnnotateFields(data_struct) => StructAnnotated::new(
                &self.constant,
                self.derive,
                &self.attr,
                data_struct,
                &self.field_attrs,
            )
            .to_tokens(tokens),
        }
    }
}
impl<'a> Semigroup<'a> {
    pub fn new(derive: &'a DeriveInput, semigroup_derive: SemigroupDerive) -> syn::Result<Self> {
        let attr = ContainerAttr::new(derive)?;
        let constant = attr.derive_attr().constant();
        let field_attrs = match &derive.data {
            Data::Struct(data_struct) => data_struct
                .fields
                .iter()
                .map(FieldAttr::new)
                .collect::<syn::Result<_>>()?,
            Data::Enum(_) | Data::Union(_) => Vec::new(),
        };
        let error = |error| syn::Error::new_spanned(&derive.ident, error);
        let data_struct = || match &derive.data {
            Data::Struct(data_struct) => Ok(data_struct),
            Data::Enum(_) | Data::Union(_) => {
                Err(error(SemigroupError::ExpectStruct(semigroup_derive)))
            }
        };
        let semigroup_impl = match semigroup_derive {
            SemigroupDerive::Semigroup => SemigroupImpl::Semigroup(data_struct()?),
            SemigroupDerive::Monoid => match attr.identity() {
                Some(identity) => SemigroupImpl::GivenMonoid(identity.clone()),
                None => {
                    let data_struct = data_struct()?;
                    let fields_identity =
                        StructSemigroup::new(&constant, derive, &attr, data_struct, &field_attrs)
                            .fields_identity()?;
                    SemigroupImpl::FieldsMonoid(data_struct, fields_identity)
                }
            },
            SemigroupDerive::Commutative => {
                match (
                    attr.derive_attr().bound(Property::Commutative),
                    &derive.data,
                ) {
                    (Some(_), _) => SemigroupImpl::GivenCommutative,
                    (None, Data::Struct(data_struct)) => {
                        SemigroupImpl::FieldsCommutative(data_struct)
                    }
                    (None, Data::Enum(_) | Data::Union(_)) => {
                        return Err(error(SemigroupError::BoundRequired(
                            semigroup_derive,
                            Property::Commutative,
                        )));
                    }
                }
            }
            SemigroupDerive::AnnotateFields => SemigroupImpl::AnnotateFields(data_struct()?),
        };
        Ok(Self {
            constant,
            derive,
            attr,
            field_attrs,
            semigroup_impl,
        })
    }

    fn fields(&self, data_struct: &'a DataStruct) -> StructSemigroup<'_> {
        StructSemigroup::new(
            &self.constant,
            self.derive,
            &self.attr,
            data_struct,
            &self.field_attrs,
        )
    }

    fn impl_monoid(&self, identity: &Expr) -> ItemImpl {
        let Self {
            constant: Constant { path_monoid, .. },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            ..
        } = self;
        let g = attr
            .derive_attr()
            .generics(Property::Monoid, generics, Vec::new);
        let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
        parse_quote! {
            #[automatically_derived]
            impl #impl_generics #path_monoid for #ident #ty_generics #where_clause {
                fn identity() -> Self {
                    #identity
                }
            }
        }
    }

    fn impl_commutative(&self) -> ItemImpl {
        let Self {
            constant: Constant {
                path_commutative, ..
            },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            ..
        } = self;
        let g = attr
            .derive_attr()
            .generics(Property::Commutative, generics, Vec::new);
        let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
        parse_quote! {
            #[automatically_derived]
            impl #impl_generics #path_commutative for #ident #ty_generics #where_clause {}
        }
    }
}
