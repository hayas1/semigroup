use std::ops::{Deref, DerefMut};

use crate::semigroup::Semigroup;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct LazySemigroup<T>(Vec<T>);
impl<T> Semigroup for LazySemigroup<T> {
    fn op(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

impl<T> LazySemigroup<T> {
    pub fn with(t: T) -> Self {
        Self(vec![t])
    }
    pub fn evaluate(self) -> T
    where
        T: Semigroup,
    {
        let mut iter = self.into_iter();
        let base = iter.next().unwrap_or_else(|| unreachable!());
        iter.fold(base, |acc, item| acc.op(item))
    }
}

impl<T> Deref for LazySemigroup<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for LazySemigroup<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> From<LazySemigroup<T>> for Vec<T> {
    fn from(value: LazySemigroup<T>) -> Self {
        value.0
    }
}
impl<T> IntoIterator for LazySemigroup<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<T> Extend<T> for LazySemigroup<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}
