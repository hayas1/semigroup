use darling::FromDeriveInput;
use syn::{DeriveInput, Expr, WherePredicate};

use crate::{
    attr::{Bound, PropertiesAttr, resolve_bound},
    error::ConstructionError,
    name::Name,
};

/// `#[semigroup_op(properties(..), bound = "..", hidden_inner)]`
#[derive(Debug, Clone, PartialEq, FromDeriveInput)]
#[darling(attributes(semigroup_op), and_then = Self::validate)]
pub struct ContainerAttr {
    #[darling(default)]
    properties: PropertiesAttr,
    bound: Option<Bound>,
    #[darling(default)]
    hidden_inner: bool,
}
impl ContainerAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }
    pub fn validate(self) -> darling::Result<Self> {
        let Self { properties, .. } = &self;
        if properties.annotated().is_some() {
            return Err(darling::Error::custom(
                ConstructionError::UnsupportedProperty(Name("annotated")),
            ));
        }
        if properties.semigroup().is_some_and(|p| p.manual_impl) && !self.is_selective() {
            return Err(darling::Error::custom(
                ConstructionError::ManualSemigroupWithoutSelective,
            ));
        }
        Ok(self)
    }

    /// Bounds of the `Semigroup` impl, and of the `SemigroupOp` impl derived from `SelectiveOp`.
    pub fn semigroup_bound(&self) -> &[WherePredicate] {
        resolve_bound(self.properties.semigroup_bound(), self.bound.as_ref()).unwrap_or_default()
    }
    /// `SemigroupOp` is derived from `SelectiveOp` unless `semigroup(manual_impl)`.
    pub fn gen_semigroup_op_impl(&self) -> bool {
        !self.properties.semigroup().is_some_and(|p| p.manual_impl)
    }

    pub fn is_selective(&self) -> bool {
        self.properties.selective().is_some()
    }
    pub fn selective_bound(&self) -> &[WherePredicate] {
        resolve_bound(self.properties.selective_bound(), self.bound.as_ref()).unwrap_or_default()
    }
    pub fn gen_selective_impl(&self) -> bool {
        !self.properties.selective().is_some_and(|p| p.manual_impl)
    }

    pub fn is_monoid(&self) -> bool {
        self.properties.monoid().is_some()
    }
    pub fn identity(&self) -> Option<&Expr> {
        self.properties.monoid().and_then(|p| p.identity.as_ref())
    }
    /// `None` when nothing is given, so that the derive infers the bounds.
    pub fn monoid_bound(&self) -> Option<&[WherePredicate]> {
        resolve_bound(self.properties.monoid_bound(), self.bound.as_ref())
    }
    pub fn gen_monoid_impl(&self) -> bool {
        !self.properties.monoid().is_some_and(|p| p.manual_impl)
    }

    pub fn is_commutative(&self) -> bool {
        self.properties.commutative().is_some()
    }
    pub fn commutative_bound(&self) -> &[WherePredicate] {
        resolve_bound(self.properties.commutative_bound(), self.bound.as_ref()).unwrap_or_default()
    }

    pub fn open_inner(&self) -> bool {
        !self.hidden_inner
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use syn::parse_quote;

    use super::*;

    fn default_container_attr() -> ContainerAttr {
        ContainerAttr::new(&parse_quote! {
            #[derive(SemigroupOp)]
            pub struct Construct<T>(T);
        })
        .unwrap()
    }

    #[rstest]
    #[case::ok(
        syn::parse_quote! {
            #[derive(SemigroupOp)]
            #[semigroup_op(properties(selective), bound = "T: Clone")]
            pub struct Coalesce<T>(pub Option<T>);
        },
        Ok(ContainerAttr {
            properties: PropertiesAttr {
                selective: Some(Default::default()),
                ..Default::default()
            },
            bound: Some(<Bound as darling::FromMeta>::from_value(&parse_quote!("T: Clone")).unwrap()),
            ..default_container_attr()
        }),
    )]
    #[case::property_at_top_level(
        syn::parse_quote! {
            #[derive(SemigroupOp)]
            #[semigroup_op(monoid)]
            pub struct Construct<T>(T);
        },
        Err("Unknown field: `monoid`. Available values: `bound`, `hidden_inner`, `properties`"),
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
            #[semigroup_op(properties(semigroup(manual_impl)))]
            pub struct Construct<T>(T);
        },
        Err("`semigroup(manual_impl)` of derive SemigroupOp only skips the `SemigroupOp` impl derived from `SelectiveOp`, so it requires `selective`"),
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
