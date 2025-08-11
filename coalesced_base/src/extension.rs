use crate::coalesce::Coalesce;

pub trait Extension<X> {
    type WithExt;
    fn with_extension(self, extension: X) -> Self::WithExt;
    fn into_value(ext: Self::WithExt) -> Self;
    fn coalesce(left: Self::WithExt, right: Self::WithExt) -> Self::WithExt;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct WithExtension<T, X> {
    pub value: T,
    pub extension: X,
}
impl<T, X> Coalesce for WithExtension<T, X>
where
    T: Extension<X, WithExt = WithExtension<T, X>>,
{
    fn coalesce(self, other: Self) -> Self {
        T::coalesce(self, other)
    }
}
