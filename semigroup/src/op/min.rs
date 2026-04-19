use semigroup_derive::{OpPriv, properties_priv};

use crate::{IdempotentOp, Selected};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::Op) that returns the minimum value.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Min, Construction, Semigroup};
///
/// let a = Min(1);
/// let b = Min(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, OpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[op(idempotent, monoid, commutative, identity = Self(T::max_value()), monoid_where = "T: num::Bounded")]
#[properties_priv(idempotent, monoid, commutative, monoid_where = "T: num::Bounded")]
pub struct Min<T: Ord>(pub T);
impl<T: Ord> IdempotentOp<T> for Min<T> {
    fn lift_select(base: &T, other: &T) -> Selected {
        if base > other { Selected::Other } else { Selected::Base }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Annotate, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_min_semigroup() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_min_monoid() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    fn test_min_commutative() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        crate::assert_commutative!(a, b, c);
    }

    #[test]
    fn test_min() {
        let (a, b) = (Min(1), Min(2));
        assert_eq!(a.semigroup(b).into_inner(), 1);
        assert_eq!(b.semigroup(a).into_inner(), 1);
    }

    #[test]
    fn test_min_annotated() {
        let a = Min(1).annotated("first");
        let b = Min(2).annotated("second");
        let ab = a.semigroup(b);
        assert_eq!(ab.value(), &Min(1));
        assert_eq!(ab.annotation(), &"first");
        let ba = b.semigroup(a);
        assert_eq!(ba.value(), &Min(1));
        assert_eq!(ba.annotation(), &"first");
    }
}
