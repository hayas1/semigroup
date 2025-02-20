// TODO derive
pub trait Coalesce {
    fn coalesce(self, other: Self) -> Self;
}
impl<T> Coalesce for Option<T> {
    fn coalesce(self, other: Self) -> Self {
        other.or(self)
    }
}
impl<T, E> Coalesce for Result<T, E> {
    fn coalesce(self, other: Self) -> Self {
        other.or(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Coalesced<C> {
    priority: std::collections::VecDeque<C>,
}
impl<C> Coalesced<C> {
    pub fn new() -> Self {
        Self {
            priority: Default::default(),
        }
    }
    pub fn push(&mut self, coalesce: C) {
        self.priority.push_back(coalesce);
    }
    pub fn pop(&mut self) -> Option<C> {
        self.priority.pop_back()
    }
    pub fn push_front(&mut self, coalesce: C) {
        self.priority.push_front(coalesce);
    }
    pub fn pop_front(&mut self) -> Option<C> {
        self.priority.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.priority.is_empty()
    }
    pub fn peek(&self) -> Option<&C> {
        self.priority.back()
    }
    pub fn peek_mut(&mut self) -> Option<&mut C> {
        self.priority.back_mut()
    }
    pub fn peek_front(&self) -> Option<&C> {
        self.priority.front()
    }
    pub fn peek_front_mut(&mut self) -> Option<&mut C> {
        self.priority.front_mut()
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
