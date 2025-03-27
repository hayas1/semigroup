use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum DeriveError {
    UnsupportedUnion,
}
impl std::error::Error for DeriveError {}
impl Display for DeriveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedUnion => write!(f, "derive Coalesce for union is not supported"),
        }
    }
}
