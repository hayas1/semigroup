use semigroup::{AnnotateFields, Semigroup};

#[derive(Debug, Clone, PartialEq, Semigroup)]
#[semigroup(annotated, with = "semigroup::op::Coalesce")]
pub struct NamedStruct {
    #[semigroup(with = "semigroup::op::Overwrite")]
    pub name: String,
    pub value: Option<u32>,
}

#[test]
fn test_named_struct_semigroup() {
    let a = NamedStruct {
        name: "A".to_string(),
        value: Some(10u32),
    }
    .annotated("First");

    let b = NamedStruct {
        name: "B".to_string(),
        value: None,
    }
    .annotated("Second");

    // a.name uses Overwrite: always takes other → ab.name = "B"
    // a.value uses Coalesce: keeps first Some → ab.value = Some(10u32)
    let ab = Semigroup::op(a.clone(), b.clone());
    assert_eq!(ab.name.value(), &"B".to_string());
    assert_eq!(ab.name.annotation(), &"Second");
    assert_eq!(ab.value.value(), &Some(10u32));
    assert_eq!(ab.value.annotation(), &"First");

    // b.name uses Overwrite: always takes other → ba.name = "A"
    // b.value uses Coalesce: None+Some → ba.value = Some(10u32)
    let ba = Semigroup::op(b.clone(), a.clone());
    assert_eq!(ba.name.value(), &"A".to_string());
    assert_eq!(ba.name.annotation(), &"First");
    assert_eq!(ba.value.value(), &Some(10u32));
    assert_eq!(ba.value.annotation(), &"First");
}

#[test]
fn test_named_struct_into() {
    let original = NamedStruct {
        name: "A".to_string(),
        value: Some(10u32),
    };
    let annotated = original.clone().annotated("ann");
    let recovered: NamedStruct = annotated.into();
    assert_eq!(recovered, original);
}

#[derive(Debug, Clone, PartialEq, Semigroup)]
#[semigroup(annotated, with = "semigroup::op::Coalesce")]
pub struct UnnamedStruct(
    #[semigroup(with = "semigroup::op::Overwrite")] String,
    Option<u32>,
);

#[test]
fn test_unnamed_struct_semigroup() {
    let a = UnnamedStruct("A".to_string(), Some(10u32)).annotated(1.0_f64);
    let b = UnnamedStruct("B".to_string(), None).annotated(2.0_f64);

    // field 0: Overwrite always takes other → ab.0 = "B"
    // field 1: Coalesce keeps first Some → ab.1 = Some(10u32)
    let ab = Semigroup::op(a.clone(), b.clone());
    assert_eq!(ab.0.value(), &"B".to_string());
    assert_eq!(ab.0.annotation(), &2.0_f64);
    assert_eq!(ab.1.value(), &Some(10u32));
    assert_eq!(ab.1.annotation(), &1.0_f64);

    let ba = Semigroup::op(b.clone(), a.clone());
    assert_eq!(ba.0.value(), &"A".to_string());
    assert_eq!(ba.0.annotation(), &1.0_f64);
    assert_eq!(ba.1.value(), &Some(10u32));
    assert_eq!(ba.1.annotation(), &1.0_f64);
}

#[test]
fn test_unnamed_struct_into() {
    let original = UnnamedStruct("A".to_string(), Some(10u32));
    let annotated = original.clone().annotated(1.0_f64);
    let recovered: UnnamedStruct = annotated.into();
    assert_eq!(recovered, original);
}

#[derive(Debug, Clone, PartialEq, Semigroup)]
#[semigroup(annotated)]
pub struct DualCoalesce {
    #[semigroup(with = "semigroup::Dual(semigroup::op::Overwrite(_))")]
    pub key: String,
    #[semigroup(with = "semigroup::Dual(semigroup::op::Coalesce(_))")]
    pub value: Option<u32>,
}

#[test]
fn test_constructor_with_annotated() {
    let a = DualCoalesce {
        key: "a".to_string(),
        value: None,
    }
    .annotated("first");
    let b = DualCoalesce {
        key: "b".to_string(),
        value: Some(2),
    }
    .annotated("second");
    let ab = Semigroup::op(a.clone(), b.clone());
    assert_eq!(ab.value.value(), &Some(2u32));
    assert_eq!(ab.value.annotation(), &"second");

    let c = DualCoalesce {
        key: "c".to_string(),
        value: Some(1),
    }
    .annotated("first");
    let d = DualCoalesce {
        key: "d".to_string(),
        value: None,
    }
    .annotated("second");
    let cd = Semigroup::op(c.clone(), d.clone());
    assert_eq!(cd.value.value(), &Some(1u32));
    assert_eq!(cd.value.annotation(), &"first");
}
