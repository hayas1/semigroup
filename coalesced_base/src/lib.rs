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

pub trait CoalesceOther {}
impl<T: CoalesceOther> Coalesce for T {
    fn coalesce(self, other: Self) -> Self {
        other
    }
}
impl CoalesceOther for () {}
impl CoalesceOther for bool {}
impl CoalesceOther for char {}
impl CoalesceOther for str {}
impl CoalesceOther for String {}
impl CoalesceOther for u8 {}
impl CoalesceOther for u16 {}
impl CoalesceOther for u32 {}
impl CoalesceOther for u64 {}
impl CoalesceOther for u128 {}
impl CoalesceOther for i8 {}
impl CoalesceOther for i16 {}
impl CoalesceOther for i32 {}
impl CoalesceOther for i64 {}
impl CoalesceOther for i128 {}
impl CoalesceOther for f32 {}
impl CoalesceOther for f64 {}
