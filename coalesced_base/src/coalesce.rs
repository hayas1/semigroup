use crate::extension::Extension;

pub trait Coalesce<C = Self> {
    fn coalesce(self, other: C) -> Self;
}
impl<T: Extension<()>> Coalesce for T {
    fn coalesce(self, other: Self) -> Self {
        let with_unit_ext = T::coalesce(self.with_extension(()), other.with_extension(()));
        T::into_value(with_unit_ext)
    }
}
