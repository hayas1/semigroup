use crate::annotate::Annotated;

pub trait Semigroup {
    fn semigroup_op(base: Self, other: Self) -> Self;
}
pub trait AnnotatedSemigroup<A>: Sized {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A>;
}
impl<T: AnnotatedSemigroup<A>, A> Semigroup for Annotated<T, A> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        AnnotatedSemigroup::annotated_op(base, other)
    }
}
