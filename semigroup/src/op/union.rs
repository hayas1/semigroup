use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

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

/// A [`Semigroup`](crate::Semigroup) [construction](crate::Construction) that union two maps.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::UnionMap, Construction, Semigroup};
///
/// let a = UnionMap(vec![("one", 1), ("two",2)].into_iter().collect());
/// let b = UnionMap(vec![("three", 3), ("four", 4)].into_iter().collect());
///
/// assert_eq!(a.semigroup(b).into_inner(), vec![("one", 1), ("two",2), ("three", 3), ("four", 4)].into_iter().collect());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(monoid, commutative, identity = Self(HashMap::new()))]
#[properties_priv(monoid, commutative)]
pub struct UnionMap<K: Eq + Hash, V>(pub HashMap<K, V>);
impl<K: Eq + Hash, V> Semigroup for UnionMap<K, V> {
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
    fn test_union_map_semigroup() {
        let (a, b, c) = (
            UnionMap(vec![("one", 1), ("two", 2)].into_iter().collect()),
            UnionMap(vec![("three", 3), ("four", 4)].into_iter().collect()),
            UnionMap(vec![("five", 5), ("six", 6)].into_iter().collect()),
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
    #[cfg(feature = "monoid")]
    fn test_union_map_monoid() {
        let (a, b, c) = (
            UnionMap(vec![("one", 1), ("two", 2)].into_iter().collect()),
            UnionMap(vec![("three", 3), ("four", 4)].into_iter().collect()),
            UnionMap(vec![("five", 5), ("six", 6)].into_iter().collect()),
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
    #[test]
    fn test_union_map() {
        let (a, b) = (
            UnionMap(vec![("one", 1), ("two", 2)].into_iter().collect()),
            UnionMap(vec![("three", 3), ("four", 4)].into_iter().collect()),
        );
        assert_eq!(
            a.clone().semigroup(b.clone()).into_inner(),
            vec![("one", 1), ("two", 2), ("three", 3), ("four", 4)]
                .into_iter()
                .collect()
        );
        assert_eq!(
            b.semigroup(a).into_inner(),
            vec![("three", 3), ("four", 4), ("one", 1), ("two", 2)]
                .into_iter()
                .collect()
        );
    }
}
