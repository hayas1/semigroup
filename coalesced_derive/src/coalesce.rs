use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident};

use crate::error::DeriveError;

enum Method {
    Prior,
    Posterior,
}
impl ToTokens for Method {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident: Ident = match self {
            Method::Prior => parse_quote! { prior },
            Method::Posterior => parse_quote! { posterior },
        };
        tokens.extend(ident.into_token_stream())
    }
}

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
            self.snippet_struct(s, Method::Prior),
            self.snippet_struct(s, Method::Posterior),
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

    fn snippet_struct(&self, s: &DataStruct, p: Method) -> TokenStream {
        match &s.fields {
            Fields::Named(f) => {
                let snippet = self.snippet_fields_named(f, p);
                quote! {
                    Self { #snippet }
                }
            }
            Fields::Unnamed(f) => {
                let snippet = self.snippet_fields_unnamed(f, p);
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
    fn snippet_fields_named(&self, f: &FieldsNamed, p: Method) -> TokenStream {
        let fields: Vec<_> = f.named.iter().map(|f| &f.ident).collect();
        quote! {
            #(#fields: self.#fields.#p(other.#fields)),*
        }
    }
    fn snippet_fields_unnamed(&self, f: &FieldsUnnamed, p: Method) -> TokenStream {
        let enumerates: Vec<_> = (0..f.unnamed.len()).map(syn::Index::from).collect();
        quote! {
            #(self.#enumerates.#p(other.#enumerates)),*
        }
    }
}
