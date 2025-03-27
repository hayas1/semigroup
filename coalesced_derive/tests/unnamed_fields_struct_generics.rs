use coalesced::Coalesce;

#[derive(Coalesce)]
struct Config<T>(&'static str, Option<T>);

#[test]
fn test_derive_named_fields_struct() {
    let config = Config("c1", Some(1));
    let config2 = Config("c2", None);

    let c = config.prior(config2);
    assert_eq!(c.0, "c2");
    assert_eq!(c.1, Some(1));
}
