use semigroup_derive::{SelectiveOp, SemigroupOp, semigroup_properties_doc};

use crate::Selected;

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that returns the first value.
///
/// This is the dual of [`Last`](crate::op::Last): `Last::op(a, b) = First::op(b, a)`.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::First, Construction, Semigroup};
///
/// let a = First(1);
/// let b = First(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 1);
/// ```
#[semigroup_properties_doc]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOp, SelectiveOp,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(
    impl(semigroup = Self::lift_select_assign, selective = |_, _| Selected::Base),
    crate = crate,
)]
pub struct First<T>(pub T);

#[cfg(test)]
mod tests {
    use crate::{Annotate, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_first_semigroup() {
        let (a, b, c) = (First(1), First(2), First(3));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_first() {
        let (a, b) = (First(Some(1)), First(Some(2)));
        assert_eq!(a.semigroup(b).into_inner(), Some(1));
        assert_eq!(b.semigroup(a).into_inner(), Some(2));
    }

    #[test]
    fn test_first_annotated() {
        let a = First(1).annotated("first");
        let b = First(2).annotated("second");
        let ab = a.semigroup(b);
        assert_eq!(ab.value(), &First(1));
        assert_eq!(ab.annotation(), &"first");
        let ba = b.semigroup(a);
        assert_eq!(ba.value(), &First(2));
        assert_eq!(ba.annotation(), &"second");
    }
}
