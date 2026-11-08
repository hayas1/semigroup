use darling::{FromDeriveInput, FromMeta};
use syn::{DeriveInput, Expr};

use crate::attr::DeriveAttr;

#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default)]
pub struct Impls {
    semigroup: Option<Expr>,
    selective: Option<Expr>,
    monoid: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, FromDeriveInput)]
#[darling(attributes(semigroup_op))]
pub struct ContainerAttr {
    #[darling(flatten)]
    derive_attr: DeriveAttr,
    #[darling(rename = "impl", default)]
    impls: Impls,
}
impl ContainerAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }
    pub fn derive_attr(&self) -> &DeriveAttr {
        &self.derive_attr
    }
    pub fn semigroup(&self) -> Option<&Expr> {
        self.impls.semigroup.as_ref()
    }
    pub fn selective(&self) -> Option<&Expr> {
        self.impls.selective.as_ref()
    }
    pub fn identity(&self) -> Option<&Expr> {
        self.impls.monoid.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::impls(
        syn::parse_quote! {
            #[derive(SemigroupOp, SelectiveOp, MonoidOp)]
            #[semigroup_op(
                impl(semigroup = Self::lift_select_assign, selective = |_, _| Selected::Base, monoid = Self(None)),
                bound = "T: Clone",
                crate = crate,
            )]
            pub struct Coalesce<T>(pub Option<T>);
        },
        Ok(ContainerAttr {
            derive_attr: <DeriveAttr as darling::FromMeta>::from_list(
                &darling::ast::NestedMeta::parse_meta_list(quote::quote! {
                    bound = "T: Clone", crate = crate
                })
                .unwrap(),
            )
            .unwrap(),
            impls: Impls {
                semigroup: Some(syn::parse_quote! { Self::lift_select_assign }),
                selective: Some(syn::parse_quote! { |_, _| Selected::Base }),
                monoid: Some(syn::parse_quote! { Self(None) }),
            },
        }),
    )]
    #[case::impl_of_marker(
        syn::parse_quote! {
            #[derive(CommutativeOp)]
            #[semigroup_op(impl(commutative))]
            pub struct Construct<T>(T);
        },
        Err("Unknown field: `commutative`. Available values: `monoid`, `selective`, `semigroup`"),
    )]
    #[case::legacy_properties(
        syn::parse_quote! {
            #[derive(SemigroupOp)]
            #[semigroup_op(properties(monoid))]
            pub struct Construct<T>(T);
        },
        Err("Unknown field: `properties`. Available values: `bound`, `crate`, `impl`"),
    )]
    fn test_op_container_attr(
        #[case] input: DeriveInput,
        #[case] expected: Result<ContainerAttr, &str>,
    ) {
        let actual = ContainerAttr::new(&input);
        assert_eq!(
            actual.as_ref().map_err(ToString::to_string),
            expected.as_ref().map_err(ToString::to_string),
        );
    }
}
