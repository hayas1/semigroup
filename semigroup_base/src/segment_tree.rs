use std::ops::{Bound, Range, RangeBounds};

use crate::monoid::Monoid;

pub mod index;
pub mod iter;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct SegmentTree<T> {
    tree: Vec<T>, // 0-indexed perfect binary tree, left child: 2i+1, right child: 2i+2, parent: (i-1)/2
    len: usize,
}
impl<T: Monoid + Clone> FromIterator<T> for SegmentTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iterator = iter.into_iter();
        if let (_lower, Some(upper)) = iterator.size_hint() {
            Self::with_length(upper).construct(iterator)
        } else {
            let v: Vec<T> = iterator.collect();
            Self::from(v)
        }
    }
}
impl<T: Monoid + Clone> From<Vec<T>> for SegmentTree<T> {
    fn from(v: Vec<T>) -> Self {
        Self::with_length(v.len()).construct(v)
    }
}
impl<T> SegmentTree<T> {
    /// **O(1)**, get size of the segment tree by given length.
    fn size(len: usize) -> usize {
        2 * len.next_power_of_two() + 1
    }
    /// **O(1)**, get beginning index of the segment tree leaf.
    #[inline]
    fn leaf_offset(&self) -> usize {
        self.len().next_power_of_two() - 1
    }
    /// **O(1)**, return this segment tree's number of data.
    pub fn len(&self) -> usize {
        self.len
    }
    /// **O(1)**, check if this segment tree is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
impl<T: Monoid + Clone> SegmentTree<T> {
    /// **O(len)**, init segment tree with capacity.
    fn with_length(len: usize) -> Self {
        let tree = (0..Self::size(len)).map(|_| T::unit()).collect();
        Self { tree, len }
    }
    /// **O(n)**, init segment tree by given data.
    fn construct<I: IntoIterator<Item = T>>(mut self, iter: I) -> Self {
        let leaf_offset = self.leaf_offset();
        for (i, d) in iter.into_iter().enumerate() {
            self.tree[leaf_offset + i] = d;
        }
        for i in (1..leaf_offset).rev() {
            self.tree[i] =
                T::semigroup_op(self.tree[i * 2 + 1].clone(), self.tree[i * 2 + 2].clone());
        }
        self
    }

    /// **O(log(n))**, set `leaf[k] = x`, and update segment tree.
    pub fn update(&mut self, i: usize, x: T) -> T {
        self.update_with(i, |_| x)
    }
    /// **O(log(n))**, update `leaf[k]` by `f(leaf[k])`, and update segment tree.
    pub fn update_with<F>(&mut self, i: usize, f: F) -> T
    where
        F: FnOnce(&T) -> T,
    {
        assert!(i < self.len(), "index {} is out of 0..{}", i, self.len());
        let mut node = self.leaf_offset() + i;
        let mut result = f(&self.tree[node]);
        std::mem::swap(&mut self.tree[node], &mut result);
        while node > 0 {
            node = (node - 1) / 2;
            self.tree[node] = T::semigroup_op(
                self.tree[node * 2 + 1].clone(),
                self.tree[node * 2 + 2].clone(),
            );
        }
        result
    }

    /// **O(1)**, range to leaf index half interval `[start, end)`.
    fn indices<R>(&self, range: R) -> Range<usize>
    where
        R: RangeBounds<usize>,
    {
        // TODO `range` is nightly only https://doc.rust-lang.org/std/slice/fn.range.html
        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Excluded(&l) => (l + 1).max(0),
            Bound::Included(&l) => l.max(0),
        };
        let end = match range.end_bound() {
            Bound::Unbounded => self.len(),
            Bound::Excluded(&r) => r.min(self.len()),
            Bound::Included(&r) => (r + 1).min(self.len()),
        };
        assert!(start <= end);
        start..end
    }

    /// **O(log(n))**, calculate `f(range)`.
    pub fn fold<R>(&self, range: R) -> T
    where
        R: RangeBounds<usize>,
    {
        let Range { start, end } = self.indices(range);
        let (mut left, mut right) = (self.leaf_offset() + start, self.leaf_offset() + end);
        let (mut left_result, mut right_result) = (T::unit(), T::unit());
        while left < right {
            if left % 2 == 0 {
                left_result = T::semigroup_op(left_result.clone(), self.tree[left].clone());
                left += 1; //  move to next subtree.
            }
            if right % 2 == 0 {
                right_result = T::semigroup_op(self.tree[right - 1].clone(), right_result.clone());
            }
            (left, right) = ((left - 1) / 2, (right - 1) / 2);
        }
        T::semigroup_op(left_result.clone(), right_result.clone())
    }

