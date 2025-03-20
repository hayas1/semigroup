use coalesced::Coalesce;
use coalesced_derive::Coalesce;

#[derive(Coalesce)]
struct Config {
    value: Option<u32>,
}

#[test]
fn test_derive_compile() {
    let config = Config { value: None };
    let config2 = Config { value: Some(1) };
    assert_eq!(config.value, None);
    assert_eq!(config2.value, Some(1));

    assert_eq!(config.coalesce(config2).value, Some(1));
}
