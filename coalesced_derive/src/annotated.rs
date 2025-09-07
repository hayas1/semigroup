use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, GenericParam, Generics, Ident, ImplGenerics, Path, Type, TypeParam, WhereClause,
    WherePredicate,
};

#[derive(Debug, Clone)]
pub struct Annotated<'a> {
    path_annotated: &'a Path,
    type_ident: &'a Ident, // TODO TypePath? Path?
    generics: &'a Generics,
    annotation: TypeParam,
    annotation_type: Type,
    annotation_where: Option<WherePredicate>,
}
impl<'a> Annotated<'a> {
    pub fn new(
        path_annotated: &'a Path,
        type_ident: &'a Ident,
        generics: &'a Generics,
        annotation: TypeParam,
        annotation_type: Option<Type>,
        annotation_where: Option<WherePredicate>,
    ) -> Self {
        let annotation_type = annotation_type.unwrap_or_else(|| {
            let TypeParam { ident, .. } = &annotation;
            parse_quote! { #ident }
        });
        Self {
            generics,
            path_annotated,
            type_ident,
            annotation,
            annotation_type,
            annotation_where,
        }
    }

    pub fn annotation_param(&self) -> TypeParam {
        self.annotation.clone()
    }
    pub fn annotation_type(&self) -> Type {
        self.annotation_type.clone()
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
        let mut generics = self.0.generics.clone();
        self.impl_generics(&mut generics).to_tokens(tokens);
    }
}
impl<'a> AnnotatedImplGenerics<'a> {
    pub fn impl_generics(&self, generics: &'a mut Generics) -> ImplGenerics<'a> {
        let generic_param = GenericParam::Type(self.0.annotation_param());
        generics.params.push(generic_param);
        let (impl_generics, _, _) = generics.split_for_impl();
        impl_generics
    }
}

pub struct AnnotatedType<'a>(&'a Annotated<'a>);
impl<'a> ToTokens for AnnotatedType<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ty().to_tokens(tokens);
    }
}
impl AnnotatedType<'_> {
    pub fn ty(&self) -> Type {
        let Self(&Annotated {
            path_annotated,
            type_ident,
            generics,
            ..
        }) = self;
        let a = self.0.annotation_type();
        let (_, ty_generics, _) = generics.split_for_impl();

        parse_quote! { #path_annotated<#type_ident #ty_generics, #a> }
    }
    pub fn ty_self(&self) -> Type {
        let Self(&Annotated { path_annotated, .. }) = self;
        let a = self.0.annotation_type();
        parse_quote! { #path_annotated<Self, #a> }
    }
}

pub struct AnnotatedWhereClause<'a>(&'a Annotated<'a>);
impl<'a> ToTokens for AnnotatedWhereClause<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut generics = self.0.generics.clone();
        let where_clause = generics.make_where_clause();
        self.push_where_clause(where_clause);
        where_clause.to_tokens(tokens);
    }
}
impl AnnotatedWhereClause<'_> {
    pub fn push_where_clause(&self, where_clause: &mut WhereClause) {
        self.0.annotation_where.iter().for_each(|p| {
            where_clause.predicates.push(p.clone());
        });
    }
}
