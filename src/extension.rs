use crate::Coalesce;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Extension<T, E = ()> {
    value: T,
    extension: E,
}

impl<T, E> Coalesce for Extension<T, E>
where
    T: Coalesce,
{
    fn straight(&self, other: &Self) -> bool {
        self.value.straight(&other.value)
    }
}

impl<T, E> Extension<T, E> {
    pub fn new(value: T, extension: E) -> Self {
        Self { value, extension }
    }
}
