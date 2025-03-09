use std::{cmp::Ordering, marker::PhantomData};

pub trait Priority {
    fn order(&self, other: &Self) -> Ordering;
}
impl<T> Priority for Option<T> {
    fn order(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}
impl<T, E> Priority for Result<T, E> {
    fn order(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Ok(_), Err(_)) => Ordering::Greater,
            (Err(_), Ok(_)) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

pub(crate) mod sealed {
    pub trait Access {
        type Accessor;
        fn position(accessor: &Self::Accessor) -> usize;
    }
    pub trait Length {}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Prior {}
impl sealed::Access for Prior {
    type Accessor = Accessor<Self>;
    fn position(accessor: &Self::Accessor) -> usize {
        accessor.prior
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Posterior {}
impl sealed::Access for Posterior {
    type Accessor = Accessor<Self>;
    fn position(accessor: &Self::Accessor) -> usize {
        accessor.posterior
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Single {}
impl sealed::Length for Single {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Multiple {}
impl sealed::Length for Multiple {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Accessor<A> {
    // TODO visibility
    pub(crate) prior: usize,
    pub(crate) posterior: usize,
    pub(crate) access_type: PhantomData<A>,
}
impl<A> Accessor<A> {
    pub fn new() -> Self {
        Self {
            prior: 0,
            posterior: 0,
            access_type: PhantomData,
        }
    }
}
impl Accessor<Prior> {
    pub fn as_posterior(self) -> Accessor<Posterior> {
        Accessor {
            prior: self.prior,
            posterior: self.posterior,
            access_type: PhantomData,
        }
    }
}
impl Accessor<Posterior> {
    pub fn as_prior(self) -> Accessor<Prior> {
        Accessor {
            prior: self.prior,
            posterior: self.posterior,
            access_type: PhantomData,
        }
    }
}
