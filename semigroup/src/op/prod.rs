use std::ops::MulAssign;

use semigroup_derive::{OpPriv, properties_priv};

use crate::Op;

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::Op) that returns the product.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, OpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[op(monoid, commutative, identity = Self(T::one()), monoid_where = "T: num::One")]
#[properties_priv(monoid, commutative, monoid_where = "T: num::One")]
pub struct Prod<T: MulAssign>(pub T);
impl<T: MulAssign> Op<T> for Prod<T> {
    fn lift_op_assign(base: &mut T, other: T) {
        *base *= other;
    }
}

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
