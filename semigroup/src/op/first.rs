use semigroup_derive::{OpPriv, properties_priv};

use crate::{Annotated, AnnotatedOp};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::Op) that returns the first value.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, OpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[op(annotated)]
#[properties_priv(annotated)]
pub struct First<T>(pub T);
impl<T, A> AnnotatedOp<T, A> for First<T> {
    fn lift_annotated_op_assign(_base: Annotated<&mut T, &mut A>, _other: Annotated<T, A>) {}
}

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

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
}
