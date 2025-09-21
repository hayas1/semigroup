use crate::semigroup::Semigroup;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct LazySemigroup<T> {
    head: T,
    tail: Vec<T>,
}
impl<T> Semigroup for LazySemigroup<T> {
    fn semigroup_op(mut base: Self, other: Self) -> Self {
        base.extend(other);
        base
    }
}
impl<T: Semigroup> LazySemigroup<T> {
    pub fn fold(self) -> T {
        let Self { head, tail } = self;
        tail.into_iter()
            .fold(head, |acc, item| T::semigroup_op(acc, item))
    }
    pub fn fold_cloned(&self) -> T
    where
        T: Clone,
    {
        let Self { head, tail } = self;
        tail.iter()
            .cloned()
            .fold(head.clone(), |acc, item| T::semigroup_op(acc, item))
    }
}

impl<T> LazySemigroup<T> {
    pub fn with(value: T) -> Self {
        Self {
            head: value,
            tail: Vec::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        false
    }
    pub fn is_single(&self) -> bool {
        self.tail.is_empty()
    }
    pub fn len(&self) -> usize {
        1 + self.tail.len()
    }
    pub fn push(&mut self, value: T) {
        self.tail.push(value)
    }
    pub fn first(&self) -> &T {
        &self.head
    }
    pub fn last(&self) -> &T {
        self.tail.last().unwrap_or(&self.head)
    }
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> LazySemigroup<U> {
        let (head, tail) = (f(self.head), self.tail.into_iter().map(f).collect());
        LazySemigroup { head, tail }
    }
}
impl<T> IntoIterator for LazySemigroup<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            head: Some(self.head),
            tail: self.tail.into_iter(),
        }
    }
}
impl<T> Extend<T> for LazySemigroup<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.tail.extend(iter);
    }
}

#[derive(Debug, Clone, Default)]
pub struct IntoIter<T> {
    head: Option<T>,
    tail: <Vec<T> as IntoIterator>::IntoIter,
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_some() {
            self.head.take()
        } else {
            self.tail.next()
        }
    }
}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use std::fmt::Debug;

    use super::*;

    pub fn assert_lazy_evaluation<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let mut lazy = LazySemigroup::with(a.clone());
        lazy.push(b.clone());
        lazy.push(c.clone());

        assert_eq!(lazy.fold(), T::semigroup_op(T::semigroup_op(a, b), c))
    }

    #[test]
    fn test_first_last() {
        let mut lazy = LazySemigroup::with(1);
        assert_eq!(lazy.first(), &1);
        assert_eq!(lazy.last(), &1);

        lazy.push(2);
        assert_eq!(lazy.first(), &1);
        assert_eq!(lazy.last(), &2);

        lazy.push(3);
        assert_eq!(lazy.first(), &1);
        assert_eq!(lazy.last(), &3);
    }
}
