use coalesced::{Coalesce, History, IntoHistory};

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

#[test]
fn test_extension_prior() {
    use coalesced::Extension;

    let from_file = Config {
        num: Some(10),
        str: Some("ten"),
    };
    let from_env = Config {
        num: Some(100),
        str: None,
    };
    let from_cli = Config {
        num: None,
        str: None,
    };

    let (file, env, cli) = (
        from_file.with_extension(&"file"),
        from_env.with_extension(&"env"),
        from_cli.with_extension(&"cli"),
    );

    let config = file.prior(env).prior(cli);
    assert_eq!(config.num.extension, &"env");
    assert_eq!(config.str.extension, &"file");
    assert!(matches!(
        config.into(),
        Config {
            num: Some(100),
            str: Some("ten"),
        }
    ));
}
