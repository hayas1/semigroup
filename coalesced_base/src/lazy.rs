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
    pub fn push(&mut self, value: T) {
        self.tail.push(value)
    }
    pub fn first(&self) -> &T {
        &self.head
    }
    pub fn last(&self) -> &T {
        self.tail.last().unwrap_or(&self.head)
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
