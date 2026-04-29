<!-- cargo-rdme start -->

[`Semigroup`](https://docs.rs/semigroup/latest/semigroup/semigroup/trait.Semigroup.html) is a trait for any **associative
binary operation** — a way to merge two values of the same type into one.
This crate provides a rich set of practical building blocks (and `derive`
macros) for composing such operations, so you can express common "combine"
workflows declaratively instead of writing ad-hoc merge code by hand.

The focus is on **everyday combining problems**, not on modeling abstract
algebra. Associativity matters here because it is the property that lets
you safely fold, parallelize, and stream-aggregate with results that don't
depend on grouping — not because we want algebraic structures for their own
sake. If you need to merge configs, aggregate statistics, union sets, or
reduce values across an iterator/stream, this crate is for you.

## Use cases
- **Layered configuration** — merge config from CLI, environment, and files
  with explicit precedence; see [`op::Coalesce`], [`op::Overwrite`].
  A worked `clap` + `serde` example is in [`Examples`](#examples).
- **Statistical aggregation** — combine histograms over partitions or a
  `Stream` of partial results to compute mean, p99 latency, throughput, etc.
  via [`op::HdrHistogram`] (feature `histogram`).
- **Numeric reductions** — [`op::Sum`], [`op::Prod`], [`op::Min`],
  [`op::Max`], [`op::Xor`], [`op::Gcd`], [`op::Lcm`].
- **Boolean reductions** — [`op::All`], [`op::Any`] for merging flags or
  pass/fail signals.
- **Set & map merging** — [`op::Union`], [`op::Intersection`],
  [`op::UnionMap`], [`op::IntersectionMap`].
- **Collection concatenation** — [`op::Concat`] for `Vec`, `String`, and any
  `Default + Extend + IntoIterator`.
- **First / last wins** — [`op::First`], [`op::Last`] for order-based selection.
- **Range queries** with O(log n) updates and queries via
  [`segment_tree::SegmentTree`] over any [`Monoid`].
- **Concurrent / streaming aggregation** via [`CombineStream`] — fold a
  `Stream` of partial results into one when the operation is [`Commutative`].
- **Per-field provenance** — track *which input each merged field came from*
  with [`Annotate`] (e.g. "`port` came from CLI, `host` came from env").
- **Deferred / multi-shot evaluation** — buffer merge inputs and evaluate
  later, or evaluate multiple ways from the same buffer, with [`Lazy`].

## Usage
```sh
cargo add semigroup --features derive,monoid
```

## Examples
A CLI example of `clap` and `serde` integration, see <https://github.com/hayas1/semigroup/blob/master/semigroup/examples/clap_serde.rs>

### Simple coalesce
```rust
use semigroup::Semigroup;
#[derive(Debug, Clone, PartialEq, Semigroup)]
#[semigroup(with = "semigroup::op::Coalesce")]
pub struct Config<'a> {
    pub num: Option<u32>,
    pub str: Option<&'a str>,
    #[semigroup(with = "semigroup::op::Any")]
    pub boolean: bool,
}

let cli = Config { num: Some(1), str: None, boolean: false };
let file = Config { num: None, str: Some("ten"), boolean: false };
let env = Config { num: Some(100), str: None, boolean: true };

let config = cli.semigroup(file).semigroup(env);

assert_eq!(config, Config { num: Some(1), str: Some("ten"), boolean: true });
```

### Coalesce with rich enum annotation and lazy evaluation
More detail is in [`Annotate`] and [`Lazy`].
```rust
use semigroup::{Annotate, AnnotateFields, Lazy, Semigroup};
#[derive(Debug, Clone, PartialEq, Semigroup)]
#[semigroup(annotated, with = "semigroup::op::Coalesce")]
pub struct Config<'a> {
    pub num: Option<u32>,
    pub str: Option<&'a str>,
    #[semigroup(with = "semigroup::op::Any")]
    pub boolean: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    File,
    Env,
    Cli,
}

let cli = Config { num: Some(1), str: None, boolean: false }.annotated(Source::Cli);
let file = Config { num: None, str: Some("ten"), boolean: false }.annotated(Source::File);
let env = Config { num: Some(100), str: None, boolean: true }.annotated(Source::Env);

let lazy = Lazy::from(cli).semigroup(file.into()).semigroup(env.into());
assert_eq!(lazy.first().num.value(), &Some(1u32));
assert_eq!(lazy.last().boolean.value(), &true);

let config = lazy.combine();
assert_eq!(config.num.value(), &Some(1u32));
assert_eq!(config.num.annotation(), &Source::Cli);
assert_eq!(config.str.value(), &Some("ten"));
assert_eq!(config.str.annotation(), &Source::File);
assert_eq!(config.boolean.value(), &true);
assert_eq!(config.boolean.annotation(), &Source::Env);
```

## Highlights
- **Two derives** that cover the common cases.
  - `#[derive(Semigroup)]` implements [`Semigroup`] for a struct *field by field*.
    Each field is merged with its own [`Semigroup`] impl, or with the operation
    given by `#[semigroup(with = "...")]`. No need to hand-write merge logic.
  - `#[derive(Op)]` defines a brand-new combining operation as a thin newtype
    wrapper. Many ready-made ones live in [`crate::op`], but you can plug in
    your own with the same ergonomics.
- **Annotations** for tracking provenance via [`Annotate`]. Particularly useful
  with selection-style ops like [`op::Coalesce`], where you want to know *which
  input* the surviving value came from after a merge.
- **Combine helpers** for collections of values:
  - [`CombineIterator`] — `fold` / `reduce` / `combine` over iterators.
  - [`Lazy`] — defer the merge and evaluate it later, or evaluate it multiple
    ways from the same buffered inputs.
  - [`segment_tree::SegmentTree`] — O(log n) range queries on a [`Monoid`].
  - [`CombineStream`] — combine `Stream`s asynchronously (commutative ops only).

| | [`Semigroup`] | [`Annotate`] | [`Monoid`] | [`Commutative`] |
| :---: | :---: | :---: | :---: | :---: |
| **property** | *associativity* | *annotation* | *identity element* | *commutativity* |
| **`#[derive(Semigroup)]`** <br> **`#[semigroup(...)]`** | | `annotated` | `monoid` | `commutative` |
| **`#[derive(Op)]`** <br> **`#[op(...)]`** | | `idempotent` | `monoid` | `commutative` |
| **testing** | [`assert_semigroup!`] |  | [`assert_monoid!`] | [`assert_commutative!`] |
| **typical combiner** | [`CombineIterator`] | [`Lazy`] | [`SegmentTree`](`segment_tree::SegmentTree`) | [`CombineStream`] |

## Links
- GitHub: <https://github.com/hayas1/semigroup>
- GitHub Pages: <https://hayas1.github.io/semigroup/semigroup>
- Release Notes: <https://github.com/hayas1/semigroup/releases>
- Crates.io: <https://crates.io/crates/semigroup>
- Docs.rs: <https://docs.rs/semigroup>

## Testing
### Benchmarks
// TODO

### Coverage
<https://hayas1.github.io/semigroup/semigroup/tarpaulin-report.html>

<!-- cargo-rdme end -->
