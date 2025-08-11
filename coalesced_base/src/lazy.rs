use crate::coalesce::Coalesce;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Lazy<C>(Vec<C>);
impl<C> Coalesce<C> for Lazy<C> {
    fn coalesce(mut self, other: C) -> Self {
        self.0.push(other);
        self
    }
}
impl<C> Coalesce<Lazy<C>> for Lazy<C> {
    fn coalesce(mut self, other: Lazy<C>) -> Self {
        self.0.extend(other.0);
        self
    }
}

impl<C> Lazy<C> {
    pub fn new(base: C) -> Self {
        Self(vec![base])
    }
}
impl<C: Coalesce> Lazy<C> {
    pub fn into_iter_prior(self) -> impl Iterator<Item = C> {
        Prior::lazy_into_iter(self)
    }
    pub fn into_iter_posterior(self) -> impl Iterator<Item = C> {
        Posterior::lazy_into_iter(self)
    }
    pub fn iter_prior(&self) -> impl Iterator<Item = &C> {
        Prior::lazy_iter(self)
    }
    pub fn iter_posterior(&self) -> impl Iterator<Item = &C> {
        Posterior::lazy_iter(self)
    }

    pub fn into_prior(self) -> C {
        Self::impl_into(self.into_iter_prior())
    }
    pub fn into_posterior(self) -> C {
        Self::impl_into(self.into_iter_posterior())
    }
    fn impl_into(mut iter: impl Iterator<Item = C>) -> C {
        let base = iter.next().unwrap_or_else(|| unreachable!());
        iter.fold(base, |acc, item| acc.coalesce(item))
    }
}
impl<C: Coalesce + Clone> Lazy<C> {
    pub fn cloned_prior(&self) -> C {
        Self::impl_into(self.iter_prior().cloned())
    }
    pub fn cloned_posterior(&self) -> C {
        Self::impl_into(self.iter_posterior().cloned())
    }
}

enum Prior {}
impl<C: Coalesce> LazyIter<C> for Prior {
    fn lazy_into_iter(lazy: Lazy<C>) -> impl Iterator<Item = C> {
        lazy.0.into_iter()
    }
    fn lazy_iter(lazy: &Lazy<C>) -> impl Iterator<Item = &C> {
        lazy.0.iter()
    }
}
enum Posterior {}
impl<C: Coalesce> LazyIter<C> for Posterior {
    fn lazy_into_iter(lazy: Lazy<C>) -> impl Iterator<Item = C> {
        lazy.0.into_iter().rev()
    }
    fn lazy_iter(lazy: &Lazy<C>) -> impl Iterator<Item = &C> {
        lazy.0.iter().rev()
    }
}
trait LazyIter<C: Coalesce> {
    fn lazy_iter(lazy: &Lazy<C>) -> impl Iterator<Item = &C>;
    fn lazy_into_iter(lazy: Lazy<C>) -> impl Iterator<Item = C>;
}
