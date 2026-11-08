use darling::{FromDeriveInput, FromField, FromMeta};
use syn::{
    AngleBracketedGenericArguments, DeriveInput, Expr, ExprPath, Field, GenericArgument, Ident,
    PathArguments, Type, TypePath, parse_quote,
};

use crate::attr::DeriveAttr;

#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default)]
pub struct Impls {
    monoid: Option<Expr>,
}

#[derive(Debug, Clone, Default, PartialEq, FromMeta)]
#[darling(default, from_word = || Ok(Self::default()))]
pub struct AnnotatedArgs {
    param: Option<Ident>,
}

#[derive(Debug, Clone, PartialEq, FromDeriveInput)]
#[darling(attributes(semigroup))]
pub struct ContainerAttr {
    #[darling(flatten)]
    derive_attr: DeriveAttr,
    #[darling(rename = "impl", default)]
    impls: Impls,
    #[darling(default)]
    annotated: AnnotatedArgs,
    with: Option<Expr>,
}
impl ContainerAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }
    pub fn derive_attr(&self) -> &DeriveAttr {
        &self.derive_attr
    }
    pub fn identity(&self) -> Option<&Expr> {
        self.impls.monoid.as_ref()
    }
    pub fn annotation_param(&self) -> Ident {
        self.annotated
            .param
            .clone()
            .unwrap_or_else(|| parse_quote! { A })
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
            #[derive(Semigroup, Monoid, AnnotateFields)]
            #[semigroup(
                impl(monoid = Self { num: None }),
                annotated(param = B),
                with = "semigroup::op::Coalesce",
            )]
            pub struct NamedStruct {}
        },
        Ok(ContainerAttr {
            derive_attr: DeriveAttr::default(),
            impls: Impls { monoid: Some(parse_quote! { Self { num: None } }) },
            annotated: AnnotatedArgs { param: Some(parse_quote! { B }) },
            with: Some(parse_quote! { semigroup::op::Coalesce }),
        }),
    )]
    #[case::impl_of_op(
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(impl(semigroup = |_, _| ()))]
            pub struct UnnamedStruct();
        },
        Err("Unknown field: `semigroup`. Available values: `monoid`"),
    )]
    #[case::legacy_properties(
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(properties(monoid))]
            pub struct UnnamedStruct();
        },
        Err("Unknown field: `properties`. Available values: `annotated`, `bound`, `crate`, `impl`, `with`"),
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
