use semigroup_derive::{ConstructionPriv, properties_priv};

use crate::{Annotated, AnnotatedSemigroup};

/// A [`Semigroup`](crate::Semigroup) [construction](crate::Construction) that returns `true` if both values are `true`.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(annotated, monoid, commutative, identity = Self(true))]
#[properties_priv(annotated, monoid, commutative)]
pub struct All(pub bool);
impl<A> AnnotatedSemigroup<A> for All {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        std::cmp::min_by(base, other, |a, b| a.value().cmp(b.value()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

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
}
