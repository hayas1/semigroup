use darling::{FromDeriveInput, FromField};
use syn::{DeriveInput, Field, Path};

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(semigroup))]
pub struct ContainerAttr {
    #[darling(default)]
    pub annotated: bool,

    pub with: Option<Path>,
}
impl ContainerAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }
}

#[derive(Debug, Clone, FromField)]
pub struct FieldAttr {
    pub with: Option<Path>,
}
impl FieldAttr {
    pub fn new(field: &Field) -> syn::Result<Self> {
        Ok(Self::from_field(field)?)
    }
}
