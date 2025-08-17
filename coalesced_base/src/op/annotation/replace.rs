use crate::{
    annotate::Annotated,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Construct<T>(T);
pub trait Replace: Sized + Semigroup {
    fn replace(self, other: Self) -> Self {
        Semigroup::semigroup_op(self, other)
    }
}
impl<T> Replace for Construct<T> {}
impl<T, P> Replace for Annotated<Construct<T>, P> {
    fn replace(self, other: Self) -> Self {
        AnnotatedSemigroup::annotated_op(self, other)
    }
}

impl<T> From<T> for Construct<T> {
    fn from(value: T) -> Self {
        Construct(value)
    }
}
impl<T> Construct<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Semigroup for Construct<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        AnnotatedSemigroup::annotated_op(
            Annotated::lift_with(base, ()),
            Annotated::lift_with(other, ()),
        )
        .value
    }
}
impl<T, A> AnnotatedSemigroup<A> for Construct<T> {
    fn annotated_op(base: Annotated<Self, A>, _other: Annotated<Self, A>) -> Annotated<Self, A> {
        base
    }
}
