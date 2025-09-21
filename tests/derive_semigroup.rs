use coalesced::Semigroup;

#[derive(Debug, Clone, PartialEq, Semigroup)]
pub struct NamedStruct {
    #[semigroup(with = "coalesced::op::annotation::replace::Replace")]
    pub name: String,
    pub value: Option<u32>,
}

#[test]
fn test_named_struct_semigroup_op() {
    let a = NamedStruct {
        name: "A".to_string(),
        value: Some(10),
    };
    let b = NamedStruct {
        name: "B".to_string(),
        value: None,
    };

    assert_eq!(
        NamedStruct::semigroup_op(a.clone(), b.clone()),
        NamedStruct {
            name: "B".to_string(),
            value: Some(10),
        }
    );
    assert_eq!(
        NamedStruct::semigroup_op(b.clone(), a.clone()),
        NamedStruct {
            name: "A".to_string(),
            value: Some(10),
        }
    );
}

#[derive(Debug, Clone, PartialEq, Semigroup)]
pub struct UnnamedStruct(
    #[semigroup(with = "coalesced::op::annotation::replace::Replace")] String,
    Option<u32>,
);

#[test]
fn test_unnamed_struct_semigroup_op() {
    let a = UnnamedStruct("A".to_string(), Some(10));
    let b = UnnamedStruct("B".to_string(), None);

    assert_eq!(
        UnnamedStruct::semigroup_op(a.clone(), b.clone()),
        UnnamedStruct("B".to_string(), Some(10))
    );
    assert_eq!(
        UnnamedStruct::semigroup_op(b.clone(), a.clone()),
        UnnamedStruct("A".to_string(), Some(10))
    );
}

#[derive(Debug, Clone, PartialEq, Semigroup)]
pub struct UnitStruct;
#[test]
fn test_unit_struct_semigroup_op() {
    let a = UnitStruct;
    let b = UnitStruct;
    assert_eq!(UnitStruct::semigroup_op(a.clone(), b.clone()), UnitStruct);
    assert_eq!(UnitStruct::semigroup_op(b.clone(), a.clone()), UnitStruct);
}
