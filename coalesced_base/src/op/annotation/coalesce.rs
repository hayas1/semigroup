use coalesced_derive::ConstructionUse;

use crate::{
    annotate::{Annotate, Annotated},
    op::{Construction, ConstructionAnnotated},
    reverse::Reversed,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
mod sealed {
    use super::*;

    impl<T> Coalesce for Option<T> {}
    impl<T> Semigroup for Option<T> {
        fn semigroup_op(base: Self, other: Self) -> Self {
            Coalesced::lift_op(base, other)
        }
    }
    impl<T, A> Annotate<A> for Option<T> {
        type Annotation = A;
        fn annotated(self, annotation: Self::Annotation) -> Annotated<Self, A> {
            Annotated::new(self, annotation)
        }
    }
    impl<T, A> Coalesce for Annotated<Option<T>, A> {}
    impl<T, A> AnnotatedSemigroup<A> for Option<T> {
        fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
            Coalesced::lift_annotated_op(base, other)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::semigroup::tests::{
        assert_associative_law, assert_lazy_evaluation, assert_reversed_associative_law,
    };

    use super::*;

    #[test]
    fn test_coalesce_as_semigroup_op() {
        let (a, b, c) = (Coalesced(Some(1)), Coalesced(Some(2)), Coalesced(Some(3)));
        assert_associative_law(a, b, c);
        assert_reversed_associative_law(a, b, c);
        assert_lazy_evaluation(a, b, c);
        let (a, b, c) = (Coalesced(None), Coalesced(Some(2)), Coalesced(Some(3)));
        assert_associative_law(a, b, c);
        assert_reversed_associative_law(a, b, c);
        assert_lazy_evaluation(a, b, c);
        let (a, b, c) = (Coalesced(None), Coalesced(Some(2)), Coalesced(None));
        assert_associative_law(a, b, c);
        assert_reversed_associative_law(a, b, c);
        assert_lazy_evaluation(a, b, c);
        let (a, b, c) = (Coalesced::<u32>(None), Coalesced(None), Coalesced(None));
        assert_associative_law(a, b, c);
        assert_reversed_associative_law(a, b, c);
        assert_lazy_evaluation(a, b, c);
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