    /// **O(log^2(n))**, search the leftmost leaf where `cmp(x)` is true in the range.
    pub fn bisect_left<R, F>(&self, range: R, cmp: F) -> Option<usize>
    where
        R: RangeBounds<usize>,
        F: Fn(&T) -> bool,
    {
        self.bisect::<_, _, true>(range, cmp)
    }
    /// **O(log^2(n))**, search the rightmost leaf where `cmp(x)` is true in the range.
    pub fn bisect_right<R, F>(&self, range: R, cmp: F) -> Option<usize>
    where
        R: RangeBounds<usize>,
        F: Fn(&T) -> bool,
    {
        self.bisect::<_, _, false>(range, cmp)
    }
    /// **O(log^2(n))**, search the leaf where `cmp(x)` is true in the range.
    fn bisect<R, F, const L: bool>(&self, range: R, cmp: F) -> Option<usize>
    where
        R: RangeBounds<usize>,
        F: Fn(&T) -> bool,
    {
        let Range { mut start, mut end } = self.indices(range);
        while end - start > 1 {
            let mid = (start + end) / 2;
            let (left_cmp, right_cmp) = (cmp(&self.fold(start..mid)), cmp(&self.fold(mid..end)));
            if L && left_cmp || !L && !right_cmp {
                end = mid;
            } else if L && !left_cmp || !L && right_cmp {
                start = mid;
            } else {
                unreachable!();
            }
        }
        if cmp(&self.tree[self.leaf_offset() + start]) {
            Some(start)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::{Add, BitXor, Mul};

    use num::{
        integer::{gcd, lcm},
        Bounded, Integer, One, Unsigned, Zero,
    };
    use rand::seq::IndexedRandom;
    use semigroup_derive::ConstructionUse;

    use crate::{
        assert_monoid,
        monoid::OptionMonoid,
        op::{annotation::coalesce::Coalesce, Construction},
        semigroup::Semigroup,
    };

    use super::*;

    #[test]
    fn test_sum() {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse,
        )]
        struct Sum<T: Zero + Add<Output = T> + Clone>(pub T);
        impl<T: Zero + Add<Output = T> + Clone> Semigroup for Sum<T> {
            fn semigroup_op(base: Self, other: Self) -> Self {
                Sum(base.0 + other.0)
            }
        }
        impl<T: Zero + Add<Output = T> + Clone> Monoid for Sum<T> {
            fn unit() -> Self {
                Sum(T::zero())
            }
        }
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        match data.choose_multiple_array(&mut rand::rng()) {
            Some([a, b, c]) => assert_monoid!(Sum(a), Sum(b), Sum(c)),
            _ => unreachable!(),
        }
        let mut sum_tree: SegmentTree<_> = data.into_iter().map(Sum).collect();
        assert_eq!(sum_tree.fold(3..5).0, 7);
        assert_eq!(sum_tree.fold(2..7).0, 20);
        assert_eq!(sum_tree.fold(..).0, 55);
        assert_eq!(sum_tree.fold(5..5).0, 0);
        sum_tree.update(5, 10.into());
        assert_eq!(sum_tree.fold(3..=4).0, 7);
        assert_eq!(sum_tree.fold(2..7).0, 25);
        assert_eq!(sum_tree.fold(1..).0, 60);
        sum_tree.update_with(7, |Sum(x)| Sum(x * 2)); // t[7] = 14
        assert_eq!(sum_tree.fold(..6).0, 20);
        assert_eq!(sum_tree.fold(6..=8).0, 28);
    }

