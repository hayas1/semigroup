use crate::annotate::Annotated;

pub trait Semigroup {
    fn op(self, other: Self) -> Self;
}
pub trait AnnotatedSemigroup<A>: Sized {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A>;
}
impl<T: AnnotatedSemigroup<()>> Semigroup for T {
    fn op(self, other: Self) -> Self {
        let (base, other) = (
            Annotated::lift_with(self, ()),
            Annotated::lift_with(other, ()),
        );
        Self::annotated_op(base, other).value
    }
}
