use coalesced_derive::ConstructionUse;

use crate::{
    annotate::Annotated,
    op::{Construction, ConstructionAnnotated},
    reverse::Reversed,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(
    annotated,
    op = Concat,
    annotation_type_param = "A: IntoIterator + FromIterator<A::Item>",
    annotation_where = "A::Item: Clone",
    unit = "vec![(); 0]",
    without_annotate_impl
)]
pub struct Concatenated<T: IntoIterator + FromIterator<T::Item>>(pub T);
impl<T: IntoIterator + FromIterator<T::Item>, A: IntoIterator + FromIterator<A::Item>>
    AnnotatedSemigroup<A> for Concatenated<T>
{
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        let (base_value, base_annotation) = base.into_parts();
        let (other_value, other_annotation) = other.into_parts();

        Annotated::new(
            Concatenated(base_value.0.into_iter().chain(other_value.0).collect()),
            base_annotation
                .into_iter()
                .chain(other_annotation)
                .collect(),
        )
    }
}
impl<T: IntoIterator + FromIterator<T::Item>, A: IntoIterator + FromIterator<A::Item>>
    crate::annotate::Annotate<A> for Concatenated<T>
where
    A::Item: Clone,
{
    type Annotation = A::Item;
    fn annotated(self, annotation: Self::Annotation) -> Annotated<Self, A> {
        let iter = self.0.into_iter();
        let (len, _) = iter.size_hint(); // TODO use exact size
        Annotated::new(
            Self(iter.collect()),
            std::iter::repeat_n(annotation, len).collect(),
        )
    }
}
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

#[cfg(test)]
mod tests {
    use crate::semigroup::tests::{
        assert_associative_law, assert_lazy_evaluation, assert_reversed_associative_law,
    };

    use super::*;

    #[test]
    fn test_concat_as_semigroup_op() {
        let (a, b, c) = (
            Concatenated(vec![1]),
            Concatenated(vec![2]),
            Concatenated(vec![3]),
        );
        assert_associative_law(a.clone(), b.clone(), c.clone());
        assert_reversed_associative_law(a.clone(), b.clone(), c.clone());
        assert_lazy_evaluation(a, b, c);
    }

    #[test]
    fn test_concat() {
        let (a, b) = (Concatenated(vec![1]), Concatenated(vec![2]));
        assert_eq!(a.clone().concat(b.clone()).into_inner(), vec![1, 2]);

        let (ra, rb) = (Reversed(a.clone()), Reversed(b.clone()));
        assert_eq!(ra.concat(rb).0.into_inner(), vec![2, 1]);
    }
}
