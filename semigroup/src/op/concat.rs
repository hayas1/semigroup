use semigroup_derive::{SemigroupOp, semigroup_properties_doc};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that concatenates two values.
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
#[semigroup_properties_doc]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOp)]
#[cfg_attr(feature = "monoid", derive(semigroup_derive::MonoidOp))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(
    impl(semigroup = |base, other| base.extend(other), monoid = Self(Default::default())),
    crate = crate,
)]
pub struct Concat<T: Default + Extend<T::Item> + IntoIterator>(pub T);

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
