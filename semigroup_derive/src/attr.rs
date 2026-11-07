use std::{
    collections::{BTreeMap, BTreeSet, btree_map::Entry},
    fmt::{Display, Formatter},
};

use darling::{FromMeta, ast::NestedMeta};
use syn::{
    Expr, Generics, Ident, Meta, Token, WherePredicate, parse_quote, punctuated::Punctuated,
};

use crate::error::AttrError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Property {
    Semigroup,
    Annotated,
    Selective,
    Monoid,
    Commutative,
}
impl Display for Property {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
impl Property {
    const ALL: [Self; 5] = [
        Self::Semigroup,
        Self::Annotated,
        Self::Selective,
        Self::Monoid,
        Self::Commutative,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Semigroup => "semigroup",
            Self::Annotated => "annotated",
            Self::Selective => "selective",
            Self::Monoid => "monoid",
            Self::Commutative => "commutative",
        }
    }

    // `annotated` generates a struct rather than an impl, and `commutative` is a marker with nothing to write.
    fn implementable_by_hand(self) -> bool {
        matches!(self, Self::Semigroup | Self::Selective | Self::Monoid)
    }

    fn parse_list(
        items: &[NestedMeta],
        accepts: impl Fn(Self) -> bool,
    ) -> darling::Result<BTreeMap<Self, &Meta>> {
        let mut parsed = BTreeMap::new();
        let mut errors = darling::Error::accumulator();
        for item in items {
            if let Some((property, meta)) = errors.handle(Self::parse_item(item, &accepts)) {
                match parsed.entry(property) {
                    Entry::Vacant(entry) => {
                        entry.insert(meta);
                    }
                    Entry::Occupied(_) => errors
                        .push(darling::Error::duplicate_field(property.name()).with_span(meta)),
                }
            }
        }
        errors.finish_with(parsed)
    }

    fn parse_item(
        item: &NestedMeta,
        accepts: impl Fn(Self) -> bool,
    ) -> darling::Result<(Self, &Meta)> {
        let NestedMeta::Meta(meta) = item else {
            return Err(darling::Error::unsupported_format("literal").with_span(item));
        };
        let accepted = Self::ALL.into_iter().filter(|&p| accepts(p));
        accepted
            .clone()
            .find(|p| meta.path().is_ident(p.name()))
            .map(|p| (p, meta))
            .ok_or_else(|| {
                let mut names: Vec<_> = accepted.map(Self::name).collect();
                names.sort();
                let field = quote::ToTokens::to_token_stream(meta.path()).to_string();
                darling::Error::unknown_field_with_alts(&field, &names).with_span(meta.path())
            })
    }
}

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
    pub fn raw(&self) -> &str {
        &self.raw
    }
}

pub type SemigroupArgs = ();

#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default, from_word = || Ok(Self::default()))]
pub struct AnnotatedArgs {
    param: Option<Ident>,
}

pub type SelectiveArgs = ();

#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default, from_word = || Ok(Self::default()))]
pub struct MonoidArgs {
    identity: Option<Expr>,
}

pub type CommutativeArgs = ();

