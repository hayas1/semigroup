use std::marker::PhantomData;

pub trait Access {
    type Accessor;
    fn position(accessor: &Self::Accessor) -> usize;
}
pub enum Prior {}
impl Access for Prior {
    type Accessor = Accessor<Self>;
    fn position(accessor: &Self::Accessor) -> usize {
        accessor.prior
    }
}
pub enum Posterior {}
impl Access for Posterior {
    type Accessor = Accessor<Self>;
    fn position(accessor: &Self::Accessor) -> usize {
        accessor.posterior
    }
}

pub enum Single {}
pub enum Multiple {}

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
