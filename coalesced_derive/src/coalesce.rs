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
                    let ((base_fields, base_binding), (other_fields, other_binding)) = (
                        self.fields_named_binding(f, &Target::Base),
                        self.fields_named_binding(f, &Target::Other),
                    );
                    quote! {
                        (
                            Self::#ident { #(#base_fields: #base_binding),* },
                            Self::#ident { #(#other_fields: #other_binding),* },
                        ) => { Self::#ident { #(#base_fields: #base_binding.#p(#other_binding)),* } }
                    }
                }
                Fields::Unnamed(f) => {
                    let (base_binding, other_binding) = (
                        self.fields_unnamed_binding(f, &Target::Base),
                        self.fields_unnamed_binding(f, &Target::Other),
                    );
                    quote! {
                        (
                            Self::#ident( #(#base_binding),* ),
                            Self::#ident( #(#other_binding),* )
                        ) => { Self::#ident( #(#base_binding.#p(#other_binding)),* ) }
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
                let fields = self.fields_named(f);
                quote! {
                    Self { #(#fields: self.#fields.#p(other.#fields)),* }
                }
            }
            Fields::Unnamed(f) => {
                let indices = self.fields_unnamed(f);
                quote! {
                    Self( #(self.#indices.#p(other.#indices)),* )
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
    fn fields_unnamed_binding(&self, f: &FieldsUnnamed, t: &Target) -> Vec<Ident> {
        (0..f.unnamed.len())
            .map(|i| t.index_varname(i, f.span()))
            .collect()
    }

    fn fields_named<'a>(&self, f: &'a FieldsNamed) -> Vec<&'a Option<Ident>> {
        f.named.iter().map(|f| &f.ident).collect()
    }
    fn fields_unnamed(&self, f: &FieldsUnnamed) -> Vec<syn::Index> {
        (0..f.unnamed.len()).map(syn::Index::from).collect()
    }
}
