use darling::FromDeriveInput;
use syn::{DeriveInput, Ident};

use crate::error::ConstructionError;

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(construction))]
pub struct ConstructionAttr {
    #[darling(default)]
    pub annotated: bool,
    pub op: Ident,
}
impl ConstructionAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }
}
