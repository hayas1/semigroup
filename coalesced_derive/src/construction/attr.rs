use darling::FromDeriveInput;
use syn::{parse_quote, DeriveInput, Expr, Ident, TypeParam, WherePredicate};

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(construction))]
pub struct ContainerAttr {
    #[darling(default)]
    pub annotated: bool,
    pub unit: Option<Expr>,

    pub op: Ident,

    pub annotation_type_param: Option<TypeParam>,
    pub annotation_where: Option<String>,
    #[darling(default)]
    pub without_annotate_impl: bool,
}
impl ContainerAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }

    pub fn is_annotated(&self) -> bool {
        self.annotated || self.unit.is_some() || self.annotation_type_param.is_some()
    }

    pub fn unit_annotate(&self) -> Expr {
        self.unit.clone().unwrap_or_else(|| parse_quote!(()))
    }

    pub fn annotation_type_param(&self) -> TypeParam {
        self.annotation_type_param
            .clone()
            .unwrap_or_else(|| parse_quote!(A))
    }

    pub fn annotation_where(&self) -> Option<WherePredicate> {
        self.annotation_where
            .as_ref()
            .map(|p| syn::parse_str(p).unwrap_or_else(|_| todo!()))
    }
}
