use crate::semigroup::Semigroup;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reversed<T>(pub T);

impl<T: Semigroup> Semigroup for Reversed<T> {
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
        let (ra, rb, rc) = (
            Reversed(a.clone()),
            Reversed(b.clone()),
            Reversed(c.clone()),
        );
        assert_eq!(
            T::semigroup_op(a.clone(), b.clone()),
            Reversed::<T>::semigroup_op(rb.clone(), ra.clone()).0
        );
        assert_eq!(
            T::semigroup_op(b.clone(), c.clone()),
            Reversed::<T>::semigroup_op(rc.clone(), rb.clone()).0
        );
        assert_eq!(
            T::semigroup_op(a.clone(), c.clone()),
            Reversed::<T>::semigroup_op(rc.clone(), ra.clone()).0
        );
    }
    pub fn assert_reversed_associative_law<T: Semigroup + Clone + PartialEq + Debug>(
        a: T,
        b: T,
        c: T,
    ) {
        let (ra, rb, rc) = (Reversed(a), Reversed(b), Reversed(c));
        assert_associative_law(ra, rb, rc);
    }
}
