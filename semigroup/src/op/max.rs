use semigroup_derive::{OpPriv, properties_priv};

use crate::{IdempotentOp, Selected};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::Op) that returns the maximum value.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Max, Construction, Semigroup};
///
/// let a = Max(1);
/// let b = Max(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, OpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[op(idempotent, monoid, commutative, identity = Self(T::min_value()), monoid_where = "T: num::Bounded")]
#[properties_priv(idempotent, monoid, commutative, monoid_where = "T: num::Bounded")]
pub struct Max<T: Ord>(pub T);
impl<T: Ord> IdempotentOp<T> for Max<T> {
    fn lift_select(base: &T, other: &T) -> Selected {
        if base < other { Selected::Other } else { Selected::Base }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Annotate, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_max_semigroup() {
        let (a, b, c) = (Max(1), Max(2), Max(3));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_max_monoid() {
        let (a, b, c) = (Max(1), Max(2), Max(3));
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    fn test_max_commutative() {
        let (a, b, c) = (Max(1), Max(2), Max(3));
        crate::assert_commutative!(a, b, c);
    }

    #[test]
    fn test_max() {
        let (a, b) = (Max(1), Max(2));
        assert_eq!(a.semigroup(b).into_inner(), 2);
        assert_eq!(b.semigroup(a).into_inner(), 2);
    }

    #[test]
    fn test_max_annotated() {
        let a = Max(1).annotated("first");
        let b = Max(2).annotated("second");
        let ab = a.semigroup(b);
        assert_eq!(ab.value(), &Max(2));
        assert_eq!(ab.annotation(), &"second");
        let ba = b.semigroup(a);
        assert_eq!(ba.value(), &Max(2));
        assert_eq!(ba.annotation(), &"second");
    }
}
