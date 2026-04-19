use syn::{Attribute, Path, parse_quote};

pub const DERIVE_OP: &str = "Op";
pub const DERIVE_SEMIGROUP: &str = "Semigroup";

#[derive(Debug, Clone)]
pub struct Constant {
    pub path_semigroup: Path,
    pub path_idempotent: Path,
    pub path_selected: Path,
    pub path_annotated: Path,
    pub path_annotate: Path,
    pub path_monoid: Path,
    pub path_commutative: Path,
    pub path_construction_trait: Path,
    pub path_semigroup_op: Path,
    pub path_idempotent_op: Path,
    pub path_monoid_op: Path,
    pub attr_feature_monoid: Option<Attribute>,
}
pub trait ConstantExt {
    fn constant() -> Constant;
}
pub enum External {}
impl ConstantExt for External {
    fn constant() -> Constant {
        Constant {
            path_semigroup: parse_quote! {::semigroup::Semigroup},
            path_idempotent: parse_quote! {::semigroup::Idempotent},
            path_selected: parse_quote! {::semigroup::Selected},
            path_annotated: parse_quote! {::semigroup::Annotated},
            path_annotate: parse_quote! {::semigroup::Annotate},
            path_monoid: parse_quote! {::semigroup::Monoid},
            path_commutative: parse_quote! {::semigroup::Commutative},
            path_construction_trait: parse_quote! {::semigroup::Construction},
            path_semigroup_op: parse_quote! {::semigroup::Op},
            path_idempotent_op: parse_quote! {::semigroup::IdempotentOp},
            path_monoid_op: parse_quote! {::semigroup::MonoidOp},
            attr_feature_monoid: None,
        }
    }
}
pub enum Internal {}
impl ConstantExt for Internal {
    fn constant() -> Constant {
        Constant {
            path_semigroup: parse_quote! {crate::Semigroup},
            path_idempotent: parse_quote! {crate::Idempotent},
            path_selected: parse_quote! {crate::Selected},
            path_annotated: parse_quote! {crate::Annotated},
            path_annotate: parse_quote! {crate::Annotate},
            path_monoid: parse_quote! {crate::Monoid},
            path_commutative: parse_quote! {crate::Commutative},
            path_construction_trait: parse_quote! {crate::Construction},
            path_semigroup_op: parse_quote! {crate::Op},
            path_idempotent_op: parse_quote! {crate::IdempotentOp},
            path_monoid_op: parse_quote! {crate::MonoidOp},
            attr_feature_monoid: Some(parse_quote! {#[cfg(feature = "monoid")]}),
        }
    }
}
