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

pub trait Coalesce: CoalescePrior + CoalescePosterior {
    fn prior(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if <Self as CoalescePrior>::is_other(&self, &other) {
            other
        } else {
            self
        }
    }

    fn posterior(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if <Self as CoalescePosterior>::is_other(&self, &other) {
            other
        } else {
            self
        }
    }
}
impl<T> Coalesce for Option<T> {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Coalesced<C> {
    history: Vec<C>,
    current: C,
}

impl<C: CoalescePrior> CoalescePrior for Coalesced<C> {
    fn is_other(&self, other: &Self) -> bool {
        <C as CoalescePrior>::is_other(&self.current, &other.current)
    }
}
impl<C: CoalescePosterior> CoalescePosterior for Coalesced<C> {
    fn is_other(&self, other: &Self) -> bool {
        <C as CoalescePosterior>::is_other(&self.current, &other.current)
    }
}
impl<C: Coalesce> Coalesce for Coalesced<C> {
    fn prior(self, other: Self) -> Self {
        let (low, high) = if <Self as CoalescePrior>::is_other(&self, &other) {
            (self, other)
        } else {
            (other, self)
        };
        let (mut history, current) = (low.history, high.current);
        history.extend(high.history);
        history.push(low.current);
        Coalesced { history, current }
    }

    fn posterior(self, other: Self) -> Self {
        let (low, high) = if <Self as CoalescePosterior>::is_other(&self, &other) {
            (self, other)
        } else {
            (other, self)
        };
        let (mut history, current) = (low.history, high.current);
        history.extend(high.history);
        history.push(low.current);
        Coalesced { history, current }
    }
}
impl<C> std::ops::Deref for Coalesced<C> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        &self.current
    }
}
impl<C> std::ops::DerefMut for Coalesced<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.current
    }
}
impl<C> Coalesced<C> {
    pub fn new(coalesce: C) -> Self {
        Self {
            history: Vec::new(),
            current: coalesce,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coalesce_trait() {
        let none = None;
        let some1 = Some(1);
        let some2 = Some(2);

        assert_eq!(none.prior(some1).prior(some2), some1);
        assert_eq!(none.posterior(some1).posterior(some2), some2);
    }

    #[test]
    fn test_coalesced_prior_history() {
        let from_file = Coalesced::new(Some("file"));
        let from_env = Coalesced::new(Some("env"));
        let from_cli = Coalesced::new(Some("cli"));

        let config = from_file.prior(from_env).prior(from_cli);
        assert_eq!(config.unwrap(), "file");
        assert_eq!(config.history, vec![Some("env"), Some("cli")]);
    }

    #[test]
    fn test_coalesced_posterior_history() {
        let from_file = Coalesced::new(Some("file"));
        let from_env = Coalesced::new(Some("env"));
        let from_cli = Coalesced::new(Some("cli"));

        let config = from_file.posterior(from_env).posterior(from_cli);
        assert_eq!(config.unwrap(), "cli");
        assert_eq!(config.history, vec![Some("file"), Some("env")]);
    }
}
