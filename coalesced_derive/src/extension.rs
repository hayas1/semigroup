use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{
    parse_quote, Data, DataStruct, DeriveInput, Expr, Fields, GenericParam, Ident, ItemImpl,
    ItemStruct, TypeGenerics, TypeParam, TypeParamBound, WhereClause,
};

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

    fn ident_with_ext(&self) -> Ident {
        let DeriveInput { ident, .. } = &self.input;
        format_ident!("{}WithExt", ident)
    }

    fn x_param(&self) -> Ident {
        parse_quote! { X }
    }
    fn extension_generic(&self) -> GenericParam {
        let clone_bound = TypeParamBound::Trait(parse_quote! {Clone});
        GenericParam::Type(TypeParam {
            attrs: Vec::new(),
            ident: self.x_param(),
            colon_token: Some(parse_quote! { : }),
            bounds: vec![clone_bound].into_iter().collect(),
            eq_token: None,
            default: None,
        })
    }
    fn split_with_extension_generics(
        &self,
    ) -> (
        ExImplGenerics,
        ExTypeGenerics,
        TypeGenerics,
        Option<&WhereClause>,
    ) {
        let ex_impl = ExImplGenerics(self);
        let ex_type = ExTypeGenerics(self);
        let (_, g_type, g_where) = self.input.generics.split_for_impl();
        (ex_impl, ex_type, g_type, g_where)
    }

    fn implement_struct(&self, s: &DataStruct) -> TokenStream {
        let extension = self.implement_struct_extension(s);
        let with_ext_def = self.definition_struct_with_ext(s);
        let coalesce_with_ext = self.implement_struct_coalesce_with_ext(s);
        let from_with_ext = self.implement_struct_from_with_ext();
        parse_quote! {
            #extension
            #with_ext_def
            #coalesce_with_ext
            #from_with_ext
        }
    }

    fn implement_struct_extension(&self, s: &DataStruct) -> ItemImpl {
        let DeriveInput { ident, .. } = &self.input;
        let (g_impl, g_ext, g_type, g_where) = self.split_with_extension_generics();
        let x_param = self.x_param();

        let with_ext = self.ident_with_ext();
        let (ex, we) = (parse_quote! { extension }, parse_quote! { with_ext });
        let with_extension = self.implement_struct_extension_with_extension(&s.fields, &ex);
        let unwrap_extension = self.implement_struct_extension_unwrap_extension(&s.fields, &we);
        parse_quote! {
            impl #g_impl ::coalesced::Extension<#x_param> for #ident #g_type #g_where {
                type WithExt = #with_ext #g_ext;
                fn with_extension(self, #ex: #x_param) -> Self::WithExt {
                    #with_extension
                }
                fn unwrap_extension(#we: Self::WithExt) -> Self {
                    #unwrap_extension
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
    fn implement_struct_extension_with_extension(&self, f: &Fields, ex: &Ident) -> Expr {
        let with_ext = self.ident_with_ext();
        match f {
            Fields::Named(n) => {
                let (fields, _types): (Vec<_>, Vec<_>) =
                    n.named.iter().map(|f| (&f.ident, &f.ty)).unzip();
                parse_quote! {
                    #with_ext { #(#fields: self.#fields.with_extension(#ex.clone())),* }
                }
            }
            Fields::Unnamed(u) => {
                let indices = (0..u.unnamed.len()).map(syn::Index::from);
                parse_quote! {
                    #with_ext( #(self.#indices.with_extension(#ex.clone())),* )
                }
            }
            Fields::Unit => todo!(),
        }
    }
    fn implement_struct_extension_unwrap_extension(&self, f: &Fields, we: &Ident) -> Expr {
        match f {
            Fields::Named(n) => {
                let (fields, _types): (Vec<_>, Vec<_>) =
                    n.named.iter().map(|f| (&f.ident, &f.ty)).unzip();
                parse_quote! {
                    Self { #(#fields: Extension::unwrap_extension(#we.#fields)),* }
                }
            }
            Fields::Unnamed(u) => {
                let indices = (0..u.unnamed.len()).map(syn::Index::from);
                parse_quote! {
                    Self( #(Extension::unwrap_extension(#we.#indices)),* )
                }
            }
            Fields::Unit => todo!(),
        }
    }
    fn definition_struct_with_ext(&self, s: &DataStruct) -> ItemStruct {
        let DeriveInput { vis, .. } = &self.input;
        let (_, g_ext, _, g_where) = self.split_with_extension_generics();
        let x_param = self.x_param();
        let with_ext = self.ident_with_ext();
        match &s.fields {
            Fields::Named(f) => {
                let (fields, types): (Vec<_>, Vec<_>) =
                    f.named.iter().map(|f| (&f.ident, &f.ty)).unzip();
                parse_quote! {
                    #[doc(hidden)]
                    #vis struct #with_ext #g_ext #g_where {
                        #(#fields: ::coalesced::WithExt<#types, #x_param>),*
                    }
                }
            }
            Fields::Unnamed(u) => {
                let (_indices, types): (Vec<_>, Vec<_>) = u
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, f)| (syn::Index::from(i), &f.ty))
                    .unzip();
                parse_quote! {
                    #[doc(hidden)]
                    #vis struct #with_ext #g_ext (
                        #(::coalesced::WithExt<#types, #x_param>),*
                    ) #g_where;
                }
            }
            Fields::Unit => todo!(),
        }
    }
    fn implement_struct_coalesce_with_ext(&self, s: &DataStruct) -> ItemImpl {
        let (g_impl, g_ext, _, g_where) = self.split_with_extension_generics();
        let with_ext = self.ident_with_ext();

        match &s.fields {
            Fields::Named(f) => {
                let (fields, _types): (Vec<_>, Vec<_>) =
                    f.named.iter().map(|f| (&f.ident, &f.ty)).unzip();
                parse_quote! {
                    impl #g_impl ::coalesced::Coalesce for #with_ext #g_ext #g_where {
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
            Fields::Unnamed(u) => {
                let (indices, _types): (Vec<_>, Vec<_>) = u
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, f)| (syn::Index::from(i), &f.ty))
                    .unzip();
                parse_quote! {
                    impl #g_impl ::coalesced::Coalesce for #with_ext #g_ext #g_where {
                        fn prior(self, other: Self) -> Self {
                            Self( #(self.#indices.prior(other.#indices)),* )
                        }
                        fn posterior(self, other: Self) -> Self {
                            Self( #(self.#indices.posterior(other.#indices)),* )
                        }
                    }
                }
            }
            Fields::Unit => todo!(),
        }
    }
    fn implement_struct_from_with_ext(&self) -> ItemImpl {
        let DeriveInput { ident, .. } = &self.input;
        let (g_impl, g_ext, g_type, g_where) = self.split_with_extension_generics();
        let with_ext = self.ident_with_ext();
        parse_quote! {
            impl #g_impl From<#with_ext #g_ext> for #ident #g_type #g_where {
                fn from(with_ext: #with_ext #g_ext) -> Self {
                    ::coalesced::Extension::unwrap_extension(with_ext)
                }
            }
        }
    }
}

struct ExImplGenerics<'a>(&'a Implementor);
impl ToTokens for ExImplGenerics<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let x = self.0.extension_generic();
        let mut generics = self.0.input.generics.clone();
        generics.params.push(x);
        let (g_impl, _, _) = generics.split_for_impl();
        g_impl.to_tokens(tokens);
    }
}

struct ExTypeGenerics<'a>(&'a Implementor);
impl ToTokens for ExTypeGenerics<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let x = self.0.extension_generic();
        let mut generics = self.0.input.generics.clone();
        generics.params.push(x);
        let (_, g_type, _) = generics.split_for_impl();
        g_type.to_tokens(tokens);
    }
}
