use crate::Coalesce;

pub trait History<S = Self> {
    fn coalesce(self, other: S) -> Self;
}
pub trait IntoHistory: Sized {
    type History;
    fn into_history(self) -> Self::History;
    fn history_coalesce<S>(self, other: S) -> Self::History
    where
        Self::History: History,
        S: IntoHistory<History = Self::History>,
    {
        self.into_history().coalesce(other.into_history())
    }
}
impl<T: Coalesce> IntoHistory for T {
    type History = Coalesced<T>;
    fn into_history(self) -> Self::History {
        Coalesced::new(self)
    }
}
impl<T> IntoHistory for Coalesced<T> {
    type History = Self;
    fn into_history(self) -> Self::History {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Coalesced<T> {
    history: Vec<T>,
}
impl<S, T> History<S> for Coalesced<T>
where
    S: IntoHistory<History = Coalesced<T>>,
{
    fn coalesce(mut self, other: S) -> Self {
        self.history.extend(other.into_history().history);
        self
    }
}
impl<T: Coalesce> Coalesced<T> {
    pub fn new(value: T) -> Self {
        Self {
            history: vec![value],
        }
    }
    pub fn history(&self) -> &Vec<T> {
        &self.history
    }

    pub fn prior(mut self) -> T {
        let remain = self.history.split_off(1);
        remain
            .into_iter()
            .fold(self.history.swap_remove(0), |c, x| c.coalesce(x))
    }
    pub fn posterior(mut self) -> T {
        let mut tail = self.history.split_off(self.history.len() - 1);
        self.history
            .into_iter()
            .rev()
            .fold(tail.swap_remove(0), |c, x| c.coalesce(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_coalesced() {
        let v1 = Some(1);
        let v2 = None;

        let coalesced = v1.history_coalesce(v2);
        assert_eq!(coalesced.history(), &vec![Some(1), None]);
    }

    #[test]
    fn test_history_coalesced_prior() {
        let v1 = 1;
        let v2 = 2;
        let v3 = 3;

        let coalesced = v1.history_coalesce(v2).coalesce(v3);
        assert_eq!(coalesced.prior(), 3);
    }

    #[test]
    fn test_history_coalesced_posterior() {
        let v1 = 1;
        let v2 = 2;
        let v3 = 3;

        let coalesced = v1.history_coalesce(v2).coalesce(v3);
        assert_eq!(coalesced.posterior(), 1);
    }
}
