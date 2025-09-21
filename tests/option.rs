use coalesced::{op::annotation::coalesce::CoalesceExt, Annotate};

#[test]
fn test_option() {
    let val1 = None;
    let val2 = Some(10);
    let val3 = Some(100);
    let val4 = None;

    assert_eq!(val1.coalesce(val2), Some(10));
    assert_eq!(val1.coalesce(val3), Some(100));
    assert_eq!(val1.coalesce(val4), None);
    assert_eq!(val2.coalesce(val1), Some(10));
    assert_eq!(val2.coalesce(val3), Some(10));
    assert_eq!(val2.coalesce(val4), Some(10));
    assert_eq!(val3.coalesce(val1), Some(100));
    assert_eq!(val3.coalesce(val2), Some(100));
    assert_eq!(val3.coalesce(val4), Some(100));
    assert_eq!(val4.coalesce(val1), None);
    assert_eq!(val4.coalesce(val2), Some(10));
    assert_eq!(val4.coalesce(val3), Some(100));
}

#[test]
fn test_option_with_extension() {
    let val1 = None.annotated("first");
    let val2 = Some(10).annotated("second");
    let val3 = Some(100).annotated("third");
    let val4 = None.annotated("fourth");

    let val1c2 = val1.coalesce(val2);
    let val1c3 = val1.coalesce(val3);
    let val1c4 = val1.coalesce(val4);
    let val2c1 = val2.coalesce(val1);
    let val2c3 = val2.coalesce(val3);
    let val2c4 = val2.coalesce(val4);
    let val3c1 = val3.coalesce(val1);
    let val3c2 = val3.coalesce(val2);
    let val3c4 = val3.coalesce(val4);
    let val4c1 = val4.coalesce(val1);
    let val4c2 = val4.coalesce(val2);
    let val4c3 = val4.coalesce(val3);

    assert_eq!(val1c2.into_value(), Some(10));
    assert_eq!(val1c3.into_value(), Some(100));
    assert_eq!(val1c4.into_value(), None);
    assert_eq!(val2c1.into_value(), Some(10));
    assert_eq!(val2c3.into_value(), Some(10));
    assert_eq!(val2c4.into_value(), Some(10));
    assert_eq!(val3c1.into_value(), Some(100));
    assert_eq!(val3c2.into_value(), Some(100));
    assert_eq!(val3c4.into_value(), Some(100));
    assert_eq!(val4c1.into_value(), None);
    assert_eq!(val4c2.into_value(), Some(10));
    assert_eq!(val4c3.into_value(), Some(100));

    assert_eq!(val1c2.into_annotation(), "second");
    assert_eq!(val1c3.into_annotation(), "third");
    assert_eq!(val1c4.into_annotation(), "first");
    assert_eq!(val2c1.into_annotation(), "second");
    assert_eq!(val2c3.into_annotation(), "second");
    assert_eq!(val2c4.into_annotation(), "second");
    assert_eq!(val3c1.into_annotation(), "third");
    assert_eq!(val3c2.into_annotation(), "third");
    assert_eq!(val3c4.into_annotation(), "third");
    assert_eq!(val4c1.into_annotation(), "fourth");
    assert_eq!(val4c2.into_annotation(), "second");
    assert_eq!(val4c3.into_annotation(), "third");

    assert_eq!(val1c2, Some(10).annotated("second"));
    assert_eq!(val1c3, Some(100).annotated("third"));
    assert_eq!(val1c4, None.annotated("first"));
    assert_eq!(val2c1, Some(10).annotated("second"));
    assert_eq!(val2c3, Some(10).annotated("second"));
    assert_eq!(val2c4, Some(10).annotated("second"));
    assert_eq!(val3c1, Some(100).annotated("third"));
    assert_eq!(val3c2, Some(100).annotated("third"));
    assert_eq!(val3c4, Some(100).annotated("third"));
    assert_eq!(val4c1, None.annotated("fourth"));
    assert_eq!(val4c2, Some(10).annotated("second"));
    assert_eq!(val4c3, Some(100).annotated("third"));
}
