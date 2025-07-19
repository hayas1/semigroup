mod error;
mod extension;

use proc_macro::TokenStream;

#[proc_macro_derive(Coalesce, attributes(coalesced))]
pub fn derive_extension(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input);
    let implementor = extension::Implementor::new(derive_input);
    implementor.implement().into()
}
