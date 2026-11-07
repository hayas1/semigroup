use darling::{FromDeriveInput, FromField};
use syn::{
    AngleBracketedGenericArguments, DeriveInput, Expr, ExprPath, Field, GenericArgument,
    PathArguments, Type, TypePath, parse_quote,
};

use crate::{
    attr::{DeriveAttr, Property},
    error::SemigroupError,
};

#[derive(Debug, Clone, PartialEq, FromDeriveInput)]
#[darling(attributes(semigroup), and_then = Self::validate)]
pub struct ContainerAttr {
    #[darling(flatten)]
    derive_attr: DeriveAttr,
    with: Option<Expr>,
}
impl ContainerAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }
    fn validate(self) -> darling::Result<Self> {
        let Self { derive_attr, .. } = &self;
        if derive_attr.implements(Property::Selective) {
            return Err(darling::Error::custom(SemigroupError::UnsupportedProperty(
                Property::Selective,
            )));
        }
        if !derive_attr.derives(Property::Semigroup) {
            return Err(darling::Error::custom(SemigroupError::ManualSemigroupImpl));
        }
        Ok(self)
    }
    pub fn derive_attr(&self) -> &DeriveAttr {
        &self.derive_attr
    }
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(semigroup))]
pub struct FieldAttr {
    with: Option<Expr>,
}
impl FieldAttr {
    pub fn new(field: &Field) -> syn::Result<Self> {
        Ok(Self::from_field(field)?)
    }
    pub fn with<'a>(&'a self, container: &'a ContainerAttr) -> Option<With<'a>> {
        let expr = self.with.as_ref().or(container.with.as_ref())?;
        Some(match expr {
            Expr::Path(p) => With::Path(p),
            other => With::Constructor(other),
        })
    }
}

/// The resolved value of a `#[semigroup(with = "...")]` attribute.
#[derive(Debug, Clone)]
pub enum With<'a> {
    /// Bare path, e.g. `Dual` — backward-compatible lift_* API.
    Path(&'a ExprPath),
    /// Constructor expression, e.g. `Dual(Coalesce(_))`.
    Constructor(&'a Expr),
}

impl With<'_> {
    /// Replace every [`Expr::Infer`] (`_`) placeholder with `replacement`.
    /// For a bare path there is no `_`, so the path is returned unchanged.
    pub fn substitute(&self, replacement: &Expr) -> Expr {
        fn substitute_recursive(expr: Expr, replacement: &Expr) -> Expr {
            match expr {
                Expr::Infer(_) => replacement.clone(),
                Expr::Call(mut call) => {
                    call.args = call
                        .args
                        .into_iter()
                        .map(|arg| substitute_recursive(arg, replacement))
                        .collect();
                    Expr::Call(call)
                }
                other => other,
            }
        }
        match self {
            With::Path(p) => Expr::Path((*p).clone()),
            With::Constructor(expr) => substitute_recursive((*expr).clone(), replacement),
        }
    }

    /// Number of constructor-call layers wrapping the `_` placeholder.
    /// `Dual(Coalesce(_))` → 2, `Coalesce(_)` → 1, bare path → 0.
    pub fn depth(&self) -> usize {
        fn wrap_depth_recursive(expr: &Expr) -> usize {
            match expr {
                Expr::Call(call) => {
                    1 + call
                        .args
                        .iter()
                        .map(wrap_depth_recursive)
                        .max()
                        .unwrap_or(0)
                }
                _ => 0,
            }
        }
        match self {
            With::Path(_) => 0,
            With::Constructor(expr) => wrap_depth_recursive(expr),
        }
    }

    /// Convert to the equivalent type suitable for UFCS.
    /// `Dual(Coalesce(_))` → `Dual<Coalesce<_>>`, bare path → the path as a type.
    pub fn as_type(&self) -> syn::Result<Type> {
        fn type_recursive(expr: &Expr) -> syn::Result<Type> {
            match expr {
                Expr::Infer(_) => Ok(parse_quote! { _ }),
                Expr::Path(p) => Ok(Type::Path(TypePath {
                    attrs: Vec::new(),
                    qself: p.qself.clone(),
                    path: p.path.clone(),
                })),
                Expr::Call(call) => {
                    let base_ty = type_recursive(&call.func)?;
                    let args = call
                        .args
                        .iter()
                        .map(|a| type_recursive(a).map(GenericArgument::Type))
                        .collect::<syn::Result<Vec<_>>>()?;
                    match base_ty {
                        Type::Path(mut tp) => {
                            let last = tp.path.segments.last_mut().expect("path has segments");
                            last.arguments =
                                PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: Default::default(),
                                    args: args.into_iter().collect(),
                                    gt_token: Default::default(),
                                });
                            Ok(Type::Path(tp))
                        }
                        _ => Err(syn::Error::new_spanned(
                            &call.func,
                            "expected a type path as constructor function",
                        )),
                    }
                }
                _ => Err(syn::Error::new_spanned(
                    expr,
                    "expected a constructor call like `Wrapper(_)` or a bare path",
                )),
            }
        }

        match self {
            With::Path(p) => Ok(Type::Path(TypePath {
                attrs: Vec::new(),
                qself: p.qself.clone(),
                path: p.path.clone(),
            })),
            With::Constructor(expr) => type_recursive(expr),
        }
    }

    /// Build `base.into_inner().into_inner()...` (`self.depth()` times).
    pub fn chain_into_inner(&self, base: Expr) -> Expr {
        (0..self.depth()).fold(base, |acc: Expr, _| parse_quote! { #acc.into_inner() })
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::ok(
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(properties(annotated(param = B), monoid), with = "semigroup::op::Coalesce")]
            pub struct NamedStruct {}
        },
        Ok(ContainerAttr {
            derive_attr: <DeriveAttr as darling::FromMeta>::from_list(
                &darling::ast::NestedMeta::parse_meta_list(quote::quote! {
                    properties(annotated(param = B), monoid)
                })
                .unwrap(),
            )
            .unwrap(),
            with: Some(parse_quote! { semigroup::op::Coalesce }),
        }),
    )]
    #[case::identity_at_top_level(
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(identity = ())]
            pub struct UnnamedStruct();
        },
        Err("Unknown field: `identity`. Available values: `bound`, `manual_impl`, `properties`, `with`"),
    )]
    #[case::selective(
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(properties(selective))]
            pub struct UnnamedStruct();
        },
        Err("derive Semigroup does not support property `selective`"),
    )]
    fn test_semigroup_container_attr(
        #[case] input: DeriveInput,
        #[case] expected: Result<ContainerAttr, &str>,
    ) {
        let actual = ContainerAttr::new(&input);
        assert_eq!(
            actual.as_ref().map_err(ToString::to_string),
            expected.as_ref().map_err(ToString::to_string),
        );
    }
}