    #[test]
    fn test_prod() {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse,
        )]
        struct Prod<T: One + Mul<Output = T> + Clone>(pub T);
        impl<T: One + Mul<Output = T> + Clone> Semigroup for Prod<T> {
            fn semigroup_op(base: Self, other: Self) -> Self {
                Prod(base.0 * other.0)
            }
        }
        impl<T: One + Mul<Output = T> + Clone> Monoid for Prod<T> {
            fn unit() -> Self {
                Prod(T::one())
            }
        }
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        match data.choose_multiple_array(&mut rand::rng()) {
            Some([a, b, c]) => assert_monoid!(Prod(a), Prod(b), Prod(c)),
            _ => unreachable!(),
        }
        let mut prod_tree: SegmentTree<_> = data.into_iter().map(Prod).collect();
        assert_eq!(prod_tree.fold(3..5).0, 12);
        assert_eq!(prod_tree.fold(2..7).0, 720);
        assert_eq!(prod_tree.fold(0..11).0, 0);
        assert_eq!(prod_tree.fold(6..6).0, 1);
        prod_tree.update(5, 10.into());
        assert_eq!(prod_tree.fold(3..5).0, 12);
        assert_eq!(prod_tree.fold(2..7).0, 1440);
        assert_eq!(prod_tree.fold(0..).0, 0);
        prod_tree.update_with(7, |Prod(x)| Prod(x / 2)); // t[7] = 3
        assert_eq!(prod_tree.fold(5..=7).0, 180);
        assert_eq!(prod_tree.fold(8..).0, 720);
    }

    #[test]
    fn test_max() {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse,
        )]
        struct Max<T: Bounded + Ord + Clone>(pub T);
        impl<T: Bounded + Ord + Clone> Semigroup for Max<T> {
            fn semigroup_op(base: Self, other: Self) -> Self {
                Max(std::cmp::max(base.0, other.0))
            }
        }
        impl<T: Bounded + Ord + Clone> Monoid for Max<T> {
            fn unit() -> Self {
                Max(T::min_value())
            }
        }
        let data = [2, -5, 122, -33, -12, 14, -55, 500, 3];
        match data.choose_multiple_array(&mut rand::rng()) {
            Some([a, b, c]) => assert_monoid!(Max(a), Max(b), Max(c)),
            _ => unreachable!(),
        }
        let mut max_tree: SegmentTree<_> = data.into_iter().map(Max).collect();
        assert_eq!(max_tree.fold(3..5).0, -12);
        assert_eq!(max_tree.fold(2..=6).0, 122);
        assert_eq!(max_tree.fold(..).0, 500);
        assert_eq!(max_tree.fold(0..0).0, i32::MIN);
        max_tree.update(5, 1000.into());
        assert_eq!(max_tree.fold(3..=4).0, -12);
        assert_eq!(max_tree.fold(2..7).0, 1000);
        assert_eq!(max_tree.fold(..10).0, 1000);
    }

    #[test]
    fn test_min() {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse,
        )]
        struct Min<T: Bounded + Ord + Clone>(pub T);
        impl<T: Bounded + Ord + Clone> Semigroup for Min<T> {
            fn semigroup_op(base: Self, other: Self) -> Self {
                Min(std::cmp::min(base.0, other.0))
            }
        }
        impl<T: Bounded + Ord + Clone> Monoid for Min<T> {
            fn unit() -> Self {
                Min(T::max_value())
            }
        }
        let data = [2, -5, 122, 33, 12, 14, -55, 500, 3];
        match data.choose_multiple_array(&mut rand::rng()) {
            Some([a, b, c]) => assert_monoid!(Min(a), Min(b), Min(c)),
            _ => unreachable!(),
        }
        let mut min_tree: SegmentTree<_> = data.into_iter().map(Min).collect();
        assert_eq!(min_tree.fold(3..5).0, 12);
        assert_eq!(min_tree.fold(2..7).0, -55);
        assert_eq!(min_tree.fold(0..).0, -55);
        assert_eq!(min_tree.fold(5..5).0, i32::MAX);
        min_tree.update(5, (-1000).into());
        assert_eq!(min_tree.fold(3..5).0, 12);
        assert_eq!(min_tree.fold(2..7).0, -1000);
        assert_eq!(min_tree.fold(..10).0, -1000);
    }

    #[test]
    fn test_gcd() {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse,
        )]
        struct Gcd<T: Unsigned + Integer + Clone>(T);
        impl<T: Unsigned + Integer + Clone> Semigroup for Gcd<T> {
            fn semigroup_op(base: Self, other: Self) -> Self {
                Gcd(gcd(base.0, other.0))
            }
        }
        impl<T: Unsigned + Integer + Clone> Monoid for Gcd<T> {
            fn unit() -> Self {
                Gcd(T::zero())
            }
        }
        let data = [10u32, 3, 4, 8, 6, 2];
        match data.choose_multiple_array(&mut rand::rng()) {
            Some([a, b, c]) => assert_monoid!(Gcd(a), Gcd(b), Gcd(c)),
            _ => unreachable!(),
        }
        let mut gcd_tree: SegmentTree<_> = data.into_iter().map(Gcd).collect();
        assert_eq!(gcd_tree.fold(2..4).0, 4);
        assert_eq!(gcd_tree.fold(2..6).0, 2);
        assert_eq!(gcd_tree.fold(0..6).0, 1);
        assert_eq!(gcd_tree.fold(3..3).0, 0);
        gcd_tree.update(5, 7.into());
        assert_eq!(gcd_tree.fold(2..4).0, 4);
        assert_eq!(gcd_tree.fold(2..6).0, 1);
        assert_eq!(gcd_tree.fold(0..6).0, 1);
    }

    #[test]
    fn test_lcm() {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse,
        )]
        struct Lcm<T: Unsigned + Integer + Clone>(T);
        impl<T: Unsigned + Integer + Clone> Semigroup for Lcm<T> {
            fn semigroup_op(base: Self, other: Self) -> Self {
                Lcm(lcm(base.0, other.0))
            }
        }
        impl<T: Unsigned + Integer + Clone> Monoid for Lcm<T> {
            fn unit() -> Self {
                Lcm(T::one())
            }
        }
        let data = vec![10u32, 3, 4, 8, 6, 2];
        match data.choose_multiple_array(&mut rand::rng()) {
            Some([a, b, c]) => assert_monoid!(Lcm(a), Lcm(b), Lcm(c)),
            _ => unreachable!(),
        }
        let mut lcm_tree: SegmentTree<_> = data.into_iter().map(Lcm).collect();
        assert_eq!(lcm_tree.fold(2..4).0, 8);
        assert_eq!(lcm_tree.fold(2..6).0, 24);
        assert_eq!(lcm_tree.fold(..).0, 120);
        assert_eq!(lcm_tree.fold(4..4).0, 1);
        lcm_tree.update(5, 7.into());
        assert_eq!(lcm_tree.fold(2..4).0, 8);
        assert_eq!(lcm_tree.fold(2..6).0, 168);
        assert_eq!(lcm_tree.fold(..).0, 840);
    }

    #[test]
    fn test_xor() {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse,
        )]
        struct Xor<T: Zero + BitXor<Output = T> + Clone>(T);
        impl<T: Zero + BitXor<Output = T> + Clone> Semigroup for Xor<T> {
            fn semigroup_op(base: Self, other: Self) -> Self {
                Xor(base.0 ^ other.0)
            }
        }
        impl<T: Zero + BitXor<Output = T> + Clone> Monoid for Xor<T> {
            fn unit() -> Self {
                Xor(T::zero())
            }
        }
        let data = [0b111, 0b101, 0b100, 0b000, 0b010];
        match data.choose_multiple_array(&mut rand::rng()) {
            Some([a, b, c]) => assert_monoid!(Xor(a), Xor(b), Xor(c)),
            _ => unreachable!(),
        }
        let mut xor_tree: SegmentTree<_> = data.into_iter().map(Xor).collect();
        assert_eq!(xor_tree.fold(2..4).0, 0b100);
        assert_eq!(xor_tree.fold(2..5).0, 0b110);
        assert_eq!(xor_tree.fold(0..5).0, 0b100);
        assert_eq!(xor_tree.fold(5..5).0, 0b000);
        xor_tree.update(4, 0b110.into());
        assert_eq!(xor_tree.fold(2..4).0, 0b100);
        assert_eq!(xor_tree.fold(2..5).0, 0b010);
        assert_eq!(xor_tree.fold(0..5).0, 0b000);
    }

    #[test]
    fn test_empty_tree() {
        let empty = SegmentTree::<OptionMonoid<Coalesce<u64>>>::from(vec![]);
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
        assert_eq!(
            empty.tree,
            // TODO optimize
            vec![
                OptionMonoid::unit(),
                OptionMonoid::unit(),
                OptionMonoid::unit(),
            ]
        );

        assert_eq!(empty.fold(..), OptionMonoid::unit());
        assert_eq!(empty.fold(0..0), OptionMonoid::unit());
    }

    #[test]
    fn test_singleton_tree() {
        let mut single = SegmentTree::<_>::from(vec![OptionMonoid::from(Coalesce(Some(3)))]);
        assert!(!single.is_empty());
        assert_eq!(single.len(), 1);
        assert_eq!(
            single.tree,
            // TODO optimize
            vec![
                OptionMonoid::from(Coalesce(Some(3))),
                OptionMonoid::unit(),
                OptionMonoid::unit(),
            ]
        );

        assert_eq!(single.fold(..), OptionMonoid::from(Coalesce(Some(3))));
        assert_eq!(single.fold(1..1), OptionMonoid::unit());
        assert_eq!(single.fold(1..), OptionMonoid::unit());
        single.update(0, OptionMonoid::from(Coalesce(Some(5))));
        assert_eq!(single.fold(..), OptionMonoid::from(Coalesce(Some(5))));
        assert_eq!(single.fold(1..), OptionMonoid::unit());
    }

    #[test]
    fn test_large_tree() {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse,
        )]
        struct Sum(u128);
        impl Semigroup for Sum {
            fn semigroup_op(base: Self, other: Self) -> Self {
                Sum(base.0 + other.0)
            }
        }
        impl Monoid for Sum {
            fn unit() -> Self {
                Sum(0)
            }
        }
        let cum_sum = |s, t| (t - s + 1) * (s + t) / 2;
        let mut sum_tree: SegmentTree<_> = (0..2000000).map(Sum).collect();
        assert_eq!(sum_tree.fold(0..=10).0, cum_sum(0u128, 10u128));
        assert_eq!(sum_tree.fold(5..=15).0, cum_sum(5, 15));
        assert_eq!(sum_tree.fold(123..=1234567).0, cum_sum(123, 1234567));
        assert_eq!(sum_tree.fold(123456..=345678).0, cum_sum(123456, 345678));
        assert_eq!(sum_tree.fold(888888..=999999).0, cum_sum(888888, 999999));
        assert_eq!(sum_tree.fold(..).0, cum_sum(0, 1999999));

        for i in 0..2000000 {
            sum_tree.update_with(i, |Sum(x)| Sum(x + 1)); // expensive loop
        }
        let cum_sum_1 = |s, t| (t - s + 1) * (s + t + 2) / 2;
        assert_eq!(sum_tree.fold(0..=10).0, cum_sum_1(0u128, 10u128));
        assert_eq!(sum_tree.fold(5..=15).0, cum_sum_1(5, 15));
        assert_eq!(sum_tree.fold(123..=1234567).0, cum_sum_1(123, 1234567));
        assert_eq!(sum_tree.fold(123456..=345678).0, cum_sum_1(123456, 345678));
        assert_eq!(sum_tree.fold(888888..=999999).0, cum_sum_1(888888, 999999));
        assert_eq!(sum_tree.fold(..).0, cum_sum_1(0, 1999999));
    }

    #[test]
    fn test_bisect() {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse,
        )]
        struct Max<T: Bounded + Ord + Clone>(pub T);
        impl<T: Bounded + Ord + Clone> Semigroup for Max<T> {
            fn semigroup_op(base: Self, other: Self) -> Self {
                Max(std::cmp::max(base.0, other.0))
            }
        }
        impl<T: Bounded + Ord + Clone> Monoid for Max<T> {
            fn unit() -> Self {
                Max(T::min_value())
            }
        }
        let data1 = [2, -5, 122, -33, -12, 14, -55, 500, 3];
        let mut max_tree1: SegmentTree<_> = data1.into_iter().map(Max).collect();
        assert_eq!(max_tree1.bisect_left(2..5, |&Max(x)| x >= 10), Some(2));
        assert_eq!(max_tree1.bisect_left(3..5, |&Max(x)| x >= 10), None);
        max_tree1.update(2, (-5).into());
        assert_eq!(max_tree1.bisect_left(1..3, |&Max(x)| x >= -5), Some(1));
        assert_eq!(max_tree1.bisect_left(1..5, |&Max(x)| x >= 500), None);
        assert_eq!(max_tree1.bisect_left(5..10, |&Max(x)| x >= 500), Some(7));

        let data2 = [2, -5, 122, -33, -12, 14, -55, 500, 3];
        let mut max_tree2: SegmentTree<_> = data2.into_iter().map(Max).collect();
        assert_eq!(max_tree2.bisect_right(2..5, |&Max(x)| x >= 10), Some(2));
        assert_eq!(max_tree2.bisect_right(3..5, |&Max(x)| x >= 10), None);
        max_tree2.update(2, (-5).into());
        assert_eq!(max_tree2.bisect_right(1..3, |&Max(x)| x >= -5), Some(2));
        assert_eq!(max_tree2.bisect_right(1..5, |&Max(x)| x >= 500), None);
        assert_eq!(max_tree2.bisect_right(5..10, |&Max(x)| x >= 500), Some(7));
        max_tree2.update(3, (-5).into());
        assert_eq!(max_tree2.bisect_right(1..5, |&Max(x)| x >= -5), Some(3));
        max_tree2.update(4, (-5).into());
        assert_eq!(max_tree2.bisect_right(1..5, |&Max(x)| x >= -5), Some(4));
    }
}
