pub trait Coalesce {
    fn coalesce(self, other: Self) -> Self;
}
impl<T: Default + PartialEq> Coalesce for T {
    fn coalesce(self, other: Self) -> Self {
        if self == T::default() {
            other
        } else {
            self
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
