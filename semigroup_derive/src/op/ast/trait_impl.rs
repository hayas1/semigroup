use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, ItemImpl, parse_quote};

use crate::{constant::Constant, op::attr::ContainerAttr};

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
            let mut g = generics.clone();
            attr.semigroup_bound()
                .iter()
                .cloned()
                .for_each(|w| g.make_where_clause().predicates.push(w));
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

        (attr.is_selective() && attr.gen_selective_impl()).then(|| {
            let mut g = generics.clone();
            attr.selective_bound()
                .iter()
                .cloned()
                .for_each(|w| g.make_where_clause().predicates.push(w));
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
        (attr.is_monoid() && attr.gen_monoid_impl()).then(|| {
            let mut g = generics.clone();
            let identity = attr.identity().cloned().unwrap_or_else(|| {
                // inferred: without `identity`, the identity is `Default::default()`
                if attr.monoid_bound().is_none() {
                    g.make_where_clause()
                        .predicates
                        .push(parse_quote! { Self: Default });
                }
                parse_quote! { Default::default() }
            });
            attr.monoid_bound()
                .unwrap_or_default()
                .iter()
                .cloned()
                .for_each(|w| g.make_where_clause().predicates.push(w));
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
        let mut g = generics.clone();
        attr.commutative_bound()
            .iter()
            .cloned()
            .for_each(|w| g.make_where_clause().predicates.push(w));
        let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
        attr.is_commutative().then(|| {
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_commutative for #ident #ty_generics #where_clause {}
            }
        })
    }
}
