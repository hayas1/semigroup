use coalesced::{op::Construction, Annotated, AnnotatedSemigroup, Construction, Semigroup};
use coalesced_base::semigroup::tests::{assert_associative_law, assert_reversed_associative_law};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Construction)]
#[construction(annotated, op = Coalesce)]
pub struct Coalesced<T>(pub Option<T>);

impl<T, A> AnnotatedSemigroup<A> for Coalesced<T> {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        match (&base.value().0, &other.value().0) {
            (Some(_), _) | (None, None) => base,
            (None, Some(_)) => other,
        }
    }
}

#[test]
fn test_coalesce_as_semigroup_op() {
    let (a, b, c) = (Coalesced(Some(1)), Coalesced(Some(2)), Coalesced(Some(3)));
    assert_associative_law(a, b, c);
    assert_reversed_associative_law(a, b, c);
    let (a, b, c) = (Coalesced(None), Coalesced(Some(2)), Coalesced(Some(3)));
    assert_associative_law(a, b, c);
    assert_reversed_associative_law(a, b, c);
    let (a, b, c) = (Coalesced(None), Coalesced(Some(2)), Coalesced(None));
    assert_associative_law(a, b, c);
    assert_reversed_associative_law(a, b, c);
    let (a, b, c) = (Coalesced::<u32>(None), Coalesced(None), Coalesced(None));
    assert_associative_law(a, b, c);
    assert_reversed_associative_law(a, b, c);
}

#[test]
fn test_coalesce() {
    let (some_value1, some_value2, none) = (
        Coalesced(Some("value1")),
        Coalesced(Some("value2")),
        Coalesced(None),
    );
    assert_eq!(none.coalesce(none).into_inner(), None);
    assert_eq!(some_value1.coalesce(none).into_inner(), Some("value1"));
    assert_eq!(none.coalesce(some_value1).into_inner(), Some("value1"));
    assert_eq!(
        some_value1.coalesce(some_value2).into_inner(),
        Some("value1")
    );
    assert_eq!(
        some_value2.coalesce(some_value1).into_inner(),
        Some("value2")
    );
}
