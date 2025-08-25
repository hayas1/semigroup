use darling::FromDeriveInput;
use syn::{parse_quote, DeriveInput, Ident, Type};

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(construction))]
pub struct ConstructionAttr {
    #[darling(default)]
    pub annotated: bool,
    #[darling(default)]
    pub unit: Option<Type>,

    pub op: Ident,

    pub annotation_generic_param: Option<Ident>,
}
impl ConstructionAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }

    pub fn is_annotated(&self) -> bool {
        self.annotated || self.unit.is_some() || self.annotation_generic_param.is_some()
    }

    pub fn unit_annotate(&self) -> Type {
        self.unit.clone().unwrap_or_else(|| parse_quote!(()))
    }
}
