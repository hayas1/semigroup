use crate::{
    annotate::Annotated,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Concatenated<T>(pub T);
pub trait Concat: Sized + Semigroup {
    fn concat(self, other: Self) -> Self {
        Semigroup::semigroup_op(self, other)
    }
}
impl<T: IntoIterator + FromIterator<T::Item>> Concat for Concatenated<T> {}
impl<T: IntoIterator + FromIterator<T::Item>, A: IntoIterator + FromIterator<A::Item>> Concat
    for Annotated<Concatenated<T>, A>
{
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

impl<T> From<T> for Concatenated<T> {
    fn from(value: T) -> Self {
        Concatenated(value)
    }
}
impl<T> Concatenated<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: IntoIterator + FromIterator<T::Item>> Semigroup for Concatenated<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        AnnotatedSemigroup::annotated_op(
            Annotated::lift_with(base, vec![(); 0]),
            Annotated::lift_with(other, vec![(); 0]),
        )
        .value
    }
}
impl<T: IntoIterator + FromIterator<T::Item>, A: IntoIterator + FromIterator<A::Item>>
    AnnotatedSemigroup<A> for Concatenated<T>
{
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        let value = Concatenated(base.value.0.into_iter().chain(other.value.0).collect());
        let annotation = base
            .annotation
            .into_iter()
            .chain(other.annotation)
            .collect();
        Annotated::lift_with(value, annotation)
    }
}
