use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, Field, ItemImpl, parse_quote};

use crate::{constant::Constant, op::attr::ContainerAttr};

#[derive(Debug, Clone)]
pub struct OpTrait<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,
    attr: &'a ContainerAttr,

    field: &'a Field,
}

impl ToTokens for OpTrait<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_op_with_unit_annotation().to_tokens(tokens);
        self.impl_monoid_op().to_tokens(tokens);
    }
}
impl<'a> OpTrait<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
        field: &'a Field,
    ) -> syn::Result<Self> {
        Ok(Self {
            constant,
            derive,
            attr,
            field,
        })
    }

    pub fn impl_op_with_unit_annotation(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_annotated,
                    path_semigroup_op,
                    path_annotated_op,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            field: Field { ty, .. },
            ..
        } = self;

        attr.is_annotated().then(|| {
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
            let unit_annotation = attr.unit_annotation();
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_semigroup_op<#ty> for #ident #ty_generics #where_clause {
                    fn lift_op_assign(base: &mut #ty, other: #ty) {
                        let (mut base_unit, other_unit) = (#unit_annotation, #unit_annotation);
                        let (b, o) = (
                            #path_annotated::new(base, &mut base_unit),
                            #path_annotated::new(other, other_unit),
                        );
                        <Self as #path_annotated_op<_, _>>::lift_annotated_op_assign(b, o);
                    }
                }
            }
        })
    }

    pub fn impl_monoid_op(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_monoid,
                    path_monoid_op,
                    attr_feature_monoid,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            field: Field { ty, .. },
            attr,
            ..
        } = self;

        attr.is_monoid().then(|| {
            let mut g = generics.clone();
            g.make_where_clause()
                .predicates
                .push(parse_quote! { Self: #path_monoid });
            let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
            parse_quote! {
                #[automatically_derived]
                #attr_feature_monoid
                impl #impl_generics #path_monoid_op<#ty> for #ident #ty_generics #where_clause {}
            }
        })
    }
}
