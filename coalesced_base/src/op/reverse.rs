use crate::semigroup::Semigroup;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reverse<T>(pub T);

impl<T: Semigroup> Semigroup for Reverse<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Self(Semigroup::semigroup_op(other.0, base.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::op::annotation::replace::Replaced;

    use super::*;

    #[test]
    fn test_reverse() {
        let a = Replaced(1);
        let b = Replaced(2);
        let result = Semigroup::semigroup_op(a, b);
        assert_eq!(result.0, 2);

        let a = Reverse(Replaced(1));
        let b = Reverse(Replaced(2));
        let result = Semigroup::semigroup_op(a, b);
        assert_eq!(result.0 .0, 1);
    }

    #[test]
    fn test_associative() {
        let a = Replaced(1);
        let b = Replaced(2);
        let c = Replaced(3);

        let left = Semigroup::semigroup_op(Semigroup::semigroup_op(a, b), c);
        let right = Semigroup::semigroup_op(a, Semigroup::semigroup_op(b, c));

        assert_eq!(left.0, right.0);

        let a = Reverse(Replaced(1));
        let b = Reverse(Replaced(2));
        let c = Reverse(Replaced(3));

        let left = Semigroup::semigroup_op(Semigroup::semigroup_op(a, b), c);
        let right = Semigroup::semigroup_op(a, Semigroup::semigroup_op(b, c));

        assert_eq!(left.0 .0, right.0 .0);
    }
}
