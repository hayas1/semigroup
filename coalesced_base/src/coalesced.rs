use crate::Coalesce;

pub trait History<S = Self> {
    fn coalesce(self, other: S) -> Self;
}
pub trait IntoHistory<T>: Sized {
    type History;
    fn into_history(self) -> Self::History;
}
impl<T: Coalesce> IntoHistory<T> for T {
    type History = Coalesced<T>;
    fn into_history(self) -> Self::History {
        Coalesced::new(self)
    }
}
impl<T> IntoHistory<T> for Coalesced<T> {
    type History = Self;
    fn into_history(self) -> Self::History {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Coalesced<T> {
    history: Vec<T>,
}
impl<S: IntoHistory<T, History = Coalesced<T>>, T> History<S> for Coalesced<T> {
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
    pub fn into(mut self) -> T {
        let remain = self.history.split_off(1);
        remain
            .into_iter()
            .fold(self.history.swap_remove(0), |c, x| c.coalesce(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coalesced() {
        let v1 = Some(1);
        let v2 = None;

        let coalesced = Coalesced::new(v1).coalesce(v2.into_history());
        let confirmed = coalesced.into();

        assert_eq!(confirmed, Some(1));
    }
}
