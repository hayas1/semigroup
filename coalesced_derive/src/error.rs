use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::constant::DERIVE_CONSTRUCTION;

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
