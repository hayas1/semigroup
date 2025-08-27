use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::{constant::ConstantExt, semigroup::attr::ContainerAttr};

mod attr;

pub fn gen_semigroup<C: ConstantExt>(derive: &DeriveInput) -> TokenStream {
    let constant = C::constant();
    let attr = match ContainerAttr::new(derive) {
        Ok(attr) => attr,
        Err(e) => return e.into_compile_error(),
    };

    todo!()
}
