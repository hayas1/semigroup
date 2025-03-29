use proc_macro2::TokenStream;
use quote::format_ident;
use syn::{parse_quote, Data, DataStruct, DeriveInput, Fields, Ident, ItemImpl, ItemStruct};

use crate::error::DeriveError;

pub struct Implementor {
    input: DeriveInput,
}
impl Implementor {
    pub fn new(input: DeriveInput) -> Self {
        Self { input }
    }

    pub fn implement(&self) -> TokenStream {
        let DeriveInput { ident, data, .. } = &self.input;
        match &data {
            Data::Enum(_e) => todo!(),
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

        let extension = self.implement_struct_extension(s);
        let with_ext_def = self.definition_struct_with_ext(s);
        let with_ext_impl = self.implement_struct_with_ext(s);
        let from_with_ext = self.implement_struct_from_with_ext();
        parse_quote! {
            #extension
            #with_ext_def
            #with_ext_impl
            #from_with_ext
        }
    }

    fn ident_with_ext(&self) -> Ident {
        let DeriveInput { ident, .. } = &self.input;
        format_ident!("{}WithExt", ident)
    }

    fn implement_struct_extension(&self, s: &DataStruct) -> ItemImpl {
        let DeriveInput {
            ident, generics, ..
        } = &self.input;
        let (g_impl, g_type, g_where) = generics.split_for_impl();

        match &s.fields {
            Fields::Named(f) => {
                let (fields, types): (Vec<_>, Vec<_>) =
                    f.named.iter().map(|f| (&f.ident, &f.ty)).unzip();
                let with_ext = self.ident_with_ext();
                parse_quote! {
                    impl<X: Clone> ::coalesced::Extension<X> for #ident {
                        type WithExt = #with_ext<X>;
                        fn with_extension(self, extension: X) -> Self::WithExt {
                            #with_ext {
                                #(#fields: self.#fields.with_extension(extension.clone())),*
                            }
                        }
                        fn unwrap_extension(with_ext: Self::WithExt) -> Self {
                            let Self::WithExt { #(#fields),* } = with_ext;
                            Self {
                                #(#fields: Extension::unwrap_extension(#fields)),*
                            }
                        }
                        fn ex_prior(base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
                            base.prior(other)
                        }
                        fn ex_posterior(base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
                            base.posterior(other)
                        }
                    }
                }
            }
            Fields::Unnamed(_) => todo!(),
            Fields::Unit => todo!(),
        }
    }
    fn definition_struct_with_ext(&self, s: &DataStruct) -> ItemStruct {
        let DeriveInput {
            ident, generics, ..
        } = &self.input;
        let (g_impl, g_type, g_where) = generics.split_for_impl();
        let with_ext = self.ident_with_ext();
        match &s.fields {
            Fields::Named(f) => {
                let (fields, types): (Vec<_>, Vec<_>) =
                    f.named.iter().map(|f| (&f.ident, &f.ty)).unzip();
                parse_quote! {
                    struct #with_ext<X> {
                        #(#fields: ::coalesced::WithExt<#types, X>),*
                    }
                }
            }
            Fields::Unnamed(_) => todo!(),
            Fields::Unit => todo!(),
        }
    }
    fn implement_struct_with_ext(&self, s: &DataStruct) -> ItemImpl {
        let DeriveInput {
            ident, generics, ..
        } = &self.input;
        let (g_impl, g_type, g_where) = generics.split_for_impl();
        let with_ext = self.ident_with_ext();

        match &s.fields {
            Fields::Named(f) => {
                let (fields, types): (Vec<_>, Vec<_>) =
                    f.named.iter().map(|f| (&f.ident, &f.ty)).unzip();
                parse_quote! {
                    impl<X> ::coalesced::Coalesce for #with_ext<X> {
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
        }
    }
    fn implement_struct_from_with_ext(&self) -> ItemImpl {
        let DeriveInput {
            ident, generics, ..
        } = &self.input;
        let (g_impl, g_type, g_where) = generics.split_for_impl();
        let with_ext = self.ident_with_ext();
        parse_quote! {
            impl<X: Clone> From<#with_ext<X>> for #ident {
                fn from(with_ext: #with_ext<X>) -> Self {
                    ::coalesced::Extension::unwrap_extension(with_ext)
                }
            }
        }
    }
}
