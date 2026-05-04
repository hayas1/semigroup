use darling::FromDeriveInput;
use syn::{DeriveInput, Expr, WherePredicate};

use crate::{error::ConstructionError, name::var_name};

#[derive(Debug, Clone, PartialEq, FromDeriveInput)]
#[darling(attributes(semigroup_op), and_then = Self::validate)]
pub struct ContainerAttr {
    semigroup_where: Option<String>, // TODO Vec

    #[darling(default)]
    idempotent: bool,
    idempotent_where: Option<String>, // TODO Vec

    #[darling(default)]
    monoid: bool,
    identity: Option<Expr>,
    monoid_where: Option<String>, // TODO Vec
    #[darling(default)]
    without_monoid_impl: bool,

    #[darling(default)]
    commutative: bool,
    commutative_where: Option<String>, // TODO Vec

    #[darling(default)]
    manual_op_impl: bool,

    #[darling(default)]
    hidden_inner: bool,
}
impl ContainerAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }
    pub fn validate(self) -> darling::Result<Self> {
        let Self {
            idempotent,
            idempotent_where,
            monoid,
            identity,
            monoid_where,
            without_monoid_impl,
            commutative,
            commutative_where,
            ..
        } = &self;
        if !idempotent && let Some(_) = idempotent_where {
            return Err(darling::Error::custom(ConstructionError::OnlyIdempotent(
                var_name!(idempotent_where),
            )));
        }
        if !monoid {
            let err_attr_name = if identity.is_some() {
                Some(var_name!(identity))
            } else if monoid_where.is_some() {
                Some(var_name!(monoid_where))
            } else if *without_monoid_impl {
                Some(var_name!(without_monoid_impl))
            } else {
                None
            };
            err_attr_name.map_or(Ok(()), |a| {
                Err(darling::Error::custom(ConstructionError::OnlyMonoid(a)))
            })?;
        }
        if !commutative {
            let err_attr_name = if commutative_where.is_some() {
                Some(var_name!(commutative_where))
            } else {
                None
            };
            err_attr_name.map_or(Ok(()), |a| {
                Err(darling::Error::custom(ConstructionError::OnlyCommutative(
                    a,
                )))
            })?;
        }
        Ok(self)
    }

    pub fn semigroup_where(&self) -> Option<WherePredicate> {
        self.semigroup_where
            .as_deref()
            .map(syn::parse_str)
            .map(|p| p.unwrap_or_else(|e| todo!("{e}")))
    }

    pub fn is_idempotent(&self) -> bool {
        self.idempotent
    }
    pub fn idempotent_where(&self) -> Option<WherePredicate> {
        self.idempotent_where
            .as_deref()
            .map(syn::parse_str)
            .map(|p| p.unwrap_or_else(|e| todo!("{e}")))
    }

    pub fn is_monoid(&self) -> bool {
        self.monoid
    }
    pub fn identity(&self) -> Option<&Expr> {
        self.identity.as_ref()
    }
    pub fn monoid_where(&self) -> Option<WherePredicate> {
        self.monoid_where
            .as_deref()
            .map(syn::parse_str)
            .map(|p| p.unwrap_or_else(|e| todo!("{e}")))
    }
    pub fn with_monoid_impl(&self) -> bool {
        !self.without_monoid_impl
    }

    pub fn is_commutative(&self) -> bool {
        self.commutative
    }
    pub fn commutative_where(&self) -> Option<WherePredicate> {
        self.commutative_where
            .as_deref()
            .map(syn::parse_str)
            .map(|p| p.unwrap_or_else(|e| todo!("{e}")))
    }

    pub fn gen_op_impl(&self) -> bool {
        !self.manual_op_impl
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
            #[semigroup_op(idempotent)]
            pub struct Coalesce<T>(pub Option<T>);
        },
        Ok(ContainerAttr {
            idempotent: true,
            ..default_container_attr()
        }),
    )]
    #[case::invalid_monoid_attr(
        syn::parse_quote! {
            #[derive(SemigroupOp)]
            #[semigroup_op(identity = ())]
            pub struct Construct<T>(T);
        },
        Err("attribute `identity` are supported only with `monoid`"),
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
