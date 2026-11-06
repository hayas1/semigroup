use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ItemStruct;

use crate::{
    constant::ConstantExt,
    properties::{attr::ContainerAttr, documented::Documented},
};

mod attr;
mod documented;

pub fn impl_properties<C: ConstantExt>(item: &ItemStruct) -> syn::Result<TokenStream> {
    let constant = C::constant();
    let attr = ContainerAttr::from_item(item);
    let documented = Documented::new(&constant, &attr, item)?;
    Ok(documented.to_token_stream())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::constant::{External, Internal};

    use super::*;

    #[rstest]
    #[case::properties_annotated(
        "properties_annotated",
        impl_properties::<Internal>,
        syn::parse_quote! {
            /// A semigroup construction that returns the first non-`None` value.
            /// # Properties
            /// <!-- properties -->
            #[derive(SemigroupOp)]
            #[semigroup_op(properties(selective, monoid))]
            pub struct Coalesce<T>(pub Option<T>);
        },
    )]
    #[case::properties_not_annotated(
        "properties_not_annotated",
        impl_properties::<External>,
        syn::parse_quote! {
            /// A semigroup struct that returns the sum and overwrite.
            /// # Properties
            /// <!-- properties -->
            #[derive(SemigroupPriv)]
            #[semigroup(with = "semigroup::op::Last")]
            pub struct UnnamedStruct<T: std::ops::Add> (
                #[semigroup(with = "semigroup::op::Added")]
                T,
                u64
            );
        },
    )]
    #[case::properties_where(
        "properties_where",
        impl_properties::<Internal>,
        syn::parse_quote! {
            /// A semigroup construction that returns the sum.
            /// # Properties
            /// <!-- properties -->
            #[derive(SemigroupOpPriv)]
            #[semigroup_op(
                properties(monoid(identity = Self(T::zero())), commutative),
                bound(monoid = "T: num::Zero"),
            )]
            pub struct Sum<T: Add<Output = T>>(pub T);
        },
    )]
    #[case::properties_bound_for_every_impl(
        "properties_bound_for_every_impl",
        impl_properties::<Internal>,
        syn::parse_quote! {
            /// A semigroup construction that returns the greater.
            /// # Properties
            /// <!-- properties -->
            #[derive(SemigroupOpPriv)]
            #[semigroup_op(
                properties(selective, monoid(identity = Self(T::min_value())), commutative),
                bound = "T: Ord + num::Bounded",
            )]
            pub struct Max<T>(pub T);
        },
    )]
    fn test_derive_properties_snapshot(
        #[case] case: &str,
        #[case] f: impl Fn(&ItemStruct) -> syn::Result<TokenStream>,
        #[case] input: ItemStruct,
    ) {
        let generated = f(&input).unwrap();
        let formatted = prettyplease::unparse(&syn::parse2(generated).unwrap());
        insta::with_settings!({ snapshot_path => "../tests/snapshots", prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!(case, formatted);
        });
    }
}
