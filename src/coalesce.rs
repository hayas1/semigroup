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

pub trait IntoCoalesced<A = Prior> {
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
impl<T, A> IntoCoalesced<A> for Option<T> {
    type Coalesce = Self;
    type Extension = ();
    type Length = Single;
    fn into_coalesced(self) -> Coalesced<Self::Coalesce, A, Self::Extension, Self::Length> {
        Coalesced::new(self)
    }
}
impl<T, E, A> IntoCoalesced<A> for Result<T, E> {
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
        let c = None;
        let coalesce = a.coalesce(b).coalesce(c).prior();
        assert_eq!(coalesce.unwrap(), 2);
    }
    #[test]
    fn test_coalesce_result() {
        let a = Ok(1);
        let b = Ok(2);
        let c = Err(3);
        let coalesce = a.coalesce(b).coalesce(c).prior();
        assert_eq!(coalesce.unwrap(), 2);
    }

    #[test]
    fn test_coalesce_option_extension() {
        let a = Some(1).set_extension("A");
        let b = Some(2).set_extension("B");
        let c = None.set_extension("C");
        let coalesce = a.coalesce(b).coalesce(c).prior();
        assert_eq!(coalesce.value(), &Some(2));
        assert_eq!(coalesce.extension(), &"B");
    }
    #[test]
    fn test_coalesce_result_extension() {
        let a = Ok(1).set_extension("A");
        let b = Ok(2).set_extension("B");
        let c = Err(3).set_extension("C");
        let coalesced = a.coalesce(b).coalesce(c).prior();
        assert_eq!(coalesced.value(), &Ok(2));
        assert_eq!(coalesced.extension(), &"B");
    }

    #[test]
    fn test_coalesce_option_prior_posterior() {
        let a = Some(1);
        let b = Some(2);
        let c = None;
        let coalesce = a.coalesce(b).coalesce(c);
        assert_eq!(coalesce.unwrap(), 2);

        let posterior = coalesce.posterior();
        assert_eq!(posterior.unwrap(), 1);
        let prior = posterior.prior();
        assert_eq!(prior.unwrap(), 2);
    }
    #[test]
    fn test_coalesce_result_prior_posterior() {
        let a = Ok(1);
        let b = Ok(2);
        let c = Err(3);
        let coalesce = a.coalesce(b).coalesce(c);
        assert_eq!(coalesce.unwrap(), 2);

        let posterior = coalesce.posterior();
        assert_eq!(posterior.unwrap(), 1);
        let prior = posterior.prior();
        assert_eq!(prior.unwrap(), 2);
    }

    #[test]
    fn test_posterior_coalesce_option() {
        let posterior = Some(1).posterior();
        let coalesce = posterior.coalesce(Some(2));

        assert_eq!(coalesce.unwrap(), 1);
    }
}
