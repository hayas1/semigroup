use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::{
    constant::{DERIVE_OP, DERIVE_SEMIGROUP},
    name::Name,
};

#[derive(Debug, Clone)]
pub enum ConstructionError {
    ExpectNewType,
    UnsupportedProperty(Name),
    ManualSemigroupWithoutSelective,
}
impl Error for ConstructionError {}
impl Display for ConstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectNewType => {
                write!(f, "derive {DERIVE_OP} only supports newtype structs",)
            }
            Self::UnsupportedProperty(Name(name)) => {
                write!(f, "derive {DERIVE_OP} does not support property `{name}`")
            }
            Self::ManualSemigroupWithoutSelective => {
                write!(
                    f,
                    "`semigroup(manual_impl)` of derive {DERIVE_OP} only skips the `SemigroupOp` impl derived from `SelectiveOp`, so it requires `selective`"
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SemigroupError {
    UnsupportedEnum,
    UnsupportedUnion,
    UnsupportedProperty(Name),
    ManualSemigroupImpl,
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
            Self::UnsupportedProperty(Name(name)) => {
                write!(
                    f,
                    "derive {DERIVE_SEMIGROUP} does not support property `{name}`"
                )
            }
            Self::ManualSemigroupImpl => {
                write!(
                    f,
                    "derive {DERIVE_SEMIGROUP} does not support `semigroup(manual_impl)`: deriving the `Semigroup` impl is what it does"
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PropertiesError {
    InvalidDocAttr,
}
impl Error for PropertiesError {}
impl Display for PropertiesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDocAttr => write!(f, "invalid doc attribute"),
        }
    }
}
