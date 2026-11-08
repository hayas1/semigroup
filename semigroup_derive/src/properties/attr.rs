use syn::{Attribute, DeriveInput, ItemStruct, Path, Token, punctuated::Punctuated};

use crate::{
    attr::{Bound, DeriveAttr, Property},
    constant::Constant,
    derive::{OpDerive, SemigroupDerive},
    error::PropertiesError,
    op, semigroup,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Derive {
    Semigroup(SemigroupDerive),
    Op(OpDerive),
}
impl Derive {
    fn from_path(path: &Path) -> Option<Self> {
        SemigroupDerive::from_path(path)
            .map(Self::Semigroup)
            .or_else(|| OpDerive::from_path(path).map(Self::Op))
    }
    fn listed(attrs: &[Attribute]) -> syn::Result<Vec<Self>> {
        let mut listed = Vec::new();
        for attr in attrs.iter().filter(|attr| attr.path().is_ident("derive")) {
            let paths = attr.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)?;
            listed.extend(paths.iter().filter_map(Self::from_path));
        }
        Ok(listed)
    }
    fn property(self) -> Property {
        match self {
            Self::Semigroup(derive) => derive.property(),
            Self::Op(derive) => derive.property(),
        }
    }
    fn derive_attr(self, input: &DeriveInput) -> syn::Result<DeriveAttr> {
        Ok(match self {
            Self::Semigroup(_) => semigroup::attr::ContainerAttr::new(input)?
                .derive_attr()
                .clone(),
            Self::Op(_) => op::attr::ContainerAttr::new(input)?.derive_attr().clone(),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct PropertiesAttr {
    listed: Vec<Derive>,
    derive_attr: DeriveAttr,
}
impl TryFrom<&ItemStruct> for PropertiesAttr {
    type Error = syn::Error;
    fn try_from(item: &ItemStruct) -> syn::Result<Self> {
        let listed = Derive::listed(&item.attrs)?;
        let Some(&derive) = listed.first() else {
            return Err(syn::Error::new_spanned(
                &item.ident,
                PropertiesError::DeriveNotFound,
            ));
        };
        // An attribute the derive rejects is reported by the derive, so it reads as no bounds here.
        let derive_attr = derive
            .derive_attr(&DeriveInput::from(item.clone()))
            .unwrap_or_default();
        Ok(Self {
            listed,
            derive_attr,
        })
    }
}
impl PropertiesAttr {
    pub fn constant(&self) -> Constant {
        self.derive_attr.constant()
    }
    pub fn implements(&self, property: Property) -> bool {
        self.listed
            .iter()
            .any(|derive| derive.property() == property)
    }
    pub fn bound(&self, property: Property) -> Option<&Bound> {
        self.derive_attr.bound(property)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::op(
        syn::parse_quote! {
            #[derive(SemigroupOp, MonoidOp)]
            #[semigroup_op(impl(semigroup = |b, o| *b += o, monoid = Self(0)), bound(monoid = "T: Zero"))]
            pub struct Sum<T>(pub T);
        },
        Property::Monoid,
        (true, Some("T: Zero")),
    )]
    #[case::semigroup(
        syn::parse_quote! {
            #[derive(Semigroup, Commutative)]
            #[semigroup(bound = "T: Ord")]
            pub struct Config<T> { pub num: T }
        },
        Property::Commutative,
        (true, Some("T: Ord")),
    )]
    #[case::not_listed(
        syn::parse_quote! {
            #[derive(Semigroup)]
            pub struct Config { pub num: u32 }
        },
        Property::Monoid,
        (false, None),
    )]
    fn test_properties_attr(
        #[case] item: ItemStruct,
        #[case] property: Property,
        #[case] expected: (bool, Option<&str>),
    ) {
        let attr = PropertiesAttr::try_from(&item).unwrap();
        assert_eq!(
            (
                attr.implements(property),
                attr.bound(property).map(Bound::raw)
            ),
            expected,
        );
    }

    #[test]
    fn test_properties_attr_without_derive() {
        let item: ItemStruct = syn::parse_quote! {
            #[semigroup_op(impl(semigroup = |b, o| *b += o))]
            pub struct Sum(pub u32);
        };
        let actual = PropertiesAttr::try_from(&item).map_err(|e| e.to_string());
        assert_eq!(
            actual.map(|_| ()),
            Err(PropertiesError::DeriveNotFound.to_string()),
        );
    }
}
