use crate::{
    priority::{
        sealed::{Access, Length},
        Accessor,
    },
    Coalesced, Multiple, Single,
};

pub trait Straight {
    fn straight(&self, other: &Self) -> bool;
}
impl<T> Straight for Option<T> {
    fn straight(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(_), _) => true,
            _ => false,
        }
    }
}
impl<T, E> Straight for Result<T, E> {
    fn straight(&self, other: &Self) -> bool {
        match (self, other) {
            (Ok(_), _) => true,
            _ => false,
        }
    }
}

pub trait Coalesce<C, A, E>
where
    C: Straight,
    A: Access<Accessor = Accessor<A>>,
{
    fn coalesce<T>(self, other: T) -> Coalesced<C, A, E, Multiple>
    where
        Self: Sized + IntoCoalesced<A, Coalesce = C, Extension = E>,
        T: IntoCoalesced<A, Coalesce = C, Extension = E>,
    {
        self.into_coalesced().coalesce_impl(other.into_coalesced())
    }
}
impl<T, A, E> Coalesce<Self, A, E> for Option<T> where A: Access<Accessor = Accessor<A>> {}
impl<T, Err, A, E> Coalesce<Self, A, E> for Result<T, Err> where A: Access<Accessor = Accessor<A>> {}
impl<C, A, E, L> Coalesce<C, A, E> for Coalesced<C, A, E, L>
where
    C: Straight,
    A: Access<Accessor = Accessor<A>>,
    L: Length,
{
}

pub trait CoalesceExt {
    fn set_extension<A, E>(self, extension: E) -> Coalesced<Self, A, E, Single>
    where
        Self: Sized,
        A: Access<Accessor = Accessor<A>>,
    {
        Coalesced::new(self).set_extension(extension)
    }
}
impl<T> CoalesceExt for Option<T> {}
impl<T, E> CoalesceExt for Result<T, E> {}
impl<C, A, E, L> CoalesceExt for Coalesced<C, A, E, L> where L: Length {}

pub trait IntoCoalesced<A> {
    type Coalesce;
    type Extension;
    type Length: Length;
    fn into_coalesced(self) -> Coalesced<Self::Coalesce, A, Self::Extension, Self::Length>;
}
impl<C, A, E, L> IntoCoalesced<A> for Coalesced<C, A, E, L>
where
    L: Length,
{
    type Coalesce = C;
    type Extension = E;
    type Length = L;
    fn into_coalesced(self) -> Coalesced<Self::Coalesce, A, Self::Extension, Self::Length> {
        self
    }
}
impl<T, A> IntoCoalesced<A> for Option<T>
where
    A: Access<Accessor = Accessor<A>>,
{
    type Coalesce = Self;
    type Extension = ();
    type Length = Single;
    fn into_coalesced(self) -> Coalesced<Self::Coalesce, A, Self::Extension, Self::Length> {
        Coalesced::new(self)
    }
}
