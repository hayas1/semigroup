use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, GenericParam, Generics, Ident, Path, Type, TypeParam, WherePredicate};

#[derive(Debug, Clone)]
pub struct Annotated<'a> {
    path_annotated: &'a Path,
    type_ident: &'a Ident, // TODO TypePath? Path?
    generics: &'a Generics,
    annotation: TypeParam,
    annotation_where: Option<WherePredicate>,
}
impl<'a> Annotated<'a> {
    pub fn new(
        path_annotated: &'a Path,
        type_ident: &'a Ident,
        generics: &'a Generics,
        annotation: TypeParam,
        annotation_where: Option<WherePredicate>,
    ) -> Self {
        Self {
            generics,
            path_annotated,
            type_ident,
            annotation,
            annotation_where,
        }
    }

    pub fn annotation(&self) -> TypeParam {
        self.annotation.clone()
    }

    pub fn split_for_impl(&self) -> (AnnotatedImplGenerics, AnnotatedType, AnnotatedWhereClause) {
        (
            AnnotatedImplGenerics(self),
            AnnotatedType(self),
            AnnotatedWhereClause(self),
        )
    }
}

pub struct AnnotatedImplGenerics<'a>(&'a Annotated<'a>);
impl<'a> ToTokens for AnnotatedImplGenerics<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(&Annotated { generics, .. }) = self;
        let mut new_generics = generics.clone();
        new_generics
            .params
            .push(GenericParam::Type(self.0.annotation()));

        let (impl_generics, _, _) = new_generics.split_for_impl();
        impl_generics.to_tokens(tokens);
    }
}
pub struct AnnotatedType<'a>(&'a Annotated<'a>);
impl<'a> ToTokens for AnnotatedType<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(&Annotated {
            path_annotated,
            type_ident,
            generics,
            ..
        }) = self;
        let a = self.0.annotation().ident;
        let (_, ty_generics, _) = generics.split_for_impl();

        let ty: Type = parse_quote! { #path_annotated<#type_ident #ty_generics, #a> };
        ty.to_tokens(tokens);
    }
}

pub struct AnnotatedWhereClause<'a>(&'a Annotated<'a>);
impl<'a> ToTokens for AnnotatedWhereClause<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(&Annotated {
            generics,
            ref annotation_where,
            ..
        }) = self;
        let mut g = generics.clone();
        let where_clause = g.make_where_clause();
        annotation_where.iter().for_each(|p| {
            where_clause.predicates.push(p.clone());
        });
        where_clause.to_tokens(tokens);
    }
}
