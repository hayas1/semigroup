//! Attributes shared by `#[derive(Semigroup)]` and `#[derive(SemigroupOp)]`, also read by `#[semigroup_properties_doc]`.
//!
//! ```text
//! properties(semigroup, annotated(param = A), selective, monoid(identity = ..), commutative),
//! manual_impl(semigroup, selective, monoid),
//! bound = "..",  // every impl, or each impl:
//! bound(semigroup = "..", annotated = "..", selective = "..", monoid = "..", commutative = ".."),
//! ```
//!
//! `properties(..)` lists the properties to derive; only values that belong to a property,
//! such as the identity of a monoid, are written inside it.
//! Which properties and `manual_impl`s a derive accepts is checked by the derive itself.

use darling::FromMeta;
use syn::{Expr, Ident, Token, WherePredicate, parse_quote, punctuated::Punctuated};

use crate::{error::AttrError, name::Name};

/// Where predicates written as a string, such as `"T: Default, U: Copy"`.
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

/// `annotated(param = A)`
#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default, from_word = || Ok(Self::default()))]
pub struct AnnotatedProp {
    pub param: Option<Ident>,
}

/// `monoid(identity = ..)`
#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default, from_word = || Ok(Self::default()))]
pub struct MonoidProp {
    pub identity: Option<Expr>,
}

/// `properties(semigroup, annotated, selective, monoid, commutative)`
#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default)]
pub struct PropertiesAttr {
    pub semigroup: bool,
    pub annotated: Option<AnnotatedProp>,
    pub selective: bool,
    pub monoid: Option<MonoidProp>,
    pub commutative: bool,
}

/// `manual_impl(semigroup, selective, monoid)`
#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default)]
pub struct ManualImplAttr {
    pub semigroup: bool,
    pub selective: bool,
    pub monoid: bool,
}

/// The impls `bound(..)` can name.
#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default)]
pub struct BoundEach {
    pub semigroup: Option<Bound>,
    pub annotated: Option<Bound>,
    pub selective: Option<Bound>,
    pub monoid: Option<Bound>,
    pub commutative: Option<Bound>,
}

/// `bound = ".."` for every impl, or `bound(monoid = "..", ..)` for each impl, as in serde.
/// It is one attribute, so the two forms are not combined; an impl `bound(..)` does not name infers its bounds.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoundAttr {
    all: Option<Bound>,
    each: BoundEach,
}
impl FromMeta for BoundAttr {
    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        Ok(Self {
            all: Some(Bound::from_value(value)?),
            ..Default::default()
        })
    }
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        Ok(Self {
            each: BoundEach::from_list(items)?,
            ..Default::default()
        })
    }
}

/// The part of `#[semigroup(..)]` and `#[semigroup_op(..)]` about properties.
#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default, and_then = Self::validate)]
pub struct CommonAttr {
    properties: PropertiesAttr,
    manual_impl: ManualImplAttr,
    bound: BoundAttr,
}
impl CommonAttr {
    /// `manual_impl(..)` and `bound(..)` may only name a property that `properties(..)` lists.
    /// `semigroup` is always derived, so it needs no listing.
    pub fn validate(self) -> darling::Result<Self> {
        let Self {
            properties,
            manual_impl,
            bound: BoundAttr { each, .. },
        } = &self;
        let listed = [
            ("annotated", properties.annotated.is_some()),
            ("selective", properties.selective),
            ("monoid", properties.monoid.is_some()),
            ("commutative", properties.commutative),
        ];
        let manual = [
            ("selective", manual_impl.selective),
            ("monoid", manual_impl.monoid),
        ];
        let bounded = [
            ("annotated", each.annotated.is_some()),
            ("selective", each.selective.is_some()),
            ("monoid", each.monoid.is_some()),
            ("commutative", each.commutative.is_some()),
        ];
        let is_listed = |name| listed.iter().any(|&(n, l)| n == name && l);
        let mut errors = darling::Error::accumulator();
        for (name, _) in manual.iter().filter(|&&(n, m)| m && !is_listed(n)) {
            errors.push(darling::Error::custom(
                AttrError::ManualImplWithoutProperty(Name(name)),
            ));
        }
        for (name, _) in bounded.iter().filter(|&&(n, b)| b && !is_listed(n)) {
            errors.push(darling::Error::custom(AttrError::BoundWithoutProperty(
                Name(name),
            )));
        }
        errors.finish_with(self)
    }

    fn bound_of<'a>(&'a self, each: Option<&'a Bound>) -> Option<&'a Bound> {
        each.or(self.bound.all.as_ref())
    }

