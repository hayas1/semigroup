use std::marker::PhantomData;

pub trait Access {
    type Accessor;
    fn position(accessor: &Self::Accessor) -> usize;
}
pub enum Prior {}
impl Access for Prior {
    type Accessor = PriorityAccessor<Self>;
    fn position(accessor: &Self::Accessor) -> usize {
        accessor.prior
    }
}
pub enum Posterior {}
impl Access for Posterior {
    type Accessor = PriorityAccessor<Self>;
    fn position(accessor: &Self::Accessor) -> usize {
        accessor.posterior
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct PriorityAccessor<A> {
    // TODO visibility
    pub(crate) prior: usize,
    pub(crate) posterior: usize,
    pub(crate) access_type: PhantomData<A>,
}
impl<A> PriorityAccessor<A> {
    pub fn new() -> Self {
        Self {
            prior: 0,
            posterior: 0,
            access_type: PhantomData,
        }
    }
}
impl PriorityAccessor<Prior> {
    pub fn as_posterior(self) -> PriorityAccessor<Posterior> {
        PriorityAccessor {
            prior: self.prior,
            posterior: self.posterior,
            access_type: PhantomData,
        }
    }
}
impl PriorityAccessor<Posterior> {
    pub fn as_prior(self) -> PriorityAccessor<Prior> {
        PriorityAccessor {
            prior: self.prior,
            posterior: self.posterior,
            access_type: PhantomData,
        }
    }
}
