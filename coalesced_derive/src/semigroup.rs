use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

use crate::{
    constant::ConstantExt,
    semigroup::{ast::Semigroup, attr::ContainerAttr},
};

mod ast;
mod attr;

pub fn gen_semigroup<C: ConstantExt>(derive: &DeriveInput) -> syn::Result<TokenStream> {
    let constant = C::constant();
    let attr = ContainerAttr::new(derive)?;
    let semigroup = Semigroup::new(&constant, derive, &attr)?;
    Ok(semigroup.into_token_stream())
}

#[cfg(test)]
mod tests {
    use crate::constant::{Absolute, Use};

    use super::*;

    #[test]
    fn test_derive_snapshot() {
        let cases = vec![
            (
                "semigroup_annotated",
                Box::new(gen_semigroup::<Absolute> as fn(&_) -> _),
                syn::parse_quote! {
                    #[derive(Semigroup)]
                    #[semigroup(annotated)]
                    pub struct NamedStruct {
                        #[semigroup(with = "coalesced::op::annotation::replace::Replaced")]
                        pub foo: String,
                        pub bar: Option<u32>,
                        pub baz: coalesced::op::annotation::replace::Replaced<bool>,
                    }
                },
            ),
            (
                "semigroup_not_annotated",
                Box::new(gen_semigroup::<Use>),
                syn::parse_quote! {
                    #[derive(SemigroupUse)]
                    #[semigroup(with = "coalesced::op::annotation::replace::Replaced")]
                    pub struct UnnamedStruct<T: std::ops::Add> (
                        #[semigroup(with = "coalesced::op::semigroup::add::Added")]
                        T,
                        u64
                    );
                },
            ),
            (
                "semigroup_custom_annotation",
                Box::new(gen_semigroup::<Absolute>),
                syn::parse_quote! {
                    #[derive(Semigroup)]
                    #[semigroup(annotated, annotation_param = X, with = "coalesced::op::annotation::replace::Replaced")]
                    pub struct NamedStruct{
                        pub foo: String,
                        pub bar: Option<u32>,
                        pub baz: bool,
                    }
                },
            ),
        ];
        cases.into_iter().for_each(|(case, f, derive)| {
            let generated = f(&derive).unwrap();
            let formatted = prettyplease::unparse(&syn::parse2(generated).unwrap());
            insta::with_settings!({ snapshot_path => "../tests/snapshots", prepend_module_to_snapshot => false }, {
                insta::assert_snapshot!(case, formatted);
            });
        });
    }
}
