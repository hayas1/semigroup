use syn::{DeriveInput, Expr, Field, FieldValue, Fields, Member, Stmt, Type, parse_quote};

use crate::{
    constant::Constant,
    semigroup::attr::{ContainerAttr, FieldAttr, With},
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
        match with {
            None => {
                parse_quote! {
                    #path_semigroup::op_assign(&mut base.#member, other.#member);
                }
            }
            Some(With::Path(path)) => {
                // Backward-compatible: bare path → call lift_op_assign
                parse_quote! {
                    #path::lift_op_assign(&mut base.#member, other.#member);
                }
            }
            Some(with) => {
                // Constructor expression, e.g. `Dual(Coalesce(_))`:
                // wrap both values, run Semigroup::op_assign, unwrap the result.
                let Constant {
                    path_construction_trait,
                    ..
                } = self.constant;
                let base_accessor: Expr = parse_quote! { base.#member };
                let other_accessor: Expr = parse_quote! { other.#member };
                let base_wrapped = with.substitute(&base_accessor);
                let other_wrapped = with.substitute(&other_accessor);
                let chain_into_inner = with.chain_into_inner(parse_quote! { __semigroup_base });
                parse_quote! {
                    {
                        use #path_construction_trait;
                        let mut __semigroup_base = #base_wrapped;
                        let __semigroup_other = #other_wrapped;
                        #path_semigroup::op_assign(&mut __semigroup_base, __semigroup_other);
                        base.#member = #chain_into_inner;
                    }
                }
            }
        }
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
        match with {
            None => {
                parse_quote! {
                    #member: #path_monoid::identity()
                }
            }
            Some(With::Path(path)) => {
                // Backward-compatible: bare path → lift_identity()
                parse_quote! {
                    #member: #path::lift_identity()
                }
            }
            Some(with) => {
                // Constructor expression: call Monoid::identity() on the wrapped type,
                // then unwrap back to the field type.
                let Constant {
                    path_construction_trait,
                    ..
                } = self.constant;
                let wrapped_ty = with
                    .as_type()
                    .expect("with expr must be a constructor call");
                let chain_into_inner = with.chain_into_inner(parse_quote! { __monoid_identity });
                parse_quote! {
                    #member: {
                        use #path_construction_trait;
                        let __monoid_identity = <#wrapped_ty as #path_monoid>::identity();
                        #chain_into_inner
                    }
                }
            }
        }
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

        match with {
            None => {
                parse_quote! {
                    #path_annotated_semigroup::annotated_op_assign(
                        #path_annotated::new(&mut base_value.#member, &mut base_annotation.#member),
                        #path_annotated::new(other_value.#member, other_annotation.#member),
                    );
                }
            }
            Some(With::Path(path)) => {
                // Backward-compatible: bare path → lift_annotated_op_assign
                parse_quote! {
                    #path::lift_annotated_op_assign(
                        #path_annotated::new(&mut base_value.#member, &mut base_annotation.#member),
                        #path_annotated::new(other_value.#member, other_annotation.#member),
                    );
                }
            }
            Some(with) => {
                // Constructor expression: wrap the value, run annotated_op_assign, unwrap.
                let Constant {
                    path_construction_trait,
                    ..
                } = constant;
                let base_accessor: Expr = parse_quote! { base_value.#member };
                let other_accessor: Expr = parse_quote! { other_value.#member };
                let base_wrapped = with.substitute(&base_accessor);
                let other_wrapped = with.substitute(&other_accessor);
                let chain_into_inner = with.chain_into_inner(parse_quote! { __annotated_base });
                parse_quote! {
                    {
                        use #path_construction_trait;
                        let mut __annotated_base = #base_wrapped;
                        let __annotated_other = #other_wrapped;
                        #path_annotated_semigroup::annotated_op_assign(
                            #path_annotated::new(&mut __annotated_base, &mut base_annotation.#member),
                            #path_annotated::new(__annotated_other, other_annotation.#member),
                        );
                        base_value.#member = #chain_into_inner;
                    }
                }
            }
        }
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
