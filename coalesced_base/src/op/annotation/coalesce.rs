use crate::{
    annotate::Annotated,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Construct<T>(pub Option<T>);
pub trait Coalesce: Sized + Semigroup {
    fn coalesce(self, other: Self) -> Self {
        Semigroup::semigroup_op(self, other)
    }
}
impl<T> Coalesce for Construct<T> {}
impl<T, P> Coalesce for Annotated<Construct<T>, P> {
    fn coalesce(self, other: Self) -> Self {
        AnnotatedSemigroup::annotated_op(self, other)
    }
}
mod sealed {
    use super::*;

    impl<T, A> AnnotatedSemigroup<A> for Option<T> {
        fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
            AnnotatedSemigroup::annotated_op(base.map(Construct), other.map(Construct))
                .map(Construct::into_inner)
        }
    }
}

impl<T> From<T> for Construct<T> {
    fn from(value: T) -> Self {
        Construct(Some(value))
    }
}
impl<T> Construct<T> {
    pub fn into_inner(self) -> Option<T> {
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
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        match (&base.value.0, &other.value.0) {
            (Some(_), _) | (None, None) => base,
            (None, Some(_)) => other,
        }
    }
}
