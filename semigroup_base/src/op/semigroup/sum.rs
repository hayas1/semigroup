use semigroup_derive::ConstructionUse;

use crate::{op::Construction, reverse::Reverse, semigroup::Semigroup};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(op_trait = SumExt)]
pub struct Sum<T: std::ops::Add<Output = T>>(pub T);
impl<T: std::ops::Add<Output = T>> Semigroup for Sum<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Sum(base.0 + other.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::semigroup::tests::assert_semigroup_op;

    use super::*;

    #[test]
    fn test_sum_as_semigroup_op() {
        let (a, b, c) = (Sum(1), Sum(2), Sum(3));
        assert_semigroup_op!(a, b, c);
    }

    #[test]
    fn test_sum() {
        let (a, b) = (Sum(1), Sum(2));
        assert_eq!(a.sum(b).into_inner(), 3);

        let (ra, rb) = (Reverse(a), Reverse(b));
        assert_eq!(ra.sum(rb).0.into_inner(), 3);
    }
}
