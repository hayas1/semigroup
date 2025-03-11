use coalesced_derive::Coalesce;

#[derive(Coalesce)]
struct Unit;

#[derive(Coalesce)]
struct Empty {}

#[test]
fn test_derive_compile() {
    let unit = Unit;
    assert!(matches!(unit, Unit));

    let empty = Empty {};
    assert!(matches!(empty, Empty {}));
}
