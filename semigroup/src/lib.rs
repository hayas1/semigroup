#![cfg_attr(doc_cfg, feature(doc_cfg))]
//! [`Semigroup`](`crate::semigroup::Semigroup`) is a trait for any **associative
//! binary operation** — a way to merge two values of the same type into one.
//! This crate provides a rich set of practical building blocks (and `derive`
//! macros) for composing such operations, so you can express common "combine"
//! workflows declaratively instead of writing ad-hoc merge code by hand.
//!
//! The focus is on **everyday combining problems**, not on modeling abstract
//! algebra. Associativity matters here because it is the property that lets
//! you safely fold, parallelize, and stream-aggregate with results that don't
//! depend on grouping — not because we want algebraic structures for their own
//! sake. If you need to merge configs, aggregate statistics, union sets, or
//! reduce values across an iterator/stream, this crate is for you.
//!
//! # Usage
//! ```sh
//! cargo add semigroup --features derive,monoid
//! ```
//!
//! # Examples
//! A CLI example of `clap` and `serde` integration, see <https://github.com/hayas1/semigroup/blob/master/semigroup/examples/clap_serde.rs>
//!
//! ## Simple coalesce
//! ```
//! use semigroup::Semigroup;
//! #[derive(Debug, Clone, PartialEq, Semigroup)]
//! #[semigroup(with = "semigroup::op::Coalesce")]
//! pub struct Config<'a> {
//!     pub num: Option<u32>,
//!     pub str: Option<&'a str>,
//!     #[semigroup(with = "semigroup::op::Any")]
//!     pub boolean: bool,
//! }
//!
//! let cli = Config { num: Some(1), str: None, boolean: false };
//! let file = Config { num: None, str: Some("ten"), boolean: false };
//! let env = Config { num: Some(100), str: None, boolean: true };
//!
//! let config = cli.semigroup(file).semigroup(env);
//!
//! assert_eq!(config, Config { num: Some(1), str: Some("ten"), boolean: true });
//! ```
//!
//! ## Coalesce with rich enum annotation and lazy evaluation
//! More detail is in [`Annotate`] and [`Lazy`].
//! ```
//! use semigroup::{Annotate, AnnotateFields, Lazy, Semigroup};
//! #[derive(Debug, Clone, PartialEq, Semigroup)]
//! #[semigroup(annotated, with = "semigroup::op::Coalesce")]
//! pub struct Config<'a> {
//!     pub num: Option<u32>,
//!     pub str: Option<&'a str>,
//!     #[semigroup(with = "semigroup::op::Any")]
//!     pub boolean: bool,
//! }
//! #[derive(Debug, Clone, PartialEq)]
//! pub enum Source {
//!     File,
//!     Env,
//!     Cli,
//! }
//!
//! let cli = Config { num: Some(1), str: None, boolean: false }.annotated(Source::Cli);
//! let file = Config { num: None, str: Some("ten"), boolean: false }.annotated(Source::File);
//! let env = Config { num: Some(100), str: None, boolean: true }.annotated(Source::Env);
//!
//! let lazy = Lazy::from(cli).semigroup(file.into()).semigroup(env.into());
//! assert_eq!(lazy.first().num.value(), &Some(1u32));
//! assert_eq!(lazy.last().boolean.value(), &true);
//!
//! let config = lazy.combine();
//! assert_eq!(config.num.value(), &Some(1u32));
//! assert_eq!(config.num.annotation(), &Source::Cli);
//! assert_eq!(config.str.value(), &Some("ten"));
//! assert_eq!(config.str.annotation(), &Source::File);
//! assert_eq!(config.boolean.value(), &true);
//! assert_eq!(config.boolean.annotation(), &Source::Env);
//! ```
//!
//! # Use cases
//! The crate ships practical operations under [`crate::op`] that you can use directly
//! or compose into your own structs via `#[derive(Semigroup)]`. The two most common ones:
//! - [`op::Coalesce`] — **layered configuration**: merge CLI / environment / file with
//!   explicit precedence; see [Examples](#examples) for a worked use.
//! - [`op::HdrHistogram`] — **statistical aggregation**: combine histograms over partitions
//!   or a `Stream` to compute mean, p99 latency, throughput, etc. (feature `histogram`).
//!
//! See [`crate::op`] for the full catalog (numeric, boolean, set / map merging, concat,
//! first / last, ...).
//!
//! # Concepts at a glance
//! | | [`Semigroup`] | [`Annotate`] | [`Monoid`] | [`Commutative`] |
//! | :---: | :---: | :---: | :---: | :---: |
//! | **property** | *associativity* | *annotation* | *identity element* | *commutativity* |
//! | **`#[derive(Semigroup)]`** <br> **`#[semigroup(...)]`** | | `annotated` | `monoid` | `commutative` |
//! | **`#[derive(SemigroupOp)]`** <br> **`#[semigroup_op(...)]`** | | `idempotent` | `monoid` | `commutative` |
//! | **testing** | [`assert_semigroup!`] |  | [`assert_monoid!`] | [`assert_commutative!`] |
//! | **typical combiner** | [`CombineIterator`] | [`Lazy`] | [`SegmentTree`](`segment_tree::SegmentTree`) | [`CombineStream`] |
//!
//! # Links
//! - GitHub: <https://github.com/hayas1/semigroup>
//! - GitHub Pages: <https://hayas1.github.io/semigroup/semigroup>
//! - Release Notes: <https://github.com/hayas1/semigroup/releases>
//! - Crates.io: <https://crates.io/crates/semigroup>
//! - Docs.rs: <https://docs.rs/semigroup>
//!
//! # Testing
//! ## Benchmarks
//! <https://hayas1.github.io/semigroup/semigroup/criterion/report/index.html>
//!
//! ## Coverage
//! <https://hayas1.github.io/semigroup/semigroup/tarpaulin-report.html>
//!

mod annotate;
mod combine;
mod commutative;
#[cfg(feature = "async")]
mod concurrent;
mod construction;
mod lazy;
#[cfg(feature = "monoid")]
mod monoid;
pub mod op;
#[cfg(feature = "monoid")]
pub mod segment_tree;
mod semigroup;

#[cfg(feature = "async")]
pub use self::concurrent::*;
#[cfg(feature = "monoid")]
pub use self::monoid::*;
pub use self::{annotate::*, combine::*, commutative::*, construction::*, lazy::*, semigroup::*};

#[cfg(feature = "derive")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "derive")))]
pub use semigroup_derive::{SemigroupOp, Semigroup, properties};
