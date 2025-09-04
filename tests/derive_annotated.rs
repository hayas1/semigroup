use coalesced::{Annotated, Semigroup};

#[derive(Debug, Clone, PartialEq, Semigroup)]
#[semigroup(annotated)]
pub struct NamedStruct {
    #[semigroup(with = "coalesced::op::annotation::replace::Replaced")]
    pub name: String,
    pub value: Option<u32>,
}

#[test]
fn test_named_struct_semigroup_op() {
    let a = Annotated {
        value: NamedStruct {
            name: "A".to_string(),
            value: Some(10),
        },
        annotation: NamedStructAnnotation {
            name: "First",
            value: "First",
        },
    };
    let b = Annotated {
        value: NamedStruct {
            name: "B".to_string(),
            value: None,
        },
        annotation: NamedStructAnnotation {
            name: "Second",
            value: "First",
        },
    };

    let ab = Semigroup::semigroup_op(a.clone(), b.clone());
    assert_eq!(ab.value.name, "B");
    assert_eq!(ab.annotation.name, "Second");
    assert_eq!(ab.value.value, Some(10));
    assert_eq!(ab.annotation.value, "First");
    assert_eq!(
        ab,
        Annotated {
            value: NamedStruct {
                name: "B".to_string(),
                value: Some(10),
            },
            annotation: NamedStructAnnotation {
                name: "Second",
                value: "First",
            },
        },
    );
    let ba = Semigroup::semigroup_op(b.clone(), a.clone());
    assert_eq!(ba.value.name, "A");
    assert_eq!(ba.annotation.name, "First");
    assert_eq!(ba.value.value, Some(10));
    assert_eq!(ba.annotation.value, "First");
    assert_eq!(
        ba,
        Annotated {
            value: NamedStruct {
                name: "A".to_string(),
                value: Some(10),
            },
            annotation: NamedStructAnnotation {
                name: "First",
                value: "First",
            },
        },
    );
}

// #[derive(Debug, Clone, PartialEq, Semigroup)]
// pub struct UnnamedStruct(
//     #[semigroup(with = "coalesced::op::annotation::replace::Replaced")] String,
//     Option<u32>,
// );

// #[test]
// fn test_unnamed_struct_semigroup_op() {
//     let a = UnnamedStruct("A".to_string(), Some(10));
//     let b = UnnamedStruct("B".to_string(), None);

//     assert_eq!(
//         UnnamedStruct::semigroup_op(a.clone(), b.clone()),
//         UnnamedStruct("B".to_string(), Some(10))
//     );
//     assert_eq!(
//         UnnamedStruct::semigroup_op(b.clone(), a.clone()),
//         UnnamedStruct("A".to_string(), Some(10))
//     );
// }

// #[derive(Debug, Clone, PartialEq, Semigroup)]
// pub struct UnitStruct;
// #[test]
// fn test_unit_struct_semigroup_op() {
//     let a = UnitStruct;
//     let b = UnitStruct;
//     assert_eq!(UnitStruct::semigroup_op(a.clone(), b.clone()), UnitStruct);
//     assert_eq!(UnitStruct::semigroup_op(b.clone(), a.clone()), UnitStruct);
// }
