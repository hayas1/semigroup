use coalesced::Coalesce;

#[derive(Coalesce)]
struct Config(&'static str, Option<u32>);

#[test]
fn test_derive_unnamed_fields_struct() {
    let config = Config("c1", Some(1));
    let config2 = Config("c2", None);

    let c = config.prior(config2);
    assert_eq!(c.0, "c2");
    assert_eq!(c.1, Some(1));
}
