mod coalesce;
mod error;
mod extension;

use proc_macro::TokenStream;

// #[proc_macro_derive(Coalesce)]
// pub fn derive_coalesce(input: TokenStream) -> TokenStream {
//     let derive_input = syn::parse_macro_input!(input);
//     let implementor = coalesce::Implementor::new(derive_input);
//     implementor.implement().into()
// }

#[proc_macro_derive(Coalesce)]
pub fn derive_extension(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input);
    let implementor = extension::Implementor::new(derive_input);
    implementor.implement().into()
}
