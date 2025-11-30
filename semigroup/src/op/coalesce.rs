use semigroup_derive::{ConstructionPriv, properties_priv};

use crate::{Annotated, AnnotatedOp};

/// A [`Semigroup`](crate::Semigroup) [construction](crate::Construction) that returns the first non-`None` value.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Coalesce, Construction, Semigroup};
///
/// let a = Coalesce(None);
/// let b = Coalesce(Some(2));
///
/// assert_eq!(a.semigroup(b).into_inner(), Some(2));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(annotated, monoid, identity = Self(None))]
#[properties_priv(annotated, monoid)]
pub struct Coalesce<T>(pub Option<T>);
impl<T, A> AnnotatedOp<Option<T>, A> for Coalesce<T> {
    fn lift_annotated_op_assign(
        mut base: Annotated<&mut Option<T>, &mut A>,
        other: Annotated<Option<T>, A>,
    ) {
        match (&base.value(), &other.value()) {
            (Some(_), _) | (None, None) => {}
            (None, Some(_)) => {
                base.replace(other);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

    use super::*;

    #[test]
    fn test_coalesce_semigroup() {
        let (a, b, c) = (Coalesce(Some(1)), Coalesce(Some(2)), Coalesce(Some(3)));
        crate::assert_semigroup!(a, b, c);
        let (a, b, c) = (Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3)));
        crate::assert_semigroup!(a, b, c);
        let (a, b, c) = (Coalesce(None), Coalesce(Some(2)), Coalesce(None));
        crate::assert_semigroup!(a, b, c);
        let (a, b, c) = (Coalesce::<u32>(None), Coalesce(None), Coalesce(None));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_coalesce_monoid() {
        let (a, b, c) = (Coalesce(Some(1)), Coalesce(Some(2)), Coalesce(Some(3)));
        crate::assert_monoid!(a, b, c);
        let (a, b, c) = (Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3)));
        crate::assert_monoid!(a, b, c);
        let (a, b, c) = (Coalesce(None), Coalesce(Some(2)), Coalesce(None));
        crate::assert_monoid!(a, b, c);
        let (a, b, c) = (Coalesce::<u32>(None), Coalesce(None), Coalesce(None));
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    fn test_coalesce() {
        let (a, b) = (Coalesce(None), Coalesce(Some("value")));
        assert_eq!(a.semigroup(b).into_inner(), Some("value"));
        assert_eq!(b.semigroup(a).into_inner(), Some("value"));

        let (a, b) = (Coalesce(Some(1)), Coalesce(Some(2)));
        assert_eq!(a.semigroup(b).into_inner(), Some(1));
        assert_eq!(b.semigroup(a).into_inner(), Some(2));
    }
}
