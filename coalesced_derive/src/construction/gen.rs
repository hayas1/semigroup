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
