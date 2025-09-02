use syn::{parse_quote, Ident, Path};

pub const DERIVE_CONSTRUCTION: &str = "Construction";
pub const DERIVE_SEMIGROUP: &str = "Semigroup";

#[derive(Debug, Clone)]
pub struct Constant {
    pub path_semigroup: Path,
    pub ident_semigroup_op: Ident,
    pub path_annotated_semigroup: Path,
    pub ident_annotated_op: Ident,
    pub path_annotated: Path,
    pub path_reversed: Path,
    pub path_construction_trait: Path,
    pub path_construction_annotated: Path,
}
pub trait ConstantExt {
    fn constant() -> Constant;
}
pub enum Absolute {}
impl ConstantExt for Absolute {
    fn constant() -> Constant {
        Constant {
            path_semigroup: parse_quote!(::coalesced::Semigroup),
            ident_semigroup_op: parse_quote!(semigroup_op),
            path_annotated_semigroup: parse_quote!(::coalesced::AnnotatedSemigroup),
            ident_annotated_op: parse_quote!(annotated_op),
            path_annotated: parse_quote!(::coalesced::Annotated),
            path_reversed: parse_quote!(::coalesced::Reversed),
            path_construction_trait: parse_quote!(::coalesced::op::Construction),
            path_construction_annotated: parse_quote!(::coalesced::op::ConstructionAnnotated),
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
            ident_semigroup_op: parse_quote!(semigroup_op),
            path_annotated_semigroup: parse_quote!(AnnotatedSemigroup),
            ident_annotated_op: parse_quote!(annotated_op),
            path_annotated: parse_quote!(Annotated),
            path_reversed: parse_quote!(Reversed),
            path_construction_trait: parse_quote!(Construction),
            path_construction_annotated: parse_quote!(ConstructionAnnotated),
        }
    }
}
