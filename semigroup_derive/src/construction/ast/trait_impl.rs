use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, ItemImpl, parse_quote};

use crate::{annotation::Annotation, constant::Constant, construction::attr::ContainerAttr};

#[derive(Debug, Clone)]
pub struct TraitImpl<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,

    attr: &'a ContainerAttr,
    annotation: Annotation,
}
impl ToTokens for TraitImpl<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_monoid().to_tokens(tokens);
        self.impl_commutative().to_tokens(tokens);
        self.impl_semigroup().to_tokens(tokens);
        self.impl_annotated_semigroup().to_tokens(tokens);
        self.impl_annotate().to_tokens(tokens);
    }
}
impl<'a> TraitImpl<'a> {
    pub fn new(constant: &'a Constant, derive: &'a DeriveInput, attr: &'a ContainerAttr) -> Self {
        let annotation = attr.annotation(constant);

        Self {
            constant,
            derive,
            attr,
            annotation,
        }
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

    pub fn impl_semigroup(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_semigroup,
                    path_construction_semigroup,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            ..
        } = self;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        attr.with_construction().then(|| {
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_semigroup for #ident #ty_generics #where_clause {
                    fn op_assign(&mut self, other: Self) {
                        <Self as #path_construction_semigroup<_>>::lift_op_assign(&mut self.0, other.0);
                    }
                }
            }
        })
    }

    pub fn impl_annotate(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_annotate,
                    path_annotated,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            annotation,
            ..
        } = self;

        (attr.is_annotated() && attr.with_annotate_impl()).then(|| {
            let (_, ty_generics, _) = generics.split_for_impl();
            let (impl_generics, annotated_type, where_clause) = annotation.split_for_impl(generics);
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_annotate<#annotated_type> for #ident #ty_generics #where_clause {
                    type Annotation = #annotated_type;
                    fn annotated(self, annotation: Self::Annotation) -> #path_annotated<Self, #annotated_type> {
                        #path_annotated::new(
                            self,
                            annotation,
                        )
                    }
                }
            }
        })
    }

    pub fn impl_annotated_semigroup(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_construction_annotated,
                    path_annotated_semigroup,
                    path_annotated,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            annotation,
            ..
        } = self;

        attr.is_annotated().then(|| {
            let (_, ty_generics, _) = generics.split_for_impl();
            let (impl_generics, annotated_type, where_clause) = annotation.split_for_impl(generics);
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_annotated_semigroup<#annotated_type> for #ident #ty_generics #where_clause {
                    fn annotated_op_assign(base: #path_annotated<&mut Self, &mut #annotated_type>, other: #path_annotated<Self, #annotated_type>) {
                        <Self as #path_construction_annotated<_, #annotated_type>>::lift_annotated_op_assign(
                            base.map(|v| &mut v.0),
                            other.map(|v| v.0),
                        );
                    }
                }
            }
        })
    }
}
