use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
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
                parse_quote! {
                    #path::lift_op_assign(&mut base.#member, other.#member);
                }
            }
            Some(with) => {
                let Constant {
                    path_construction_trait,
                    ..
                } = self.constant;
                let base_accessor: Expr = parse_quote! { __semigroup_base_owned };
                let other_accessor: Expr = parse_quote! { other.#member };
                let base_wrapped = with.substitute(&base_accessor);
                let other_wrapped = with.substitute(&other_accessor);
                let chain_into_inner = with.chain_into_inner(parse_quote! { __semigroup_base });
                parse_quote! {
                    {
                        use #path_construction_trait;
                        struct __AbortOnDrop;
                        impl ::core::ops::Drop for __AbortOnDrop {
                            fn drop(&mut self) {
                                ::std::process::abort();
                            }
                        }
                        let __guard = __AbortOnDrop;
                        let __semigroup_base_owned = unsafe { ::core::ptr::read(&raw const base.#member) };
                        let (mut __semigroup_base, __semigroup_other) = (#base_wrapped, #other_wrapped);
                        #path_semigroup::op_assign(&mut __semigroup_base, __semigroup_other);
                        let __semigroup_result = #chain_into_inner;
                        unsafe { ::core::ptr::write(&raw mut base.#member, __semigroup_result); }
                        ::core::mem::forget(__guard);
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
                parse_quote! {
                    #member: #path::lift_identity()
                }
            }
            Some(with) => {
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

/// Per-field code generation for `#[semigroup(annotated)]`.
///
/// Each field of the original struct becomes an `Annotated<FieldType, A>` field in the generated
/// `XxxAnnotated<A>` struct (storing the raw field type, not the op-wrapper type). The semigroup
/// operation is generated inline, calling the op's `lift_select` and `lift_op_assign` methods.
#[derive(Debug, Clone)]
pub struct FieldAnnotated<'a> {
    pub constant: &'a Constant,
    pub container_attr: &'a ContainerAttr,
    pub member: Member,
    pub ty: &'a Type,
    pub field_attr: FieldAttr,
}
impl<'a> FieldAnnotated<'a> {
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

    /// Named struct field definition: `pub name: Annotated<FieldType, A>`.
    /// For tuple struct fields, returns only the type.
    pub fn struct_field_tokens(&self, a: &syn::Ident, path_annotated: &impl ToTokens) -> TokenStream {
        let ty = self.ty;
        match &self.member {
            Member::Named(ident) => {
                quote! { pub #ident: #path_annotated<#ty, #a> }
            }
            Member::Unnamed(_) => {
                quote! { pub #path_annotated<#ty, #a> }
            }
        }
    }

    /// Generates a variable name for this field's selected value (unique per field).
    fn selected_var(&self) -> syn::Ident {
        match &self.member {
            Member::Named(ident) => format_ident!("__selected_{}", ident),
            Member::Unnamed(idx) => format_ident!("__selected_{}", idx.index),
        }
    }

    /// Generates variable names for other_val and other_ann for this field.
    fn other_vars(&self) -> (syn::Ident, syn::Ident) {
        match &self.member {
            Member::Named(ident) => (
                format_ident!("__other_{}_val", ident),
                format_ident!("__other_{}_ann", ident),
            ),
            Member::Unnamed(idx) => (
                format_ident!("__other_{}_val", idx.index),
                format_ident!("__other_{}_ann", idx.index),
            ),
        }
    }

    /// Generates the per-field op_assign statements for `Semigroup for XxxAnnotated<A>`.
    ///
    /// Uses `With::lift_select` and `With::lift_op_assign` to drive selection and mutation,
    /// then updates the annotation if `Selected::Other` was returned.
    pub fn annotated_op_assign_stmts(&self, path_selected: &impl ToTokens) -> TokenStream {
        let member = &self.member;
        let selected_var = self.selected_var();
        let (other_val_var, other_ann_var) = self.other_vars();
        let ty = self.ty;
        let with = self.field_attr.with(self.container_attr);

        match with {
            None => {
                // No with: field type must implement Idempotent + Semigroup directly.
                let path_idempotent = &self.constant.path_idempotent;
                let path_semigroup = &self.constant.path_semigroup;
                quote! {
                    let #selected_var = <#ty as #path_idempotent>::select(base.#member.value(), other.#member.value());
                    let (#other_val_var, #other_ann_var) = other.#member.into_parts();
                    #path_semigroup::op_assign(base.#member.value_mut(), #other_val_var);
                    if let #path_selected::Other = #selected_var {
                        *base.#member.annotation_mut() = #other_ann_var;
                    }
                }
            }
            Some(With::Path(p)) => {
                quote! {
                    let #selected_var = #p::lift_select(base.#member.value(), other.#member.value());
                    let (#other_val_var, #other_ann_var) = other.#member.into_parts();
                    #p::lift_op_assign(base.#member.value_mut(), #other_val_var);
                    if let #path_selected::Other = #selected_var {
                        *base.#member.annotation_mut() = #other_ann_var;
                    }
                }
            }
            Some(With::Constructor(_)) => {
                // Constructor forms (e.g. `Dual(Coalesce(_))`) are not yet supported
                // for annotated struct fields.
                quote! {
                    compile_error!("constructor `with` expressions are not supported in annotated struct fields; use a bare path like `\"semigroup::op::Overwrite\"`");
                }
            }
        }
    }

    /// Generates the field initialiser for the `annotated()` method:
    /// just wraps the field value with an `Annotated::new`.
    /// Returns `name: Annotated::new(self.name, annotation.clone())` for named,
    /// or `Annotated::new(self.name, annotation.clone())` for unnamed (is_last determines clone).
    pub fn annotated_init_named(&self, path_annotated: &impl ToTokens, is_last: bool) -> TokenStream {
        let ident = match &self.member {
            Member::Named(ident) => ident,
            Member::Unnamed(_) => panic!("annotated_init_named called on unnamed field"),
        };
        let val = self.annotated_init_value_expr(path_annotated, is_last);
        quote! { #ident: #val }
    }

    /// For tuple struct fields: just the expression, no field name.
    pub fn annotated_init_value_expr(&self, path_annotated: &impl ToTokens, is_last: bool) -> TokenStream {
        let member = &self.member;
        let annotation_expr = if is_last {
            quote! { annotation }
        } else {
            quote! { annotation.clone() }
        };
        quote! { #path_annotated::new(self.#member, #annotation_expr) }
    }
}
