use crate::semigroup::Semigroup;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reverse<T>(pub T);

impl<T: Semigroup> Semigroup for Reverse<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Self(Semigroup::semigroup_op(other.0, base.0))
    }
}
