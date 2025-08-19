use darling::{ast::NestedMeta, FromMeta};
use syn::{Attribute, Ident, Meta, MetaList};

use crate::{constant::ATTR_CONSTRUCTION, error::ConstructionError};

#[derive(Debug, Clone, FromMeta)]
pub struct ConstructionAttr {
    #[darling(default)]
    pub annotated: bool,
    #[darling(default)]
    pub semigroup: bool,
    pub op: Ident,
}
impl ConstructionAttr {
    pub fn new(attrs: &[Attribute], ident: &Ident) -> syn::Result<Self> {
        let attr = attrs
            .iter()
            .find_map(|Attribute { meta, .. }| match meta {
                Meta::List(MetaList { path, tokens, .. }) if path.is_ident(ATTR_CONSTRUCTION) => {
                    Some(tokens)
                }
                _ => None,
            })
            .ok_or(syn::Error::new_spanned(
                ident,
                ConstructionError::NoConstructionAttr,
            ))?;

        let this = Self::from_list(&NestedMeta::parse_meta_list(attr.clone())?)?;
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
