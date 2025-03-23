use coalesced::Coalesce;
use coalesced_derive::Coalesce;

#[derive(Coalesce)]
struct Config {
    name: &'static str,
    value: Option<u32>,
}

#[test]
fn test_derive_compile() {
    let config = Config {
        name: "c1",
        value: Some(1),
    };
    let config2 = Config {
        name: "c2",
        value: None,
    };
    assert!(matches!(
        config,
        Config {
            name: "c1",
            value: Some(1)
        }
    ));
    assert!(matches!(
        config2,
        Config {
            name: "c2",
            value: None
        }
    ));

    let c = config.prior(config2);
    assert_eq!(c.name, "c2");
    assert_eq!(c.value, Some(1));
}
