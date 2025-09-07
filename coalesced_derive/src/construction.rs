use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

use crate::{
    constant::ConstantExt,
    construction::{ast::Construction, attr::ContainerAttr},
};

mod ast;
mod attr;

pub fn gen_construction<C: ConstantExt>(derive: &DeriveInput) -> syn::Result<TokenStream> {
    let constant = C::constant();
    let attr = ContainerAttr::new(derive)?;
    let construction = Construction::new(&constant, derive, &attr)?;
    Ok(construction.into_token_stream())
}

#[cfg(test)]
mod tests {
    use crate::constant::{Absolute, Use};

    use super::*;

    #[test]
    fn test_derive_snapshot() {
        let cases = vec![
            (
                "construction_annotated",
                Box::new(gen_construction::<Absolute> as fn(&_) -> _),
                syn::parse_quote! {
                    #[derive(Construction)]
                    #[construction(annotated, op = Coalesce)]
                    pub struct Coalesced<T>(pub Option<T>);
                },
            ),
            (
                "construction_not_annotated",
                Box::new(gen_construction::<Use>),
                syn::parse_quote! {
                    #[derive(ConstructionUse)]
                    #[construction(op = Coalesce)]
                    pub struct Coalesced<T>(pub Option<T>);
                },
            ),
            (
                "construction_custom_annotation",
                Box::new(gen_construction::<Absolute>),
                syn::parse_quote! {
                    #[derive(Construction)]
                    #[construction(
                        op = Coalesce,
                        annotated,
                        annotation_type_param = "X: IntoIterator + FromIterator<X::Item>",
                        annotation_where = "X::Item: Clone",
                        unit = "vec![(); 0]"
                    )]
                    pub struct Concatenated<T: IntoIterator + FromIterator<T::Item>>(pub T);
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
