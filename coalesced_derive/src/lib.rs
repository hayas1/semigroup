use proc_macro::TokenStream;
use quote::ToTokens;

mod constant;
mod construction;
mod error;
mod semigroup;

#[proc_macro_derive(Construction, attributes(construction))]
pub fn derive_construction(input: TokenStream) -> TokenStream {
    let derive = syn::parse_macro_input!(input);
    let implementor = construction::Construction::new(&derive);
    implementor
        .map(ToTokens::into_token_stream)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
