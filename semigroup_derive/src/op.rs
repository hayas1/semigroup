use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

use crate::{
    constant::ConstantExt,
    op::{ast::Op, attr::ContainerAttr},
};

mod ast;
mod attr;

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
    #[case::op_idempotent(
        "op_idempotent",
        impl_op::<External>,
        syn::parse_quote! {
            #[derive(SemigroupOp)]
            #[semigroup_op(idempotent)]
            pub struct Coalesce<T>(pub Option<T>);
        },
    )]
    #[case::op_not_annotated(
        "op_not_annotated",
        impl_op::<Internal>,
        syn::parse_quote! {
            #[derive(SemigroupOpPriv)]
            #[semigroup_op(monoid, commutative, identity = Default::default())]
            pub struct Sum<T: std::ops::Add>(pub T);
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
