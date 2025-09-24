use crate::semigroup::Semigroup;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Lazy<T> {
    head: T,
    tail: Vec<T>,
}
impl<T> Semigroup for Lazy<T> {
    fn semigroup_op(mut base: Self, other: Self) -> Self {
        base.extend(other);
        base
    }
}
impl<T: Semigroup> Lazy<T> {
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
    pub fn rfold(self) -> T {
        let Self { head, mut tail } = self;
        if let Some(rightmost) = tail.pop() {
            Some(head)
                .into_iter()
                .chain(tail)
                .rfold(rightmost, |acc, item| T::semigroup_op(item, acc))
        } else {
            head
        }
    }
    pub fn rfold_cloned(&self) -> T
    where
        T: Clone,
    {
        let Self { head, tail } = self;
        if let Some((rightmost, left)) = tail.split_last() {
            Some(head)
                .into_iter()
                .chain(left)
                .cloned()
                .rfold(rightmost.clone(), |acc, item| T::semigroup_op(item, acc))
        } else {
            head.clone()
        }
    }
}

impl<T> Lazy<T> {
    pub fn with(value: T) -> Self {
        Self {
            head: value,
            tail: Vec::new(),
        }
    }
    pub fn from_iterator<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        // compile error: type parameter `T` must be used as the type parameter for some local type
        // impl<T> FromIterator<T> for Option<Lazy<T>> {
        //     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        //         todo!()
        //     }
        // }
        let mut iterator = iter.into_iter();
        iterator.next().map(|head| Self {
            head,
            tail: iterator.collect(),
        })
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
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: Some(&self.head),
            tail: self.tail.iter(),
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: Some(&mut self.head),
            tail: self.tail.iter_mut(),
        }
    }
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Lazy<U> {
        let (head, tail) = (f(self.head), self.tail.into_iter().map(f).collect());
        Lazy { head, tail }
    }
}
impl<T> IntoIterator for Lazy<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            head: Some(self.head),
            tail: self.tail.into_iter(),
        }
    }
}
impl<T> Extend<T> for Lazy<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.tail.extend(iter);
    }
}

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Iter<'a, T> {
    head: Option<&'a T>,
    tail: <&'a Vec<T> as IntoIterator>::IntoIter,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_some() {
            self.head.take()
        } else {
            self.tail.next()
        }
    }
}
#[derive(Debug)]
pub struct IterMut<'a, T> {
    head: Option<&'a mut T>,
    tail: <&'a mut Vec<T> as IntoIterator>::IntoIter,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

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
        let mut lazy = Lazy::with(a.clone());
        lazy.push(b.clone());
        lazy.push(c.clone());

        assert_eq!(lazy.fold_cloned(), lazy.clone().fold());
        assert_eq!(
            lazy.fold_cloned(),
            T::semigroup_op(T::semigroup_op(a.clone(), b.clone()), c.clone())
        );

        assert_eq!(lazy.rfold_cloned(), lazy.clone().rfold());
        assert_eq!(
            lazy.rfold(),
            T::semigroup_op(a.clone(), T::semigroup_op(b.clone(), c.clone()),)
        );
    }

    #[test]
    fn test_push() {
        let mut lazy = Lazy::with(1);
        assert!(!lazy.is_empty());
        assert!(lazy.is_single());
        assert_eq!(lazy.len(), 1);
        assert_eq!(lazy.first(), &1);
        assert_eq!(lazy.last(), &1);

        lazy.push(2);
        assert!(!lazy.is_empty());
        assert!(!lazy.is_single());
        assert_eq!(lazy.len(), 2);
        assert_eq!(lazy.first(), &1);
        assert_eq!(lazy.last(), &2);

        lazy.push(3);
        assert!(!lazy.is_empty());
        assert!(!lazy.is_single());
        assert_eq!(lazy.len(), 3);
        assert_eq!(lazy.first(), &1);
        assert_eq!(lazy.last(), &3);
    }

    #[test]
    fn test_iter() {
        let lazy = Lazy::from_iterator(vec![1, 2, 3, 4, 5]).unwrap();
        assert!(!lazy.is_empty());
        assert!(!lazy.is_single());
        assert_eq!(lazy.len(), 5);
        assert_eq!(lazy.first(), &1);
        assert_eq!(lazy.last(), &5);
    }
}
