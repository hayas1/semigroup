use crate::semigroup::Semigroup;

pub trait SemigroupIterator: Sized + Iterator {
    fn fold_last(mut self, last: Self::Item) -> Self::Item
    where
        Self::Item: Semigroup,
    {
        if let Some(init) = self.next() {
            self.chain(Some(last))
                .fold(init, |acc, item| acc.semigroup(item))
        } else {
            last
        }
    }
}
impl<I: Iterator> SemigroupIterator for I {}

pub trait SemigroupDoubleEndedIterator: Sized + DoubleEndedIterator {
    fn rfold_last(mut self, last: Self::Item) -> Self::Item
    where
        Self::Item: Semigroup,
    {
        if let Some(init) = self.next_back() {
            Some(last)
                .into_iter()
                .chain(self)
                .rfold(init, |acc, item| acc.semigroup(item))
        } else {
            last
        }
    }
}
impl<I: DoubleEndedIterator> SemigroupDoubleEndedIterator for I {}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use std::{fmt::Debug, vec};

    use super::*;

    pub fn assert_lazy_evaluation_iter<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let empty = vec![];
        assert_eq!(
            empty
                .iter()
                .cloned()
                .reduce(|acc, item| T::semigroup_op(acc, item)),
            None
        );
        assert_eq!(
            empty
                .into_iter()
                .rev()
                .reduce(|acc, item| T::semigroup_op(item, acc)),
            None
        );

        let lazy = vec![a.clone(), b.clone(), c.clone()];
        assert_eq!(
            lazy.iter()
                .cloned()
                .reduce(|acc, item| T::semigroup_op(acc, item)),
            Some(T::semigroup_op(
                T::semigroup_op(a.clone(), b.clone()),
                c.clone()
            ))
        );
        assert_eq!(
            lazy.into_iter()
                .rev()
                .reduce(|acc, item| T::semigroup_op(item, acc)),
            Some(T::semigroup_op(
                a.clone(),
                T::semigroup_op(b.clone(), c.clone())
            ))
        );

        let ab = vec![a.clone(), b.clone()];
        assert_eq!(
            ab.iter()
                .cloned()
                .rfold(c.clone(), |acc, item| T::semigroup_op(item, acc)),
            T::semigroup_op(a.clone(), T::semigroup_op(b.clone(), c.clone()))
        );
        assert_eq!(
            ab.into_iter().fold_last(c.clone()),
            T::semigroup_op(T::semigroup_op(a.clone(), b.clone()), c.clone())
        );

        let bc = vec![b.clone(), c.clone()];
        assert_eq!(
            bc.iter()
                .cloned()
                .fold(a.clone(), |acc, item| T::semigroup_op(acc, item)),
            T::semigroup_op(T::semigroup_op(a.clone(), b.clone()), c.clone())
        );
        assert_eq!(
            bc.into_iter().rfold_last(a.clone()),
            T::semigroup_op(T::semigroup_op(c.clone(), b.clone()), a.clone())
        );
    }
}
