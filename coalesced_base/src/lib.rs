pub mod coalesced;
pub mod libs;

pub trait Coalesce {
    fn prior(self, other: Self) -> Self;
    fn posterior(self, other: Self) -> Self;
}

impl<T> Coalesce for Option<T> {
    fn prior(self, other: Self) -> Self {
        other.or(self)
    }
    fn posterior(self, other: Self) -> Self {
        self.or(other)
    }
}
impl<T, E> Coalesce for Result<T, E> {
    fn prior(self, other: Self) -> Self {
        other.or(self)
    }
    fn posterior(self, other: Self) -> Self {
        self.or(other)
    }
}

pub trait CoalesceOther {}
impl<T: CoalesceOther> Coalesce for T {
    fn prior(self, other: Self) -> Self {
        other
    }
    fn posterior(self, _other: Self) -> Self {
        self
    }
}
impl CoalesceOther for () {}
impl CoalesceOther for bool {}
impl CoalesceOther for char {}
impl CoalesceOther for str {}
impl CoalesceOther for &str {}
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
