use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, GenericParam, Generics, Ident, ImplGenerics, Path, Type, TypeParam, WhereClause,
    WherePredicate,
};

#[derive(Debug, Clone)]
pub struct Annotated<'a> {
    path_annotated: &'a Path,
    type_ident: &'a Ident, // TODO Type?
    generics: &'a Generics,
    annotation: &'a Annotation,
}
impl<'a> Annotated<'a> {
    pub fn new(
        path_annotated: &'a Path,
        type_ident: &'a Ident,
        generics: &'a Generics,
        annotation: &'a Annotation,
    ) -> Self {
        Self {
            generics,
            path_annotated,
            type_ident,
            annotation,
        }
    }

    pub fn annotation(&self) -> &Annotation {
        self.annotation
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
        let generic_param = GenericParam::Type(self.0.annotation().param().clone());
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
        let a = self.0.annotation().ty();
        let (_, ty_generics, _) = generics.split_for_impl();

        parse_quote! { #path_annotated<#type_ident #ty_generics, #a> }
    }
    pub fn ty_self(&self) -> Type {
        let Self(&Annotated { path_annotated, .. }) = self;
        let a = self.0.annotation().ty();
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
        let mut annotation_generics = self.0.annotation().generics.clone();
        let annotation_where = annotation_generics.make_where_clause();
        where_clause
            .predicates
            .extend(annotation_where.predicates.iter().cloned());
    }
}

#[derive(Debug, Clone)]
pub struct Annotation {
    type_param: TypeParam,
    ty: Type,
    generics: Generics,
}
impl Annotation {
    pub fn new(
        type_param: TypeParam,
        ty: Option<Type>,
        where_predicate: Option<WherePredicate>,
    ) -> Self {
        let ty = ty.unwrap_or_else(|| {
            let TypeParam { ident, .. } = &type_param;
            parse_quote! { #ident }
        });

        let mut generics = Generics::default();
        generics.params.push(GenericParam::Type(type_param.clone()));
        generics
            .make_where_clause()
            .predicates
            .extend(where_predicate);

        Self {
            type_param,
            ty,
            generics,
        }
    }

    pub fn param(&self) -> &TypeParam {
        &self.type_param
    }
    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn annotated<'a>(
        &'a self,
        path_annotated: &'a Path,
        type_ident: &'a Ident,
        generics: &'a Generics,
    ) -> Annotated<'a> {
        Annotated::new(path_annotated, type_ident, generics, self)
    }
}
