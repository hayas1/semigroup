<!-- cargo-rdme start -->

coalesced supports reading configs from multiple sources

## Usage
```toml
[dependencies]
coalesced = { git = "https://github.com/hayas1/coalesced" }
```

## Examples

### Annotation
#### simple string annotation
```rust
use coalesced::{Annotate, Semigroup};
#[derive(Debug, Clone, PartialEq, Semigroup)]
#[semigroup(annotated, with = "coalesced::op::annotation::coalesce::Coalesce")]
pub struct Config<'a> {
    pub num: Option<u32>,
    pub str: Option<&'a str>,
    #[semigroup(with = "coalesced::op::annotation::replace::Replace")]
    pub boolean: bool,
}

let file = Config { num: Some(1), str: None, boolean: true }.annotated("File");
let env = Config { num: None, str: Some("ten"), boolean: false }.annotated("Env");
let cli = Config { num: Some(100), str: None, boolean: true }.annotated("Cli");

let config = file.semigroup(env).semigroup(cli);

assert_eq!(config.value(), &Config { num: Some(1), str: Some("ten"), boolean: true });
assert_eq!(config.annotation().num, "File");
assert_eq!(config.annotation().str, "Env");
assert_eq!(config.annotation().boolean, "Cli");
```

#### rich enum annotation
```rust
use coalesced::{Annotate, Semigroup};
#[derive(Debug, Clone, PartialEq, Semigroup)]
#[semigroup(annotated, with = "coalesced::op::annotation::coalesce::Coalesce")]
pub struct Config<'a> {
    pub num: Option<u32>,
    pub str: Option<&'a str>,
    #[semigroup(with = "coalesced::op::annotation::replace::Replace")]
    pub boolean: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    File,
    Env,
    Cli,
}

let file = Config { num: Some(1), str: None, boolean: true }.annotated(Source::File);
let env = Config { num: None, str: Some("ten"), boolean: false }.annotated(Source::Env);
let cli = Config { num: Some(100), str: None, boolean: true }.annotated(Source::Cli);

let config = file.semigroup(env).semigroup(cli);

assert_eq!(config.value(), &Config { num: Some(1), str: Some("ten"), boolean: true });
assert_eq!(config.annotation().num, Source::File);
assert_eq!(config.annotation().str, Source::Env);
assert_eq!(config.annotation().boolean, Source::Cli);
```

### Lazy Evaluation
```rust
use coalesced::{Annotate, Semigroup, Lazy};
#[derive(Debug, Clone, PartialEq, Semigroup)]
#[semigroup(annotated, with = "coalesced::op::annotation::coalesce::Coalesce")]
pub struct Config<'a> {
    pub num: Option<u32>,
    pub str: Option<&'a str>,
    #[semigroup(with = "coalesced::op::annotation::replace::Replace")]
    pub boolean: bool,
}

let lazy = Lazy::with(Config { num: Some(1), str: None, boolean: true }.annotated("File"))
    .semigroup(Lazy::with(Config { num: None, str: Some("ten"), boolean: false }.annotated("Env")))
    .semigroup(Lazy::with(Config { num: Some(100), str: None, boolean: true }.annotated("Cli")));

assert_eq!(lazy.first(), &Config { num: Some(1), str: None, boolean: true }.annotated("File"));
assert_eq!(lazy.last(), &Config { num: Some(100), str: None, boolean: true }.annotated("Cli"));

let config = lazy.fold();

assert_eq!(config.value(), &Config { num: Some(1), str: Some("ten"), boolean: true });
assert_eq!(config.annotation().num, "File");
assert_eq!(config.annotation().str, "Env");
assert_eq!(config.annotation().boolean, "Cli");
```

## Documents
<https://hayas1.github.io/coalesced/coalesced>

## Testing
### Benchmarks
// TODO

### Coverage
<https://hayas1.github.io/coalesced/coalesced/tarpaulin-report.html>

<!-- cargo-rdme end -->
