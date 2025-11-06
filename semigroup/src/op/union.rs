use std::{collections::HashSet, hash::Hash};

use semigroup_derive::{ConstructionPriv, properties_priv};

use crate::Semigroup;

/// A [`Semigroup`](crate::Semigroup) [construction](crate::Construction) that union two sets.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Union, Construction, Semigroup};
///
/// let a = Union(vec![1, 2].into_iter().collect());
/// let b = Union(vec![3, 4].into_iter().collect());
///
/// assert_eq!(a.semigroup(b).into_inner(), vec![1, 2, 3, 4].into_iter().collect());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(monoid, commutative, identity = Self(HashSet::new()))]
#[properties_priv(monoid, commutative)]
pub struct Union<T: Eq + Hash>(pub HashSet<T>);
impl<T: Eq + Hash> Semigroup for Union<T> {
    fn op(base: Self, other: Self) -> Self {
        let (Self(mut b), Self(o)) = (base, other);
        b.extend(o);
        Self(b)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

    use super::*;

    #[test]
    fn test_union_semigroup() {
        let (a, b, c) = (
            Union(vec![1].into_iter().collect()),
            Union(vec![2].into_iter().collect()),
            Union(vec![3].into_iter().collect()),
        );
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_union_monoid() {
        let (a, b, c) = (
            Union(vec![1].into_iter().collect()),
            Union(vec![2].into_iter().collect()),
            Union(vec![3].into_iter().collect()),
        );
        crate::assert_monoid!(a, b, c)
    }

    #[test]
    fn test_union() {
        let (a, b) = (
            Union(vec![1].into_iter().collect()),
            Union(vec![2].into_iter().collect()),
        );
        assert_eq!(
            a.clone().semigroup(b.clone()).into_inner(),
            vec![1, 2].into_iter().collect()
        );
        assert_eq!(
            b.semigroup(a).into_inner(),
            vec![2, 1].into_iter().collect()
        );
    }
}
