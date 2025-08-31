use coalesced::Semigroup;

#[derive(Debug, Clone, PartialEq, Semigroup)]
pub struct NamedStruct {
    #[semigroup(with = "coalesced::op::annotation::replace::Replaced")]
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
