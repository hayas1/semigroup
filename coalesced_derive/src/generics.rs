use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, GenericParam, Generics, Ident, Path, Type, TypeParam, WhereClause};

pub struct Annotated<'a> {
    pub path_annotated: &'a Path,
    pub value_type_ident: &'a Ident, // TODO TypePath? Path?
    pub generics: &'a Generics,
    pub type_param: TypeParam,
}
impl<'a> Annotated<'a> {
    pub fn new(
        path_annotated: &'a Path,
        value_type_ident: &'a Ident,
        generics: &'a Generics,
        type_param: TypeParam,
    ) -> Self {
        Self {
            generics,
            path_annotated,
            value_type_ident,
            type_param,
        }
    }

    pub fn type_param(&self) -> TypeParam {
        self.type_param.clone()
    }
    pub fn generic_param(&self) -> GenericParam {
        GenericParam::Type(self.type_param())
    }

    pub fn split_for_impl(&self) -> (AnnotatedImplGenerics, AnnotatedType, Option<&WhereClause>) {
        (
            AnnotatedImplGenerics(self),
            AnnotatedType(self),
            self.generics.where_clause.as_ref(),
        )
    }
}

pub struct AnnotatedImplGenerics<'a>(&'a Annotated<'a>);
impl<'a> ToTokens for AnnotatedImplGenerics<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(&Annotated { generics, .. }) = self;
        let mut new_generics = generics.clone();
        new_generics.params.push(self.0.generic_param());

        let (impl_generics, _, _) = new_generics.split_for_impl();
        impl_generics.to_tokens(tokens);
    }
}
pub struct AnnotatedType<'a>(&'a Annotated<'a>);
impl<'a> ToTokens for AnnotatedType<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(&Annotated {
            path_annotated,
            value_type_ident,
            generics,
            ..
        }) = self;
        let a = self.0.type_param().ident;
        let (_, ty_generics, _) = generics.split_for_impl();

        let ty: Type = parse_quote! { #path_annotated<#value_type_ident #ty_generics, #a> };
        ty.to_tokens(tokens);
    }
}
