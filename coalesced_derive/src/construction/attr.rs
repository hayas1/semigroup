use darling::FromDeriveInput;
use syn::{parse_quote, DeriveInput, Ident, Type, TypeParam};

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(construction))]
pub struct ConstructionAttr {
    #[darling(default)]
    pub annotated: bool,
    pub unit: Option<Type>,

    pub op: Ident,

    pub annotation_type_param: Option<TypeParam>,
}
impl ConstructionAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }

    pub fn is_annotated(&self) -> bool {
        self.annotated || self.unit.is_some() || self.annotation_type_param.is_some()
    }

    pub fn unit_annotate(&self) -> Type {
        self.unit.clone().unwrap_or_else(|| parse_quote!(()))
    }

    pub fn annotation_type_param(&self) -> TypeParam {
        self.annotation_type_param
            .clone()
            .unwrap_or_else(|| parse_quote!(A))
    }
}