    pub fn semigroup_bound(&self) -> Option<&Bound> {
        self.bound_of(self.bound.each.semigroup.as_ref())
    }
    pub fn manual_semigroup_impl(&self) -> bool {
        self.manual_impl.semigroup
    }

    pub fn is_annotated(&self) -> bool {
        self.properties.annotated.is_some()
    }
    /// The type parameter of the annotation, `A` unless given.
    pub fn annotation_param(&self) -> Ident {
        self.properties
            .annotated
            .as_ref()
            .and_then(|p| p.param.clone())
            .unwrap_or_else(|| parse_quote! { A })
    }
    pub fn annotated_bound(&self) -> Option<&Bound> {
        self.bound_of(self.bound.each.annotated.as_ref())
    }

    pub fn is_selective(&self) -> bool {
        self.properties.selective
    }
    pub fn selective_bound(&self) -> Option<&Bound> {
        self.bound_of(self.bound.each.selective.as_ref())
    }
    pub fn manual_selective_impl(&self) -> bool {
        self.manual_impl.selective
    }

    pub fn is_monoid(&self) -> bool {
        self.properties.monoid.is_some()
    }
    pub fn identity(&self) -> Option<&Expr> {
        self.properties
            .monoid
            .as_ref()
            .and_then(|p| p.identity.as_ref())
    }
    pub fn monoid_bound(&self) -> Option<&Bound> {
        self.bound_of(self.bound.each.monoid.as_ref())
    }
    pub fn manual_monoid_impl(&self) -> bool {
        self.manual_impl.monoid
    }

    pub fn is_commutative(&self) -> bool {
        self.properties.commutative
    }
    pub fn commutative_bound(&self) -> Option<&Bound> {
        self.bound_of(self.bound.each.commutative.as_ref())
    }
}

/// The predicates of a bound, or `None` so that the derive infers them.
pub fn predicates(bound: Option<&Bound>) -> Option<&[WherePredicate]> {
    bound.map(Bound::predicates)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn bound(raw: &str) -> Bound {
        Bound::from_value(&syn::Lit::Str(syn::LitStr::new(
            raw,
            proc_macro2::Span::call_site(),
        )))
        .unwrap()
    }

    #[rstest]
    #[case::words(
        quote::quote! { properties(selective, monoid, commutative) },
        Ok(CommonAttr {
            properties: PropertiesAttr {
                selective: true,
                monoid: Some(MonoidProp::default()),
                commutative: true,
                ..Default::default()
            },
            ..Default::default()
        }),
    )]
    #[case::lists(
        quote::quote! {
            properties(annotated(param = B), selective, monoid(identity = Self(T::zero()))),
            manual_impl(semigroup, selective),
            bound(monoid = "T: Zero, T: Copy"),
        },
        Ok(CommonAttr {
            properties: PropertiesAttr {
                annotated: Some(AnnotatedProp { param: Some(syn::parse_quote! { B }) }),
                selective: true,
                monoid: Some(MonoidProp { identity: Some(syn::parse_quote! { Self(T::zero()) }) }),
                ..Default::default()
            },
            manual_impl: ManualImplAttr {
                semigroup: true,
                selective: true,
                ..Default::default()
            },
            bound: BoundAttr {
                each: BoundEach { monoid: Some(bound("T: Zero, T: Copy")), ..Default::default() },
                ..Default::default()
            },
        }),
    )]
    #[case::bound_for_every_impl(
        quote::quote! { bound = "T: Clone" },
        Ok(CommonAttr {
            bound: BoundAttr { all: Some(bound("T: Clone")), ..Default::default() },
            ..Default::default()
        }),
    )]
    #[case::typo(
        quote::quote! { properties(monoid(identiti = Self(0))) },
        Err("Unknown field: `identiti`. Did you mean `identity`? at properties/monoid"),
    )]
    #[case::invalid_bound(
        quote::quote! { properties(monoid), bound(monoid = "T Zero") },
        Err("expected `:` at bound/monoid"),
    )]
    #[case::manual_impl_without_property(
        quote::quote! { manual_impl(monoid) },
        Err("`manual_impl(monoid)` requires `monoid` in `properties(..)`"),
    )]
    #[case::bound_without_property(
        quote::quote! { bound(commutative = "T: Ord") },
        Err("`bound(commutative = ..)` requires `commutative` in `properties(..)`"),
    )]
    fn test_common_attr(
        #[case] input: proc_macro2::TokenStream,
        #[case] expected: Result<CommonAttr, &str>,
    ) {
        let items = darling::ast::NestedMeta::parse_meta_list(input).unwrap();
        let actual = CommonAttr::from_list(&items);
        assert_eq!(
            actual.as_ref().map_err(ToString::to_string),
            expected.as_ref().map_err(ToString::to_string),
        );
    }
}
