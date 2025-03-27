use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

use crate::error::DeriveError;

pub struct CoalesceImplementor {
    input: DeriveInput,
}

impl CoalesceImplementor {
    pub fn new(input: DeriveInput) -> Self {
        Self { input }
    }

    pub fn implement(&self) -> TokenStream {
        let DeriveInput { ident, data, .. } = &self.input;
        match &data {
            Data::Enum(_) => todo!(),
            Data::Struct(s) => self.implement_struct(s),
            Data::Union(_) => {
                syn::Error::new_spanned(ident, DeriveError::UnsupportedUnion).to_compile_error()
            }
        }
    }

    fn implement_struct(&self, s: &DataStruct) -> TokenStream {
        let DeriveInput {
            ident, generics, ..
        } = &self.input;
        let (g_impl, g_type, g_where) = generics.split_for_impl();

        let (prior, posterior) = (
            self.implement_struct_prior(s),
            self.implement_struct_posterior(s),
        );
        quote! {
            impl #g_impl ::coalesced::Coalesce for #ident #g_type #g_where {
                fn prior(self, other: Self) -> Self {
                    #prior
                }
                fn posterior(self, other: Self) -> Self {
                    #posterior
                }
            }
        }
    }

    fn implement_struct_prior(&self, s: &DataStruct) -> TokenStream {
        match &s.fields {
            Fields::Named(f) => {
                let snippet = self.implement_named_prior_snippet(f);
                quote! {
                    Self { #snippet }
                }
            }
            Fields::Unnamed(f) => {
                let snippet = self.implement_unnamed_prior_snippet(f);
                quote! {
                    Self( #snippet )
                }
            }
            Fields::Unit => quote! {
                let _ = self;
                other
            },
        }
    }
    fn implement_struct_posterior(&self, s: &DataStruct) -> TokenStream {
        match &s.fields {
            Fields::Named(f) => {
                let snippet = self.implement_named_posterior_snippet(f);
                quote! {
                    Self { #snippet }
                }
            }
            Fields::Unnamed(f) => {
                let snippet = self.implement_unnamed_posterior_snippet(f);
                quote! {
                    Self( #snippet )
                }
            }
            Fields::Unit => quote! {
                let _ = other;
                self
            },
        }
    }
    fn implement_named_prior_snippet(&self, f: &FieldsNamed) -> TokenStream {
        let fields: Vec<_> = f.named.iter().map(|f| &f.ident).collect();
        quote! {
            #(#fields: self.#fields.prior(other.#fields)),*
        }
    }
    fn implement_named_posterior_snippet(&self, f: &FieldsNamed) -> TokenStream {
        let fields: Vec<_> = f.named.iter().map(|f| &f.ident).collect();
        quote! {
            #(#fields: self.#fields.prior(other.#fields)),*
        }
    }

    fn implement_unnamed_prior_snippet(&self, f: &FieldsUnnamed) -> TokenStream {
        let enumerates: Vec<_> = (0..f.unnamed.len()).map(syn::Index::from).collect();
        quote! {
            #(self.#enumerates.prior(other.#enumerates)),*
        }
    }
    fn implement_unnamed_posterior_snippet(&self, f: &FieldsUnnamed) -> TokenStream {
        let enumerates: Vec<_> = (0..f.unnamed.len()).map(syn::Index::from).collect();
        quote! {
            #(self.#enumerates.posterior(other.#enumerates)),*
        }
    }
}
