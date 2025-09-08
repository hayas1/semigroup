use coalesced_derive::ConstructionUse;

use crate::{op::Construction, reverse::Reversed, semigroup::Semigroup};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(op = Add)]
pub struct Added<T: std::ops::Add<Output = T>>(pub T);
impl<T: std::ops::Add<Output = T>> Semigroup for Added<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Added(base.0 + other.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::semigroup::tests::{assert_associative_law, assert_reversed_associative_law};

    use super::*;

    #[test]
    fn test_add_as_semigroup_op() {
        let (a, b, c) = (Added(1), Added(2), Added(3));
        assert_associative_law(a, b, c);
        assert_reversed_associative_law(a, b, c);
    }

    #[test]
    fn test_add() {
        let (a, b) = (Added(1), Added(2));
        assert_eq!(a.add(b).into_inner(), 3);

        let (ra, rb) = (Reversed(a), Reversed(b));
        assert_eq!(ra.add(rb).0.into_inner(), 3);
    }
}
