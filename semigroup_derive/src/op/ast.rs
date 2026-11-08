use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Data, DataStruct, DeriveInput, Expr, Field, Fields, FieldsUnnamed, ItemImpl, WherePredicate,
    parse_quote,
};

use crate::{
    attr::Property,
    constant::Constant,
    derive::OpDerive,
    error::OpError,
    op::{ast::construction_trait::ConstructionTrait, attr::ContainerAttr},
};

pub mod construction_trait;

#[derive(Debug, Clone)]
enum OpImpl {
    Semigroup(Expr),
    Selective(Expr),
    Monoid,
    Commutative,
}
impl OpImpl {
    fn new(derive: &DeriveInput, attr: &ContainerAttr, op_derive: OpDerive) -> syn::Result<Self> {
        let required = |impl_expr: Option<&Expr>| {
            impl_expr.cloned().ok_or_else(|| {
                syn::Error::new_spanned(&derive.ident, OpError::ImplRequired(op_derive))
            })
        };
        Ok(match op_derive {
            OpDerive::Semigroup => Self::Semigroup(required(attr.semigroup())?),
            OpDerive::Selective => Self::Selective(required(attr.selective())?),
            OpDerive::Monoid => Self::Monoid,
            OpDerive::Commutative => Self::Commutative,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Op<'a> {
    constant: Constant,
    derive: &'a DeriveInput,
    attr: ContainerAttr,
    field: &'a Field,
    op_impl: OpImpl,
}
impl ToTokens for Op<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.op_impl {
            OpImpl::Semigroup(op) => {
                ConstructionTrait::new(&self.constant, self.derive, self.field).to_tokens(tokens);
                self.impl_semigroup_op(op).to_tokens(tokens);
                self.impl_semigroup().to_tokens(tokens);
            }
            OpImpl::Selective(select) => {
                self.impl_selective_op(select).to_tokens(tokens);
                self.impl_selective().to_tokens(tokens);
            }
            OpImpl::Monoid => {
                self.impl_monoid().to_tokens(tokens);
                self.impl_monoid_op().to_tokens(tokens);
            }
            OpImpl::Commutative => {
                self.impl_commutative().to_tokens(tokens);
                self.impl_commutative_op().to_tokens(tokens);
            }
        }
    }
}
impl<'a> Op<'a> {
    pub fn new(derive: &'a DeriveInput, op_derive: OpDerive) -> syn::Result<Self> {
        let attr = ContainerAttr::new(derive)?;
        Ok(Self {
            constant: attr.derive_attr().constant(),
            field: Self::newtype_field(derive, op_derive)?,
            op_impl: OpImpl::new(derive, &attr, op_derive)?,
            derive,
            attr,
        })
    }

    fn newtype_field(derive: &'a DeriveInput, op_derive: OpDerive) -> syn::Result<&'a Field> {
        match &derive.data {
            Data::Struct(DataStruct {
                fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }),
                ..
            }) if unnamed.len() == 1 => Ok(&unnamed[0]),
            _ => Err(syn::Error::new_spanned(
                &derive.ident,
                OpError::ExpectNewType(op_derive),
            )),
        }
    }

