use darling::FromDeriveInput;
use syn::DeriveInput;

use crate::{
    attr::{DeriveAttr, Property},
    error::ConstructionError,
};

#[derive(Debug, Clone, PartialEq, FromDeriveInput)]
#[darling(attributes(semigroup_op), and_then = Self::validate)]
pub struct ContainerAttr {
    #[darling(flatten)]
    derive_attr: DeriveAttr,
    #[darling(default)]
    hidden_inner: bool,
}
impl ContainerAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }
    fn validate(self) -> darling::Result<Self> {
        let Self { derive_attr, .. } = &self;
        if derive_attr.implements(Property::Annotated) {
            return Err(darling::Error::custom(
                ConstructionError::UnsupportedProperty(Property::Annotated),
            ));
        }
        if !derive_attr.derives(Property::Semigroup) && !derive_attr.implements(Property::Selective)
        {
            return Err(darling::Error::custom(
                ConstructionError::ManualSemigroupWithoutSelective,
            ));
        }
        Ok(self)
    }
    pub fn derive_attr(&self) -> &DeriveAttr {
        &self.derive_attr
    }
    pub fn open_inner(&self) -> bool {
        !self.hidden_inner
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::ok(
        syn::parse_quote! {
            #[derive(SemigroupOp)]
            #[semigroup_op(properties(selective), bound = "T: Clone", hidden_inner)]
            pub struct Coalesce<T>(pub Option<T>);
        },
        Ok(ContainerAttr {
            derive_attr: <DeriveAttr as darling::FromMeta>::from_list(
                &darling::ast::NestedMeta::parse_meta_list(quote::quote! {
                    properties(selective), bound = "T: Clone"
                })
                .unwrap(),
            )
            .unwrap(),
            hidden_inner: true,
        }),
    )]
    #[case::property_at_top_level(
        syn::parse_quote! {
            #[derive(SemigroupOp)]
            #[semigroup_op(monoid)]
            pub struct Construct<T>(T);
        },
        Err("Unknown field: `monoid`. Available values: `bound`, `hidden_inner`, `manual_impl`, `properties`"),
    )]
    #[case::annotated(
        syn::parse_quote! {
            #[derive(SemigroupOp)]
            #[semigroup_op(properties(annotated))]
            pub struct Construct<T>(T);
        },
        Err("derive SemigroupOp does not support property `annotated`"),
    )]
    #[case::manual_semigroup_without_selective(
        syn::parse_quote! {
            #[derive(SemigroupOp)]
            #[semigroup_op(manual_impl(semigroup))]
            pub struct Construct<T>(T);
        },
        Err("`manual_impl(semigroup)` of derive SemigroupOp only skips the `SemigroupOp` impl derived from `SelectiveOp`, so it requires `selective`"),
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
