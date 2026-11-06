use darling::FromDeriveInput;
use syn::{DeriveInput, Expr, WherePredicate};

use crate::{
    attr::{CommonAttr, predicates},
    error::ConstructionError,
    name::Name,
};

/// `#[semigroup_op(properties(..), manual_impl(..), bound(..), hidden_inner)]`, see [`crate::attr`].
#[derive(Debug, Clone, PartialEq, FromDeriveInput)]
#[darling(attributes(semigroup_op), and_then = Self::validate)]
pub struct ContainerAttr {
    #[darling(flatten)]
    common: CommonAttr,
    #[darling(default)]
    hidden_inner: bool,
}
impl ContainerAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }
    pub fn validate(self) -> darling::Result<Self> {
        let Self { common, .. } = &self;
        if common.is_annotated() {
            return Err(darling::Error::custom(
                ConstructionError::UnsupportedProperty(Name("annotated")),
            ));
        }
        if common.manual_semigroup_impl() && !common.is_selective() {
            return Err(darling::Error::custom(
                ConstructionError::ManualSemigroupWithoutSelective,
            ));
        }
        Ok(self)
    }
    pub fn common(&self) -> &CommonAttr {
        &self.common
    }

    /// Bounds of the `Semigroup` impl, and of the `SemigroupOp` impl derived from `SelectiveOp`.
    pub fn semigroup_bound(&self) -> &[WherePredicate] {
        predicates(self.common.semigroup_bound()).unwrap_or_default()
    }
    /// `SemigroupOp` is derived from `SelectiveOp` unless `manual_impl(semigroup)`.
    pub fn gen_semigroup_op_impl(&self) -> bool {
        !self.common.manual_semigroup_impl()
    }

    pub fn is_selective(&self) -> bool {
        self.common.is_selective()
    }
    pub fn selective_bound(&self) -> &[WherePredicate] {
        predicates(self.common.selective_bound()).unwrap_or_default()
    }
    pub fn gen_selective_impl(&self) -> bool {
        !self.common.manual_selective_impl()
    }

    pub fn is_monoid(&self) -> bool {
        self.common.is_monoid()
    }
    pub fn identity(&self) -> Option<&Expr> {
        self.common.identity()
    }
    /// `None` when nothing is given, so that the derive infers the bounds.
    pub fn monoid_bound(&self) -> Option<&[WherePredicate]> {
        predicates(self.common.monoid_bound())
    }
    pub fn gen_monoid_impl(&self) -> bool {
        !self.common.manual_monoid_impl()
    }

    pub fn is_commutative(&self) -> bool {
        self.common.is_commutative()
    }
    pub fn commutative_bound(&self) -> &[WherePredicate] {
        predicates(self.common.commutative_bound()).unwrap_or_default()
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
            common: <CommonAttr as darling::FromMeta>::from_list(
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
