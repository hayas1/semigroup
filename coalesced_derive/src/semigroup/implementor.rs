use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

use crate::{constant::Constant, semigroup::attr::ContainerAttr};

#[derive(Debug, Clone)]
pub struct Semigroup<'a> {
    derive: &'a DeriveInput,
}
impl ToTokens for Semigroup<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { derive, .. } = self;
        tokens.extend(quote::quote! {
            #derive
        });
    }
}
impl<'a> Semigroup<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
    ) -> syn::Result<Self> {
        Ok(Self { derive })
    }
}
