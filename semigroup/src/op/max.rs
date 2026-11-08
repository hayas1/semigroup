use semigroup_derive::{CommutativeOp, SelectiveOp, SemigroupOp, semigroup_properties_doc};

use crate::Selected;

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that returns the maximum value.
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
        selective = |base, other| if base < other { Selected::Other } else { Selected::Base },
        monoid = Self(T::min_value()),
    ),
    bound(monoid = "T: num::Bounded"),
    crate = crate,
)]
pub struct Max<T: Ord>(pub T);

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
