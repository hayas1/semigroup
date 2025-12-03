use syn::{DeriveInput, Field, FieldValue, Fields, Member, Stmt, Type, parse_quote};

use crate::{
    constant::Constant,
    semigroup::attr::{ContainerAttr, FieldAttr},
};

#[derive(Debug, Clone)]
pub struct FieldSemigroupOp<'a> {
    constant: &'a Constant,
    container_attr: &'a ContainerAttr,
    member: Member,
    ty: &'a Type,
    field_attr: FieldAttr,
}
impl<'a> FieldSemigroupOp<'a> {
    pub fn new(
        constant: &'a Constant,
        _derive: &'a DeriveInput,
        container_attr: &'a ContainerAttr,
        member: Member,
        field: &'a Field,
    ) -> syn::Result<Self> {
        Ok(Self {
            constant,
            container_attr,
            member,
            ty: &field.ty,
            field_attr: FieldAttr::new(field)?,
        })
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
            .map(|(field, member)| Self::new(constant, derive, container_attr, member, field))
            .collect()
    }

    pub fn impl_field_semigroup_op_assign(&self) -> Stmt {
        let Self {
            constant: Constant { path_semigroup, .. },
            container_attr,
            member,
            field_attr,
            ..
        } = self;
        let with = field_attr.with(container_attr);
        with.map(|path| {
            parse_quote! {
                #path::lift_op_assign(&mut base.#member, other.#member);
            }
        })
        .unwrap_or_else(|| {
            parse_quote! {
               #path_semigroup::op_assign(&mut base.#member, other.#member);
            }
        })
    }
    pub fn impl_field_monoid_identity(&self) -> FieldValue {
        let Self {
            constant: Constant { path_monoid, .. },
            container_attr,
            member,
            field_attr,
            ..
        } = self;
        let with = field_attr.with(container_attr);
        with.map(|path| {
            parse_quote! {
                #member: #path::lift_identity()
            }
        })
        .unwrap_or_else(|| {
            parse_quote! {
                #member: #path_monoid::identity()
            }
        })
    }

    pub fn where_ty(&self) -> Option<&Type> {
        let Self {
            container_attr,
            ty,
            field_attr,
            ..
        } = self;
        let with = field_attr.with(container_attr);
        with.is_none().then_some(*ty)
    }
}

#[derive(Debug, Clone)]
pub struct FieldAnnotatedOp<'a> {
    constant: &'a Constant,
    container_attr: &'a ContainerAttr,
    member: Member,
    ty: &'a Type,
    field_attr: FieldAttr,
}
impl<'a> FieldAnnotatedOp<'a> {
    pub fn new(
        constant: &'a Constant,
        _derive: &'a DeriveInput,
        container_attr: &'a ContainerAttr,
        member: Member,
        field: &'a Field,
    ) -> syn::Result<Self> {
        Ok(Self {
            constant,
            container_attr,
            member,
            ty: &field.ty,
            field_attr: FieldAttr::new(field)?,
        })
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
            .map(|(field, member)| Self::new(constant, derive, container_attr, member, field))
            .collect()
    }

    pub fn impl_field_annotated_op_assign(&self) -> Stmt {
        let Self {
            constant,
            container_attr,
            member,
            field_attr,
            ..
        } = self;
        let Constant {
            path_annotated_semigroup,
            path_annotated,
            ..
        } = constant;
        let with = field_attr.with(container_attr);

        with.map(|path| {
            parse_quote! {
                #path::lift_annotated_op_assign(
                    #path_annotated::new(&mut base_value.#member, &mut base_annotation.#member),
                    #path_annotated::new(other_value.#member, other_annotation.#member),
                );

            }
        })
        .unwrap_or_else(|| {
            parse_quote! {
                #path_annotated_semigroup::annotated_op_assign(
                    #path_annotated::new(&mut base_value.#member, &mut base_annotation.#member),
                    #path_annotated::new(other_value.#member, other_annotation.#member),
                );
            }
        })
    }

    pub fn where_ty(&self) -> Option<&Type> {
        let Self {
            container_attr,
            ty,
            field_attr,
            ..
        } = self;
        let with = field_attr.with(container_attr);
        with.is_none().then_some(*ty)
    }
}
