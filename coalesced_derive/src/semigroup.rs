use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::{constant::ConstantExt, semigroup::attr::ContainerAttr};

mod attr;
mod implementor;

pub fn gen_semigroup<C: ConstantExt>(derive: &DeriveInput) -> syn::Result<TokenStream> {
    let constant = C::constant();
    let attr = ContainerAttr::new(derive)?;

    todo!()
}
