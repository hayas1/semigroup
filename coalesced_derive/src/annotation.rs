use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, GenericParam, Generics, ImplGenerics, Type, TypeParam, WhereClause, WherePredicate,
};

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

    pub fn split_for_impl<'a>(
        &'a self,
        generics: &'a Generics,
    ) -> (
        AnnotationImplGenerics<'a>,
        AnnotationType<'a>,
        AnnotationWhereClause<'a>,
    ) {
        (
            AnnotationImplGenerics {
                annotation: self,
                generics,
            },
            AnnotationType { annotation: self },
            AnnotationWhereClause {
                annotation: self,
                generics,
            },
        )
    }
}

pub struct AnnotationImplGenerics<'a> {
    annotation: &'a Annotation,
    generics: &'a Generics,
}
impl<'a> ToTokens for AnnotationImplGenerics<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut generics = self.generics.clone();
        self.impl_generics(&mut generics).to_tokens(tokens);
    }
}
impl<'a> AnnotationImplGenerics<'a> {
    pub fn impl_generics(&self, generics: &'a mut Generics) -> ImplGenerics<'a> {
        let generic_param = GenericParam::Type(self.annotation.param().clone());
        generics.params.push(generic_param);
        let (impl_generics, _, _) = generics.split_for_impl();
        impl_generics
    }
}

pub struct AnnotationType<'a> {
    annotation: &'a Annotation,
}
impl<'a> ToTokens for AnnotationType<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.annotation.ty().to_tokens(tokens);
    }
}

pub struct AnnotationWhereClause<'a> {
    annotation: &'a Annotation,
    generics: &'a Generics,
}
impl<'a> ToTokens for AnnotationWhereClause<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut generics = self.generics.clone();
        let where_clause = generics.make_where_clause();
        self.extend_where_clause(where_clause);
        where_clause.to_tokens(tokens);
    }
}
impl AnnotationWhereClause<'_> {
    pub fn extend_where_clause(&self, where_clause: &mut WhereClause) {
        let mut annotation_generics = self.annotation.generics.clone();
        let annotation_where = annotation_generics.make_where_clause();
        where_clause
            .predicates
            .extend(annotation_where.predicates.iter().cloned());
    }
}
