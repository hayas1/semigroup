use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use semigroup_derive::{SemigroupOpPriv, properties_priv};

use crate::SemigroupOp;

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that intersection two sets.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Intersection, Construction, Semigroup};
///
/// let a = Intersection(vec![1, 2, 3].into_iter().collect());
/// let b = Intersection(vec![2, 3, 4].into_iter().collect());
///
/// assert_eq!(a.semigroup(b).into_inner(), vec![2, 3].into_iter().collect());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default, SemigroupOpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[properties_priv]
pub struct Intersection<T: Eq + Hash>(pub HashSet<T>);
impl<T: Eq + Hash> SemigroupOp<HashSet<T>> for Intersection<T> {
    fn lift_op_assign(base: &mut HashSet<T>, other: HashSet<T>) {
        base.retain(|k| other.contains(k));
    }
}

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that intersection two maps.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::IntersectionMap, Construction, Semigroup};
///
/// let a = IntersectionMap(vec![("one", 1), ("two",2), ("three", 3)].into_iter().collect());
/// let b = IntersectionMap(vec![("two", 22), ("three", 33), ("four", 44)].into_iter().collect());
///
/// assert_eq!(a.semigroup(b).into_inner(), vec![("two",2), ("three", 3)].into_iter().collect());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default, SemigroupOpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[properties_priv]
pub struct IntersectionMap<K: Eq + Hash, V>(pub HashMap<K, V>);
impl<K: Eq + Hash, V> SemigroupOp<HashMap<K, V>> for IntersectionMap<K, V> {
    fn lift_op_assign(base: &mut HashMap<K, V>, other: HashMap<K, V>) {
        base.retain(|k, _| other.contains_key(k));
    }
}

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

    use super::*;

    #[test]
    fn test_intersection_semigroup() {
        let (a, b, c) = (
            Intersection(vec![1, 2, 3].into_iter().collect()),
            Intersection(vec![2, 3, 4].into_iter().collect()),
            Intersection(vec![3, 4, 5].into_iter().collect()),
        );
        crate::assert_semigroup!(a, b, c);
    }
    #[test]
    fn test_intersection_map_semigroup() {
        let (a, b, c) = (
            IntersectionMap(
                vec![("one", 1), ("two", 2), ("three", 3)]
                    .into_iter()
                    .collect(),
            ),
            IntersectionMap(
                vec![("two", 22), ("three", 33), ("four", 44)]
                    .into_iter()
                    .collect(),
            ),
            IntersectionMap(
                vec![("three", 333), ("four", 444), ("five", 555)]
                    .into_iter()
                    .collect(),
            ),
        );
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_intersection() {
        let (a, b, c) = (
            Intersection(vec![1, 2, 3].into_iter().collect()),
            Intersection(vec![2, 3, 4].into_iter().collect()),
            Intersection(vec![3, 4, 5].into_iter().collect()),
        );
        assert_eq!(
            a.clone().semigroup(b.clone()).into_inner(),
            vec![2, 3].into_iter().collect()
        );
        assert_eq!(
            a.clone()
                .semigroup(b.clone())
                .semigroup(c.clone())
                .into_inner(),
            vec![3].into_iter().collect()
        );
    }
    #[test]
    fn test_intersection_map() {
        let (a, b, c) = (
            IntersectionMap(
                vec![("one", 1), ("two", 2), ("three", 3)]
                    .into_iter()
                    .collect(),
            ),
            IntersectionMap(
                vec![("two", 22), ("three", 33), ("four", 44)]
                    .into_iter()
                    .collect(),
            ),
            IntersectionMap(
                vec![("three", 333), ("four", 444), ("five", 555)]
                    .into_iter()
                    .collect(),
            ),
        );
        assert_eq!(
            a.clone().semigroup(b.clone()).into_inner(),
            vec![("two", 2), ("three", 3)].into_iter().collect()
        );
        assert_eq!(
            a.clone()
                .semigroup(b.clone())
                .semigroup(c.clone())
                .into_inner(),
            vec![("three", 3),].into_iter().collect()
        );
    }
}
