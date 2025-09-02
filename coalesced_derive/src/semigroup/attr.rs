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
    pub fn is_annotated(&self) -> bool {
        self.annotated
    }
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(semigroup))]
pub struct FieldAttr {
    pub with: Option<Path>,
}
impl FieldAttr {
    pub fn new(field: &Field) -> syn::Result<Self> {
        Ok(Self::from_field(field)?)
    }
    pub fn with<'a>(&'a self, container: &'a ContainerAttr) -> Option<&'a Path> {
        self.with.as_ref().or(container.with.as_ref())
    }
}
