use semigroup_derive::{OpPriv, properties_priv};

use crate::{IdempotentOp, Selected};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::Op) that returns the second value.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Overwrite, Construction, Semigroup};
///
/// let a = Overwrite(1);
/// let b = Overwrite(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, OpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[op(idempotent)]
#[properties_priv(idempotent)]
pub struct Overwrite<T>(pub T);
impl<T> IdempotentOp<T> for Overwrite<T> {
    fn lift_select(_base: &T, _other: &T) -> Selected {
        Selected::Other
    }
}

#[cfg(test)]
mod tests {
    use crate::{Annotate, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_overwrite_semigroup() {
        let (a, b, c) = (Overwrite(1), Overwrite(2), Overwrite(3));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_overwrite() {
        let (a, b) = (Overwrite(Some(1)), Overwrite(Some(2)));
        assert_eq!(a.semigroup(b).into_inner(), Some(2));
        assert_eq!(b.semigroup(a).into_inner(), Some(1));
    }

    #[test]
    fn test_overwrite_annotated() {
        let a = Overwrite(1).annotated("first");
        let b = Overwrite(2).annotated("second");
        let ab = a.semigroup(b);
        assert_eq!(ab.value(), &Overwrite(2));
        assert_eq!(ab.annotation(), &"second");
        let ba = b.semigroup(a);
        assert_eq!(ba.value(), &Overwrite(1));
        assert_eq!(ba.annotation(), &"first");
    }
}
