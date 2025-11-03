use semigroup_derive::{ConstructionPriv, properties_priv};

use crate::{Annotated, AnnotatedSemigroup};

/// A [`Semigroup`](crate::Semigroup) [construction](crate::Construction) that returns `true` if either value is `true`.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(annotated, monoid, commutative, identity = Self(false))]
#[properties_priv(annotated, monoid, commutative)]
pub struct Any(pub bool);
impl<A> AnnotatedSemigroup<A> for Any {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        std::cmp::max_by(base, other, |a, b| a.value().cmp(b.value()))
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
        assert_eq!(a.semigroup(b).semigroup(c).semigroup(d), d);
        assert_eq!(d.semigroup(c).semigroup(b).semigroup(a), b);
        assert_eq!(a.semigroup(c), c);
        assert_eq!(b.semigroup(d), d);
    }
}