#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default)]
pub struct Properties {
    semigroup: Option<SemigroupArgs>,
    annotated: Option<AnnotatedArgs>,
    selective: Option<SelectiveArgs>,
    monoid: Option<MonoidArgs>,
    commutative: Option<CommutativeArgs>,
}
impl Properties {
    fn lists(&self, property: Property) -> bool {
        match property {
            Property::Semigroup => self.semigroup.is_some(),
            Property::Annotated => self.annotated.is_some(),
            Property::Selective => self.selective.is_some(),
            Property::Monoid => self.monoid.is_some(),
            Property::Commutative => self.commutative.is_some(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ManualImpl(BTreeSet<Property>);
impl FromMeta for ManualImpl {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let parsed = Property::parse_list(items, Property::implementable_by_hand)?;
        let mut manual = BTreeSet::new();
        let mut errors = darling::Error::accumulator();
        for (property, meta) in parsed {
            if errors
                .handle(<()>::from_meta(meta).map_err(|e| e.at(property.name())))
                .is_some()
            {
                manual.insert(property);
            }
        }
        errors.finish_with(Self(manual))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoundAttr {
    Every(Bound),
    Each(BTreeMap<Property, Bound>),
}
impl Default for BoundAttr {
    fn default() -> Self {
        Self::Each(BTreeMap::new())
    }
}
impl FromMeta for BoundAttr {
    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        Ok(Self::Every(Bound::from_value(value)?))
    }
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let parsed = Property::parse_list(items, |_| true)?;
        let mut each = BTreeMap::new();
        let mut errors = darling::Error::accumulator();
        for (property, meta) in parsed {
            if let Some(bound) =
                errors.handle(Bound::from_meta(meta).map_err(|e| e.at(property.name())))
            {
                each.insert(property, bound);
            }
        }
        errors.finish_with(Self::Each(each))
    }
}
impl BoundAttr {
    fn get(&self, property: Property) -> Option<&Bound> {
        match self {
            Self::Every(bound) => Some(bound),
            Self::Each(each) => each.get(&property),
        }
    }
    fn named(&self) -> impl Iterator<Item = &Property> {
        let each = match self {
            Self::Every(_) => None,
            Self::Each(each) => Some(each),
        };
        each.into_iter().flat_map(BTreeMap::keys)
    }
}

#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default, and_then = Self::validate)]
pub struct DeriveAttr {
    properties: Properties,
    manual_impl: ManualImpl,
    bound: BoundAttr,
}
impl DeriveAttr {
    fn validate(self) -> darling::Result<Self> {
        let mut errors = darling::Error::accumulator();
        for &property in self.manual_impl.0.iter().filter(|&&p| !self.implements(p)) {
            errors.push(darling::Error::custom(
                AttrError::ManualImplWithoutProperty(property),
            ));
        }
        for &property in self.bound.named().filter(|&&p| !self.implements(p)) {
            errors.push(darling::Error::custom(AttrError::BoundWithoutProperty(
                property,
            )));
        }
        errors.finish_with(self)
    }

    pub fn implements(&self, property: Property) -> bool {
        property == Property::Semigroup || self.properties.lists(property)
    }
    pub fn derives(&self, property: Property) -> bool {
        self.implements(property) && !self.manual_impl.0.contains(&property)
    }

    pub fn annotation_param(&self) -> Ident {
        self.properties
            .annotated
            .as_ref()
            .and_then(|annotated| annotated.param.clone())
            .unwrap_or_else(|| parse_quote! { A })
    }
    pub fn identity(&self) -> Option<&Expr> {
        self.properties.monoid.as_ref()?.identity.as_ref()
    }

    pub fn bound(&self, property: Property) -> Option<&Bound> {
        self.bound.get(property)
    }
    pub fn generics(
        &self,
        property: Property,
        generics: &Generics,
        inferred: impl FnOnce() -> Vec<WherePredicate>,
    ) -> Generics {
        let mut generics = generics.clone();
        generics
            .make_where_clause()
            .predicates
            .extend(self.predicates(property, inferred));
        generics
    }
    fn predicates(
        &self,
        property: Property,
        inferred: impl FnOnce() -> Vec<WherePredicate>,
    ) -> Vec<WherePredicate> {
        self.bound(property)
            .map_or_else(inferred, |bound| bound.predicates.clone())
    }
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
        Ok(DeriveAttr {
            properties: Properties {
                selective: Some(()),
                monoid: Some(MonoidArgs::default()),
                commutative: Some(()),
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
        Ok(DeriveAttr {
            properties: Properties {
                annotated: Some(AnnotatedArgs { param: Some(syn::parse_quote! { B }) }),
                selective: Some(()),
                monoid: Some(MonoidArgs { identity: Some(syn::parse_quote! { Self(T::zero()) }) }),
                ..Default::default()
            },
            manual_impl: ManualImpl(BTreeSet::from([Property::Semigroup, Property::Selective])),
            bound: BoundAttr::Each(BTreeMap::from([(Property::Monoid, bound("T: Zero, T: Copy"))])),
        }),
    )]
    #[case::bound_for_every_impl(
        quote::quote! { bound = "T: Clone" },
        Ok(DeriveAttr {
            bound: BoundAttr::Every(bound("T: Clone")),
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
    #[case::unknown_property(
        quote::quote! { properties(monoids) },
        Err("Unknown field: `monoids`. Did you mean `monoid`? at properties"),
    )]
    #[case::duplicate_property(
        quote::quote! { properties(monoid, monoid) },
        Err("Duplicate field `monoid` at properties"),
    )]
    #[case::manual_impl_of_marker(
        quote::quote! { properties(commutative), manual_impl(commutative) },
        Err("Unknown field: `commutative`. Available values: `monoid`, `selective`, `semigroup` at manual_impl"),
    )]
    #[case::unknown_bound(
        quote::quote! { bound(monoids = "T: Zero") },
        Err("Unknown field: `monoids`. Did you mean `monoid`? at bound"),
    )]
    #[case::manual_impl_without_property(
        quote::quote! { manual_impl(monoid) },
        Err("`manual_impl(monoid)` requires `monoid` in `properties(..)`"),
    )]
    #[case::bound_without_property(
        quote::quote! { bound(commutative = "T: Ord") },
        Err("`bound(commutative = ..)` requires `commutative` in `properties(..)`"),
    )]
    fn test_derive_attr(
        #[case] input: proc_macro2::TokenStream,
        #[case] expected: Result<DeriveAttr, &str>,
    ) {
        let items = darling::ast::NestedMeta::parse_meta_list(input).unwrap();
        let actual = DeriveAttr::from_list(&items);
        assert_eq!(
            actual.as_ref().map_err(ToString::to_string),
            expected.as_ref().map_err(ToString::to_string),
        );
    }
}
