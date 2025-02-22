// TODO derive
pub trait Coalesce {
    fn is_other(&self, other: &Self) -> bool;
    fn coalesce(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.is_other(&other) {
            other
        } else {
            self
        }
    }
}
impl<T> Coalesce for Option<T> {
    fn is_other(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(_), None) => false,
            _ => true,
        }
    }
}
impl<T, E> Coalesce for Result<T, E> {
    fn is_other(&self, other: &Self) -> bool {
        match (self, other) {
            (Ok(_), Err(_)) => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Coalesced<C> {
    history: Vec<C>,
    current: C,
}
impl<C> std::ops::Deref for Coalesced<C> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        &self.current
    }
}
impl<C: Coalesce> Coalesce for Coalesced<C> {
    fn is_other(&self, other: &Self) -> bool {
        self.current.is_other(&other.current)
    }
    fn coalesce(self, other: Self) -> Self {
        let (low, high) = if self.current.is_other(&other.current) {
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
        let target = None;
        let other = Some(1);
        assert_eq!(target.coalesce(other), Some(1));
    }
}
