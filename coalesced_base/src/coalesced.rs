use crate::Coalesce;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Coalesced<T> {
    history: Vec<T>,
}
impl<T> Coalesce for Coalesced<T> {
    fn coalesce(mut self, other: Self) -> Self {
        self.history.extend(other.history);
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

        let coalesced = Coalesced::new(v1).coalesce(Coalesced::new(v2));
        let confirmed = coalesced.into();

        assert_eq!(confirmed, Some(1));
    }
}
