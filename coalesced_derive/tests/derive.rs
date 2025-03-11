use coalesced_derive::Coalesce;

#[derive(Coalesce)]
struct Config {
    value: Option<u32>,
}

#[test]
fn test_derive_compile() {
    let config = Config { value: None };
    assert_eq!(config.value, None);
}
