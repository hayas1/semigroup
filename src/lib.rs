pub mod extension;
pub mod priority;

#[cfg(feature = "clap")]
pub mod clap;
#[cfg(feature = "serde")]
pub mod serde;

pub use priority::{Multiple, Posterior, Prior, Single};

pub trait Coalesce {
    fn straight(&self, other: &Self) -> bool;
}
impl<T> Coalesce for Option<T> {
    fn straight(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(_), _) => true,
            _ => false,
        }
    }
}
impl<T, E> Coalesce for Result<T, E> {
    fn straight(&self, other: &Self) -> bool {
        match (self, other) {
            (Ok(_), _) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Coalesced<C, A = Prior, E = (), L = Single>
where
    L: priority::sealed::Length,
{
    priority: Vec<extension::Extension<C, E>>,
    accessor: priority::Accessor<A>,
    phantom: std::marker::PhantomData<L>,
}

impl<C, A, L> std::ops::Deref for Coalesced<C, A, (), L>
where
    A: priority::Access<Accessor = priority::Accessor<A>>,
    L: priority::sealed::Length,
{
    type Target = C;
    fn deref(&self) -> &Self::Target {
        self.value()
    }
}
impl<C, A, L> std::ops::DerefMut for Coalesced<C, A, (), L>
where
    A: priority::Access<Accessor = priority::Accessor<A>>,
    L: priority::sealed::Length,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value_mut()
    }
}
impl<C, A> Coalesced<C, A, ()> {
    fn new(coalesce: C) -> Self {
        Self::new_with(coalesce, ())
    }
}
impl<C, A, E, L> Coalesced<C, A, E, L>
where
    L: priority::sealed::Length,
{
    fn new_with(coalesce: C, extension: E) -> Self {
        Self {
            priority: vec![extension::Extension::new_with(coalesce, extension)],
            accessor: priority::Accessor::new(),
            phantom: std::marker::PhantomData,
        }
    }
}
impl<C, A, E, L> Coalesced<C, A, E, L>
where
    C: Coalesce,
    A: priority::Access<Accessor = priority::Accessor<A>>,
    L: priority::sealed::Length,
{
    pub fn register<L2>(mut self, other: Coalesced<C, A, E, L2>) -> Coalesced<C, A, E, Multiple>
    where
        L2: priority::sealed::Length,
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
        Coalesced {
            priority: self.priority,
            accessor: self.accessor,
            phantom: std::marker::PhantomData,
        }
    }
}
impl<C, A, E, L> Coalesced<C, A, E, L>
where
    A: priority::Access<Accessor = priority::Accessor<A>>,
    L: priority::sealed::Length,
{
    pub fn access_owned(mut self) -> extension::Extension<C, E> {
        self.priority.swap_remove(A::position(&self.accessor))
    }
    pub fn access(&self) -> &extension::Extension<C, E> {
        &self.priority[A::position(&self.accessor)]
    }
    pub fn access_mut(&mut self) -> &mut extension::Extension<C, E> {
        &mut self.priority[A::position(&self.accessor)]
    }

    // TODO impl as Into trait ?
    pub fn into_value(self) -> C {
        self.access_owned().value
    }
    pub fn value(&self) -> &C {
        &self.access().value
    }
    pub fn value_mut(&mut self) -> &mut C {
        &mut self.access_mut().value
    }

    pub fn into_extension(self) -> E {
        self.access_owned().extension
    }
    pub fn extension(&self) -> &E {
        &self.access().extension
    }
    pub fn extension_mut(&mut self) -> &mut E {
        &mut self.access_mut().extension
    }
}
impl<C, A, E> Coalesced<C, A, E, Single>
where
    A: priority::Access<Accessor = priority::Accessor<A>>,
{
    pub fn set_extension<E2>(self, extension: E2) -> Coalesced<C, A, E2, Single> {
        let Self {
            mut priority,
            accessor,
            phantom,
        } = self;
        Coalesced {
            priority: vec![extension::Extension::new_with(
                priority.swap_remove(A::position(&accessor)).value,
                extension,
            )],
            accessor,
            phantom,
        }
    }
}
impl<C, A, E> Coalesced<C, A, E, Multiple>
where
    A: priority::Access<Accessor = priority::Accessor<A>>,
{
    pub fn into_single(self) -> Coalesced<C, A, E, Single> {
        let ext = self.access_owned();
        Coalesced::new_with(ext.value, ext.extension)
    }
}

