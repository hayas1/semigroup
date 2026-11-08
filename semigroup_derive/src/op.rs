mod ast;
pub mod attr;

pub use ast::Op;

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use syn::DeriveInput;

    use crate::derive::OpDerive;

    use super::*;

    #[rstest]
    #[case::op_semigroup(
        "op_semigroup",
        &[OpDerive::Semigroup, OpDerive::Monoid, OpDerive::Commutative],
        syn::parse_quote! {
            #[semigroup_op(
                impl(semigroup = |base, other| *base += other, monoid = Self(T::zero())),
                bound = "T: std::ops::AddAssign + num::Zero",
            )]
            pub struct Sum<T>(pub T);
        },
    )]
    #[case::op_selective(
        "op_selective",
        &[OpDerive::Semigroup, OpDerive::Selective, OpDerive::Monoid],
        syn::parse_quote! {
            #[semigroup_op(
                impl(
                    semigroup = Self::lift_select_assign,
                    selective = |base, other| match (base, other) {
                        (Some(_), _) | (None, None) => Selected::Base,
                        (None, Some(_)) => Selected::Other,
                    },
                    monoid = Self(None),
                ),
            )]
            pub struct Coalesce<T>(pub Option<T>);
        },
    )]
    #[case::op_bound_for_each_impl(
        "op_bound_for_each_impl",
        &[OpDerive::Semigroup, OpDerive::Selective, OpDerive::Monoid, OpDerive::Commutative],
        syn::parse_quote! {
            #[semigroup_op(
                impl(semigroup = Self::rop, selective = Self::rselect),
                bound(
                    semigroup = "T: crate::Semigroup",
                    selective = "T: crate::Selective",
                    monoid = "T: crate::Monoid",
                    commutative = "T: crate::Commutative",
                ),
                crate = crate,
            )]
            pub struct Dual<T>(pub T);
        },
    )]
    fn test_derive_op_snapshot(
        #[case] case: &str,
        #[case] derives: &[OpDerive],
        #[case] input: DeriveInput,
    ) {
        let generated: TokenStream = derives
            .iter()
            .map(|&derive| Op::new(&input, derive).unwrap().into_token_stream())
            .collect();
        let formatted = prettyplease::unparse(&syn::parse2(generated).unwrap());
        insta::with_settings!({ snapshot_path => "../tests/snapshots", prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!(case, formatted);
        });
    }

    #[rstest]
    #[case::not_newtype(
        syn::parse_quote! {
            #[derive(SemigroupOp)]
            #[semigroup_op(impl(semigroup = |_, _| ()))]
            pub struct Pair(u32, u32);
        },
        OpDerive::Semigroup,
        "derive SemigroupOp only supports newtype structs",
    )]
    #[case::semigroup_without_impl(
        syn::parse_quote! {
            #[derive(SemigroupOp)]
            pub struct Sum(u32);
        },
        OpDerive::Semigroup,
        "derive SemigroupOp requires `impl(semigroup = ..)`",
    )]
    #[case::selective_without_impl(
        syn::parse_quote! {
            #[derive(SelectiveOp)]
            #[semigroup_op(impl(semigroup = Self::lift_select_assign))]
            pub struct Max(u32);
        },
        OpDerive::Selective,
        "derive SelectiveOp requires `impl(selective = ..)`",
    )]
    fn test_derive_op_error(
        #[case] input: DeriveInput,
        #[case] op_derive: OpDerive,
        #[case] expected: &str,
    ) {
        let actual = Op::new(&input, op_derive)
            .map(|_| ())
            .map_err(|e| e.to_string());
        assert_eq!(actual, Err(expected.to_string()));
    }
}
