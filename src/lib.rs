//! coalesced supports reading configs from multiple sources
//!
//! # Usage
//! ```toml
//! [dependencies]
//! coalesced = { git = "https://github.com/hayas1/coalesced" }
//! ```
//!
//! # Examples
//!
//! ## Annotation
//!
//! ## Lazy Evaluation
//!
//! # Documents
//! <https://hayas1.github.io/coalesced/coalesced>
//!
//! # Testing
//! ## Benchmarks
//! // TODO
//!
//! ## Coverage
//! <https://hayas1.github.io/coalesced/coalesced/tarpaulin-report.html>
//!

pub use coalesced_base::{
    annotate::{Annotate, Annotated},
    lazy::LazySemigroup,
    op,
    reverse::Reversed,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[cfg(feature = "derive")]
pub use coalesced_derive::{Construction, Semigroup};

#[cfg(feature = "test")]
pub use coalesced_base::semigroup::tests::assert_semigroup_op;
