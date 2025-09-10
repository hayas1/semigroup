use darling::FromDeriveInput;
use heck::ToSnakeCase;
use quote::format_ident;
use syn::{parse_quote, DeriveInput, Expr, Ident, TypeParam};

use crate::{annotation::Annotation, error::ConstructionError};

#[derive(Debug, Clone, PartialEq, FromDeriveInput)]
#[darling(attributes(construction), and_then = Self::validate)]
pub struct ContainerAttr {
    #[darling(default)]
    annotated: bool,
    unit: Option<Expr>,

    op: Ident,

    annotation_type_param: Option<TypeParam>,
    annotation_where: Option<String>,
    #[darling(default)]
    without_annotate_impl: bool,
}
impl ContainerAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }
    pub fn validate(self) -> darling::Result<Self> {
        let Self {
            annotated,
            unit,
            annotation_type_param,
            annotation_where,
            without_annotate_impl,
            ..
        } = &self;
        if !annotated
            && (unit.is_some()
                || annotation_type_param.is_some()
                || annotation_where.is_some()
                || *without_annotate_impl)
        {
            Err(darling::Error::custom(ConstructionError::OnlyAnnotated))
        } else {
            Ok(self)
        }
    }

    pub fn is_annotated(&self) -> bool {
        self.annotated
    }

    pub fn op_trait(&self) -> &Ident {
        &self.op
    }
    pub fn op_method(&self) -> Ident {
        format_ident!("{}", self.op.to_string().to_snake_case())
    }

    pub fn unit_annotate(&self) -> Expr {
        self.unit.clone().unwrap_or_else(|| parse_quote!(()))
    }

    pub fn annotation(&self) -> Annotation {
        Annotation::new(
            self.annotation_type_param
                .clone()
                .unwrap_or_else(|| parse_quote! { A }),
            None,
            self.annotation_where
                .as_ref()
                .map(|p| syn::parse_str(p).unwrap_or_else(|_| todo!())),
        )
    }
    pub fn with_annotate_impl(&self) -> bool {
        !self.without_annotate_impl
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_container_attr() -> ContainerAttr {
        ContainerAttr::new(&parse_quote! {
            #[derive(Construction)]
            #[construction(op = "op")]
            pub struct Construct<T>(T);
        })
        .unwrap()
    }

    #[test]
    fn test_construction_container_attr() {
        let cases = vec![
            (
                syn::parse_quote! {
                    #[derive(Construction)]
                    #[construction(annotated, op = Coalesce)]
                    pub struct Coalesced<T>(pub Option<T>);
                },
                Ok(ContainerAttr {
                    annotated: true,
                    op: format_ident!("Coalesce"),
                    ..default_container_attr()
                }),
            ),
            (
                syn::parse_quote! {
                    #[derive(Construction)]
                    pub struct Construct<T>(T);
                },
                Err(darling::Error::custom("Missing field `op`")),
            ),
            (
                syn::parse_quote! {
                    #[derive(Construction)]
                    #[construction(op = Coalesce, unit = ())]
                    pub struct Coalesced<T>(pub Option<T>);
                },
                Err(darling::Error::custom(ConstructionError::OnlyAnnotated)),
            ),
        ];
        cases.into_iter().for_each(|(input, expected)| {
            let actual = ContainerAttr::new(&input);
            match (actual, expected) {
                (Ok(actual), Ok(expected)) => assert_eq!(actual, expected),
                (Ok(actual), Err(expected)) => panic!("actual: {actual:?}, expected: {expected:?}"),
                (Err(actual), Ok(expected)) => panic!("actual: {actual:?}, expected: {expected:?}"),
                (Err(actual), Err(expected)) => {
                    assert_eq!(actual.to_string(), expected.to_string())
                }
            }
        });
    }
}
