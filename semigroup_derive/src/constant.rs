use syn::{Path, parse_quote};

#[derive(Debug, Clone)]
pub struct Constant {
    pub path_semigroup: Path,
    pub path_selective: Path,
    pub path_selected: Path,
    pub path_annotated: Path,
    pub path_annotate: Path,
    pub path_annotate_fields: Path,
    pub path_monoid: Path,
    pub path_commutative: Path,
    pub path_construction_trait: Path,
    pub path_semigroup_op: Path,
    pub path_selective_op: Path,
    pub path_monoid_op: Path,
    pub path_commutative_op: Path,
}
impl Constant {
    pub fn new(krate: &Path) -> Self {
        Self {
            path_semigroup: parse_quote! { #krate::Semigroup },
            path_selective: parse_quote! { #krate::Selective },
            path_selected: parse_quote! { #krate::Selected },
            path_annotated: parse_quote! { #krate::Annotated },
            path_annotate: parse_quote! { #krate::Annotate },
            path_annotate_fields: parse_quote! { #krate::AnnotateFields },
            path_monoid: parse_quote! { #krate::Monoid },
            path_commutative: parse_quote! { #krate::Commutative },
            path_construction_trait: parse_quote! { #krate::Construction },
            path_semigroup_op: parse_quote! { #krate::SemigroupOp },
            path_selective_op: parse_quote! { #krate::SelectiveOp },
            path_monoid_op: parse_quote! { #krate::MonoidOp },
            path_commutative_op: parse_quote! { #krate::CommutativeOp },
        }
    }
}
