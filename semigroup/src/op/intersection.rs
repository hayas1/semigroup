use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use semigroup_derive::{SemigroupOp, semigroup_properties_doc};

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
#[semigroup_properties_doc]
#[derive(Debug, Clone, PartialEq, Eq, Default, SemigroupOp)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(
    impl(semigroup = |base, other| base.retain(|k| other.contains(k))),
    crate = crate,
)]
pub struct Intersection<T: Eq + Hash>(pub HashSet<T>);

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
#[semigroup_properties_doc]
#[derive(Debug, Clone, PartialEq, Eq, Default, SemigroupOp)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(
    impl(semigroup = |base, other| base.retain(|k, _| other.contains_key(k))),
    crate = crate,
)]
pub struct IntersectionMap<K: Eq + Hash, V>(pub HashMap<K, V>);

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
