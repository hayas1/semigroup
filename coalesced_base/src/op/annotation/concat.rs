use coalesced_derive::ConstructionUse;

use crate::{
    annotate::{Annotate, Annotated},
    reverse::Reversed,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(annotated, op = Concat, annotation_type_param = "S: IntoIterator + FromIterator<S::Item>", unit = "vec![(); 0]")]
pub struct Concatenated<T: IntoIterator + FromIterator<T::Item>>(pub T);

mod sealed {
    // use super::*;

    // impl<T: IntoIterator + FromIterator<T::Item>, A: IntoIterator + FromIterator<A::Item>>
    //     AnnotatedSemigroup<A> for T
    // {
    //     fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
    //         AnnotatedSemigroup::annotated_op(base.map(Construct), other.map(Construct))
    //             .map(Construct::into_inner)
    //     }
    // }
}

impl<T: IntoIterator + FromIterator<T::Item>, A: IntoIterator + FromIterator<A::Item>>
    AnnotatedSemigroup<A> for Concatenated<T>
{
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        let value: Concatenated<T> =
            Concatenated(base.value.0.into_iter().chain(other.value.0).collect());
        let annotation = base
            .annotation
            .into_iter()
            .chain(other.annotation)
            .collect();
        value.annotated(annotation)
    }
}

#[cfg(test)]
mod tests {
    use crate::semigroup::tests::{assert_associative_law, assert_reversed_associative_law};

    use super::*;

    #[test]
    fn test_concat_as_semigroup_op() {
        let (a, b, c) = (
            Concatenated(vec![1]),
            Concatenated(vec![2]),
            Concatenated(vec![3]),
        );
        assert_associative_law(a.clone(), b.clone(), c.clone());
        assert_reversed_associative_law(a, b, c);
    }

    #[test]
    fn test_concat() {
        let (a, b) = (Concatenated(vec![1]), Concatenated(vec![2]));
        assert_eq!(a.clone().concat(b.clone()).into_inner(), vec![1, 2]);

        let (ra, rb) = (Reversed(a.clone()), Reversed(b.clone()));
        assert_eq!(ra.concat(rb).0.into_inner(), vec![2, 1]);
    }
}
