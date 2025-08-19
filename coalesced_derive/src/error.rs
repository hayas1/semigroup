use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::constant::{ATTR_ANNOTATED, ATTR_SEMIGROUP, DERIVE_CONSTRUCTION};

#[derive(Debug, Clone)]
pub enum ConstructionError {
    OnlyNewType,
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
                    "derive {DERIVE_CONSTRUCTION} only supports newtype structs",
                )
            }
            Self::ConstructionTypeNotFound => {
                write!(
                    f,
                    "Expected either `{ATTR_ANNOTATED}` or `{ATTR_SEMIGROUP}` attribute on the field",
                )
            }
            Self::DuplicateConstructionType => {
                write!(
                    f,
                    "Cannot derive both `{ATTR_ANNOTATED}` and `{ATTR_SEMIGROUP}` at the same time",
                )
            }
        }
    }
}
