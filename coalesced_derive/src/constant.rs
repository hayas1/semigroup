use syn::{parse_quote, Ident, Path};

pub const DERIVE_CONSTRUCTION: &str = "Construction";

#[derive(Debug, Clone)]
pub struct Constant {
    pub path_semigroup: Path,
    pub _path_annotated_semigroup: Path,
    pub ident_semigroup_op: Ident,
    pub path_annotated: Path,
    pub path_reversed: Path,
}
pub trait ConstantExt {
    fn constant() -> Constant;
}
pub enum Absolute {}
impl ConstantExt for Absolute {
    fn constant() -> Constant {
        Constant {
            path_semigroup: parse_quote!(::coalesced::Semigroup),
            _path_annotated_semigroup: parse_quote!(::coalesced::AnnotatedSemigroup),
            ident_semigroup_op: parse_quote!(semigroup_op),
            path_annotated: parse_quote!(::coalesced::Annotated),
            path_reversed: parse_quote!(::coalesced::op::reverse::Reversed),
        }
    }
}
#[cfg(feature = "use_scope")]
pub enum Use {}
#[cfg(feature = "use_scope")]
impl ConstantExt for Use {
    fn constant() -> Constant {
        Constant {
            path_semigroup: parse_quote!(Semigroup),
            _path_annotated_semigroup: parse_quote!(AnnotatedSemigroup),
            ident_semigroup_op: parse_quote!(semigroup_op),
            path_annotated: parse_quote!(Annotated),
            path_reversed: parse_quote!(Reversed),
        }
    }
}
