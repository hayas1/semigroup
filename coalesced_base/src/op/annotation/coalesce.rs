use crate::{
    annotate::Annotated,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Construct<T>(Option<T>);
pub trait Coalesce {
    fn coalesce(self, other: Self) -> Self;
}
impl<T> Coalesce for Construct<T> {
    fn coalesce(self, other: Self) -> Self {
        self.op(other)
    }
}
impl<T, P> Coalesce for Annotated<Construct<T>, P> {
    fn coalesce(self, other: Self) -> Self {
        AnnotatedSemigroup::annotated_op(self, other)
    }
}
impl<T> Coalesce for Option<T> {
    fn coalesce(self, other: Self) -> Self {
        Construct(self).coalesce(Construct(other)).into_inner()
    }
}
impl<T, P> Coalesce for Annotated<Option<T>, P> {
    fn coalesce(self, other: Self) -> Self {
        AnnotatedSemigroup::annotated_op(self.map(Construct), other.map(Construct))
            .map(Construct::into_inner)
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

impl<T> AnnotatedSemigroup for Construct<T> {
    fn annotated_op<P>(base: Annotated<Self, P>, other: Annotated<Self, P>) -> Annotated<Self, P> {
        match (&base.value.0, &other.value.0) {
            (Some(_), _) | (None, None) => base,
            (None, Some(_)) => other,
        }
    }
}
