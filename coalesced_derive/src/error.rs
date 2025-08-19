use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use heck::ToUpperCamelCase;

use crate::constant::{ANNOTATED, CONSTRUCTION, SEMIGROUP};

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
                write!(
                    f,
                    "derive {} only supports newtype structs",
                    CONSTRUCTION.to_upper_camel_case()
                )
            }
            Self::NoConstructionAttr => {
                write!(
                    f,
                    "Expected `#[{CONSTRUCTION}(...)]` attribute on the struct",
                )
            }
            Self::ConstructionTypeNotFound => {
                write!(
                    f,
                    "Expected either `{ANNOTATED}` or `{SEMIGROUP}` attribute on the field",
                )
            }
            Self::DuplicateConstructionType => {
                write!(
                    f,
                    "Cannot derive both `{ANNOTATED}` and `{SEMIGROUP}` at the same time",
                )
            }
        }
    }
}
