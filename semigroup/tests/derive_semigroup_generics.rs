use semigroup::Semigroup;

#[derive(Debug, Clone, PartialEq, Semigroup)]
pub struct NamedStruct<T> {
    pub name: T,
    #[semigroup(with = "semigroup::op::Coalesce")]
    pub value: Option<u32>,
}

#[test]
fn test_named_struct_semigroup() {
    let a = NamedStruct {
        name: semigroup::op::Last("A".to_string()),
        value: Some(10),
    };
    let b = NamedStruct {
        name: semigroup::op::Last("B".to_string()),
        value: None,
    };

    assert_eq!(
        NamedStruct::op(a.clone(), b.clone()),
        NamedStruct {
            name: semigroup::op::Last("B".to_string()),
            value: Some(10),
        }
    );
    assert_eq!(
        NamedStruct::op(b.clone(), a.clone()),
        NamedStruct {
            name: semigroup::op::Last("A".to_string()),
            value: Some(10),
        }
    );
}

#[derive(Debug, Clone, PartialEq, Semigroup)]
pub struct UnnamedStruct<T>(
    T,
    #[semigroup(with = "semigroup::op::Coalesce")] Option<u32>,
);

#[test]
fn test_unnamed_struct_semigroup() {
    let a = UnnamedStruct(semigroup::op::Last("A".to_string()), Some(10));
    let b = UnnamedStruct(semigroup::op::Last("B".to_string()), None);

    assert_eq!(
        UnnamedStruct::op(a.clone(), b.clone()),
        UnnamedStruct(semigroup::op::Last("B".to_string()), Some(10))
    );
    assert_eq!(
        UnnamedStruct::op(b.clone(), a.clone()),
        UnnamedStruct(semigroup::op::Last("A".to_string()), Some(10))
    );
}
