use coalesced_derive::ConstructionUse;

use crate::{op::Construction, reverse::Reverse, semigroup::Semigroup};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(op_trait = AddExt)]
pub struct Add<T: std::ops::Add<Output = T>>(pub T);
impl<T: std::ops::Add<Output = T>> Semigroup for Add<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Add(base.0 + other.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::semigroup::tests::assert_semigroup_op;

    use super::*;

    #[test]
    fn test_add_as_semigroup_op() {
        let (a, b, c) = (Add(1), Add(2), Add(3));
        assert_semigroup_op!(a, b, c);
    }

    #[test]
    fn test_add() {
        let (a, b) = (Add(1), Add(2));
        assert_eq!(a.add(b).into_inner(), 3);

        let (ra, rb) = (Reverse(a), Reverse(b));
        assert_eq!(ra.add(rb).0.into_inner(), 3);
    }
}
