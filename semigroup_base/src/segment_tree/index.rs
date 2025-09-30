use std::{
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use crate::segment_tree::SegmentTree;

impl<T> SegmentTree<T> {
    pub fn get<I: SegmentTreeIndex<T>>(&self, index: I) -> Option<&I::Output> {
        // index.get(self) // warn: a method with this name may be added to the standard library in the future
        SegmentTreeIndex::get(index, self)
    }
    pub fn get_mut<I: SegmentTreeIndex<T>>(&mut self, index: I) -> Option<&mut I::Output> {
        SegmentTreeIndex::get_mut(index, self)
    }
}
impl<T, I: SegmentTreeIndex<T>> Index<I> for SegmentTree<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        SegmentTreeIndex::index(index, self)
    }
}
impl<T, I: SegmentTreeIndex<T>> IndexMut<I> for SegmentTree<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        SegmentTreeIndex::index_mut(index, self)
    }
}

pub trait SegmentTreeIndex<T> {
    type Output: ?Sized;

    fn get(self, segment_tree: &SegmentTree<T>) -> Option<&Self::Output>;
    fn get_mut(self, segment_tree: &mut SegmentTree<T>) -> Option<&mut Self::Output>;
    // TODO unsafe fn get_unchecked(self, segment_tree: *const SegmentTree<T>) -> *const Self::Output;
    // TODO unsafe fn get_unchecked_mut(self, segment_tree: *mut SegmentTree<T>) -> *mut Self::Output;
    fn index(self, segment_tree: &SegmentTree<T>) -> &Self::Output;
    fn index_mut(self, segment_tree: &mut SegmentTree<T>) -> &mut Self::Output;
}

impl<T, I: SliceIndex<[T]>> SegmentTreeIndex<T> for I {
    type Output = I::Output;

    fn get(self, segment_tree: &SegmentTree<T>) -> Option<&Self::Output> {
        let (leaf_offset, len) = (segment_tree.leaf_offset(), segment_tree.len());
        segment_tree.tree[leaf_offset..leaf_offset + len].get(self) // TODO optimize ?
    }
    fn get_mut(self, segment_tree: &mut SegmentTree<T>) -> Option<&mut Self::Output> {
        let (leaf_offset, len) = (segment_tree.leaf_offset(), segment_tree.len());
        segment_tree.tree[leaf_offset..leaf_offset + len].get_mut(self) // TODO optimize ?
    }
    fn index(self, segment_tree: &SegmentTree<T>) -> &Self::Output {
        let (leaf_offset, len) = (segment_tree.leaf_offset(), segment_tree.len());
        &segment_tree.tree[leaf_offset..leaf_offset + len][self] // TODO optimize ?
    }
    fn index_mut(self, segment_tree: &mut SegmentTree<T>) -> &mut Self::Output {
        let (leaf_offset, len) = (segment_tree.leaf_offset(), segment_tree.len());
        &mut segment_tree.tree[leaf_offset..leaf_offset + len][self] // TODO optimize ?
    }
}

#[cfg(test)]
mod tests {
    use crate::{monoid::OptionMonoid, op::annotation::replace::Replace};

    use super::*;

    #[test]
    fn test_index() {
        let mut segment_tree: SegmentTree<_> = ["one", "two", "three", "four", "five"]
            .into_iter()
            .map(Replace)
            .map(OptionMonoid::from)
            .collect();
        assert_eq!(segment_tree[1], OptionMonoid::from(Replace("two")));
        assert_eq!(
            segment_tree[1..3],
            [
                OptionMonoid::from(Replace("two")),
                OptionMonoid::from(Replace("three")),
            ]
        );
        assert_eq!(
            segment_tree[1..=3],
            [
                OptionMonoid::from(Replace("two")),
                OptionMonoid::from(Replace("three")),
                OptionMonoid::from(Replace("four")),
            ]
        );
        assert_eq!(
            segment_tree[..2],
            [
                OptionMonoid::from(Replace("one")),
                OptionMonoid::from(Replace("two")),
            ]
        );
        assert_eq!(
            segment_tree[..=2],
            [
                OptionMonoid::from(Replace("one")),
                OptionMonoid::from(Replace("two")),
                OptionMonoid::from(Replace("three")),
            ]
        );
        assert_eq!(segment_tree[4..], [OptionMonoid::from(Replace("five"))]);
        assert_eq!(segment_tree[5..], []);
        segment_tree.update(2, OptionMonoid::from(Replace("3")));
        assert_eq!(
            segment_tree[..],
            [
                OptionMonoid::from(Replace("one")),
                OptionMonoid::from(Replace("two")),
                OptionMonoid::from(Replace("3")),
                OptionMonoid::from(Replace("four")),
                OptionMonoid::from(Replace("five")),
            ]
        );
    }

    #[test]
    fn test_get() {
        let mut segment_tree: SegmentTree<_> = ["one", "two", "three", "four", "five"]
            .into_iter()
            .map(Replace)
            .map(OptionMonoid::from)
            .collect();
        assert_eq!(
            segment_tree.get(1),
            Some(&OptionMonoid::from(Replace("two")))
        );
        assert_eq!(segment_tree.get(100), None);
        assert_eq!(
            segment_tree.get(1..3),
            Some(
                &[
                    OptionMonoid::from(Replace("two")),
                    OptionMonoid::from(Replace("three")),
                ][..]
            )
        );
        assert_eq!(
            segment_tree.get(1..=3),
            Some(
                &[
                    OptionMonoid::from(Replace("two")),
                    OptionMonoid::from(Replace("three")),
                    OptionMonoid::from(Replace("four")),
                ][..]
            )
        );
        assert_eq!(segment_tree.get(3..100), None);
        assert_eq!(
            segment_tree.get(..2),
            Some(
                &[
                    OptionMonoid::from(Replace("one")),
                    OptionMonoid::from(Replace("two")),
                ][..]
            )
        );
        assert_eq!(
            segment_tree.get(..=2),
            Some(
                &[
                    OptionMonoid::from(Replace("one")),
                    OptionMonoid::from(Replace("two")),
                    OptionMonoid::from(Replace("three")),
                ][..]
            )
        );
        assert_eq!(
            segment_tree.get(4..),
            Some(&[OptionMonoid::from(Replace("five"))][..])
        );
        assert_eq!(segment_tree.get(5..), Some(&[][..]));
        assert_eq!(segment_tree.get(6..), None);
        segment_tree.update(2, OptionMonoid::from(Replace("3")));
        assert_eq!(
            segment_tree.get(..),
            Some(
                &[
                    OptionMonoid::from(Replace("one")),
                    OptionMonoid::from(Replace("two")),
                    OptionMonoid::from(Replace("3")),
                    OptionMonoid::from(Replace("four")),
                    OptionMonoid::from(Replace("five")),
                ][..]
            )
        );
    }
}
