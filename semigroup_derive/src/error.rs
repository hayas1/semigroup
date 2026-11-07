use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::{
    attr::Property,
    constant::{DERIVE_OP, DERIVE_SEMIGROUP},
};

#[derive(Debug, Clone)]
pub enum ConstructionError {
    ExpectNewType,
    UnsupportedProperty(Property),
    ManualSemigroupWithoutSelective,
}
impl Error for ConstructionError {}
impl Display for ConstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectNewType => {
                write!(f, "derive {DERIVE_OP} only supports newtype structs",)
            }
            Self::UnsupportedProperty(property) => {
                write!(
                    f,
                    "derive {DERIVE_OP} does not support property `{property}`"
                )
            }
            Self::ManualSemigroupWithoutSelective => {
                write!(
                    f,
                    "`manual_impl(semigroup)` of derive {DERIVE_OP} only skips the `SemigroupOp` impl derived from `SelectiveOp`, so it requires `selective`"
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SemigroupError {
    UnsupportedEnum,
    UnsupportedUnion,
    UnsupportedProperty(Property),
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
            Self::UnsupportedProperty(property) => {
                write!(
                    f,
                    "derive {DERIVE_SEMIGROUP} does not support property `{property}`"
                )
            }
            Self::ManualSemigroupImpl => {
                write!(
                    f,
                    "derive {DERIVE_SEMIGROUP} does not support `manual_impl(semigroup)`: deriving the `Semigroup` impl is what it does"
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

#[derive(Debug, Clone)]
pub enum AttrError {
    ManualImplWithoutProperty(Property),
    BoundWithoutProperty(Property),
}
impl Error for AttrError {}
impl Display for AttrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ManualImplWithoutProperty(property) => {
                write!(
                    f,
                    "`manual_impl({property})` requires `{property}` in `properties(..)`"
                )
            }
            Self::BoundWithoutProperty(property) => {
                write!(
                    f,
                    "`bound({property} = ..)` requires `{property}` in `properties(..)`"
                )
            }
        }
    }
}
