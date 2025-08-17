use crate::{
    annotate::{Annotate, Annotated},
    op::reverse::Reversed,
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
impl<T> Replace for Reversed<Replaced<T>> {}
impl<T, P> Replace for Annotated<Replaced<T>, P> {}
impl<T, P> Replace for Reversed<Annotated<Replaced<T>, P>> {}

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
        AnnotatedSemigroup::annotated_op(base.annotated(()), other.annotated(())).value
    }
}
impl<T, A> AnnotatedSemigroup<A> for Replaced<T> {
    fn annotated_op(_base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        other
    }
}

#[cfg(test)]
mod tests {
    use crate::semigroup::tests::{assert_associative_law, assert_reversed_associative_law};

    use super::*;

    #[test]
    fn test_replace_as_semigroup_op() {
        let (a, b, c) = (Replaced(1), Replaced(2), Replaced(3));
        assert_associative_law(a, b, c);
        assert_reversed_associative_law(a, b, c);
    }

    #[test]
    fn test_replace() {
        let (a, b) = (Replaced(1), Replaced(2));
        assert_eq!(a.replace(b).into_inner(), 2);

        let (ra, rb) = (Reversed(a), Reversed(b));
        assert_eq!(ra.replace(rb).0.into_inner(), 1);
    }
}
