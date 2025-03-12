use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Coalesce)]
pub fn derive_coalesce(input: TokenStream) -> TokenStream {
    let target = parse_macro_input!(input as DeriveInput);
    let ident = &target.ident;
    let expand = quote! {
        impl<A, E, L> ::coalesced::Coalesce<A, E, L> for #ident
        where
            L: ::coalesced::ext::Length,
        {
            type History = ::coalesced::Coalesced<Self, A, E, L>;
            fn order(&self, other: &Self) -> std::cmp::Ordering {
                std::cmp::Ordering::Less
            }
        }
    };
    proc_macro::TokenStream::from(expand)
}
