use coalesced::Coalesce;

use coalesced::Extension;
use coalesced_derive::Extension;

#[derive(Extension)]
enum Config {
    Unit,
    Named { value: i32 },
    Unnamed(&'static str),
}

#[test]
fn test_derive_coalesce_enum_unit() {
    let config = Config::Unit;
    let config2 = Config::Unit;

    let c = config.prior(config2);
    assert!(matches!(c, Config::Unit));
}

#[test]
fn test_derive_coalesce_enum_named() {
    let config = Config::Named { value: 10 };
    let config2 = Config::Named { value: 100 };

    let c = config.prior(config2);
    assert!(matches!(c, Config::Named { value: 100 }));
}

#[test]
fn test_derive_coalesce_enum_unnamed() {
    let config = Config::Unnamed("nop");
    let config2 = Config::Unnamed("op");

    let c = config.prior(config2);
    assert!(matches!(c, Config::Unnamed("op")));
}
