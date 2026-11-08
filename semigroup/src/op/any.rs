use semigroup_derive::{CommutativeOp, SelectiveOp, SemigroupOp, semigroup_properties_doc};

use crate::Selected;

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that returns `true` if either value is `true`.
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
#[semigroup_properties_doc]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Hash,
    SemigroupOp,
    SelectiveOp,
    CommutativeOp,
)]
#[cfg_attr(feature = "monoid", derive(semigroup_derive::MonoidOp))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(
    impl(
        semigroup = Self::lift_select_assign,
        selective = |base, other| if !base && *other { Selected::Other } else { Selected::Base },
        monoid = Self(false),
    ),
    crate = crate,
)]
pub struct Any(pub bool);

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
