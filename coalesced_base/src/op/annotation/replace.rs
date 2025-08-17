use crate::{
    annotate::Annotated,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Replaced<T>(pub T);
pub trait Replace: Sized + Semigroup {
    fn replace(self, other: Self) -> Self {
        Semigroup::semigroup_op(self, other)
    }
}
impl<T> Replace for Replaced<T> {}
impl<T, P> Replace for Annotated<Replaced<T>, P> {}

impl<T> From<T> for Replaced<T> {
    fn from(value: T) -> Self {
        Replaced(value)
    }
}
impl<T> Replaced<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Semigroup for Replaced<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        AnnotatedSemigroup::annotated_op(
            Annotated::lift_with(base, ()),
            Annotated::lift_with(other, ()),
        )
        .value
    }
}
impl<T, A> AnnotatedSemigroup<A> for Replaced<T> {
    fn annotated_op(base: Annotated<Self, A>, _other: Annotated<Self, A>) -> Annotated<Self, A> {
        base
    }
}
