use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use semigroup_derive::{SemigroupOp, semigroup_properties_doc};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that union two sets.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Union, Construction, Semigroup};
///
/// let a = Union(vec![1, 2, 3].into_iter().collect());
/// let b = Union(vec![2, 3, 4].into_iter().collect());
///
/// assert_eq!(a.semigroup(b).into_inner(), vec![1, 2, 3, 4].into_iter().collect());
/// ```
#[semigroup_properties_doc]
#[derive(Debug, Clone, PartialEq, Eq, Default, SemigroupOp)]
#[cfg_attr(feature = "monoid", derive(semigroup_derive::MonoidOp))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(
    impl(semigroup = |base, other| base.extend(other), monoid = Self(HashSet::new())),
    crate = crate,
)]
pub struct Union<T: Eq + Hash>(pub HashSet<T>);

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that union two maps.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::UnionMap, Construction, Semigroup};
///
/// let a = UnionMap(vec![("one", 1), ("two",2), ("three", 3)].into_iter().collect());
/// let b = UnionMap(vec![("two", 22), ("three", 33), ("four", 44)].into_iter().collect());
///
/// assert_eq!(a.semigroup(b).into_inner(), vec![("one", 1), ("two",2), ("three", 3), ("four", 44)].into_iter().collect());
/// ```
#[semigroup_properties_doc]
#[derive(Debug, Clone, PartialEq, Eq, Default, SemigroupOp)]
#[cfg_attr(feature = "monoid", derive(semigroup_derive::MonoidOp))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(
    impl(
        semigroup = |base, other| other.into_iter().for_each(|(k, v)| {
            base.entry(k).or_insert(v);
        }),
        monoid = Self(HashMap::new()),
    ),
    crate = crate,
)]
pub struct UnionMap<K: Eq + Hash, V>(pub HashMap<K, V>);

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
        let (a, b, aa) = (
            Union(vec![1].into_iter().collect()),
            Union(vec![2].into_iter().collect()),
            Union(vec![1].into_iter().collect()),
        );
        assert_eq!(
            a.clone().semigroup(b.clone()).into_inner(),
            vec![1, 2].into_iter().collect()
        );
        assert_eq!(
            a.clone()
                .semigroup(b.clone())
                .semigroup(aa.clone())
                .into_inner(),
            vec![1, 2].into_iter().collect()
        );
    }
    #[test]
    fn test_union_map() {
        let (a, b, aa) = (
            UnionMap(vec![("one", 1), ("two", 2)].into_iter().collect()),
            UnionMap(vec![("three", 3), ("four", 4)].into_iter().collect()),
            UnionMap(vec![("one", 11), ("two", 22)].into_iter().collect()),
        );
        assert_eq!(
            a.clone().semigroup(b.clone()).into_inner(),
            vec![("one", 1), ("two", 2), ("three", 3), ("four", 4)]
                .into_iter()
                .collect()
        );
        assert_eq!(
            a.clone()
                .semigroup(b.clone())
                .semigroup(aa.clone())
                .into_inner(),
            vec![("one", 1), ("two", 2), ("three", 3), ("four", 4)]
                .into_iter()
                .collect()
        );
    }
}
