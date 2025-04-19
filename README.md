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
| `Config` | file | env | cli | → | prior | posterior |
| --- | ---- | ------- | -------- | --- | -------- | --------- |
| `opt_num` | 10 | 100 | | →| 100 | 10 |
| `opt_str` | | hundred | thousand | →| thousand | hundred |

```rust
use coalesced::Coalesce;

#[derive(Coalesce)]
pub struct Config<'a> {
    opt_num: Option<i32>,
    opt_str: Option<&'a str>,
}

let from_file = Config {
    opt_num: Some(10),
    opt_str: None,
};
let from_env = Config {
    opt_num: Some(100),
    opt_str: Some("hundred"),
};
let from_cli = Config {
    opt_num: None,
    opt_str: Some("thousand"),
};
let config = from_file.prior(from_env).prior(from_cli);
assert!(matches!(config, Config {
    opt_num: Some(100),
    opt_str: Some("thousand"),
}));

let from_file = Config {
    opt_num: Some(10),
    opt_str: None,
};
let from_env = Config {
    opt_num: Some(100),
    opt_str: Some("hundred"),
};
let from_cli = Config {
    opt_num: None,
    opt_str: Some("thousand"),
};
let config = from_file.posterior(from_env).posterior(from_cli);
assert!(matches!(config, Config {
    opt_num: Some(10),
    opt_str: Some("hundred"),
}));
```

#### Lazy Evaluation
Related to [`crate::Coalesced`]. Lazy evaluation is supported so we can follow the changes until the value is confirmed.
```rust
use coalesced::{Coalesce, History, IntoHistory};

#[derive(Coalesce)]
pub struct Config<'a> {
    opt_num: Option<i32>,
    opt_str: Option<&'a str>,
}

let from_file = Config {
    opt_num: Some(10),
    opt_str: None,
};
let from_env = Config {
    opt_num: Some(100),
    opt_str: Some("hundred"),
};
let from_cli = Config {
    opt_num: None,
    opt_str: Some("thousand"),
};

let config = from_file.into_history().prior(from_env).prior(from_cli);
assert!(matches!(
    config.base(),
    Config {
        opt_num: Some(10),
        opt_str: None,
    }
));
assert!(matches!(config.into(), Config {
    opt_num: Some(100),
    opt_str: Some("thousand"),
}));
```

#### Extensions metadata
Related to [`crate::WithExt`]. Extensions metadata is supported so we can follow the source of the confirmed value.
```rust
use coalesced::{Coalesce, Extension};

#[derive(Coalesce)]
pub struct Config<'a> {
    opt_num: Option<i32>,
    opt_str: Option<&'a str>,
}

let from_file = Config {
    opt_num: Some(10),
    opt_str: None,
};
let from_env = Config {
    opt_num: Some(100),
    opt_str: Some("hundred"),
};
let from_cli = Config {
    opt_num: None,
    opt_str: Some("thousand"),
};

let (file, env, cli) = (
    from_file.with_extension(&"file"),
    from_env.with_extension(&"env"),
    from_cli.with_extension(&"cli"),
);

let config = file.prior(env).prior(cli);
assert_eq!(config.opt_num.extension, &"env");
assert_eq!(config.opt_str.extension, &"cli");
assert!(matches!(config.into(), Config {
    opt_num: Some(100),
    opt_str: Some("thousand"),
}));
```

<!-- cargo-rdme end -->
