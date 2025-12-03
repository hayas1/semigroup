use semigroup_derive::{OpPriv, properties_priv};

use crate::{Annotated, AnnotatedOp};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::Op) that returns the minimum value.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Min, Construction, Semigroup};
///
/// let a = Min(1);
/// let b = Min(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, OpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[op(annotated, monoid, commutative, identity = Self(T::max_value()), monoid_where = "T: num::Bounded")]
#[properties_priv(annotated, monoid, commutative, monoid_where = "T: num::Bounded")]
pub struct Min<T: Ord>(pub T);
impl<T: Ord, A> AnnotatedOp<T, A> for Min<T> {
    fn lift_annotated_op_assign(mut base: Annotated<&mut T, &mut A>, mut other: Annotated<T, A>) {
        if base.value() > &other.value_mut() {
            base.replace(other);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

    use super::*;

    #[test]
    fn test_min_semigroup() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_min_monoid() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    fn test_min_commutative() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        crate::assert_commutative!(a, b, c);
    }

    #[test]
    fn test_min() {
        let (a, b) = (Min(1), Min(2));
        assert_eq!(a.semigroup(b).into_inner(), 1);
        assert_eq!(b.semigroup(a).into_inner(), 1);
    }
}