impl<C> Coalesced<C, Prior> {
    pub fn new_prior(coalesce: C) -> Self {
        Coalesced::<C, Prior>::new(coalesce)
    }
}
impl<C, E> Coalesced<C, Prior, E> {
    pub fn new_prior_with(coalesce: C, extension: E) -> Self {
        Coalesced::new_with(coalesce, extension)
    }
}
impl<C, E, L> Coalesced<C, Prior, E, L>
where
    L: priority::sealed::Length,
{
    pub fn posterior(self) -> Coalesced<C, Posterior, E, L> {
        Coalesced {
            priority: self.priority,
            accessor: self.accessor.as_posterior(),
            phantom: std::marker::PhantomData,
        }
    }
}
impl<C> Coalesced<C, Posterior> {
    pub fn new_posterior(coalesce: C) -> Self {
        Coalesced::<C, Posterior>::new(coalesce)
    }
}
impl<C, E> Coalesced<C, Posterior, E> {
    pub fn new_posterior_with(coalesce: C, extension: E) -> Self {
        Coalesced::new_with(coalesce, extension)
    }
}
impl<C, E, L> Coalesced<C, Posterior, E, L>
where
    L: priority::sealed::Length,
{
    pub fn prior(self) -> Coalesced<C, Prior, E, L> {
        Coalesced {
            priority: self.priority,
            accessor: self.accessor.as_prior(),
            phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::extension::Extension;

    use super::*;

    #[test]
    fn test_coalesced_prior_history() {
        let from_file = Coalesced::new_prior(Some("file"));
        let from_env = Coalesced::new_prior(Some("env"));
        let from_cli = Coalesced::new_prior(Some("cli"));

        let config = from_file.register(from_env).register(from_cli);
        assert_eq!(config.unwrap(), "cli");
        assert_eq!(
            config.priority,
            vec![
                Extension::new(Some("file")),
                Extension::new(Some("env")),
                Extension::new(Some("cli")),
            ],
        );
    }

    #[test]
    fn test_coalesced_posterior_history() {
        let from_file = Coalesced::new_posterior(Some("file"));
        let from_env = Coalesced::new_posterior(Some("env"));
        let from_cli = Coalesced::new_posterior(Some("cli"));

        let config = from_file.register(from_env).register(from_cli);
        assert_eq!(config.unwrap(), "file");
        assert_eq!(
            config.priority,
            vec![
                Extension::new(Some("file")),
                Extension::new(Some("env")),
                Extension::new(Some("cli")),
            ],
        );
    }

    #[test]
    fn test_coalesced_prior_history_with_none() {
        let from_file = Coalesced::new_prior(Some("file"));
        let from_env = Coalesced::new_prior(Some("env"));
        let from_cli = Coalesced::new_prior(None);

        let config = from_file.register(from_env).register(from_cli);
        assert_eq!(config.unwrap(), "env");
        assert_eq!(
            config.priority,
            vec![
                Extension::new(Some("file")),
                Extension::new(Some("env")),
                Extension::new(None),
            ],
        );
    }
    #[test]
    fn test_coalesced_posterior_history_with_none() {
        let from_file = Coalesced::new_posterior(None);
        let from_env = Coalesced::new_posterior(Some("env"));
        let from_cli = Coalesced::new_posterior(Some("cli"));

        let config = from_file.register(from_env).register(from_cli);
        assert_eq!(config.unwrap(), "env");
        assert_eq!(
            config.priority,
            vec![
                Extension::new(None),
                Extension::new(Some("env")),
                Extension::new(Some("cli")),
            ],
        );
    }

    #[test]
    fn test_coalesced_switch_prior_to_posterior() {
        let from_file = Coalesced::new_prior(Some("file"));
        let from_env = Coalesced::new_prior(Some("env"));
        let from_cli = Coalesced::new_prior(Some("cli"));

        let config = from_file.register(from_env).register(from_cli);
        assert_eq!(config.unwrap(), "cli");
        assert_eq!(
            config.priority,
            vec![
                Extension::new(Some("file")),
                Extension::new(Some("env")),
                Extension::new(Some("cli")),
            ],
        );
        let config_posterior = config.posterior();
        assert_eq!(config_posterior.unwrap(), "file");
        assert_eq!(
            config_posterior.priority,
            vec![
                Extension::new(Some("file")),
                Extension::new(Some("env")),
                Extension::new(Some("cli")),
            ],
        );
    }
    #[test]
    fn test_coalesced_switch_posterior_to_prior() {
        let from_file = Coalesced::new_posterior(Some("file"));
        let from_env = Coalesced::new_posterior(Some("env"));
        let from_cli = Coalesced::new_posterior(Some("cli"));

        let config = from_file.register(from_env).register(from_cli);
        assert_eq!(config.unwrap(), "file");
        assert_eq!(
            config.priority,
            vec![
                Extension::new(Some("file")),
                Extension::new(Some("env")),
                Extension::new(Some("cli")),
            ],
        );
        let config_prior = config.prior();
        assert_eq!(config_prior.unwrap(), "cli");
        assert_eq!(
            config_prior.priority,
            vec![
                Extension::new(Some("file")),
                Extension::new(Some("env")),
                Extension::new(Some("cli")),
            ],
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
            .register(second)
            .register(third)
            .register(four)
            .register(five)
            .register(six);

        assert_eq!(coalesced.unwrap(), 5);
        assert_eq!(
            coalesced.priority,
            vec![
                Extension::new(None),
                Extension::new(Some(2)),
                Extension::new(Some(3)),
                Extension::new(None),
                Extension::new(Some(5)),
                Extension::new(None),
            ],
        );

        let coalesced = coalesced.posterior();
        assert_eq!(coalesced.unwrap(), 2);
        assert_eq!(
            coalesced.priority,
            vec![
                Extension::new(None),
                Extension::new(Some(2)),
                Extension::new(Some(3)),
                Extension::new(None),
                Extension::new(Some(5)),
                Extension::new(None),
            ],
        );
    }
}
