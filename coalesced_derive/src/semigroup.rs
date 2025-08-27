use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

use crate::{
    constant::ConstantExt,
    semigroup::{attr::ContainerAttr, implementor::Semigroup},
};

mod attr;
mod implementor;

pub fn gen_semigroup<C: ConstantExt>(derive: &DeriveInput) -> syn::Result<TokenStream> {
    let constant = C::constant();
    let attr = ContainerAttr::new(derive)?;
    let semigroup = Semigroup::new(&constant, derive, &attr)?;
    Ok(semigroup.into_token_stream())
}

#[cfg(test)]
mod tests {}
