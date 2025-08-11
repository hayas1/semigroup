pub mod concat;
pub mod optional;
pub mod replace;

pub trait Strategy<T, X> {
    type WithExt;
    fn apply(left: Self::WithExt, right: Self::WithExt) -> Self::WithExt;
}
