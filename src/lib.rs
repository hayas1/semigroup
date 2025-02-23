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
    pub fn prior(mut self, other: Self) -> Self
    where
        C: Coalesce,
    {
        let base_len = self.priority.len();
        self.priority.extend(other.priority);
        self.prior_accessor = base_len + other.prior_accessor;
        for i in (1..=self.prior_accessor).rev() {
            if !self.priority[i].straight(&self.priority[i - 1]) {
                self.prior_accessor = i - 1;
            } else {
                break;
            }
        }
        self.posterior_accessor = other.posterior_accessor;
        for i in 0..base_len + other.posterior_accessor {
            if !self.priority[i].straight(&self.priority[i + 1]) {
                self.posterior_accessor = i + 1;
            } else {
                break;
            }
        }
        self
    }
    pub fn posterior(self, mut other: Self) -> Self
    where
        C: Coalesce,
    {
        let base_len = other.priority.len();
        other.priority.extend(self.priority);
        other.prior_accessor = base_len + self.prior_accessor;
        for i in (1..=other.prior_accessor).rev() {
            if !other.priority[i].straight(&other.priority[i - 1]) {
                other.prior_accessor = i - 1;
            } else {
                break;
            }
        }
        other.posterior_accessor = self.posterior_accessor;
        for i in 0..base_len + self.posterior_accessor {
            if !other.priority[i].straight(&other.priority[i + 1]) {
                other.posterior_accessor = i + 1;
            } else {
                break;
            }
        }
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
        assert_eq!(config.unwrap(), "cli");
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

    #[test]
    fn test_coalesced_complex_prior_posterior() {
        let first = Coalesced::new(None);
        let second = Coalesced::new(Some(2));
        let third = Coalesced::new(Some(3));
        let four = Coalesced::new(None);
        let five = Coalesced::new(Some(5));
        let six = Coalesced::new(None);

        let coalesced = first
            .prior(second)
            .prior(third)
            .prior(four)
            .prior(five)
            .prior(six);

        let coalesced = coalesced.prior_access();
        assert_eq!(coalesced.unwrap(), 5);
        assert_eq!(
            coalesced.priority,
            vec![None, Some(2), Some(3), None, Some(5), None]
        );

        let coalesced = coalesced.posterior_access();
        assert_eq!(coalesced.unwrap(), 2);
        assert_eq!(
            coalesced.priority,
            vec![None, Some(2), Some(3), None, Some(5), None]
        );
    }
}
