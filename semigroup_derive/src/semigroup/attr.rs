use darling::{FromDeriveInput, FromField};
use syn::{
    AngleBracketedGenericArguments, DeriveInput, Expr, ExprPath, Field, GenericArgument, Ident,
    PathArguments, Type, TypePath, WherePredicate, parse_quote,
};

use crate::{error::SemigroupError, name::var_name};

#[derive(Debug, Clone, PartialEq, FromDeriveInput)]
#[darling(attributes(semigroup), and_then = Self::validate)]
pub struct ContainerAttr {
    #[darling(default)]
    annotated: bool,

    #[darling(default)]
    monoid: bool,
    identity: Option<Expr>,
    monoid_where: Option<String>, // TODO Vec
    #[darling(default)]
    without_monoid_impl: bool,

    #[darling(default)]
    commutative: bool,
    commutative_where: Option<String>, // TODO Vec

    with: Option<Expr>,
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
            monoid,
            identity,
            monoid_where,
            without_monoid_impl,
            commutative,
            commutative_where,
            ..
        } = &self;
        if !annotated {
            let err_attr_name = if annotation_param.is_some() {
                Some(var_name!(annotation_param))
            } else {
                None
            };
            err_attr_name.map_or(Ok(()), |a| {
                Err(darling::Error::custom(SemigroupError::OnlyAnnotated(a)))
            })?;
        }
        if !monoid {
            let err_attr_name = if identity.is_some() {
                Some(var_name!(identity))
            } else if monoid_where.is_some() {
                Some(var_name!(monoid_where))
            } else if *without_monoid_impl {
                Some(var_name!(without_monoid_impl))
            } else {
                None
            };
            err_attr_name.map_or(Ok(()), |a| {
                Err(darling::Error::custom(SemigroupError::OnlyMonoid(a)))
            })?;
        }
        if !commutative {
            let err_attr_name = if commutative_where.is_some() {
                Some(var_name!(commutative_where))
            } else {
                None
            };
            err_attr_name.map_or(Ok(()), |a| {
                Err(darling::Error::custom(SemigroupError::OnlyCommutative(a)))
            })?;
        }
        Ok(self)
    }

    pub fn is_annotated(&self) -> bool {
        self.annotated
    }

    pub fn is_monoid(&self) -> bool {
        self.monoid
    }
    pub fn identity(&self) -> Option<&Expr> {
        self.identity.as_ref()
    }
    pub fn monoid_where(&self) -> Option<WherePredicate> {
        self.monoid_where
            .as_deref()
            .map(syn::parse_str)
            .map(|p| p.unwrap_or_else(|e| todo!("{e}")))
    }
    pub fn with_monoid_impl(&self) -> bool {
        !self.without_monoid_impl
    }

    pub fn is_commutative(&self) -> bool {
        self.commutative
    }
    pub fn commutative_where(&self) -> Option<WherePredicate> {
        self.commutative_where
            .as_deref()
            .map(syn::parse_str)
            .map(|p| p.unwrap_or_else(|e| todo!("{e}")))
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
    #[case::invalid_monoid_attr(
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(identity = ())]
            pub struct UnnamedStruct();
        },
        Err("attribute `identity` are supported only with `monoid`"),
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
