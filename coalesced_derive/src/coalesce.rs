use std::fmt::Display;

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    parse_quote, spanned::Spanned, Arm, Data, DataEnum, DataStruct, DeriveInput, Expr, ExprBlock,
    Field, Fields, FieldsNamed, FieldsUnnamed, Ident, ItemImpl, Variant,
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
    fn snippet_unit(&self) -> ExprBlock {
        match self {
            Self::Prior => parse_quote! {
                {
                    let _ = self;
                    other
                }
            },
            Self::Posterior => parse_quote! {
                {
                    let _ = other;
                    self
                }
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
    fn prefix(&self) -> &str {
        match self {
            Self::Base => "self",
            Self::Other => "other",
        }
    }
    fn field_varname(&self, field: &Field, span: Span) -> Ident {
        let target = field.ident.as_ref().map(ToString::to_string);
        let var = &format!("{}_{}", self.prefix(), target.unwrap_or_default());
        Ident::new(var, span)
    }
    fn index_varname(&self, index: impl Display, span: Span) -> Ident {
        let var = &format!("{}_{}", self.prefix(), index);
        Ident::new(var, span)
    }
}

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
            Data::Enum(e) => self.implement_enum(e).into_token_stream(),
            Data::Struct(s) => self.implement_struct(s).into_token_stream(),
            Data::Union(_) => {
                syn::Error::new_spanned(ident, DeriveError::UnsupportedUnion).to_compile_error()
            }
        }
    }

    fn implement_enum(&self, e: &DataEnum) -> ItemImpl {
        let DeriveInput {
            ident, generics, ..
        } = &self.input;
        let (g_impl, g_type, g_where) = generics.split_for_impl();

        let (prior, posterior) = (
            self.snippet_enum(e, &Method::Prior),
            self.snippet_enum(e, &Method::Posterior),
        );
        parse_quote! {
            impl #g_impl ::coalesced::Coalesce for #ident #g_type #g_where {
                fn prior(self, other: Self) -> Self {
                    match (self, other) {
                        #(#prior),*
                        (_, o) => o,
                    }
                }
                fn posterior(self, other: Self) -> Self {
                    match (self, other) {
                        #(#posterior),*
                        (s, _) => s,
                    }
                }
            }
        }
    }

    fn implement_struct(&self, s: &DataStruct) -> ItemImpl {
        let DeriveInput {
            ident, generics, ..
        } = &self.input;
        let (g_impl, g_type, g_where) = generics.split_for_impl();

        let (prior, posterior) = (
            self.snippet_struct(s, &Method::Prior),
            self.snippet_struct(s, &Method::Posterior),
        );
        parse_quote! {
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

    fn snippet_enum(&self, e: &DataEnum, p: &Method) -> Vec<Arm> {
        e.variants
            .iter()
            .map(|Variant { fields, ident, .. }| match &fields {
                Fields::Named(f) => {
                    let ((fields, base_binding), (_, other_binding)) = (
                        self.fields_named_binding(f, &Target::Base),
                        self.fields_named_binding(f, &Target::Other),
                    );
                    parse_quote! {
                        (
                            Self::#ident { #(#fields: #base_binding),* },
                            Self::#ident { #(#fields: #other_binding),* },
                        ) => { Self::#ident { #(#fields: #base_binding.#p(#other_binding)),* } }
                    }
                }
                Fields::Unnamed(f) => {
                    let (base_binding, other_binding) = (
                        self.fields_unnamed_binding(f, &Target::Base),
                        self.fields_unnamed_binding(f, &Target::Other),
                    );
                    parse_quote! {
                        (
                            Self::#ident( #(#base_binding),* ),
                            Self::#ident( #(#other_binding),* )
                        ) => { Self::#ident( #(#base_binding.#p(#other_binding)),* ) }
                    }
                }
                Fields::Unit => {
                    parse_quote! {
                        (Self::#ident, Self::#ident) => { Self::#ident }
                    }
                }
            })
            .collect()
    }

    fn snippet_struct(&self, s: &DataStruct, p: &Method) -> Expr {
        match &s.fields {
            Fields::Named(f) => {
                let fields = self.fields_named(f);
                parse_quote! {
                    Self { #(#fields: self.#fields.#p(other.#fields)),* }
                }
            }
            Fields::Unnamed(f) => {
                let indices = self.fields_unnamed(f);
                parse_quote! {
                    Self( #(self.#indices.#p(other.#indices)),* )
                }
            }
            Fields::Unit => p.snippet_unit().into(),
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
