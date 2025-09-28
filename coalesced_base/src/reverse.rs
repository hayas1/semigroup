use crate::semigroup::Semigroup;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reverse<T>(pub T);

impl<T: Semigroup> Semigroup for Reverse<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Self(Semigroup::semigroup_op(other.0, base.0))
    }
}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use std::fmt::Debug;

    use crate::semigroup::tests::assert_associative_law;

    use super::*;

    pub fn assert_reverse<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let (ra, rb, rc) = (Reverse(a.clone()), Reverse(b.clone()), Reverse(c.clone()));
        assert_eq!(
            T::semigroup_op(a.clone(), b.clone()),
            Reverse::<T>::semigroup_op(rb.clone(), ra.clone()).0
        );
        assert_eq!(
            T::semigroup_op(b.clone(), c.clone()),
            Reverse::<T>::semigroup_op(rc.clone(), rb.clone()).0
        );
        assert_eq!(
            T::semigroup_op(a.clone(), c.clone()),
            Reverse::<T>::semigroup_op(rc.clone(), ra.clone()).0
        );
    }
    pub fn assert_reverse_associative_law<T: Semigroup + Clone + PartialEq + Debug>(
        a: T,
        b: T,
        c: T,
    ) {
        let (ra, rb, rc) = (Reverse(a), Reverse(b), Reverse(c));
        assert_associative_law(ra, rb, rc);
    }
}
