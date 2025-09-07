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
    fn test_derive_semigroup_annotated() {
        let derive = syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(annotated)]
            pub struct NamedStruct {
                #[semigroup(with = "coalesced::op::annotation::replace::Replaced")]
                pub foo: String,
                pub bar: Option<u32>,
                pub baz: coalesced::op::annotation::replace::Replaced<bool>,
            }
        };
        let generated = gen_semigroup::<Absolute>(&derive).unwrap();
        let formatted = prettyplease::unparse(&syn::parse2(generated).unwrap());
        insta::with_settings!({ snapshot_path => "../tests/snapshots", prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!("semigroup_annotated", formatted);
        });
    }

    #[test]
    fn test_derive_semigroup_not_annotated() {
        let derive = syn::parse_quote! {
            #[derive(SemigroupUse)]
            #[semigroup(with = "coalesced::op::annotation::replace::Replaced")]
            pub struct UnnamedStruct<T: std::ops::Add> (
                #[semigroup(with = "coalesced::op::semigroup::add::Added")]
                T,
                u64
            );
        };
        let generated = gen_semigroup::<Use>(&derive).unwrap();
        let formatted = prettyplease::unparse(&syn::parse2(generated).unwrap());
        insta::with_settings!({ snapshot_path => "../tests/snapshots", prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!("semigroup_not_annotated", formatted);
        });
    }

    #[test]
    fn test_derive_semigroup_custom_annotation_param() {
        let derive = syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(annotated, annotation_param = X, with = "coalesced::op::annotation::replace::Replaced")]
            pub struct NamedStruct{
                pub foo: String,
                pub bar: Option<u32>,
                pub baz: bool,
            }
        };
        let generated = gen_semigroup::<Absolute>(&derive).unwrap();
        let formatted = prettyplease::unparse(&syn::parse2(generated).unwrap());
        insta::with_settings!({ snapshot_path => "../tests/snapshots", prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!("semigroup_custom_annotation", formatted);
        });
    }
}
