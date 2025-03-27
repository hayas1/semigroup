use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_quote, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed,
    FieldsUnnamed, Ident, Variant,
};

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
enum Target {
    Base(Ident),
    Other(Ident),
}
impl ToTokens for Target {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident: Ident = match self {
            Target::Base(ident) => ident.clone(),
            Target::Other(ident) => ident.clone(),
        };
        tokens.extend(ident.into_token_stream())
    }
}
impl Target {
    fn ident(&self) -> &Ident {
        match self {
            Target::Base(ident) => ident,
            Target::Other(ident) => ident,
        }
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
            Data::Enum(e) => self.implement_enum(e),
            Data::Struct(s) => self.implement_struct(s),
            Data::Union(_) => {
                syn::Error::new_spanned(ident, DeriveError::UnsupportedUnion).to_compile_error()
            }
        }
    }

    fn implement_enum(&self, e: &DataEnum) -> TokenStream {
        let DeriveInput {
            ident, generics, ..
        } = &self.input;
        let (g_impl, g_type, g_where) = generics.split_for_impl();

        let (prior, posterior) = (
            self.snippet_enum(e, &Method::Prior),
            self.snippet_enum(e, &Method::Posterior),
        );
        quote! {
            impl #g_impl ::coalesced::Coalesce for #ident #g_type #g_where {
                fn prior(self, other: Self) -> Self {
                    match (self, other) {
                        #prior,
                        (_, o) => o,
                    }
                }
                fn posterior(self, other: Self) -> Self {
                    match (self, other) {
                        #posterior,
                        (s, _) => s,
                    }
                }
            }
        }
    }

    fn implement_struct(&self, s: &DataStruct) -> TokenStream {
        let DeriveInput {
            ident, generics, ..
        } = &self.input;
        let (g_impl, g_type, g_where) = generics.split_for_impl();

        let (prior, posterior) = (
            self.snippet_struct(s, &Method::Prior),
            self.snippet_struct(s, &Method::Posterior),
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

    fn snippet_enum(&self, e: &DataEnum, p: &Method) -> TokenStream {
        e.variants
            .iter()
            .map(|Variant { fields, ident, .. }| match &fields {
                Fields::Named(f) => {
                    let ((base, base_fields, base_var), (other, _other_fields, other_var)) = (
                        self.snippet_fields_named_binding(
                            f,
                            &Target::Base(Ident::new("base", f.span())),
                        ),
                        self.snippet_fields_named_binding(
                            f,
                            &Target::Other(Ident::new("other", f.span())),
                        ),
                    );
                    quote! {
                        (Self::#ident { #base }, Self::#ident { #other }) => Self::#ident { #(#base_fields: #base_var.#p(#other_var)),* }
                    }
                }
                Fields::Unnamed(f) => {
                    let snippet = self.snippet_fields_unnamed(f, p);

                    quote! {
                        (Self::#ident, Self::#ident) => Self::#ident( #snippet ),
                    }
                }
                Fields::Unit => {
                    quote! {
                        (Self::#ident, Self::#ident) => Self::#ident,
                    }
                }
            })
            .collect()
    }

    fn snippet_struct(&self, s: &DataStruct, p: &Method) -> TokenStream {
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
    fn snippet_fields_named_binding(
        &self,
        f: &FieldsNamed,
        t: &Target,
    ) -> (TokenStream, Vec<Ident>, Vec<Ident>) {
        let fields: Vec<_> = f
            .named
            .iter()
            .map(|f| f.ident.clone().expect("# TODO"))
            .collect();
        let binding: Vec<_> = f
            .named
            .iter()
            .map(|f| {
                Ident::new(
                    &format!("{}_{}", t.ident(), f.ident.as_ref().expect("# TODO")),
                    f.span(),
                )
            })
            .collect();
        let snippet = quote! {
            #(#fields: #binding),*
        };
        (snippet, fields, binding)
    }
    fn snippet_fields_named(&self, f: &FieldsNamed, p: &Method) -> TokenStream {
        let fields: Vec<_> = f.named.iter().map(|f| &f.ident).collect();
        quote! {
            #(#fields: self.#fields.#p(other.#fields)),*
        }
    }
    fn snippet_fields_unnamed(&self, f: &FieldsUnnamed, p: &Method) -> TokenStream {
        let enumerates: Vec<_> = (0..f.unnamed.len()).map(syn::Index::from).collect();
        quote! {
            #(self.#enumerates.#p(other.#enumerates)),*
        }
    }
}
