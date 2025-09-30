use crate::segment_tree::SegmentTree;

impl<T> IntoIterator for SegmentTree<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(mut self) -> Self::IntoIter {
        let (leaf_offset, len) = (self.leaf_offset(), self.len());
        let mut leaf = self.tree.split_off(leaf_offset);
        let _units = leaf.split_off(len);
        IntoIter {
            inner: leaf.into_iter(),
        }
    }
}
impl<'a, T> IntoIterator for &'a SegmentTree<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self[..].iter(),
        }
    }
}
impl<T> SegmentTree<T> {
    /// **O(1)**, get iterator of the segment tree leaf.
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            inner: self[..].iter(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IntoIter<T> {
    inner: <Vec<T> as IntoIterator>::IntoIter,
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct Iter<'a, T> {
    inner: <&'a [T] as IntoIterator>::IntoIter,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::{monoid::OptionMonoid, op::annotation::replace::Replace};

    use super::*;

    #[test]
    fn test_into_iter() {
        let segment_tree: SegmentTree<_> = ["one", "two", "three", "four", "five"]
            .into_iter()
            .map(Replace)
            .map(OptionMonoid::from)
            .collect();
        let v: Vec<_> = segment_tree
            .into_iter()
            .map(|x| match x {
                OptionMonoid(Some(Replace(s))) => s,
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(v, ["one", "two", "three", "four", "five",]);
    }

    #[test]
    fn test_iter() {
        let segment_tree: SegmentTree<_> = ["one", "two", "three", "four", "five"]
            .into_iter()
            .map(Replace)
            .map(OptionMonoid::from)
            .collect();
        let v: Vec<_> = segment_tree
            .iter()
            .map(|x| match x {
                OptionMonoid(Some(Replace(s))) => s,
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(v, [&"one", &"two", &"three", &"four", &"five",]);
    }

    #[test]
    fn test_for() {
        let segment_tree: SegmentTree<_> = ["one", "two", "three", "four", "five"]
            .into_iter()
            .map(Replace)
            .map(OptionMonoid::from)
            .collect();
        let mut v = Vec::new();
        for OptionMonoid(x) in &segment_tree {
            match x {
                Some(Replace(s)) => v.push(s),
                _ => unreachable!(),
            }
        }
        assert_eq!(v, [&"one", &"two", &"three", &"four", &"five",]);
    }
}
