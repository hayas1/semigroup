use crate::{
    extension::{Extension, WithExtension},
    strategy::Strategy,
};

impl<T, X: Clone> Extension<X> for Vec<T> {
    type WithExt = <Concat as Strategy<T, X>>::WithExt;

    fn with_extension(self, extension: X) -> Self::WithExt {
        self.into_iter()
            .map(|value| WithExtension {
                value,
                extension: extension.clone(),
            })
            .collect()
    }
    fn into_value(ext: Self::WithExt) -> Self {
        ext.into_iter().map(|we| we.value).collect()
    }
    fn coalesce(left: Self::WithExt, right: Self::WithExt) -> Self::WithExt {
        Concat::apply(left, right)
    }
}
pub enum Concat {}
impl<T, X> Strategy<T, X> for Concat {
    type WithExt = Vec<WithExtension<T, X>>; // TODO any other Vec
    fn apply(_left: Self::WithExt, right: Self::WithExt) -> Self::WithExt {
        right
    }
}
