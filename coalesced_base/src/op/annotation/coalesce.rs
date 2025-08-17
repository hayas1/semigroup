use crate::{
    annotate::Annotated,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Coalesced<T>(pub Option<T>);
pub trait Coalesce: Sized + Semigroup {
    fn coalesce(self, other: Self) -> Self {
        Semigroup::semigroup_op(self, other)
    }
}
impl<T> Coalesce for Coalesced<T> {}
impl<T, P> Coalesce for Annotated<Coalesced<T>, P> {}

mod sealed {
    use super::*;

    impl<T> Coalesce for Option<T> {}
    impl<T> Semigroup for Option<T> {
        fn semigroup_op(base: Self, other: Self) -> Self {
            Semigroup::semigroup_op(Coalesced(base), Coalesced(other)).into_inner()
        }
    }
    impl<T, A> Coalesce for Annotated<Option<T>, A> {}
    impl<T, A> AnnotatedSemigroup<A> for Option<T> {
        fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
            AnnotatedSemigroup::annotated_op(base.map(Coalesced), other.map(Coalesced))
                .map(Coalesced::into_inner)
        }
    }
}

impl<T> From<T> for Coalesced<T> {
    fn from(value: T) -> Self {
        Coalesced(Some(value))
    }
}
impl<T> Coalesced<T> {
    pub fn into_inner(self) -> Option<T> {
        self.0
    }
}

impl<T> Semigroup for Coalesced<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        AnnotatedSemigroup::annotated_op(
            Annotated::lift_with(base, ()),
            Annotated::lift_with(other, ()),
        )
        .value
    }
}
impl<T, A> AnnotatedSemigroup<A> for Coalesced<T> {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        match (&base.value.0, &other.value.0) {
            (Some(_), _) | (None, None) => base,
            (None, Some(_)) => other,
        }
    }
}
