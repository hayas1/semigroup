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
