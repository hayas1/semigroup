use crate::{
    priority::{
        sealed::{Access, Length},
        Accessor,
    },
    Coalesced, Multiple, Single,
};

pub trait Coalesce {
    fn straight(&self, other: &Self) -> bool;
    fn coalesce<A, L>(self, other: Coalesced<Self, A, (), L>) -> Coalesced<Self, A, (), Multiple>
    where
        Self: Sized,
        A: Access<Accessor = Accessor<A>>,
        L: Length,
    {
        Coalesced::new(self).coalesce(other)
    }
    fn set_extension<A, E>(self, extension: E) -> Coalesced<Self, A, E, Single>
    where
        Self: Sized,
        A: Access<Accessor = Accessor<A>>,
    {
        Coalesced::new(self).set_extension(extension)
    }
}
impl<T> Coalesce for Option<T> {
    fn straight(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(_), _) => true,
            _ => false,
        }
    }
}
impl<T, E> Coalesce for Result<T, E> {
    fn straight(&self, other: &Self) -> bool {
        match (self, other) {
            (Ok(_), _) => true,
            _ => false,
        }
    }
}
