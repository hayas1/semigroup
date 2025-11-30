use semigroup_derive::{ConstructionPriv, properties_priv};

use crate::{Annotated, AnnotatedOp};

/// A [`Semigroup`](crate::Semigroup) [construction](crate::Construction) that returns the maximum value.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(annotated, monoid, commutative, identity = Self(T::min_value()), monoid_where = "T: num::Bounded")]
#[properties_priv(annotated, monoid, commutative, monoid_where = "T: num::Bounded")]
pub struct Max<T: Ord>(pub T);
impl<T: Ord, A> AnnotatedOp<T, A> for Max<T> {
    fn lift_annotated_op_assign(mut base: Annotated<&mut T, &mut A>, mut other: Annotated<T, A>) {
        if base.value() < &other.value_mut() {
            base.replace(other);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

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
}
