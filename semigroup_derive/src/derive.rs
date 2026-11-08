use std::fmt::{Display, Formatter};

use syn::Path;

use crate::attr::Property;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemigroupDerive {
    Semigroup,
    Monoid,
    Commutative,
    AnnotateFields,
}
impl Display for SemigroupDerive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
impl SemigroupDerive {
    const ALL: [Self; 4] = [
        Self::Semigroup,
        Self::Monoid,
        Self::Commutative,
        Self::AnnotateFields,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Semigroup => "Semigroup",
            Self::Monoid => "Monoid",
            Self::Commutative => "Commutative",
            Self::AnnotateFields => "AnnotateFields",
        }
    }
    pub fn property(self) -> Property {
        match self {
            Self::Semigroup => Property::Semigroup,
            Self::Monoid => Property::Monoid,
            Self::Commutative => Property::Commutative,
            Self::AnnotateFields => Property::Annotated,
        }
    }
    pub fn from_path(path: &Path) -> Option<Self> {
        let ident = &path.segments.last()?.ident;
        Self::ALL.into_iter().find(|derive| ident == derive.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpDerive {
    Semigroup,
    Selective,
    Monoid,
    Commutative,
}
impl Display for OpDerive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
impl OpDerive {
    const ALL: [Self; 4] = [
        Self::Semigroup,
        Self::Selective,
        Self::Monoid,
        Self::Commutative,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Semigroup => "SemigroupOp",
            Self::Selective => "SelectiveOp",
            Self::Monoid => "MonoidOp",
            Self::Commutative => "CommutativeOp",
        }
    }
    pub fn property(self) -> Property {
        match self {
            Self::Semigroup => Property::Semigroup,
            Self::Selective => Property::Selective,
            Self::Monoid => Property::Monoid,
            Self::Commutative => Property::Commutative,
        }
    }
    pub fn from_path(path: &Path) -> Option<Self> {
        let ident = &path.segments.last()?.ident;
        Self::ALL.into_iter().find(|derive| ident == derive.name())
    }
}
