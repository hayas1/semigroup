use std::ops::MulAssign;

use semigroup_derive::{CommutativeOp, SemigroupOp, semigroup_properties_doc};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that returns the product.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Prod, Construction, Semigroup};
///
/// let a = Prod(1);
/// let b = Prod(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 2);
/// ```
#[semigroup_properties_doc]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOp, CommutativeOp,
)]
#[cfg_attr(feature = "monoid", derive(semigroup_derive::MonoidOp))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(
    impl(semigroup = |base, other| *base *= other, monoid = Self(T::one())),
    bound(monoid = "T: num::One"),
    crate = crate,
)]
pub struct Prod<T: MulAssign>(pub T);

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

    use super::*;

    #[test]
    fn test_prod_semigroup() {
        let (a, b, c) = (Prod(1), Prod(2), Prod(3));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_prod_monoid() {
        let (a, b, c) = (Prod(1), Prod(2), Prod(3));
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    fn test_prod_commutative() {
        let (a, b, c) = (Prod(1), Prod(2), Prod(3));
        crate::assert_commutative!(a, b, c);
    }

    #[test]
    fn test_prod() {
        let (a, b) = (Prod(1), Prod(2));
        assert_eq!(a.semigroup(b).into_inner(), 2);
        assert_eq!(b.semigroup(a).into_inner(), 2);
    }
}
