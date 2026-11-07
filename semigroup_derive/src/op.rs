use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

use crate::{
    constant::ConstantExt,
    op::{ast::Op, attr::ContainerAttr},
};

mod ast;
pub mod attr;

pub fn impl_op<C: ConstantExt>(derive: &DeriveInput) -> syn::Result<TokenStream> {
    let constant = C::constant();
    let attr = ContainerAttr::new(derive)?;
    let op = Op::new(&constant, derive, &attr)?;
    Ok(op.into_token_stream())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::constant::{External, Internal};

    use super::*;

    #[rstest]
    #[case::op_selective(
        "op_selective",
        impl_op::<External>,
        syn::parse_quote! {
            #[derive(SemigroupOp)]
            #[semigroup_op(properties(selective))]
            pub struct Coalesce<T>(pub Option<T>);
        },
    )]
    #[case::op_not_annotated(
        "op_not_annotated",
        impl_op::<Internal>,
        syn::parse_quote! {
            #[derive(SemigroupOpPriv)]
            #[semigroup_op(properties(monoid(identity = Default::default()), commutative))]
            pub struct Sum<T: std::ops::Add>(pub T);
        },
    )]
    #[case::op_where(
        "op_where",
        impl_op::<External>,
        syn::parse_quote! {
            #[derive(SemigroupOp)]
            #[semigroup_op(
                properties(monoid(identity = Self(T::default())), commutative),
                bound(
                    semigroup = "T: Clone",
                    monoid = "T: Default, T: Copy",
                    commutative = "T: Ord, T: Eq",
                ),
            )]
            pub struct Larger<T: PartialOrd>(pub T);
        },
    )]
    #[case::op_manual_selective(
        "op_manual_selective",
        impl_op::<External>,
        syn::parse_quote! {
            #[derive(SemigroupOp)]
            #[semigroup_op(
                properties(selective),
                manual_impl(selective),
                bound(selective = "T: Ord"),
            )]
            pub struct Larger<T>(pub T);
        },
    )]
    fn test_derive_op_snapshot(
        #[case] case: &str,
        #[case] f: impl Fn(&DeriveInput) -> syn::Result<TokenStream>,
        #[case] input: DeriveInput,
    ) {
        let generated = f(&input).unwrap();
        let formatted = prettyplease::unparse(&syn::parse2(generated).unwrap());
        insta::with_settings!({ snapshot_path => "../tests/snapshots", prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!(case, formatted);
        });
    }
}
