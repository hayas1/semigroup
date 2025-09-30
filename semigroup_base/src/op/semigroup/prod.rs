use semigroup_derive::ConstructionUse;

use crate::{op::Construction, reverse::Reverse, semigroup::Semigroup};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(op_trait = ProdExt)]
pub struct Prod<T: std::ops::Mul<Output = T>>(pub T);
impl<T: std::ops::Mul<Output = T>> Semigroup for Prod<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Prod(base.0 * other.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::semigroup::tests::assert_semigroup_op;

    use super::*;

    #[test]
    fn test_prod_as_semigroup_op() {
        let (a, b, c) = (Prod(1), Prod(2), Prod(3));
        assert_semigroup_op!(a, b, c);
    }

    #[test]
    fn test_prod() {
        let (a, b) = (Prod(1), Prod(2));
        assert_eq!(a.prod(b).into_inner(), 3);

        let (ra, rb) = (Reverse(a), Reverse(b));
        assert_eq!(ra.prod(rb).0.into_inner(), 3);
    }
}
