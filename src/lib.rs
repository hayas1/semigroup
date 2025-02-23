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
    prior_accessor: usize,
    posterior_accessor: usize,
    is_prior: bool,
}

impl<C> std::ops::Deref for Coalesced<C> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        if self.is_prior {
            &self.priority[self.prior_accessor]
        } else {
            &self.priority[self.posterior_accessor]
        }
    }
}
impl<C> std::ops::DerefMut for Coalesced<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.is_prior {
            &mut self.priority[self.prior_accessor]
        } else {
            &mut self.priority[self.posterior_accessor]
        }
    }
}
impl<C> Coalesced<C> {
    pub fn new(coalesce: C) -> Self {
        Self {
            priority: vec![coalesce],
            prior_accessor: 0,
            posterior_accessor: 0,
            is_prior: true,
        }
    }
    pub fn confirm(mut self) -> C {
        if self.is_prior {
            self.priority.swap_remove(self.prior_accessor)
        } else {
            self.priority.swap_remove(self.posterior_accessor)
        }
    }

    // TODO impl trait for Option<T> ?
    pub fn prior(mut self, other: Self) -> Self {
        self.priority.extend(other.priority);
        self.prior_accessor += other.prior_accessor + 1;
        self.posterior_accessor = other.posterior_accessor;
        self
    }
    pub fn posterior(self, mut other: Self) -> Self {
        other.priority.extend(self.priority);
        other.prior_accessor += self.prior_accessor + 1;
        other.posterior_accessor = self.posterior_accessor;
        other
    }

    pub fn prior_access(self) -> Self {
        Self {
            is_prior: true,
            ..self
        }
    }
    pub fn posterior_access(self) -> Self {
        Self {
            is_prior: false,
            ..self
        }
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
            vec![Some("file"), Some("env"), Some("cli")],
        );
    }

    #[test]
    fn test_coalesced_posterior_history() {
        let from_file = Coalesced::new(Some("file"));
        let from_env = Coalesced::new(Some("env"));
        let from_cli = Coalesced::new(Some("cli"));

        let config = from_file.prior(from_env).prior(from_cli);
        assert_eq!(config.unwrap(), "cli");
        assert_eq!(
            config.priority,
            vec![Some("file"), Some("env"), Some("cli")],
        );

        let config = config.posterior_access();
        assert_eq!(config.unwrap(), "file");
        assert_eq!(
            config.priority,
            vec![Some("file"), Some("env"), Some("cli")],
        );
    }
}
