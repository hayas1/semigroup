use darling::{FromDeriveInput, FromField};
use syn::{parse_quote, DeriveInput, Field, Ident, Path};

use crate::{annotation::Annotation, constant::Constant, error::SemigroupError};

#[derive(Debug, Clone, PartialEq, FromDeriveInput)]
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

    pub fn annotation(&self, constant: &Constant, annotation_ident: &Ident) -> Annotation {
        let a = self
            .annotation_param
            .as_ref()
            .unwrap_or(&constant.default_type_param.ident);
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn default_container_attr() -> ContainerAttr {
        ContainerAttr::new(&parse_quote! {
            #[derive(Semigroup)]
            pub struct NamedStruct {}
        })
        .unwrap()
    }

    #[rstest]
    #[case::ok(
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(annotated)]
            pub struct NamedStruct {}
        },
        Ok(ContainerAttr {
            annotated: true,
            ..default_container_attr()
        }),
    )]
    #[case::invalid_annotated_attr(
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(annotation_param = "X")]
            pub struct UnnamedStruct();
        },
        Err(darling::Error::custom(SemigroupError::OnlyAnnotated)),
    )]
    fn test_semigroup_container_attr(
        #[case] input: DeriveInput,
        #[case] expected: darling::Result<ContainerAttr>,
    ) {
        let actual = ContainerAttr::new(&input);
        match (actual, expected) {
            (Ok(actual), Ok(expected)) => assert_eq!(actual, expected),
            (Ok(actual), Err(expected)) => panic!("actual: {actual:?}, expected: {expected:?}"),
            (Err(actual), Ok(expected)) => panic!("actual: {actual:?}, expected: {expected:?}"),
            (Err(actual), Err(expected)) => {
                assert_eq!(actual.to_string(), expected.to_string())
            }
        }
    }
}
