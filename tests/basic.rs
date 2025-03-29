use coalesced::{Coalesce, Extension, History, IntoHistory};

#[derive(Coalesce)]
pub struct Config<'a> {
    num: Option<i32>,
    str: Option<&'a str>,
}

#[test]
fn test_basic_prior() {
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
    assert!(matches!(
        config,
        Config {
            num: Some(100),
            str: Some("thousand")
        }
    ));
}
#[test]
fn test_history_posterior() {
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

    let config = from_file
        .into_history()
        .posterior(from_env)
        .posterior(from_cli);
    assert!(matches!(
        config.base(),
        Config {
            num: Some(10),
            str: None,
        }
    ));
    assert!(matches!(
        config.history()[0].1,
        Config {
            num: Some(100),
            str: Some("hundred"),
        }
    ));
    assert!(matches!(
        config.into(),
        Config {
            num: Some(10),
            str: Some("hundred"),
        }
    ));
}

// TODO ConfigBlock -> Config (with Extension)
#[derive(Coalesce)]
pub struct ConfigBlock<'a> {
    num: i32,
    str: &'a str,
}
#[test]
fn test_extension_prior() {
    let from_file = Some(ConfigBlock {
        num: 10,
        str: "ten",
    });
    let from_env = Some(ConfigBlock {
        num: 100,
        str: "hundred",
    });
    let from_cli = None;

    let (file, env, cli) = (
        from_file.with_extension(&"file"),
        from_env.with_extension(&"env"),
        from_cli.with_extension(&"cli"),
    );

    let config = file.prior(env).prior(cli);
    assert_eq!(config.extension, &"env");
    assert!(matches!(
        config.as_ref().unwrap(),
        ConfigBlock {
            num: 100,
            str: "hundred",
        }
    ));
}
