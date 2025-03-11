use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Coalesce)]
pub fn derive_coalesce(input: TokenStream) -> TokenStream {
    let target = parse_macro_input!(input as DeriveInput);
    let ident = &target.ident;
    let expand = quote! {
        impl coalesced::Coalesce for #ident {
            fn order(&self, other: &Self) -> std::cmp::Ordering {
                std::cmp::Ordering::Less
            }
        }
    };
    proc_macro::TokenStream::from(expand)
}
