use coalesced::{Coalesce, Extension, WithExt};

#[derive(Coalesce)]
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

#[test]
fn test_derive_extension_enum_unit() {
    let config = Config::Unit.with_extension("first");
    let config2 = Config::Unit.with_extension("second");

    let c = config.posterior(config2);
    assert!(matches!(
        c,
        // TODO <Config as Extension<_>>::WithExt::Unit, but experimental
        ConfigWithExt::Unit(WithExt {
            value: (),
            extension: "first"
        })
    ));
    assert!(matches!(c.into(), Config::Unit));
}

#[test]
fn test_derive_extension_enum_named() {
    let config = Config::Named { value: 10 }.with_extension("first");
    let config2 = Config::Named { value: 100 }.with_extension("second");

    let c = config.posterior(config2);
    assert!(matches!(
        c,
        // TODO <Config as Extension<_>>::WithExt::Unit, but experimental
        ConfigWithExt::Named {
            value: WithExt {
                value: 10,
                extension: "first"
            }
        }
    ));
    assert!(matches!(c.into(), Config::Named { value: 10 }));
}

#[test]
fn test_derive_extension_enum_unnamed() {
    let config = Config::Unnamed("nop").with_extension("first");
    let config2 = Config::Unnamed("op").with_extension("second");

    let c = config.posterior(config2);
    assert!(matches!(
        c,
        // TODO <Config as Extension<_>>::WithExt::Unit, but experimental
        ConfigWithExt::Unnamed(WithExt {
            value: "nop",
            extension: "first"
        })
    ));
    assert!(matches!(c.into(), Config::Unnamed("nop")));
}
