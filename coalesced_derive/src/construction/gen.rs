use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

use crate::{
    constant::ConstantExt,
    construction::{attr::ConstructionAttr, implementor::Construction},
};

pub fn gen_construction<C: ConstantExt>(derive: &DeriveInput) -> TokenStream {
    let constant = C::constant();
    let attr = match ConstructionAttr::new(derive) {
        Ok(attr) => attr,
        Err(e) => return e.into_compile_error(),
    };
    let construction = match Construction::new(&constant, derive, &attr) {
        Ok(construction) => construction,
        Err(e) => return e.into_compile_error(),
    };
    construction.into_token_stream()
}

#[cfg(test)]
mod tests {
    use crate::constant::{Absolute, Use};

    use super::*;

    #[test]
    fn test_derive_construction_annotation() {
        let derive = syn::parse_quote! {
            #[derive(Construction)]
            #[construction(annotated, op = Coalesce)]
            pub struct Coalesced<T>(pub Option<T>);
        };
        let generated = gen_construction::<Absolute>(&derive);
        let formatted = prettyplease::unparse(&syn::parse2(generated).unwrap());
        insta::with_settings!({ snapshot_path => "../../tests/snapshots" }, {
            insta::assert_snapshot!("annotated", formatted);
        });
    }

    #[test]
    fn test_derive_construction_no_annotateion() {
        let derive = syn::parse_quote! {
            #[derive(ConstructionUse)]
            #[construction(op = Coalesce)]
            pub struct Coalesced<T>(pub Option<T>);
        };
        let generated = gen_construction::<Use>(&derive);
        let formatted = prettyplease::unparse(&syn::parse2(generated).unwrap());
        insta::with_settings!({ snapshot_path => "../../tests/snapshots" }, {
            insta::assert_snapshot!("not_annotated", formatted);
        });
    }
}
