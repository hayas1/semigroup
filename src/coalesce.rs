use crate::{
    priority::{
        sealed::{Access, Length},
        Accessor,
    },
    Coalesced, Multiple, Posterior, Prior, Single,
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
    fn set_extension<A, E>(self, extension: E) -> Coalesced<Self::Coalesce, A, E, Single>
    where
        Self: Sized + IntoCoalesced<A, Length = Single>,
        A: Access<Accessor = Accessor<A>>,
    {
        self.into_coalesced().set_extension_impl(extension)
    }
    fn posterior(self) -> Coalesced<Self::Coalesce, Posterior, Self::Extension, Self::Length>
    where
        Self: Sized + IntoCoalesced<Prior>,
        Self::Length: Length,
    {
        self.into_coalesced().posterior_impl()
    }
    fn prior(self) -> Coalesced<Self::Coalesce, Prior, Self::Extension, Self::Length>
    where
        Self: Sized + IntoCoalesced<Posterior>,
        Self::Length: Length,
    {
        self.into_coalesced().prior_impl()
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
impl<T, E, A> IntoCoalesced<A> for Result<T, E>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coalesce_option() {
        let a = Some(1);
        let b = Some(2);
        let c = a.coalesce(b);

        let pri = c.prior();
        assert_eq!(pri.unwrap(), 2);
        let post = pri.posterior();
        assert_eq!(post.unwrap(), 1);
    }
    #[test]
    fn test_coalesce_result() {
        let a = Ok(1);
        let b = Err(2);
        let c = a.coalesce(b);

        let pri = c.prior();
        assert_eq!(pri.unwrap(), 1);
        let post = pri.posterior();
        assert_eq!(post.unwrap(), 1);
    }

    #[test]
    fn test_coalesce_option_extension() {
        let a = Some(1).set_extension("A");
        let b = Some(2).set_extension("B");
        let c = a.coalesce(b);

        let pri = c.prior();
        assert_eq!(pri.value(), &Some(2));
        assert_eq!(pri.extension(), &"B");
        let post = pri.posterior();
        assert_eq!(post.value(), &Some(1));
        assert_eq!(post.extension(), &"A");
    }
    #[test]
    fn test_coalesce_result_extension() {
        let a = Ok(1).set_extension("A");
        let b = Err(2).set_extension("B");
        let c = a.coalesce(b);

        let pri = c.prior();
        assert_eq!(pri.value(), &Ok(1));
        assert_eq!(pri.extension(), &"A");
        let post = pri.posterior();
        assert_eq!(post.value(), &Ok(1));
        assert_eq!(post.extension(), &"A");
    }
}
