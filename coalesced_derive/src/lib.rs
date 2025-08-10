#[proc_macro_derive(Coalesce, attributes(coalesced))]
pub fn derive_extension(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _derive_input: syn::DeriveInput = syn::parse_macro_input!(input);
    todo!()
}
