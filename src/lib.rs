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
//! ```
//! use coalesced::{Annotate, Semigroup};
//! #[derive(Debug, Clone, PartialEq, Semigroup)]
//! #[semigroup(annotated, with = "coalesced::op::annotation::coalesce::Coalesce")]
//! pub struct Config<'a> {
//!     pub num: Option<u32>,
//!     pub str: Option<&'a str>,
//!     #[semigroup(with = "coalesced::op::annotation::replace::Replace")]
//!     pub boolean: bool,
//! }
//!
//! let file = Config { num: Some(1), str: None, boolean: true }.annotated("File");
//! let env = Config { num: None, str: Some("ten"), boolean: false }.annotated("Env");
//! let cli = Config { num: Some(100), str: None, boolean: true }.annotated("Cli");
//!
//! let config = file.semigroup(env).semigroup(cli);
//!
//! assert_eq!(config.value(), &Config { num: Some(1), str: Some("ten"), boolean: true });
//! assert_eq!(config.annotation().num, "File");
//! assert_eq!(config.annotation().str, "Env");
//! assert_eq!(config.annotation().boolean, "Cli");
//! ```
//!
//! ## Lazy Evaluation
//! ```
//! use coalesced::{Annotate, Semigroup, LazySemigroup};
//! #[derive(Debug, Clone, PartialEq, Semigroup)]
//! #[semigroup(annotated, with = "coalesced::op::annotation::coalesce::Coalesce")]
//! pub struct Config<'a> {
//!     pub num: Option<u32>,
//!     pub str: Option<&'a str>,
//!     #[semigroup(with = "coalesced::op::annotation::replace::Replace")]
//!     pub boolean: bool,
//! }
//!
//! let lazy = LazySemigroup::with(Config { num: Some(1), str: None, boolean: true }.annotated("File"))
//!     .semigroup(LazySemigroup::with(Config { num: None, str: Some("ten"), boolean: false }.annotated("Env")))
//!     .semigroup(LazySemigroup::with(Config { num: Some(100), str: None, boolean: true }.annotated("Cli")));
//!
//! assert_eq!(lazy.first(), &Config { num: Some(1), str: None, boolean: true }.annotated("File"));
//! assert_eq!(lazy.last(), &Config { num: Some(100), str: None, boolean: true }.annotated("Cli"));
//!
//! let config = lazy.fold();
//!
//! assert_eq!(config.value(), &Config { num: Some(1), str: Some("ten"), boolean: true });
//! assert_eq!(config.annotation().num, "File");
//! assert_eq!(config.annotation().str, "Env");
//! assert_eq!(config.annotation().boolean, "Cli");
//! ```
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
    reverse::Reverse,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[cfg(feature = "derive")]
pub use coalesced_derive::{Construction, Semigroup};

#[cfg(feature = "test")]
pub use coalesced_base::semigroup::tests::assert_semigroup_op;
