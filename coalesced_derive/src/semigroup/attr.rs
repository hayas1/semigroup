use darling::{FromDeriveInput, FromField};
use syn::{parse_quote, DeriveInput, Field, Ident, Path, TypeParam};

use crate::{annotation::Annotation, error::SemigroupError};

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(semigroup), and_then = Self::validate)]
pub struct ContainerAttr {
    #[darling(default)]
    annotated: bool,

    with: Option<Path>,
    annotation_param: Option<Ident>,
}
impl ContainerAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }
    pub fn validate(self) -> darling::Result<Self> {
        let Self {
            annotated,
            annotation_param,
            ..
        } = &self;
        if !annotated && annotation_param.is_some() {
            Err(darling::Error::custom(SemigroupError::OnlyAnnotated))
        } else {
            Ok(self)
        }
    }
    pub fn is_annotated(&self) -> bool {
        self.annotated
    }

    pub fn annotation(&self, annotation_ident: &Ident) -> Annotation {
        let default_param: TypeParam = parse_quote! { A };
        let a = self
            .annotation_param
            .as_ref()
            .unwrap_or(&default_param.ident);
        Annotation::new(
            parse_quote! { #a: Clone },
            Some(parse_quote! { #annotation_ident<#a> }),
            None,
        )
    }
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(semigroup))]
pub struct FieldAttr {
    with: Option<Path>,
}
impl FieldAttr {
    pub fn new(field: &Field) -> syn::Result<Self> {
        Ok(Self::from_field(field)?)
    }
    pub fn with<'a>(&'a self, container: &'a ContainerAttr) -> Option<&'a Path> {
        self.with.as_ref().or(container.with.as_ref())
    }
}
