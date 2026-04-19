use semigroup_derive::{OpPriv, properties_priv};

use crate::{IdempotentOp, Selected};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::Op) that returns `true` if either value is `true`.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Any, Construction, Semigroup};
///
/// let a = Any(true);
/// let b = Any(false);
///
/// assert_eq!(a.semigroup(b).into_inner(), true);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, OpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[op(idempotent, monoid, commutative, identity = Self(false))]
#[properties_priv(idempotent, monoid, commutative)]
pub struct Any(pub bool);
impl IdempotentOp<bool> for Any {
    fn lift_select(base: &bool, other: &bool) -> Selected {
        if !base && *other { Selected::Other } else { Selected::Base }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Annotate, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_any_semigroup() {
        let (a, b, c) = (Any(false), Any(true), Any(false));
        crate::assert_semigroup!(a, b, c);

        let (a, b, c) = (Any(false), Any(false), Any(false));
        crate::assert_semigroup!(a, b, c);

        let (a, b, c) = (Any(true), Any(true), Any(true));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_any_monoid() {
        let (a, b, c) = (Any(false), Any(true), Any(false));
        crate::assert_monoid!(a, b, c);

        let (a, b, c) = (Any(false), Any(false), Any(false));
        crate::assert_monoid!(a, b, c);

        let (a, b, c) = (Any(true), Any(true), Any(true));
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    fn test_any_commutative() {
        let (a, b, c) = (Any(false), Any(true), Any(false));
        crate::assert_commutative!(a, b, c);

        let (a, b, c) = (Any(false), Any(false), Any(false));
        crate::assert_commutative!(a, b, c);

        let (a, b, c) = (Any(true), Any(true), Any(true));
        crate::assert_commutative!(a, b, c);
    }

    #[test]
    fn test_any() {
        let (a, b, c) = (Any(false), Any(true), Any(false));
        assert!(a.semigroup(b).semigroup(c).into_inner());

        let (a, b, c) = (Any(false), Any(false), Any(false));
        assert!(!a.semigroup(b).semigroup(c).into_inner());

        let (a, b, c) = (Any(true), Any(true), Any(true));
        assert!(a.semigroup(b).semigroup(c).into_inner());
    }

    #[test]
    fn test_any_annotated() {
        let a = Any(false).annotated(0);
        let b = Any(true).annotated(1);
        let c = Any(false).annotated(2);
        let d = Any(true).annotated(3);
        assert_eq!(a.semigroup(b).semigroup(c).semigroup(d), b);
        assert_eq!(d.semigroup(c).semigroup(b).semigroup(a), d);
        assert_eq!(a.semigroup(c), a);
        assert_eq!(b.semigroup(d), b);
    }
}
