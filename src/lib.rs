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
pub struct Coalesced<C>(std::collections::VecDeque<C>);
impl<C> Coalesce for Coalesced<C> {
    fn coalesce(self, other: Self) -> Self {
        Self(self.0.into_iter().chain(other.0).collect())
    }
}
impl<C> std::ops::Deref for Coalesced<C> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        &self
            .0
            .back()
            .unwrap_or_else(|| unreachable!("Coalesced must be non-empty"))
    }
}
impl<C> std::ops::DerefMut for Coalesced<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
            .back_mut()
            .unwrap_or_else(|| unreachable!("Coalesced must be non-empty"))
    }
}
impl<C> Coalesced<C> {
    pub fn new(coalesce: C) -> Self {
        Self(vec![coalesce].into_iter().collect())
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
