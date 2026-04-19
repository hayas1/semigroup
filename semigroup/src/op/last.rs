use semigroup_derive::{OpPriv, properties_priv};

use crate::{Annotated, AnnotatedOp};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::Op) that returns the last (second) value.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, OpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[op(annotated)]
#[properties_priv(annotated)]
pub struct Last<T>(pub T);
impl<T, A> AnnotatedOp<T, A> for Last<T> {
    fn lift_annotated_op_assign(mut base: Annotated<&mut T, &mut A>, other: Annotated<T, A>) {
        // Dual of First: Last::op(a, b) = First::op(b, a)
        // Swap arguments, then apply First (which keeps its first argument)
        let swapped = base.replace(other);
        super::first::First::lift_annotated_op_assign(base, swapped);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

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
}
