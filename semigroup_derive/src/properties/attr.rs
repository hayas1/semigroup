use syn::{DeriveInput, ItemStruct};

use crate::{
    attr::DeriveAttr,
    constant::{ATTR_OP, ATTR_SEMIGROUP},
    op, semigroup,
};

impl From<&ItemStruct> for DeriveAttr {
    // An attribute the derive rejects is reported by the derive, so it reads as no properties here.
    fn from(item: &ItemStruct) -> Self {
        let derive = DeriveInput::from(item.clone());
        let has = |name| item.attrs.iter().any(|a| a.path().is_ident(name));
        let derive_attr = if has(ATTR_OP) {
            op::attr::ContainerAttr::new(&derive)
                .ok()
                .map(|a| a.derive_attr().clone())
        } else if has(ATTR_SEMIGROUP) {
            semigroup::attr::ContainerAttr::new(&derive)
                .ok()
                .map(|a| a.derive_attr().clone())
        } else {
            None
        };
        derive_attr.unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::attr::Property;

    #[rstest]
    #[case::semigroup_op(
        syn::parse_quote! {
            #[semigroup_op(properties(monoid))]
            pub struct Sum<T>(pub T);
        },
        Property::Monoid,
        true,
    )]
    #[case::semigroup(
        syn::parse_quote! {
            #[semigroup(properties(commutative))]
            pub struct Config { pub num: u32 }
        },
        Property::Commutative,
        true,
    )]
    #[case::without_attribute(
        syn::parse_quote! {
            #[other(properties(monoid))]
            pub struct Sum<T>(pub T);
        },
        Property::Monoid,
        false,
    )]
    fn test_derive_attr_from_item(
        #[case] item: ItemStruct,
        #[case] property: Property,
        #[case] expected: bool,
    ) {
        assert_eq!(DeriveAttr::from(&item).implements(property), expected);
    }
}
