use syn::{parse_quote, Ident, Path};

pub const DERIVE_CONSTRUCTION: &str = "Construction";

#[derive(Debug, Clone)]
pub struct Constant {
    pub path_semigroup: Path,
    pub ident_semigroup_op: Ident,
    pub path_annotated: Path,
    pub path_reversed: Path,
}
impl Constant {
    pub fn new() -> Self {
        Constant {
            path_semigroup: parse_quote!(::coalesced::Semigroup),
            ident_semigroup_op: parse_quote!(semigroup_op),
            path_annotated: parse_quote!(::coalesced::Annotated),
            path_reversed: parse_quote!(::coalesced::op::reverse::Reversed),
        }
    }
    #[cfg(feature = "use_scope")]
    pub fn new_use() -> Self {
        Constant {
            path_semigroup: parse_quote!(Semigroup),
            ident_semigroup_op: parse_quote!(semigroup_op),
            path_annotated: parse_quote!(Annotated),
            path_reversed: parse_quote!(Reversed),
        }
    }
}
