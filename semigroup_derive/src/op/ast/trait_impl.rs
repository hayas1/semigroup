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
        self.impl_idempotent().to_tokens(tokens);
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

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        attr.open_inner().then(|| {
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

    pub fn impl_idempotent(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_idempotent,
                    path_idempotent_op,
                    path_selected,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            ..
        } = self;

        attr.is_idempotent().then(|| {
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_idempotent for #ident #ty_generics #where_clause {
                    fn select(base: &Self, other: &Self) -> #path_selected {
                        <Self as #path_idempotent_op<_>>::lift_select(&base.0, &other.0)
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
        let mut g = generics.clone();
        attr.monoid_where()
            .into_iter()
            .for_each(|w| g.make_where_clause().predicates.push(w));
        (attr.is_monoid() && attr.with_monoid_impl()).then(|| {
            attr.identity()
                .map(|expr| {
                    let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
                    parse_quote! {
                        #[automatically_derived]
                        #attr_feature_monoid
                        impl #impl_generics #path_monoid for #ident #ty_generics #where_clause {
                            fn identity() -> Self {
                                #expr
                            }
                        }
                    }
                })
                .unwrap_or_else(|| {
                    let where_default = parse_quote! { Self: Default };
                    g.make_where_clause().predicates.push(where_default);
                    let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
                    parse_quote! {
                        #[automatically_derived]
                        #attr_feature_monoid
                        impl #impl_generics #path_monoid for #ident #ty_generics #where_clause {
                            fn identity() -> Self {
                                Default::default()
                            }
                        }
                    }
                })
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
        attr.commutative_where()
            .into_iter()
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
