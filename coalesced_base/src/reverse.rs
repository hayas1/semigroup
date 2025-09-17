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

    pub fn reverse<T: Semigroup + Clone>(a: T, b: T) -> (T, T) {
        let (ra, rb) = (Reversed(a.clone()), Reversed(b.clone()));
        (T::semigroup_op(a, b), Reversed::<T>::semigroup_op(rb, ra).0)
    }
    pub fn assert_reversed_associative_law<T: Semigroup + Clone + PartialEq + Debug>(
        a: T,
        b: T,
        c: T,
    ) {
        let (op, rev_op) = reverse(a.clone(), b.clone());
        assert_eq!(op, rev_op);
        let (op, rev_op) = reverse(b.clone(), c.clone());
        assert_eq!(op, rev_op);
        let (op, rev_op) = reverse(a.clone(), c.clone());
        assert_eq!(op, rev_op);

        let (ra, rb, rc) = (Reversed(a), Reversed(b), Reversed(c));
        assert_associative_law(ra, rb, rc);
    }
}
