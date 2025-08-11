use crate::{
    extension::{Extension, WithExtension},
    strategy::Strategy,
};

impl<T, X> Extension<X> for Option<T> {
    type WithExt = <Optional as Strategy<Self, X>>::WithExt;

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
        Optional::apply(left, right)
    }
}

pub enum Optional {}
impl<T, X> Strategy<Option<T>, X> for Optional {
    type WithExt = WithExtension<Option<T>, X>;
    fn apply(left: Self::WithExt, right: Self::WithExt) -> Self::WithExt {
        match (&left.value, &right.value) {
            (Some(_), _) | (None, None) => left,
            (None, Some(_)) => right,
        }
    }
}
