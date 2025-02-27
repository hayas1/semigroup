use crate::Coalesce;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Extension<T, E = ()> {
    pub(crate) value: T,
    pub(crate) extension: E,
}

impl<T, E> Coalesce for Extension<T, E>
where
    T: Coalesce,
{
    fn straight(&self, other: &Self) -> bool {
        self.value.straight(&other.value)
    }
}

impl<T> Extension<T, ()> {
    // TODO impl as From trait ?
    pub fn new(value: T) -> Self {
        Self::new_with(value, ())
    }
}
impl<T, E> Extension<T, E> {
    pub fn new_with(value: T, extension: E) -> Self {
        Self { value, extension }
    }
}
