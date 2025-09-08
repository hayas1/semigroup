pub use coalesced_base::{
    annotate::{Annotate, Annotated},
    lazy::LazySemigroup,
    op,
    reverse::Reversed,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[cfg(feature = "derive")]
pub use coalesced_derive::{Construction, Semigroup};
