use semigroup_derive::{OpPriv, properties_priv};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::Op) that concatenates two values.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Concat, Construction, Semigroup};
///
/// let a = Concat(vec![1, 2]);
/// let b = Concat(vec![3, 4]);
///
/// assert_eq!(a.semigroup(b).into_inner(), vec![1, 2, 3, 4]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, OpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[op(monoid, identity = Self(Default::default()))]
#[properties_priv(monoid)]
pub struct Concat<T: Default + Extend<T::Item> + IntoIterator>(pub T);
impl<T: Default + Extend<T::Item> + IntoIterator> crate::Op<T> for Concat<T> {
    fn lift_op_assign(base: &mut T, other: T) {
        base.extend(other);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

    use super::*;

    #[test]
    fn test_concat_semigroup() {
        let (a, b, c) = (Concat(vec![1]), Concat(vec![2]), Concat(vec![3]));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_concat_monoid() {
        let (a, b, c) = (Concat(vec![1]), Concat(vec![2]), Concat(vec![3]));
        crate::assert_monoid!(a, b, c)
    }

    #[test]
    fn test_concat() {
        let (a, b) = (Concat(vec![1]), Concat(vec![2]));
        assert_eq!(a.clone().semigroup(b.clone()).into_inner(), vec![1, 2]);
        assert_eq!(b.semigroup(a).into_inner(), vec![2, 1]);
    }
}
