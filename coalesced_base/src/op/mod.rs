use crate::{
    annotate::{Annotate, Annotated},
    semigroup::{AnnotatedSemigroup, Semigroup},
};

pub mod annotation;
pub mod semigroup;

pub trait Construction<T>: Sized + Semigroup {
    fn new(value: T) -> Self;
    fn into_inner(self) -> T;
    fn lift_op(base: T, other: T) -> T {
        Semigroup::semigroup_op(Self::new(base), Self::new(other)).into_inner()
    }
}

pub trait ConstructionAnnotated<T, A>:
    Construction<T> + AnnotatedSemigroup<A> + Annotate<A>
{
    fn lift_annotated_op(base: Annotated<T, A>, other: Annotated<T, A>) -> Annotated<T, A> {
        AnnotatedSemigroup::annotated_op(base.map(Self::new), other.map(Self::new))
            .map(Self::into_inner)
    }
}
