use std::fmt::Display;

use coalesced::Coalesce;

#[derive(Coalesce)]
struct Config<S, T: Clone>(S, Option<T>)
where
    S: Display + Coalesce;

#[test]
fn test_derive_unnamed_fields_struct_constraint_generics() {
    let config = Config("c1", Some(1));
    let config2 = Config("c2", None);

    let c = config.prior(config2);
    assert_eq!(c.0, "c2");
    assert_eq!(c.1, Some(1));
}
