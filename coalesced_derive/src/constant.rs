use std::cell::LazyCell;

use syn::{parse_quote, Ident, Path};

pub const CONSTRUCTION: &str = "construction";
pub const ANNOTATED: &str = "annotated";
pub const SEMIGROUP: &str = "semigroup";

pub const PATH_SEMIGROUP: LazyCell<Path> = LazyCell::new(|| parse_quote!(::coalesced::Semigroup));
pub const IDENT_SEMIGROUP_OP: LazyCell<Ident> = LazyCell::new(|| parse_quote!(semigroup_op));
pub const PATH_ANNOTATED: LazyCell<Path> = LazyCell::new(|| parse_quote!(::coalesced::Annotated));
pub const PATH_REVERSED: LazyCell<Path> =
    LazyCell::new(|| parse_quote!(::coalesced::op::reverse::Reversed));
