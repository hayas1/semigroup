use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Field, FieldValue, Fields,
    FieldsNamed, Ident, ItemImpl,
};

use crate::{constant::Constant, error::SemigroupError, semigroup::attr::ContainerAttr};

#[derive(Debug, Clone)]
pub struct Semigroup<'a> {
    derive: &'a DeriveInput,
    semigroup_trait: SemigroupTrait<'a>,
}
impl ToTokens for Semigroup<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            semigroup_trait, ..
        } = self;
        tokens.extend(quote::quote! {
            #semigroup_trait
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
                let semigroup_trait = SemigroupTrait::new(constant, derive, data_struct, attr)?;
                Ok(Self {
                    derive,
                    semigroup_trait,
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
pub struct SemigroupTrait<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,
    data_struct: &'a DataStruct,
    attr: &'a ContainerAttr,
}
impl ToTokens for SemigroupTrait<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_trait = self.impl_trait();
        impl_trait.to_tokens(tokens)
    }
}
impl<'a> SemigroupTrait<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        data_struct: &'a DataStruct,
        attr: &'a ContainerAttr,
    ) -> syn::Result<Self> {
        Ok(Self {
            constant,
            derive,
            data_struct,
            attr,
        })
    }
    pub fn impl_trait(&self) -> ItemImpl {
        match &self.data_struct.fields {
            Fields::Named(fields_named) => {
                let named =
                    SemigroupTraitNamed::new(self.constant, self.derive, fields_named, self.attr);
                named.impl_trait()
            }
            Fields::Unnamed(fields_unnamed) => todo!(),
            Fields::Unit => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SemigroupTraitNamed<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,
    fields_named: &'a FieldsNamed,
}
impl ToTokens for SemigroupTraitNamed<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_trait = self.impl_trait();
        impl_trait.to_tokens(tokens)
    }
}
impl<'a> SemigroupTraitNamed<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        fields_named: &'a FieldsNamed,
        attr: &'a ContainerAttr,
    ) -> Self {
        Self {
            constant,
            derive,
            fields_named,
        }
    }

    pub fn impl_trait(&self) -> ItemImpl {
        let Self {
            constant:
                Constant {
                    path_semigroup,
                    ident_semigroup_op,
                    ..
                },
            derive: DeriveInput { ident, .. },
            fields_named,
            ..
        } = self;
        let fields =
            SemigroupTraitField::new(self.constant, self.derive, self.fields_named.named.iter());
        parse_quote! {
            impl #path_semigroup for #ident {
                fn #ident_semigroup_op(base: Self, other: Self) -> Self {
                    Self {
                        #(#fields),*
                    }
                }
            }
        }
    }
}

pub struct SemigroupTraitField<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,
    index: usize,
    field: &'a Field,
}
impl ToTokens for SemigroupTraitField<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_field = self.impl_trait_field();
        impl_field.to_tokens(tokens)
    }
}
impl<'a> SemigroupTraitField<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        fields: impl 'a + IntoIterator<Item = &'a Field>,
    ) -> impl 'a + Iterator<Item = Self> {
        fields
            .into_iter()
            .enumerate()
            .map(move |(index, field)| Self {
                constant,
                derive,
                index,
                field,
            })
    }
    pub fn impl_trait_field(&self) -> FieldValue {
        match &self.field.ident {
            Some(ident) => self.impl_trait_field_named(ident),
            None => todo!(),
        }
    }

    pub fn impl_trait_field_named(&self, ident: &Ident) -> FieldValue {
        let Self {
            constant: Constant {
                ident_semigroup_op, ..
            },
            ..
        } = self;
        parse_quote! {
            #ident: base.#ident.#ident_semigroup_op(other.#ident)
        }
    }
}
