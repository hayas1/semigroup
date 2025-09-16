use quote::format_ident;
use syn::{parse_quote, DeriveInput, FieldValue, Fields, Ident, Member, Stmt};

use crate::{
    constant::Constant,
    semigroup::attr::{ContainerAttr, FieldAttr},
};

#[derive(Debug, Clone)]
pub struct FieldSemigroupOp<'a> {
    constant: &'a Constant,
    container_attr: &'a ContainerAttr,
    member: Member,
    field_attr: FieldAttr,
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

    pub fn impl_field_semigroup_op(&self) -> FieldValue {
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

#[derive(Debug, Clone)]
pub struct FieldAnnotatedOp<'a> {
    constant: &'a Constant,
    container_attr: &'a ContainerAttr,
    member: Member,
    field_attr: FieldAttr,
}
impl<'a> FieldAnnotatedOp<'a> {
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

    pub fn ident_variable(&self) -> Ident {
        match &self.member {
            Member::Named(ident) => ident.clone(),
            Member::Unnamed(index) => format_ident!("_{}", index.index),
        }
    }
    pub fn impl_field_annotated_op(&self) -> Stmt {
        let Self {
            constant,
            container_attr,
            member,
            field_attr,
        } = self;
        let Constant {
            path_annotated_semigroup,
            ident_annotated_op,
            path_annotated,
            ..
        } = constant;
        let ident_variable = self.ident_variable();
        let with = field_attr.with(container_attr);

        with.map(|path| {
            parse_quote! {
                let #ident_variable = #path_annotated_semigroup::#ident_annotated_op(
                    #path_annotated{ value: base.value.#member, annotation: base.annotation.#member }.map(#path::<_>::from),
                    #path_annotated{ value: other.value.#member, annotation: other.annotation.#member }.map(#path::<_>::from),
                );
            }
        })
        .unwrap_or_else(|| {
            parse_quote! {
                let #ident_variable = #path_annotated_semigroup::#ident_annotated_op(
                    #path_annotated{ value: base.value.#member, annotation: base.annotation.#member },
                    #path_annotated{ value: other.value.#member, annotation: other.annotation.#member },
                );
            }
        })
    }
    pub fn impl_field_value(&self) -> FieldValue {
        let Self {
            constant:
                Constant {
                    path_construction_trait,
                    ..
                },
            member,
            ..
        } = self;
        let ident_variable = self.ident_variable();
        let with = self.field_attr.with(self.container_attr);
        with.map(|path| {
            parse_quote! {
                #member: <#path<_> as #path_construction_trait<_>>::into_inner(#ident_variable.value)
            }
        })
        .unwrap_or_else(|| {
            parse_quote! {
                #member: #ident_variable.value
            }
        })
    }
    pub fn impl_field_annotation(&self) -> FieldValue {
        let Self { member, .. } = self;
        let ident_variable = self.ident_variable();
        parse_quote! {
            #member: #ident_variable.annotation
        }
    }
}
