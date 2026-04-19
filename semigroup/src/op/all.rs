use semigroup_derive::{OpPriv, properties_priv};

use crate::{IdempotentOp, Selected};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::Op) that returns `true` if both values are `true`.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::All, Construction, Semigroup};
///
/// let a = All(true);
/// let b = All(false);
///
/// assert_eq!(a.semigroup(b).into_inner(), false);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, OpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[op(idempotent, monoid, commutative, identity = Self(true))]
#[properties_priv(idempotent, monoid, commutative)]
pub struct All(pub bool);
impl IdempotentOp<bool> for All {
    fn lift_select(base: &bool, other: &bool) -> Selected {
        if *base && !other { Selected::Other } else { Selected::Base }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Annotate, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_all_semigroup() {
        let (a, b, c) = (All(false), All(true), All(false));
        crate::assert_semigroup!(a, b, c);

        let (a, b, c) = (All(false), All(false), All(false));
        crate::assert_semigroup!(a, b, c);

        let (a, b, c) = (All(true), All(true), All(true));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_all_monoid() {
        let (a, b, c) = (All(false), All(true), All(false));
        crate::assert_monoid!(a, b, c);

        let (a, b, c) = (All(false), All(false), All(false));
        crate::assert_monoid!(a, b, c);

        let (a, b, c) = (All(true), All(true), All(true));
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    fn test_all_commutative() {
        let (a, b, c) = (All(false), All(true), All(false));
        crate::assert_commutative!(a, b, c);

        let (a, b, c) = (All(false), All(false), All(false));
        crate::assert_commutative!(a, b, c);

        let (a, b, c) = (All(true), All(true), All(true));
        crate::assert_commutative!(a, b, c);
    }

    #[test]
    fn test_all() {
        let (a, b, c) = (All(false), All(true), All(false));
        assert!(!a.semigroup(b).semigroup(c).into_inner());

        let (a, b, c) = (All(false), All(false), All(false));
        assert!(!a.semigroup(b).semigroup(c).into_inner());

        let (a, b, c) = (All(true), All(true), All(true));
        assert!(a.semigroup(b).semigroup(c).into_inner());
    }

    #[test]
    fn test_all_annotated() {
        let a = All(false).annotated(0);
        let b = All(true).annotated(1);
        let c = All(false).annotated(2);
        let d = All(true).annotated(3);
        assert_eq!(a.semigroup(b).semigroup(c).semigroup(d), a);
        assert_eq!(d.semigroup(c).semigroup(b).semigroup(a), c);
        assert_eq!(a.semigroup(c), a);
        assert_eq!(b.semigroup(d), b);
    }
}
