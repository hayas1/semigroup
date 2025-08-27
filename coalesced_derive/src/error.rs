use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::constant::{DERIVE_CONSTRUCTION, DERIVE_SEMIGROUP};

#[derive(Debug, Clone)]
pub enum ConstructionError {
    OnlyNewType,
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
        }
    }
}

#[derive(Debug, Clone)]
pub enum SemigroupError {
    UnsupportedEnum,
    UnsupportedUnion,
}
impl Error for SemigroupError {}
impl Display for SemigroupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedEnum => {
                write!(f, "derive {DERIVE_SEMIGROUP} does not support enums")
            }
            Self::UnsupportedUnion => {
                write!(f, "derive {DERIVE_SEMIGROUP} does not support unions")
            }
        }
    }
}
