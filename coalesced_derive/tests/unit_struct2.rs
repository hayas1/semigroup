use coalesced::Coalesce;

use coalesced::Extension;
use coalesced_derive::Extension;

#[derive(Extension)]
struct Config;

#[test]
fn test_derive_coalesce_unit_struct() {
    let config = Config;
    let config2 = Config;

    let c = config.prior(config2);
    assert!(matches!(c, Config));
}

#[test]
fn test_derive_extension_unit_struct() {
    let config = Config.with_extension("first");
    let config2 = Config.with_extension("second");

    let c = config.posterior(config2);
    assert_eq!(c.extension, "first");

    assert!(matches!(c.into(), Config));
}
