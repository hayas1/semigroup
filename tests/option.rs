use coalesced::{annotate::Annotated, op::annotation::coalesce::Coalesce};

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
    let val1 = Annotated::lift_with(None, "first");
    let val2 = Annotated::lift_with(Some(10), "second");
    let val3 = Annotated::lift_with(Some(100), "third");
    let val4 = Annotated::lift_with(None, "fourth");

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

    assert_eq!(val1c2.value, Some(10));
    assert_eq!(val1c3.value, Some(100));
    assert_eq!(val1c4.value, None);
    assert_eq!(val2c1.value, Some(10));
    assert_eq!(val2c3.value, Some(10));
    assert_eq!(val2c4.value, Some(10));
    assert_eq!(val3c1.value, Some(100));
    assert_eq!(val3c2.value, Some(100));
    assert_eq!(val3c4.value, Some(100));
    assert_eq!(val4c1.value, None);
    assert_eq!(val4c2.value, Some(10));
    assert_eq!(val4c3.value, Some(100));

    assert_eq!(val1c2.annotation, "second");
    assert_eq!(val1c3.annotation, "third");
    assert_eq!(val1c4.annotation, "first");
    assert_eq!(val2c1.annotation, "second");
    assert_eq!(val2c3.annotation, "second");
    assert_eq!(val2c4.annotation, "second");
    assert_eq!(val3c1.annotation, "third");
    assert_eq!(val3c2.annotation, "third");
    assert_eq!(val3c4.annotation, "third");
    assert_eq!(val4c1.annotation, "fourth");
    assert_eq!(val4c2.annotation, "second");
    assert_eq!(val4c3.annotation, "third");

    assert_eq!(val1c2, Annotated::lift_with(Some(10), "second"));
    assert_eq!(val1c3, Annotated::lift_with(Some(100), "third"));
    assert_eq!(val1c4, Annotated::lift_with(None, "first"));
    assert_eq!(val2c1, Annotated::lift_with(Some(10), "second"));
    assert_eq!(val2c3, Annotated::lift_with(Some(10), "second"));
    assert_eq!(val2c4, Annotated::lift_with(Some(10), "second"));
    assert_eq!(val3c1, Annotated::lift_with(Some(100), "third"));
    assert_eq!(val3c2, Annotated::lift_with(Some(100), "third"));
    assert_eq!(val3c4, Annotated::lift_with(Some(100), "third"));
    assert_eq!(val4c1, Annotated::lift_with(None, "fourth"));
    assert_eq!(val4c2, Annotated::lift_with(Some(10), "second"));
    assert_eq!(val4c3, Annotated::lift_with(Some(100), "third"));
}
