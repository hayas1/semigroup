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
    history: Vec<C>,
    current: C,
}
impl<C: Coalesce> Coalesce for Coalesced<C> {
    fn coalesce(self, other: Self) -> Self {
        let mut coalesced = Self {
            history: self.history,
            current: self.current.coalesce(other.current),
        };
        coalesced.history.extend(other.history);
        coalesced
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
        let target = None;
        let other = Some(1);
        assert_eq!(target.coalesce(other), Some(1));
    }
}
