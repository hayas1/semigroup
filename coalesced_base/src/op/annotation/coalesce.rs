use crate::{
    annotate::{Annotate, Annotated},
    op::reverse::Reversed,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Coalesced<T>(pub Option<T>);
pub trait Coalesce: Sized + Semigroup {
    fn coalesce(self, other: Self) -> Self {
        Semigroup::semigroup_op(self, other)
    }
}
impl<T> Coalesce for Coalesced<T> {}
impl<T> Coalesce for Reversed<Coalesced<T>> {}
impl<T, P> Coalesce for Annotated<Coalesced<T>, P> {}
impl<T, P> Coalesce for Reversed<Annotated<Coalesced<T>, P>> {}

mod sealed {
    use super::*;

    impl<T> Coalesce for Option<T> {}
    impl<T> Semigroup for Option<T> {
        fn semigroup_op(base: Self, other: Self) -> Self {
            Semigroup::semigroup_op(Coalesced(base), Coalesced(other)).into_inner()
        }
    }
    impl<T, A> Coalesce for Annotated<Option<T>, A> {}
    impl<T, A> AnnotatedSemigroup<A> for Option<T> {
        fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
            AnnotatedSemigroup::annotated_op(base.map(Coalesced), other.map(Coalesced))
                .map(Coalesced::into_inner)
        }
    }
}

impl<T> From<T> for Coalesced<T> {
    fn from(value: T) -> Self {
        Coalesced(Some(value))
    }
}
impl<T> Coalesced<T> {
    pub fn into_inner(self) -> Option<T> {
        self.0
    }
}

impl<T> Semigroup for Coalesced<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        AnnotatedSemigroup::annotated_op(base.annotated(()), other.annotated(())).value
    }
}
impl<T, A> AnnotatedSemigroup<A> for Coalesced<T> {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        match (&base.value.0, &other.value.0) {
            (Some(_), _) | (None, None) => base,
            (None, Some(_)) => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::semigroup::tests::{assert_associative_law, assert_reversed_associative_law};

    use super::*;

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
        let (a, b) = (Coalesced(None), Coalesced(Some("value")));
        assert_eq!(a.coalesce(b).into_inner(), Some("value"));
        let (ra, rb) = (Reversed(a), Reversed(b));
        assert_eq!(ra.coalesce(rb).0.into_inner(), Some("value"));

        let (a, b) = (Coalesced(Some(1)), Coalesced(Some(2)));
        assert_eq!(a.coalesce(b).into_inner(), Some(1));
        let (ra, rb) = (Reversed(a), Reversed(b));
        assert_eq!(ra.coalesce(rb).0.into_inner(), Some(2));
    }
}
