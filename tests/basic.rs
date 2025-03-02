use coalesced::Coalesced;

pub struct Config<'a> {
    pub name: &'a str,
}

#[test]
fn test_coalesced_basic_config() {
    let from_file = Coalesced::new_prior(Some(Config { name: "file" }));
    let from_env = Coalesced::new_prior(Some(Config { name: "env" }));
    let from_cli = Coalesced::new_prior(Some(Config { name: "cli" }));

    let config = from_file.register(from_env).register(from_cli);
    assert_eq!(config.as_ref().unwrap().name, "cli");
}

struct GlobalConfig<'a> {
    _name: &'a str,
    number: Coalesced<Option<i64>>,
    locals: Vec<LocalConfig<'a>>,
}
struct LocalConfig<'a> {
    _name: &'a str,
    number: Coalesced<Option<i64>>,
}
#[test]
fn test_coalesced_complex_config() {
    let config = GlobalConfig {
        _name: "global",
        number: Coalesced::new_prior(Some(1)),
        locals: vec![
            LocalConfig {
                _name: "local1",
                number: Coalesced::new_prior(Some(10)),
            },
            LocalConfig {
                _name: "local2",
                number: Coalesced::new_prior(Some(100)),
            },
            LocalConfig {
                _name: "local3",
                number: Coalesced::new_prior(None),
            },
        ],
    };
    let expected = [Some(10), Some(100), Some(1)];
    for (cfg, exp) in config.locals.into_iter().zip(expected) {
        let number = config.number.clone().register(cfg.number);
        assert_eq!(*number, exp);
    }
}
