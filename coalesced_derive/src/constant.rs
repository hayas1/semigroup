use std::cell::LazyCell;

use syn::{parse_quote, Ident, Path};

pub const DERIVE_CONSTRUCTION: &str = "Construction";

thread_local! {
    pub static PATH_SEMIGROUP: LazyCell<Path> = LazyCell::new(|| parse_quote!(::coalesced::Semigroup));
    pub static IDENT_SEMIGROUP_OP: LazyCell<Ident> = LazyCell::new(|| parse_quote!(semigroup_op));
    pub static PATH_ANNOTATED: LazyCell<Path> = LazyCell::new(|| parse_quote!(::coalesced::Annotated));
    pub static PATH_REVERSED: LazyCell<Path> =
        LazyCell::new(|| parse_quote!(::coalesced::op::reverse::Reversed));
}
