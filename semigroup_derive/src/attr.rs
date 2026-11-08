use std::{
    collections::{BTreeMap, btree_map::Entry},
    fmt::{Display, Formatter},
};

use darling::{FromMeta, ast::NestedMeta};
use syn::{Generics, Meta, Path, Token, WherePredicate, parse_quote, punctuated::Punctuated};

use crate::constant::Constant;

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

    fn parse_list(items: &[NestedMeta]) -> darling::Result<BTreeMap<Self, &Meta>> {
        let mut parsed = BTreeMap::new();
        let mut errors = darling::Error::accumulator();
        for item in items {
            if let Some((property, meta)) = errors.handle(Self::parse_item(item)) {
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

    fn parse_item(item: &NestedMeta) -> darling::Result<(Self, &Meta)> {
        let NestedMeta::Meta(meta) = item else {
            return Err(darling::Error::unsupported_format("literal").with_span(item));
        };
        Self::ALL
            .into_iter()
            .find(|p| meta.path().is_ident(p.name()))
            .map(|p| (p, meta))
            .ok_or_else(|| {
                let mut names: Vec<_> = Self::ALL.into_iter().map(Self::name).collect();
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
        let parsed = Property::parse_list(items)?;
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
}

#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default)]
pub struct DeriveAttr {
    bound: BoundAttr,
    #[darling(rename = "crate")]
    krate: Option<Path>,
}
impl DeriveAttr {
    pub fn constant(&self) -> Constant {
        Constant::new(&self.krate())
    }
    pub fn krate(&self) -> Path {
        self.krate
            .clone()
            .unwrap_or_else(|| parse_quote! { ::semigroup })
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
    #[case::empty(quote::quote! {}, Ok(DeriveAttr::default()))]
    #[case::bound_for_each_impl(
        quote::quote! { bound(semigroup = "T: Clone", monoid = "T: Zero, T: Copy") },
        Ok(DeriveAttr {
            bound: BoundAttr::Each(BTreeMap::from([
                (Property::Semigroup, bound("T: Clone")),
                (Property::Monoid, bound("T: Zero, T: Copy")),
            ])),
            ..Default::default()
        }),
    )]
    #[case::bound_for_every_impl(
        quote::quote! { bound = "T: Clone" },
        Ok(DeriveAttr {
            bound: BoundAttr::Every(bound("T: Clone")),
            ..Default::default()
        }),
    )]
    #[case::krate(
        quote::quote! { crate = crate },
        Ok(DeriveAttr {
            krate: Some(parse_quote! { crate }),
            ..Default::default()
        }),
    )]
    #[case::invalid_bound(
        quote::quote! { bound(monoid = "T Zero") },
        Err("expected `:` at bound/monoid"),
    )]
    #[case::unknown_bound(
        quote::quote! { bound(monoids = "T: Zero") },
        Err("Unknown field: `monoids`. Did you mean `monoid`? at bound"),
    )]
    #[case::duplicate_bound(
        quote::quote! { bound(monoid = "T: Zero", monoid = "T: One") },
        Err("Duplicate field `monoid` at bound"),
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

    #[test]
    fn test_krate_default() {
        let expected: Path = parse_quote! { ::semigroup };
        assert_eq!(DeriveAttr::default().krate(), expected);
    }
}
