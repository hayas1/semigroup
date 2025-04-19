<!-- cargo-rdme start -->

coalesced supports reading configs from multiple sources

### Usage
```toml
[dependencies]
coalesced = { git = "https://github.com/hayas1/coalesced" }
```
Documentation: [https://hayas1.github.io/coalesced/coalesced/](https://hayas1.github.io/coalesced/coalesced/)

### Examples
[`Coalesce::prior`] will return the last confirmed value. [`Coalesce::posterior`] will return the first confirmed value.
```rust
use coalesced::Coalesce;

#[derive(Coalesce)]
pub struct Config<'a> {
    num: Option<i32>,
    str: Option<&'a str>,
}

let from_file = Config {
    num: Some(10),
    str: None,
};
let from_env = Config {
    num: Some(100),
    str: Some("hundred"),
};
let from_cli = Config {
    num: None,
    str: Some("thousand"),
};
let config = from_file.prior(from_env).prior(from_cli);
assert!(matches!(config, Config {
    num: Some(100),
    str: Some("thousand"),
}));

let from_file = Config {
    num: Some(10),
    str: None,
};
let from_env = Config {
    num: Some(100),
    str: Some("hundred"),
};
let from_cli = Config {
    num: None,
    str: Some("thousand"),
};
let config = from_file.posterior(from_env).posterior(from_cli);
assert!(matches!(config, Config {
    num: Some(10),
    str: Some("hundred"),
}));
```
| `Config` | file | env | cli | | prior | posterior |
| --- | ---- | ------- | -------- | --- | -------- | --------- |
| `num` | 10 | 100 | | | 100 | 10 |
| `str` | | hundred | thousand | | thousand | hundred |

#### Lazy Evaluation
Related to [`crate::Coalesced`].
```rust
use coalesced::{Coalesce, History, IntoHistory};

#[derive(Coalesce)]
pub struct Config<'a> {
    num: Option<i32>,
    str: Option<&'a str>,
}

let from_file = Config {
    num: Some(10),
    str: None,
};
let from_env = Config {
    num: Some(100),
    str: Some("hundred"),
};
let from_cli = Config {
    num: None,
    str: Some("thousand"),
};

let config = from_file.into_history().prior(from_env).prior(from_cli);
assert!(matches!(
    config.base(),
    Config {
        num: Some(10),
        str: None,
    }
));
assert!(matches!(config.into(), Config {
    num: Some(100),
    str: Some("thousand"),
}));
```

#### Extensions
Related to [`crate::WithExt`].
```rust
use coalesced::{Coalesce, Extension};

#[derive(Coalesce)]
pub struct Config<'a> {
    num: Option<i32>,
    str: Option<&'a str>,
}

let from_file = Config {
    num: Some(10),
    str: None,
};
let from_env = Config {
    num: Some(100),
    str: Some("hundred"),
};
let from_cli = Config {
    num: None,
    str: Some("thousand"),
};

let (file, env, cli) = (
    from_file.with_extension(&"file"),
    from_env.with_extension(&"env"),
    from_cli.with_extension(&"cli"),
);

let config = file.prior(env).prior(cli);
assert_eq!(config.num.extension, &"env");
assert_eq!(config.str.extension, &"cli");
assert!(matches!(config.into(), Config {
    num: Some(100),
    str: Some("thousand"),
}));
```

<!-- cargo-rdme end -->
