use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::construction::ConstructionAttr;

#[derive(Debug, Clone)]
pub enum ConstructionError {
    OnlyNewType,
    NoConstructionAttr,
    ConstructionTypeNotFound,
    DuplicateConstructionType,
}
impl Error for ConstructionError {}
impl Display for ConstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OnlyNewType => {
                write!(f, "derive Construction only supports newtype structs")
            }
            Self::NoConstructionAttr => {
                write!(f, "Expected `#[construction(...)]` attribute on the struct")
            }
            Self::ConstructionTypeNotFound => {
                write!(
                    f,
                    "Expected either `{}` or `{}` attribute on the field",
                    ConstructionAttr::Annotated,
                    ConstructionAttr::Semigroup,
                )
            }
            Self::DuplicateConstructionType => {
                write!(
                    f,
                    "Cannot derive both `{}` and `{}` at the same time",
                    ConstructionAttr::Annotated,
                    ConstructionAttr::Semigroup,
                )
            }
        }
    }
}
