use darling::FromDeriveInput;
use syn::{DeriveInput, Ident};

use crate::error::ConstructionError;

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(construction))]
pub struct ConstructionAttr {
    #[darling(default)]
    pub annotated: bool,
    #[darling(default)]
    pub semigroup: bool,
    pub op: Ident,
}
impl ConstructionAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        let DeriveInput { ident, .. } = derive;
        let this = Self::from_derive_input(derive)?;
        this.validate()
            .map_err(|e| syn::Error::new_spanned(ident, e))?;
        Ok(this)
    }
    pub fn validate(&self) -> Result<&Self, ConstructionError> {
        if self.annotated && self.semigroup {
            Err(ConstructionError::DuplicateConstructionType)
        } else if !self.annotated && !self.semigroup {
            Err(ConstructionError::ConstructionTypeNotFound)
        } else {
            Ok(self)
        }
    }
}
