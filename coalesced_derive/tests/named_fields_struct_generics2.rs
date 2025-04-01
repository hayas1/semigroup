use coalesced::Coalesce;

use coalesced::Extension;
use coalesced_derive::Extension;

#[derive(Extension)]
struct Config<T> {
    name: &'static str,
    value: Option<T>,
}

#[test]
fn test_derive_coalesce_named_fields_struct_generics() {
    let config = Config {
        name: "c1",
        value: Some(1),
    };
    let config2 = Config {
        name: "c2",
        value: None,
    };

    let c = config.prior(config2);
    assert_eq!(c.name, "c2");
    assert_eq!(c.value, Some(1));
}

#[test]
fn test_derive_extension_named_fields_struct_generics() {
    let config = Config {
        name: "c1",
        value: None,
    }
    .with_extension("first");
    let config2 = Config {
        name: "c2",
        value: Some(2),
    }
    .with_extension("second");

    let c = config.posterior(config2);
    assert_eq!(c.name.extension, "first");
    assert_eq!(*c.name, "c1");
    assert_eq!(c.value.extension, "second");
    assert_eq!(*c.value, Some(2));

    assert!(matches!(
        c.into(),
        Config {
            name: "c1",
            value: Some(2),
        }
    ));
}
