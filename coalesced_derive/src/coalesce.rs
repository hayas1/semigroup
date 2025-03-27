use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed};

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
                syn::Error::new(ident.span(), "derive Coalesce for union is not supported")
                    .to_compile_error()
            }
        }
    }

    fn implement_struct(&self, s: &DataStruct) -> TokenStream {
        match &s.fields {
            Fields::Named(f) => {
                let DeriveInput { ident, .. } = &self.input;
                let (prior, posterior) = (
                    self.implement_named_prior_snippet(f),
                    self.implement_named_posterior_snippet(f),
                );
                quote! {
                    impl ::coalesced::Coalesce for #ident {
                        fn prior(self, other: Self) -> Self {
                            Self { #prior }
                        }
                        fn posterior(self, other: Self) -> Self {
                            Self { #posterior }
                        }
                    }
                }
            }
            Fields::Unnamed(_) => todo!(),
            Fields::Unit => todo!(),
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
}
