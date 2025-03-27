mod coalesce;
mod error;

use proc_macro::TokenStream;

#[proc_macro_derive(Coalesce)]
pub fn derive_coalesce(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input);
    let implementor = coalesce::CoalesceImplementor::new(derive_input);
    let expand = implementor.implement();
    TokenStream::from(expand)
}
