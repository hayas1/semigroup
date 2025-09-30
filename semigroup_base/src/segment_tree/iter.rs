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
}
