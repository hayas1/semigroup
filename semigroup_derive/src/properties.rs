pub use crate::properties::documented::Documented;

mod attr;
pub mod documented;

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use rstest::rstest;
    use syn::ItemStruct;

    use super::*;

    #[rstest]
    #[case::properties_annotated(
        "properties_annotated",
        syn::parse_quote! {
            /// A semigroup construction that returns the first non-`None` value.
            /// # Properties
            /// <!-- properties -->
            #[derive(SemigroupOp, SelectiveOp, MonoidOp)]
            #[semigroup_op(
                impl(semigroup = Self::lift_select_assign, selective = |_, _| Selected::Base),
                crate = crate,
            )]
            pub struct Coalesce<T>(pub Option<T>);
        },
    )]
    #[case::properties_not_annotated(
        "properties_not_annotated",
        syn::parse_quote! {
            /// A semigroup struct that returns the sum and overwrite.
            /// # Properties
            /// <!-- properties -->
            #[derive(Semigroup)]
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
        syn::parse_quote! {
            /// A semigroup construction that returns the sum.
            /// # Properties
            /// <!-- properties -->
            #[derive(SemigroupOp, MonoidOp, CommutativeOp)]
            #[semigroup_op(
                impl(semigroup = |base, other| *base += other, monoid = Self(T::zero())),
                bound(monoid = "T: num::Zero"),
                crate = crate,
            )]
            pub struct Sum<T: Add<Output = T>>(pub T);
        },
    )]
    #[case::properties_bound_for_every_impl(
        "properties_bound_for_every_impl",
        syn::parse_quote! {
            /// A semigroup construction that returns the greater.
            /// # Properties
            /// <!-- properties -->
            #[derive(SemigroupOp, SelectiveOp, MonoidOp, CommutativeOp)]
            #[semigroup_op(
                impl(
                    semigroup = Self::lift_select_assign,
                    selective = |base, other| if other > base { Selected::Other } else { Selected::Base },
                    monoid = Self(T::min_value()),
                ),
                bound = "T: Ord + num::Bounded",
                crate = crate,
            )]
            pub struct Max<T>(pub T);
        },
    )]
    #[case::properties_annotated_bound(
        "properties_annotated_bound",
        syn::parse_quote! {
            /// A semigroup struct that keeps the annotation of the selected value.
            /// # Properties
            /// <!-- properties -->
            #[derive(Semigroup, AnnotateFields)]
            #[semigroup(bound(annotated = "T: Clone"), crate = crate)]
            pub struct Config<T> {
                pub value: T,
            }
        },
    )]
    fn test_derive_properties_snapshot(#[case] case: &str, #[case] input: ItemStruct) {
        let generated = Documented::new(&input).unwrap().into_token_stream();
        let formatted = prettyplease::unparse(&syn::parse2(generated).unwrap());
        insta::with_settings!({ snapshot_path => "../tests/snapshots", prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!(case, formatted);
        });
    }
}
