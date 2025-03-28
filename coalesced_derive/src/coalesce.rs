use std::fmt::Display;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse_quote, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Field, Fields,
    FieldsNamed, FieldsUnnamed, Ident, Variant,
};

use crate::error::DeriveError;

enum Method {
    Prior,
    Posterior,
}
impl ToTokens for Method {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident: Ident = match self {
            Self::Prior => parse_quote! { prior },
            Self::Posterior => parse_quote! { posterior },
        };
        tokens.extend(ident.into_token_stream())
    }
}
impl Method {
    fn snippet_unit(&self) -> TokenStream {
        match self {
            Self::Prior => quote! {
               let _ = self;
               other
            },
            Self::Posterior => quote! {
               let _ = other;
               self
            },
        }
    }
}
enum Target {
    Base,
    Other,
}
// impl ToTokens for Target {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         tokens.extend(self.ident().into_token_stream())
//     }
// }
impl Target {
    fn ident(&self) -> TokenStream {
        // Ident
        match self {
            Self::Base => parse_quote! { self }, // TODO keyword `self` cannot be used as Ident
            Self::Other => parse_quote! { other },
        }
    }
    fn field_varname(&self, field: &Field, span: Span) -> Ident {
        let target = field.ident.as_ref().map(ToString::to_string);
        let var = &format!("{}_{}", self.ident(), target.unwrap_or_default());
        Ident::new(var, span)
    }
    fn index_varname(&self, index: impl Display, span: Span) -> Ident {
        let var = &format!("{}_{}", self.ident(), index);
        Ident::new(var, span)
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
                    let ((base_fields, base_var), (other_fields, other_var)) = (
                        self.fields_named_binding(f, &Target::Base),
                        self.fields_named_binding(f, &Target::Other),
                    );
                    let (base, other) = (
                        self.snippet_fields_named_binding(&base_fields, &base_var),
                        self.snippet_fields_named_binding(&other_fields, &other_var),
                    );
                    quote! {
                        (Self::#ident { #base }, Self::#ident { #other }) => { Self::#ident { #(#base_fields: #base_var.#p(#other_var)),* } }
                    }
                }
                Fields::Unnamed(f) => {
                    let (base_var, other_var) = (
                        self.fields_unnamed_binding(f, &Target::Base),
                        self.fields_unnamed_binding(f, &Target::Other),
                    );
                    let (base, other) = (
                        self.snippet_fields_unnamed_binding(&base_var),
                        self.snippet_fields_unnamed_binding(&other_var),
                    );
                    quote! {
                        (Self::#ident( #base ), Self::#ident( #other )) => { Self::#ident( #(#base_var.#p(#other_var)),* ) }
                    }
                }
                Fields::Unit => {
                    quote! {
                        (Self::#ident, Self::#ident) => { Self::#ident }
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
            Fields::Unit => p.snippet_unit(),
        }
    }
    fn fields_named_binding<'a>(
        &self,
        f: &'a FieldsNamed,
        t: &Target,
    ) -> (Vec<&'a Option<Ident>>, Vec<Ident>) {
        f.named
            .iter()
            .map(|fi| (&fi.ident, t.field_varname(fi, f.span())))
            .collect()
    }
    fn snippet_fields_named_binding<'a>(
        &'a self,
        f: impl IntoIterator<Item = &'a &'a Option<Ident>>,
        b: impl IntoIterator<Item = &'a Ident>,
    ) -> TokenStream {
        let (fields, binging) = (f.into_iter(), b.into_iter());
        quote! {
            #(#fields: #binging),*
        }
    }
    fn fields_unnamed_binding(&self, f: &FieldsUnnamed, t: &Target) -> Vec<Ident> {
        (0..f.unnamed.len())
            .map(|i| t.index_varname(i, f.span()))
            .collect()
    }
    fn snippet_fields_unnamed_binding<'a>(
        &self,
        b: impl IntoIterator<Item = &'a Ident>,
    ) -> TokenStream {
        let binding = b.into_iter();
        quote! {
            #(#binding),*
        }
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
