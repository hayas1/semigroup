use crate::{
    priority::{
        sealed::{Access, Length},
        Accessor,
    },
    Coalesced, Multiple, Single,
};

pub trait Coalesce {
    fn straight(&self, other: &Self) -> bool;
    fn coalesce<A, L, T>(self, other: T) -> Coalesced<Self, A, T::Extension, Multiple>
    where
        Self: Sized + IntoCoalesced<A, Coalesce = Self, Extension = T::Extension, Length = L>,
        A: Access<Accessor = Accessor<A>>,
        L: Length,
        T: CoalesceTarget<A, L, Coalesce = Self>,
    {
        other.coalesce_target(self.into_coalesced())
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

pub trait CoalesceTarget<A, L>
where
    A: Access<Accessor = Accessor<A>>,
    L: Length,
{
    type Coalesce;
    type Extension;
    fn coalesce_target(
        self,
        base: Coalesced<Self::Coalesce, A, Self::Extension, L>,
    ) -> Coalesced<Self::Coalesce, A, Self::Extension, Multiple>;
}
impl<C, A, E, L> CoalesceTarget<A, L> for Coalesced<C, A, E, L>
where
    A: Access<Accessor = Accessor<A>>,
    C: Coalesce,
    L: Length,
{
    type Coalesce = C;
    type Extension = E;
    fn coalesce_target(
        self,
        base: Coalesced<Self::Coalesce, A, Self::Extension, L>,
    ) -> Coalesced<Self::Coalesce, A, Self::Extension, Multiple> {
        base.coalesce(self)
    }
}
impl<T, A, L> CoalesceTarget<A, L> for Option<T>
where
    A: Access<Accessor = Accessor<A>>,
    L: Length,
{
    type Coalesce = Self;
    type Extension = ();
    fn coalesce_target(
        self,
        base: Coalesced<Self::Coalesce, A, Self::Extension, L>,
    ) -> Coalesced<Self::Coalesce, A, Self::Extension, Multiple> {
        base.coalesce(Coalesced::new(self))
    }
}

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
