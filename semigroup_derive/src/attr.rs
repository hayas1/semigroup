//! Attributes shared by `#[derive(Semigroup)]`, `#[derive(SemigroupOp)]` and `#[properties]`.
//!
//! Each property groups everything about it in one place:
//!
//! ```text
//! properties(
//!     semigroup(bound = "..", manual_impl),
//!     annotated(param = A),
//!     selective(bound = "..", manual_impl),
//!     monoid(identity = .., bound = "..", manual_impl),
//!     commutative(bound = ".."),
//! )
//! ```
//!
//! A bare word such as `monoid` enables the property with its defaults.
//! Which properties a derive accepts is checked by the derive itself.

use darling::FromMeta;
use syn::{Expr, Ident, Token, WherePredicate, parse_quote, punctuated::Punctuated};

/// Where predicates written as a string, such as `bound = "T: Default, U: Copy"`.
///
/// As in serde, a bound replaces the predicates the derive would otherwise infer for that impl.
#[derive(Debug, Clone, PartialEq)]
pub struct Bound {
    raw: String,
    predicates: Vec<WherePredicate>,
}
impl FromMeta for Bound {
    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        match value {
            syn::Lit::Str(lit) => {
                let predicates =
                    lit.parse_with(Punctuated::<WherePredicate, Token![,]>::parse_terminated)?;
                Ok(Self {
                    raw: lit.value(),
                    predicates: predicates.into_iter().collect(),
                })
            }
            _ => Err(darling::Error::unexpected_lit_type(value)),
        }
    }
}
impl Bound {
    /// The string as written, for documentation.
    pub fn raw(&self) -> &str {
        &self.raw
    }
    pub fn predicates(&self) -> &[WherePredicate] {
        &self.predicates
    }
}

/// `semigroup(bound = "..", manual_impl)`
#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default, from_word = || Ok(Self::default()))]
pub struct SemigroupProp {
    pub bound: Option<Bound>,
    pub manual_impl: bool,
}

/// `annotated(param = A)`
#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default, from_word = || Ok(Self::default()))]
pub struct AnnotatedProp {
    pub param: Option<Ident>,
}
impl AnnotatedProp {
    /// The type parameter of the annotation, `A` unless given.
    pub fn param(&self) -> Ident {
        self.param.clone().unwrap_or_else(|| parse_quote! { A })
    }
}

/// `selective(bound = "..", manual_impl)`
#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default, from_word = || Ok(Self::default()))]
pub struct SelectiveProp {
    pub bound: Option<Bound>,
    pub manual_impl: bool,
}

/// `monoid(identity = .., bound = "..", manual_impl)`
#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default, from_word = || Ok(Self::default()))]
pub struct MonoidProp {
    pub identity: Option<Expr>,
    pub bound: Option<Bound>,
    pub manual_impl: bool,
}

/// `commutative(bound = "..")`
#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default, from_word = || Ok(Self::default()))]
pub struct CommutativeProp {
    pub bound: Option<Bound>,
}

/// The content of `properties(..)`, and of `#[properties(..)]` itself.
#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default, from_word = || Ok(Self::default()), derive_syn_parse)]
pub struct PropertiesAttr {
    pub semigroup: Option<SemigroupProp>,
    pub annotated: Option<AnnotatedProp>,
    pub selective: Option<SelectiveProp>,
    pub monoid: Option<MonoidProp>,
    pub commutative: Option<CommutativeProp>,
}
impl PropertiesAttr {
    pub fn semigroup(&self) -> Option<&SemigroupProp> {
        self.semigroup.as_ref()
    }
    pub fn annotated(&self) -> Option<&AnnotatedProp> {
        self.annotated.as_ref()
    }
    pub fn selective(&self) -> Option<&SelectiveProp> {
        self.selective.as_ref()
    }
    pub fn monoid(&self) -> Option<&MonoidProp> {
        self.monoid.as_ref()
    }
    pub fn commutative(&self) -> Option<&CommutativeProp> {
        self.commutative.as_ref()
    }

    pub fn semigroup_bound(&self) -> Option<&Bound> {
        self.semigroup().and_then(|p| p.bound.as_ref())
    }
    pub fn selective_bound(&self) -> Option<&Bound> {
        self.selective().and_then(|p| p.bound.as_ref())
    }
    pub fn monoid_bound(&self) -> Option<&Bound> {
        self.monoid().and_then(|p| p.bound.as_ref())
    }
    pub fn commutative_bound(&self) -> Option<&Bound> {
        self.commutative().and_then(|p| p.bound.as_ref())
    }
}

/// The predicates of an impl: the property's own bound, else the container's `bound`,
/// else `None` so that the derive infers them.
pub fn resolve_bound<'a>(
    property: Option<&'a Bound>,
    container: Option<&'a Bound>,
) -> Option<&'a [WherePredicate]> {
    property.or(container).map(Bound::predicates)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::words(
        quote::quote! { selective, monoid, commutative },
        Ok(PropertiesAttr {
            selective: Some(SelectiveProp::default()),
            monoid: Some(MonoidProp::default()),
            commutative: Some(CommutativeProp::default()),
            ..Default::default()
        }),
    )]
    #[case::lists(
        quote::quote! {
            selective(manual_impl),
            monoid(identity = Self(T::zero()), bound = "T: Zero, T: Copy"),
            annotated(param = B),
        },
        Ok(PropertiesAttr {
            selective: Some(SelectiveProp { manual_impl: true, ..Default::default() }),
            monoid: Some(MonoidProp {
                identity: Some(syn::parse_quote! { Self(T::zero()) }),
                bound: Some(Bound {
                    raw: "T: Zero, T: Copy".to_string(),
                    predicates: vec![syn::parse_quote! { T: Zero }, syn::parse_quote! { T: Copy }],
                }),
                manual_impl: false,
            }),
            annotated: Some(AnnotatedProp { param: Some(syn::parse_quote! { B }) }),
            ..Default::default()
        }),
    )]
    #[case::typo(
        quote::quote! { monoid(identiti = Self(0)) },
        Err("Unknown field: `identiti`. Did you mean `identity`? at monoid"),
    )]
    #[case::invalid_bound(
        quote::quote! { monoid(bound = "T Zero") },
        Err("expected `:` at monoid/bound"),
    )]
    fn test_properties_attr(
        #[case] input: proc_macro2::TokenStream,
        #[case] expected: Result<PropertiesAttr, &str>,
    ) {
        let items = darling::ast::NestedMeta::parse_meta_list(input).unwrap();
        let actual = PropertiesAttr::from_list(&items);
        assert_eq!(
            actual.as_ref().map_err(ToString::to_string),
            expected.as_ref().map_err(ToString::to_string),
        );
    }
}
