use std::ops::AddAssign;

use semigroup_derive::{SemigroupOpPriv, properties_priv};

use crate::SemigroupOp;

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that returns the sum.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Sum, Construction, Semigroup};
///
/// let a = Sum(1);
/// let b = Sum(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(
    properties(monoid(identity = Self(T::zero())), commutative),
    bound(monoid = "T: num::Zero"),
)]
#[properties_priv]
pub struct Sum<T: AddAssign>(pub T);
impl<T: AddAssign> SemigroupOp<T> for Sum<T> {
    fn lift_op_assign(base: &mut T, other: T) {
        *base += other;
    }
}

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

    use super::*;

    #[test]
    fn test_sum_semigroup() {
        let (a, b, c) = (Sum(1), Sum(2), Sum(3));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_sum_monoid() {
        let (a, b, c) = (Sum(1), Sum(2), Sum(3));
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    fn test_sum_commutative() {
        let (a, b, c) = (Sum(1), Sum(2), Sum(3));
        crate::assert_commutative!(a, b, c);
    }

    #[test]
    fn test_sum() {
        let (a, b) = (Sum(1), Sum(2));
        assert_eq!(a.semigroup(b).into_inner(), 3);
        assert_eq!(b.semigroup(a).into_inner(), 3);
    }
}
