use semigroup_derive::{SemigroupOpPriv, properties_priv};

use crate::{IdempotentOp, Selected};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that returns the first non-`None` value.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(idempotent, monoid, identity = Self(None))]
#[properties_priv(idempotent, monoid)]
pub struct Coalesce<T>(pub Option<T>);
impl<T> IdempotentOp<Option<T>> for Coalesce<T> {
    fn lift_select(base: &Option<T>, other: &Option<T>) -> Selected {
        match (base, other) {
            (None, Some(_)) => Selected::Other,
            _ => Selected::Base,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Annotate, Construction, Semigroup};

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

    #[test]
    fn test_coalesce_annotated() {
        let a = Coalesce(Some(1)).annotated("first");
        let b = Coalesce(None).annotated("second");
        let c = Coalesce(Some(3)).annotated("third");

        let ab = a.semigroup(b);
        assert_eq!(ab.value(), &Coalesce(Some(1)));
        assert_eq!(ab.annotation(), &"first");

        let bc = b.semigroup(c);
        assert_eq!(bc.value(), &Coalesce(Some(3)));
        assert_eq!(bc.annotation(), &"third");

        let ca = c.semigroup(a);
        assert_eq!(ca.value(), &Coalesce(Some(3)));
        assert_eq!(ca.annotation(), &"third");
    }
}