    fn self_bound(&self, path_trait: &impl ToTokens) -> Vec<WherePredicate> {
        let Field { ty, .. } = self.field;
        vec![parse_quote! { Self: #path_trait<#ty> }]
    }
    fn uses(&self) -> TokenStream {
        let Constant {
            path_semigroup_op,
            path_selective_op,
            path_construction_trait,
            ..
        } = &self.constant;
        quote! {
            #[allow(unused_imports)]
            use #path_semigroup_op as _;
            #[allow(unused_imports)]
            use #path_selective_op as _;
            #[allow(unused_imports)]
            use #path_construction_trait as _;
        }
    }

    fn impl_semigroup_op(&self, op: &Expr) -> ItemImpl {
        let Self {
            constant: Constant {
                path_semigroup_op, ..
            },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            field: Field { ty, .. },
            ..
        } = self;
        let uses = self.uses();
        let g = attr
            .derive_attr()
            .generics(Property::Semigroup, generics, Vec::new);
        let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
        parse_quote! {
            #[automatically_derived]
            impl #impl_generics #path_semigroup_op<#ty> for #ident #ty_generics #where_clause {
                fn lift_op_assign(base: &mut #ty, other: #ty) {
                    let op: fn(&mut #ty, #ty) = { #uses #op };
                    op(base, other)
                }
            }
        }
    }

    fn impl_semigroup(&self) -> ItemImpl {
        let Self {
            constant:
                Constant {
                    path_semigroup,
                    path_semigroup_op,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            ..
        } = self;
        let g = attr
            .derive_attr()
            .generics(Property::Semigroup, generics, || {
                self.self_bound(path_semigroup_op)
            });
        let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
        parse_quote! {
            #[automatically_derived]
            impl #impl_generics #path_semigroup for #ident #ty_generics #where_clause {
                fn op_assign(base: &mut Self, other: Self) {
                    <Self as #path_semigroup_op<_>>::lift_op_assign(&mut base.0, other.0);
                }
            }
        }
    }

    fn impl_selective_op(&self, select: &Expr) -> ItemImpl {
        let Self {
            constant:
                Constant {
                    path_selective_op,
                    path_selected,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            field: Field { ty, .. },
            ..
        } = self;
        let uses = self.uses();
        let g = attr
            .derive_attr()
            .generics(Property::Selective, generics, Vec::new);
        let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
        parse_quote! {
            #[automatically_derived]
            impl #impl_generics #path_selective_op<#ty> for #ident #ty_generics #where_clause {
                fn lift_select(base: &#ty, other: &#ty) -> #path_selected {
                    let select: fn(&#ty, &#ty) -> #path_selected = { #uses #select };
                    select(base, other)
                }
            }
        }
    }

    fn impl_selective(&self) -> ItemImpl {
        let Self {
            constant:
                Constant {
                    path_selective,
                    path_selective_op,
                    path_selected,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            ..
        } = self;
        let g = attr
            .derive_attr()
            .generics(Property::Selective, generics, || {
                self.self_bound(path_selective_op)
            });
        let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
        parse_quote! {
            #[automatically_derived]
            impl #impl_generics #path_selective for #ident #ty_generics #where_clause {
                fn select(base: &Self, other: &Self) -> #path_selected {
                    <Self as #path_selective_op<_>>::lift_select(&base.0, &other.0)
                }
            }
        }
    }

    fn impl_monoid(&self) -> ItemImpl {
        let Self {
            constant: Constant { path_monoid, .. },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            ..
        } = self;
        let (identity, inferred): (Expr, Vec<WherePredicate>) = match attr.identity() {
            Some(identity) => (identity.clone(), Vec::new()),
            None => (
                parse_quote! { Default::default() },
                vec![parse_quote! { Self: Default }],
            ),
        };
        let g = attr
            .derive_attr()
            .generics(Property::Monoid, generics, || inferred);
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

    fn impl_monoid_op(&self) -> ItemImpl {
        let Self {
            constant:
                Constant {
                    path_semigroup_op,
                    path_monoid,
                    path_monoid_op,
                    ..
                },
            ..
        } = self;
        self.impl_marker_op(path_monoid_op, path_semigroup_op, path_monoid)
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

    fn impl_commutative_op(&self) -> ItemImpl {
        let Self {
            constant:
                Constant {
                    path_semigroup_op,
                    path_commutative,
                    path_commutative_op,
                    ..
                },
            ..
        } = self;
        self.impl_marker_op(path_commutative_op, path_semigroup_op, path_commutative)
    }

    fn impl_marker_op(
        &self,
        path_marker_op: &impl ToTokens,
        path_semigroup_op: &impl ToTokens,
        path_property: &impl ToTokens,
    ) -> ItemImpl {
        let Self {
            derive: DeriveInput {
                ident, generics, ..
            },
            field: Field { ty, .. },
            ..
        } = self;
        let mut g = generics.clone();
        g.make_where_clause()
            .predicates
            .push(parse_quote! { Self: #path_semigroup_op<#ty> + #path_property });
        let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
        parse_quote! {
            #[automatically_derived]
            impl #impl_generics #path_marker_op<#ty> for #ident #ty_generics #where_clause {}
        }
    }
}
