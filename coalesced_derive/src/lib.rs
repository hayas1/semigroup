use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Coalesce)]
pub fn derive_coalesce(input: TokenStream) -> TokenStream {
    let target = parse_macro_input!(input as DeriveInput);
    let ident = &target.ident;
    let expand = match target.data {
        Data::Enum(_) => unimplemented!(),
        Data::Struct(s) => match s.fields {
            Fields::Named(ns) => {
                let fields: Vec<_> = ns.named.into_iter().map(|f| f.ident).collect();
                quote! {
                    impl ::coalesced::Coalesce for #ident {
                        fn prior(self, other: Self) -> Self {
                            Self {
                                #(#fields: self.#fields.prior(other.#fields)),*
                            }
                        }
                        fn posterior(self, other: Self) -> Self {
                            Self {
                                #(#fields: self.#fields.posterior(other.#fields)),*
                            }
                        }
                    }
                }
            }
            Fields::Unnamed(_) => todo!(),
            Fields::Unit => todo!(),
        },
        Data::Union(_) => todo!(),
    };
    // let expand = quote! {
    //     impl<A, E, L> ::coalesced::Coalesce<A, E, L> for #ident
    //     where
    //         L: ::coalesced::ext::Length,
    //     {
    //         fn order(&self, other: &Self) -> std::cmp::Ordering {
    //             std::cmp::Ordering::Less
    //         }
    //     }
    // };
    proc_macro::TokenStream::from(expand)
}
