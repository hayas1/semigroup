use coalesced::Coalesce;

#[derive(Coalesce)]
struct Config;

#[test]
fn test_derive_unit_struct() {
    let config = Config;
    let config2 = Config;

    let c = config.prior(config2);
    assert!(matches!(c, Config));
}
