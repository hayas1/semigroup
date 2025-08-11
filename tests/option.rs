use coalesced::{Coalesce, Extension};

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
    let val1 = None.with_extension("first");
    let val2 = Some(10).with_extension("second");
    let val3 = Some(100).with_extension("third");
    let val4 = None.with_extension("fourth");

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

    assert_eq!(val1c2.extension, "second");
    assert_eq!(val1c3.extension, "third");
    assert_eq!(val1c4.extension, "first");
    assert_eq!(val2c1.extension, "second");
    assert_eq!(val2c3.extension, "second");
    assert_eq!(val2c4.extension, "second");
    assert_eq!(val3c1.extension, "third");
    assert_eq!(val3c2.extension, "third");
    assert_eq!(val3c4.extension, "third");
    assert_eq!(val4c1.extension, "fourth");
    assert_eq!(val4c2.extension, "second");
    assert_eq!(val4c3.extension, "third");

    assert_eq!(val1c2, Some(10).with_extension("second"));
    assert_eq!(val1c3, Some(100).with_extension("third"));
    assert_eq!(val1c4, None.with_extension("first"));
    assert_eq!(val2c1, Some(10).with_extension("second"));
    assert_eq!(val2c3, Some(10).with_extension("second"));
    assert_eq!(val2c4, Some(10).with_extension("second"));
    assert_eq!(val3c1, Some(100).with_extension("third"));
    assert_eq!(val3c2, Some(100).with_extension("third"));
    assert_eq!(val3c4, Some(100).with_extension("third"));
    assert_eq!(val4c1, None.with_extension("fourth"));
    assert_eq!(val4c2, Some(10).with_extension("second"));
    assert_eq!(val4c3, Some(100).with_extension("third"));
}
