use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Coalesce)]
pub fn derive_coalesce(input: TokenStream) -> TokenStream {
    let target = parse_macro_input!(input as DeriveInput);
    let ident = &target.ident;
    match target.data {
        Data::Enum(_) => unimplemented!(),
        Data::Struct(s) => match s.fields {
            Fields::Named(ns) => {
                for (i, f) in ns.named.iter().enumerate() {
                    println!("{}: {}", i, f.ident.as_ref().unwrap());
                }
            }
            Fields::Unnamed(us) => {
                for (i, f) in us.unnamed.iter().enumerate() {
                    println!("{}: {}", i, f.ident.as_ref().unwrap());
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Union(_) => todo!(),
    }
    let expand = quote! {
        impl<A, E, L> ::coalesced::Coalesce<A, E, L> for #ident
        where
            L: ::coalesced::ext::Length,
        {
            fn order(&self, other: &Self) -> std::cmp::Ordering {
                std::cmp::Ordering::Less
            }
        }
    };
    proc_macro::TokenStream::from(expand)
}
