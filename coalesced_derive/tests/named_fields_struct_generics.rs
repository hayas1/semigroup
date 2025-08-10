use coalesced::{Coalesce, Extension};

#[derive(Coalesce)]
struct Config<T> {
    name: Option<&'static str>,
    value: Option<T>,
}

#[test]
fn test_derive_coalesce_named_fields_struct_generics() {
    let config = Config {
        name: Some("c1"),
        value: Some(1),
    };
    let config2 = Config {
        name: Some("c2"),
        value: None,
    };

    let c = config.prior(config2);
    assert_eq!(c.name, Some("c2"));
    assert_eq!(c.value, Some(1));
}

#[test]
fn test_derive_extension_named_fields_struct_generics() {
    let config = Config {
        name: Some("c1"),
        value: None,
    }
    .with_extension("first");
    let config2 = Config {
        name: Some("c2"),
        value: Some(2),
    }
    .with_extension("second");

    let c = config.posterior(config2);
    assert_eq!(c.name.extension, "first");
    assert_eq!(*c.name, Some("c1"));
    assert_eq!(c.value.extension, "second");
    assert_eq!(*c.value, Some(2));

    assert!(matches!(
        c.into(),
        Config {
            name: Some("c1"),
            value: Some(2),
        }
    ));
}
