use crate::{
    extension::{Extension, WithExtension},
    strategy::Strategy,
};

impl<X> Extension<X> for () {
    type WithExt = <Replace as Strategy<Self, X>>::WithExt;

    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExtension {
            value: self,
            extension,
        }
    }
    fn into_value(ext: Self::WithExt) -> Self {
        ext.value
    }
    fn coalesce(left: Self::WithExt, right: Self::WithExt) -> Self::WithExt {
        Replace::apply(left, right)
    }
}

pub enum Replace {}
impl<T, X> Strategy<T, X> for Replace {
    type WithExt = WithExtension<T, X>;
    fn apply(_left: Self::WithExt, right: Self::WithExt) -> Self::WithExt {
        right
    }
}
