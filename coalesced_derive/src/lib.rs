use proc_macro::TokenStream;

#[proc_macro_derive(Coalesce)]
pub fn derive(input: TokenStream) -> TokenStream {
    proc_macro::TokenStream::new()
}
