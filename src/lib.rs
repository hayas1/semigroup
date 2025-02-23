pub trait CoalescePrior {
    fn is_other(&self, other: &Self) -> bool;
}
impl<T> CoalescePrior for Option<T> {
    fn is_other(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(_), _) => false,
            _ => true,
        }
    }
}

pub trait CoalescePosterior {
    fn is_other(&self, other: &Self) -> bool;
}
impl<T> CoalescePosterior for Option<T> {
    fn is_other(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(_), None) => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Coalesced<C> {
    priority: Vec<C>,
    accessor: usize,
}

impl<C> std::ops::Deref for Coalesced<C> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        &self.priority[self.accessor]
    }
}
impl<C> std::ops::DerefMut for Coalesced<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.priority[self.accessor]
    }
}
impl<C> Coalesced<C> {
    pub fn new(coalesce: C) -> Self {
        Self {
            priority: vec![coalesce],
            accessor: 0,
        }
    }

    pub fn prior(mut self, other: Self) -> Self {
        self.priority.extend(other.priority);
        self.accessor += other.accessor;
        self
    }
    pub fn posterior(self, mut other: Self) -> Self {
        other.priority.extend(self.priority);
        other.accessor += self.accessor;
        other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coalesced_prior_history() {
        let from_file = Coalesced::new(Some("file"));
        let from_env = Coalesced::new(Some("env"));
        let from_cli = Coalesced::new(Some("cli"));

        let config = from_file.prior(from_env).prior(from_cli);
        assert_eq!(config.unwrap(), "file");
        assert_eq!(
            config.priority,
            vec![Some("file"), Some("env"), Some("cli")]
        );
    }

    #[test]
    fn test_coalesced_posterior_history() {
        let from_file = Coalesced::new(Some("file"));
        let from_env = Coalesced::new(Some("env"));
        let from_cli = Coalesced::new(Some("cli"));

        let config = from_file.posterior(from_env).posterior(from_cli);
        assert_eq!(config.unwrap(), "cli");
        assert_eq!(
            config.priority,
            vec![Some("file"), Some("env"), Some("cli")]
        );
    }
}
