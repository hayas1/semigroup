use coalesced::Coalesce;

#[derive(Coalesce)]
struct Config<T> {
    name: &'static str,
    value: Option<T>,
}

#[test]
fn test_derive_named_fields_struct_generics() {
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
