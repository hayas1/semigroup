pub trait Coalesce {
    fn coalesce(self, other: Self) -> Self;
}

impl<T> Coalesce for Option<T> {
    fn coalesce(self, other: Option<T>) -> Option<T> {
        self.or(other)
    }
}
impl<T, E> Coalesce for Result<T, E> {
    fn coalesce(self, other: Result<T, E>) -> Result<T, E> {
        self.or(other)
    }
}
