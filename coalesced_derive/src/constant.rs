use syn::{parse_quote, Ident, Path, Stmt};

pub const DERIVE_CONSTRUCTION: &str = "Construction";

#[derive(Debug, Clone)]
pub struct Constant {
    pub path_semigroup: Path,
    pub path_annotated_semigroup: Path,
    pub ident_semigroup_op: Ident,
    pub use_annotate: Option<Stmt>,
    pub path_annotated: Path,
    pub path_reversed: Path,
}
impl Constant {
    pub fn new<C: ConstantExt>() -> Self {
        C::constant()
    }
}
pub trait ConstantExt {
    fn constant() -> Constant;
}
pub enum Absolute {}
impl ConstantExt for Absolute {
    fn constant() -> Constant {
        Constant {
            path_semigroup: parse_quote!(::coalesced::Semigroup),
            path_annotated_semigroup: parse_quote!(::coalesced::AnnotatedSemigroup),
            ident_semigroup_op: parse_quote!(semigroup_op),
            use_annotate: Some(parse_quote!(
                use coalesced::Annotate;
            )),
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
            path_annotated_semigroup: parse_quote!(AnnotatedSemigroup),
            ident_semigroup_op: parse_quote!(semigroup_op),
            use_annotate: None,
            path_annotated: parse_quote!(Annotated),
            path_reversed: parse_quote!(Reversed),
        }
    }
}
