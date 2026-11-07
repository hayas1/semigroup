use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, Expr, ItemImpl, WherePredicate, parse_quote};

use crate::{attr::Property, constant::Constant, op::attr::ContainerAttr};

#[derive(Debug, Clone)]
pub struct TraitImpl<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,

    attr: &'a ContainerAttr,
}
impl ToTokens for TraitImpl<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_semigroup().to_tokens(tokens);
        self.impl_selective().to_tokens(tokens);
        self.impl_monoid().to_tokens(tokens);
        self.impl_commutative().to_tokens(tokens);
    }
}
impl<'a> TraitImpl<'a> {
    pub fn new(constant: &'a Constant, derive: &'a DeriveInput, attr: &'a ContainerAttr) -> Self {
        Self {
            constant,
            derive,
            attr,
        }
    }

    pub fn impl_semigroup(&self) -> Option<ItemImpl> {
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

        attr.open_inner().then(|| {
            let g = attr
                .derive_attr()
                .generics(Property::Semigroup, generics, Vec::new);
            let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_semigroup for #ident #ty_generics #where_clause {
                    fn op_assign(base: &mut Self, other: Self) {
                        <Self as #path_semigroup_op<_>>::lift_op_assign(&mut base.0, other.0);
                    }
                }
            }
        })
    }

    pub fn impl_selective(&self) -> Option<ItemImpl> {
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

        let attr = attr.derive_attr();
        attr.derives(Property::Selective).then(|| {
            let g = attr.generics(Property::Selective, generics, Vec::new);
            let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_selective for #ident #ty_generics #where_clause {
                    fn select(base: &Self, other: &Self) -> #path_selected {
                        <Self as #path_selective_op<_>>::lift_select(&base.0, &other.0)
                    }
                }
            }
        })
    }

    pub fn impl_monoid(&self) -> Option<ItemImpl> {
        let Self {
            constant,
            derive,
            attr,
            ..
        } = self;
        let Constant {
            path_monoid,
            attr_feature_monoid,
            ..
        } = constant;
        let DeriveInput {
            ident, generics, ..
        } = derive;
        let attr = attr.derive_attr();
        attr.derives(Property::Monoid).then(|| {
            let (identity, inferred): (Expr, Vec<WherePredicate>) = match attr.identity() {
                Some(identity) => (identity.clone(), Vec::new()),
                None => (
                    parse_quote! { Default::default() },
                    vec![parse_quote! { Self: Default }],
                ),
            };
            let g = attr.generics(Property::Monoid, generics, || inferred);
            let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
            parse_quote! {
                #[automatically_derived]
                #attr_feature_monoid
                impl #impl_generics #path_monoid for #ident #ty_generics #where_clause {
                    fn identity() -> Self {
                        #identity
                    }
                }
            }
        })
    }

    pub fn impl_commutative(&self) -> Option<ItemImpl> {
        let Self {
            constant,
            derive,
            attr,
            ..
        } = self;
        let Constant {
            path_commutative, ..
        } = constant;
        let DeriveInput {
            ident, generics, ..
        } = derive;
        let attr = attr.derive_attr();
        attr.derives(Property::Commutative).then(|| {
            let g = attr.generics(Property::Commutative, generics, Vec::new);
            let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_commutative for #ident #ty_generics #where_clause {}
            }
        })
    }
}
