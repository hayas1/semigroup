use crate::annotate::Annotated;

pub trait Semigroup {
    fn op(self, other: Self) -> Self;
}
pub trait AnnotatedSemigroup: Sized {
    fn annotated_op<P>(base: Annotated<Self, P>, other: Annotated<Self, P>) -> Annotated<Self, P>;
}
impl<T: AnnotatedSemigroup> Semigroup for T {
    fn op(self, other: Self) -> Self {
        let (base, other) = (
            Annotated::lift_with(self, ()),
            Annotated::lift_with(other, ()),
        );
        Self::annotated_op(base, other).value
    }
}
