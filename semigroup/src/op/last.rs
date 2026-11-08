use semigroup_derive::{SelectiveOp, SemigroupOp, semigroup_properties_doc};

use crate::Selected;

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that returns the last (second) value.
///
/// This is the dual of [`First`](crate::op::First): `Last::op(a, b) = First::op(b, a)`.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Last, Construction, Semigroup};
///
/// let a = Last(1);
/// let b = Last(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 2);
/// ```
#[semigroup_properties_doc]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOp, SelectiveOp,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(
    impl(semigroup = Self::lift_select_assign, selective = |_, _| Selected::Other),
    crate = crate,
)]
pub struct Last<T>(pub T);

#[cfg(test)]
mod tests {
    use crate::{Annotate, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_last_semigroup() {
        let (a, b, c) = (Last(1), Last(2), Last(3));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_last() {
        let (a, b) = (Last(Some(1)), Last(Some(2)));
        assert_eq!(a.semigroup(b).into_inner(), Some(2));
        assert_eq!(b.semigroup(a).into_inner(), Some(1));
    }

    #[test]
    fn test_last_annotated() {
        let a = Last(1).annotated("first");
        let b = Last(2).annotated("second");
        let ab = a.semigroup(b);
        assert_eq!(ab.value(), &Last(2));
        assert_eq!(ab.annotation(), &"second");
        let ba = b.semigroup(a);
        assert_eq!(ba.value(), &Last(1));
        assert_eq!(ba.annotation(), &"first");
    }
}
