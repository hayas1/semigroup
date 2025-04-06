use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{
    parse_quote, Data, DataStruct, DeriveInput, Fields, GenericParam, Ident, ItemImpl, ItemStruct,
    TypeGenerics, TypeParam, TypeParamBound, WhereClause,
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

    fn extension_generic(&self) -> GenericParam {
        let clone_bound = TypeParamBound::Trait(parse_quote! {Clone});
        GenericParam::Type(TypeParam {
            attrs: Vec::new(),
            ident: format_ident!("X"),
            colon_token: Some(Default::default()),
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
        let x_param = g_ext.param();

        match &s.fields {
            Fields::Named(f) => {
                let (fields, _types): (Vec<_>, Vec<_>) =
                    f.named.iter().map(|f| (&f.ident, &f.ty)).unzip();
                let with_ext = self.ident_with_ext();
                parse_quote! {
                    impl #g_impl ::coalesced::Extension<#x_param> for #ident #g_type #g_where {
                        type WithExt = #with_ext #g_ext;
                        fn with_extension(self, extension: #x_param) -> Self::WithExt {
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
        let (_, g_ext, _, g_where) = self.split_with_extension_generics();
        let x_param = g_ext.param();
        let with_ext = self.ident_with_ext();
        match &s.fields {
            Fields::Named(f) => {
                let (fields, types): (Vec<_>, Vec<_>) =
                    f.named.iter().map(|f| (&f.ident, &f.ty)).unzip();
                parse_quote! {
                    #[derive()]
                    #[doc(hidden)]
                    struct #with_ext #g_ext #g_where {
                        #(#fields: ::coalesced::WithExt<#types, #x_param>),*
                    }
                }
            }
            Fields::Unnamed(_) => todo!(),
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
            Fields::Unnamed(_) => todo!(),
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
impl ExTypeGenerics<'_> {
    fn param(&self) -> Ident {
        // TODO remove this method ?
        let x = self.0.extension_generic();
        match x {
            GenericParam::Type(t) => t.ident,
            _ => unreachable!(),
        }
    }
}
