#[cfg(feature = "serde")]
pub mod serde;

use std::marker::PhantomData;

pub trait Coalesce {
    fn straight(&self, other: &Self) -> bool;
    fn extend_prior<A>(self, other: Coalesced<Self, A>) -> Coalesced<Self, A>
    where
        Self: Sized,
    {
        Coalesced::new(self).extend_prior(other)
    }
    fn extend_posterior<A>(self, other: Coalesced<Self, A>) -> Coalesced<Self, A>
    where
        Self: Sized,
    {
        Coalesced::new(self).extend_posterior(other)
    }
}
impl<T> Coalesce for Option<T> {
    fn straight(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(_), _) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Coalesced<C, A = Prior> {
    priority: Vec<C>,
    accessor: PriorityAccessor<A>,
}

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
    prior: usize,
    posterior: usize,
    access_type: PhantomData<A>,
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

impl<C, A: Access<Accessor = PriorityAccessor<A>>> std::ops::Deref for Coalesced<C, A> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        &self.priority[A::position(&self.accessor)]
    }
}
impl<C, A: Access<Accessor = PriorityAccessor<A>>> std::ops::DerefMut for Coalesced<C, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.priority[A::position(&self.accessor)]
    }
}

impl<C, A> Coalesced<C, A> {
    fn new(coalesce: C) -> Self {
        Self {
            priority: vec![coalesce],
            accessor: PriorityAccessor::new(),
        }
    }
    pub fn confirm(mut self) -> C
    where
        A: Access<Accessor = PriorityAccessor<A>>,
    {
        self.priority.swap_remove(A::position(&self.accessor))
    }

    // TODO impl as trait ?
    pub fn extend_prior(mut self, other: Self) -> Self
    where
        C: Coalesce,
    {
        let base_len = self.priority.len();
        self.priority.extend(other.priority);
        self.accessor.prior = base_len + other.accessor.prior;
        for i in (1..=self.accessor.prior).rev() {
            if !self.priority[i].straight(&self.priority[i - 1]) {
                self.accessor.prior = i - 1;
            } else {
                break;
            }
        }
        self.accessor.posterior = other.accessor.posterior;
        for i in 0..base_len + other.accessor.posterior {
            if !self.priority[i].straight(&self.priority[i + 1]) {
                self.accessor.posterior = i + 1;
            } else {
                break;
            }
        }
        self
    }
    pub fn extend_posterior(self, mut other: Self) -> Self
    where
        C: Coalesce,
    {
        let base_len = other.priority.len();
        other.priority.extend(self.priority);
        other.accessor.prior = base_len + self.accessor.prior;
        for i in (1..=other.accessor.prior).rev() {
            if !other.priority[i].straight(&other.priority[i - 1]) {
                other.accessor.prior = i - 1;
            } else {
                break;
            }
        }
        other.accessor.posterior = self.accessor.posterior;
        for i in 0..base_len + self.accessor.posterior {
            if !other.priority[i].straight(&other.priority[i + 1]) {
                other.accessor.posterior = i + 1;
            } else {
                break;
            }
        }
        other
    }
}
impl<C> Coalesced<C, Prior> {
    pub fn new_prior(coalesce: C) -> Coalesced<C, Prior> {
        Coalesced::<C, Prior>::new(coalesce)
    }
    pub fn posterior(self) -> Coalesced<C, Posterior> {
        Coalesced {
            priority: self.priority,
            accessor: self.accessor.as_posterior(),
        }
    }
}
impl<C> Coalesced<C, Posterior> {
    pub fn new_posterior(coalesce: C) -> Coalesced<C, Posterior> {
        Coalesced::<C, Posterior>::new(coalesce)
    }
    pub fn prior(self) -> Coalesced<C, Prior> {
        Coalesced {
            priority: self.priority,
            accessor: self.accessor.as_prior(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coalesced_prior_history() {
        let from_file = Coalesced::new_prior(Some("file"));
        let from_env = Coalesced::new_prior(Some("env"));
        let from_cli = Coalesced::new_prior(Some("cli"));

        let config = from_file.extend_prior(from_env).extend_prior(from_cli);
        assert_eq!(config.unwrap(), "cli");
        assert_eq!(
            config.priority,
            vec![Some("file"), Some("env"), Some("cli")],
        );
    }

    #[test]
    fn test_coalesced_posterior_history() {
        let from_file = Coalesced::new_posterior(Some("file"));
        let from_env = Coalesced::new_posterior(Some("env"));
        let from_cli = Coalesced::new_posterior(Some("cli"));

        let config = from_file
            .extend_posterior(from_env)
            .extend_posterior(from_cli);
        assert_eq!(config.unwrap(), "cli");
        assert_eq!(
            config.priority,
            vec![Some("cli"), Some("env"), Some("file")],
        );
    }

    #[test]
    fn test_coalesced_switch_prior_to_posterior() {
        let from_file = Coalesced::new_prior(Some("file"));
        let from_env = Coalesced::new_prior(Some("env"));
        let from_cli = Coalesced::new_prior(Some("cli"));

        let config = from_file
            .extend_posterior(from_env)
            .extend_posterior(from_cli);
        assert_eq!(config.unwrap(), "file");
        assert_eq!(
            config.priority,
            vec![Some("cli"), Some("env"), Some("file")],
        );
        let config_posterior = config.posterior();
        assert_eq!(config_posterior.unwrap(), "cli");
        assert_eq!(
            config_posterior.priority,
            vec![Some("cli"), Some("env"), Some("file")],
        );
    }
    #[test]
    fn test_coalesced_switch_posterior_to_prior() {
        let from_file = Coalesced::new_posterior(Some("file"));
        let from_env = Coalesced::new_posterior(Some("env"));
        let from_cli = Coalesced::new_posterior(Some("cli"));

        let config = from_file.extend_prior(from_env).extend_prior(from_cli);
        assert_eq!(config.unwrap(), "file");
        assert_eq!(
            config.priority,
            vec![Some("file"), Some("env"), Some("cli")],
        );
        let config_prior = config.prior();
        assert_eq!(config_prior.unwrap(), "cli");
        assert_eq!(
            config_prior.priority,
            vec![Some("file"), Some("env"), Some("cli")],
        );
    }

    #[test]
    fn test_coalesced_complex_prior_posterior() {
        let first = Coalesced::new_prior(None);
        let second = Coalesced::new_prior(Some(2));
        let third = Coalesced::new_prior(Some(3));
        let four = Coalesced::new_prior(None);
        let five = Coalesced::new_prior(Some(5));
        let six = Coalesced::new_prior(None);

        let coalesced = first
            .extend_prior(second)
            .extend_prior(third)
            .extend_prior(four)
            .extend_prior(five)
            .extend_prior(six);

        assert_eq!(coalesced.unwrap(), 5);
        assert_eq!(
            coalesced.priority,
            vec![None, Some(2), Some(3), None, Some(5), None]
        );

        let coalesced = coalesced.posterior();
        assert_eq!(coalesced.unwrap(), 2);
        assert_eq!(
            coalesced.priority,
            vec![None, Some(2), Some(3), None, Some(5), None]
        );
    }
}
