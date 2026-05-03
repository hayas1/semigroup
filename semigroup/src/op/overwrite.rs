use semigroup_derive::{SemigroupOpPriv, properties_priv};

use crate::{IdempotentOp, Selected};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that returns the last non-`None` value.
///
/// This is the dual of [`Coalesce`](crate::op::Coalesce): `Overwrite::op(a, b) = Coalesce::op(b, a)`.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Overwrite, Construction, Semigroup};
///
/// let a = Overwrite(Some(1));
/// let b = Overwrite(None);
///
/// assert_eq!(a.semigroup(b).into_inner(), Some(1));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(idempotent, monoid, identity = Self(None))]
#[properties_priv(idempotent, monoid)]
pub struct Overwrite<T>(pub Option<T>);
impl<T> IdempotentOp<Option<T>> for Overwrite<T> {
    fn lift_select(base: &Option<T>, other: &Option<T>) -> Selected {
        match (base, other) {
            (Some(_), None) => Selected::Base,
            _ => Selected::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Annotate, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_overwrite_semigroup() {
        let (a, b, c) = (Overwrite(Some(1)), Overwrite(Some(2)), Overwrite(Some(3)));
        crate::assert_semigroup!(a, b, c);
        let (a, b, c) = (Overwrite(None), Overwrite(Some(2)), Overwrite(Some(3)));
        crate::assert_semigroup!(a, b, c);
        let (a, b, c) = (Overwrite(None), Overwrite(Some(2)), Overwrite(None));
        crate::assert_semigroup!(a, b, c);
        let (a, b, c) = (Overwrite::<u32>(None), Overwrite(None), Overwrite(None));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_overwrite_monoid() {
        let (a, b, c) = (Overwrite(Some(1)), Overwrite(Some(2)), Overwrite(Some(3)));
        crate::assert_monoid!(a, b, c);
        let (a, b, c) = (Overwrite(None), Overwrite(Some(2)), Overwrite(Some(3)));
        crate::assert_monoid!(a, b, c);
        let (a, b, c) = (Overwrite(None), Overwrite(Some(2)), Overwrite(None));
        crate::assert_monoid!(a, b, c);
        let (a, b, c) = (Overwrite::<u32>(None), Overwrite(None), Overwrite(None));
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    fn test_overwrite() {
        let (a, b) = (Overwrite(None), Overwrite(Some("value")));
        assert_eq!(a.semigroup(b).into_inner(), Some("value"));
        assert_eq!(b.semigroup(a).into_inner(), Some("value"));

        let (a, b) = (Overwrite(Some(1)), Overwrite(Some(2)));
        assert_eq!(a.semigroup(b).into_inner(), Some(2));
        assert_eq!(b.semigroup(a).into_inner(), Some(1));
    }

    #[test]
    fn test_overwrite_annotated() {
        let a = Overwrite(Some(1)).annotated("first");
        let b = Overwrite(None).annotated("second");
        let c = Overwrite(Some(3)).annotated("third");

        let ab = a.semigroup(b);
        assert_eq!(ab.value(), &Overwrite(Some(1)));
        assert_eq!(ab.annotation(), &"first");

        let bc = b.semigroup(c);
        assert_eq!(bc.value(), &Overwrite(Some(3)));
        assert_eq!(bc.annotation(), &"third");

        let ca = c.semigroup(a);
        assert_eq!(ca.value(), &Overwrite(Some(1)));
        assert_eq!(ca.annotation(), &"first");
    }
}
