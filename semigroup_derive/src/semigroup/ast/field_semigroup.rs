use syn::punctuated::Punctuated;
use syn::{
    AngleBracketedGenericArguments, DeriveInput, Expr, Field, FieldValue, Fields, GenericArgument,
    Member, PathArguments, Stmt, Type, TypePath, parse_quote,
};

use crate::{
    constant::Constant,
    semigroup::attr::{ContainerAttr, FieldAttr},
};

/// Replace every [`Expr::Infer`] (`_`) placeholder inside `expr` with `replacement`.
fn substitute_infer(expr: Expr, replacement: &Expr) -> Expr {
    match expr {
        Expr::Infer(_) => replacement.clone(),
        Expr::Call(mut call) => {
            call.args = call
                .args
                .into_iter()
                .map(|arg| substitute_infer(arg, replacement))
                .collect();
            Expr::Call(call)
        }
        other => other,
    }
}

/// Count the number of constructor layers wrapping the `_` placeholder.
/// `Dual(Coalesce(_))` → 2, `Coalesce(_)` → 1, plain path → 0.
fn count_wrap_depth(expr: &Expr) -> usize {
    match expr {
        Expr::Call(call) => {
            let inner = call.args.iter().map(count_wrap_depth).max().unwrap_or(0);
            1 + inner
        }
        _ => 0,
    }
}

/// Convert a constructor expression like `Dual(Coalesce(_))` into the
/// corresponding type `Dual<Coalesce<_>>`, suitable for UFCS calls.
fn expr_to_type(expr: &Expr) -> Option<Type> {
    match expr {
        Expr::Infer(_) => Some(parse_quote! { _ }),
        Expr::Path(p) => Some(Type::Path(TypePath {
            qself: p.qself.clone(),
            path: p.path.clone(),
        })),
        Expr::Call(call) => {
            let base_ty = expr_to_type(&call.func)?;
            let args: Punctuated<GenericArgument, syn::token::Comma> = call
                .args
                .iter()
                .filter_map(|a| expr_to_type(a).map(GenericArgument::Type))
                .collect();
            match base_ty {
                Type::Path(mut tp) => {
                    let last = tp.path.segments.last_mut()?;
                    last.arguments =
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Default::default(),
                            args,
                            gt_token: Default::default(),
                        });
                    Some(Type::Path(tp))
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Build `expr.into_inner().into_inner()...` (`depth` times).
fn build_into_inner_expr(base: Expr, depth: usize) -> Expr {
    (0..depth).fold(base, |acc: Expr, _| parse_quote! { #acc.into_inner() })
}

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
            Some(Expr::Path(path)) => {
                // Backward-compatible: bare path → call lift_op_assign
                parse_quote! {
                    #path::lift_op_assign(&mut base.#member, other.#member);
                }
            }
            Some(expr) => {
                // Constructor expression, e.g. `Dual(Coalesce(_))`:
                // wrap both values, run Semigroup::op_assign, unwrap the result.
                let Constant {
                    path_construction_trait,
                    ..
                } = self.constant;
                let base_accessor: Expr = parse_quote! { base.#member };
                let other_accessor: Expr = parse_quote! { other.#member };
                let base_wrapped = substitute_infer(expr.clone(), &base_accessor);
                let other_wrapped = substitute_infer(expr.clone(), &other_accessor);
                let depth = count_wrap_depth(expr);
                let unwrap_expr = build_into_inner_expr(parse_quote! { __semigroup_base }, depth);
                parse_quote! {
                    {
                        use #path_construction_trait;
                        let mut __semigroup_base = #base_wrapped;
                        let __semigroup_other = #other_wrapped;
                        #path_semigroup::op_assign(&mut __semigroup_base, __semigroup_other);
                        base.#member = #unwrap_expr;
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
            Some(Expr::Path(path)) => {
                // Backward-compatible: bare path → lift_identity()
                parse_quote! {
                    #member: #path::lift_identity()
                }
            }
            Some(expr) => {
                // Constructor expression: call Monoid::identity() on the wrapped type,
                // then unwrap back to the field type.
                let Constant {
                    path_construction_trait,
                    ..
                } = self.constant;
                let wrapped_ty = expr_to_type(expr).expect("with expr must be a constructor call");
                let depth = count_wrap_depth(expr);
                let unwrap_expr = build_into_inner_expr(parse_quote! { __monoid_identity }, depth);
                parse_quote! {
                    #member: {
                        use #path_construction_trait;
                        let __monoid_identity = <#wrapped_ty as #path_monoid>::identity();
                        #unwrap_expr
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
            Some(Expr::Path(path)) => {
                // Backward-compatible: bare path → lift_annotated_op_assign
                parse_quote! {
                    #path::lift_annotated_op_assign(
                        #path_annotated::new(&mut base_value.#member, &mut base_annotation.#member),
                        #path_annotated::new(other_value.#member, other_annotation.#member),
                    );
                }
            }
            Some(expr) => {
                // Constructor expression: wrap the value, run annotated_op_assign, unwrap.
                let Constant {
                    path_construction_trait,
                    ..
                } = constant;
                let base_accessor: Expr = parse_quote! { base_value.#member };
                let other_accessor: Expr = parse_quote! { other_value.#member };
                let base_wrapped = substitute_infer(expr.clone(), &base_accessor);
                let other_wrapped = substitute_infer(expr.clone(), &other_accessor);
                let depth = count_wrap_depth(expr);
                let unwrap_expr = build_into_inner_expr(parse_quote! { __annotated_base }, depth);
                parse_quote! {
                    {
                        use #path_construction_trait;
                        let mut __annotated_base = #base_wrapped;
                        let __annotated_other = #other_wrapped;
                        #path_annotated_semigroup::annotated_op_assign(
                            #path_annotated::new(&mut __annotated_base, &mut base_annotation.#member),
                            #path_annotated::new(__annotated_other, other_annotation.#member),
                        );
                        base_value.#member = #unwrap_expr;
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
