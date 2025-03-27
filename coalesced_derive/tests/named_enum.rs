use coalesced::Coalesce;

#[derive(Coalesce)]
enum Config {
    Unit,
    Named { value: i32 },
    // Unnamed(String),
}

#[test]
fn test_derive_named_fields_struct() {
    let config = Config::Unit;
    let config2 = Config::Unit;

    let c = config.prior(config2);
    assert!(matches!(c, Config::Unit));
}
