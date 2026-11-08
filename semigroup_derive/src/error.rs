use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::{
    attr::Property,
    derive::{OpDerive, SemigroupDerive},
};

#[derive(Debug, Clone)]
pub enum SemigroupError {
    ExpectStruct(SemigroupDerive),
    BoundRequired(SemigroupDerive, Property),
}
impl Error for SemigroupError {}
impl Display for SemigroupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectStruct(derive) => {
                write!(f, "derive {derive} only supports structs")
            }
            Self::BoundRequired(derive, property) => {
                write!(
                    f,
                    "derive {derive} infers its bounds from struct fields, so other items require `bound({property} = ..)`"
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum OpError {
    ExpectNewType(OpDerive),
    ImplRequired(OpDerive),
}
impl Error for OpError {}
impl Display for OpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectNewType(derive) => {
                write!(f, "derive {derive} only supports newtype structs")
            }
            Self::ImplRequired(derive) => {
                let property = derive.property();
                write!(f, "derive {derive} requires `impl({property} = ..)`")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PropertiesError {
    InvalidDocAttr,
    DeriveNotFound,
}
impl Error for PropertiesError {}
impl Display for PropertiesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDocAttr => write!(f, "invalid doc attribute"),
            Self::DeriveNotFound => write!(
                f,
                "no derive of semigroup is found: place `#[semigroup_properties_doc]` above `#[derive(..)]`"
            ),
        }
    }
}
