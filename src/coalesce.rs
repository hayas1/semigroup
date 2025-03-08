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
        let (sc, oc) = (self.into_coalesced(), other.into_coalesced());
        sc.coalesce_impl(oc)
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
    fn with_extension<A, E>(self, extension: E) -> Coalesced<Self::Coalesce, A, E, Single>
    where
        Self: Sized + IntoCoalesced<A, Length = Single>,
        A: Access<Accessor = Accessor<A>>,
    {
        self.into_coalesced().with_extension_impl(extension)
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
        let a = Some(1).with_extension("A");
        let b = Some(2).with_extension("B");
        let c = None.with_extension("C");
        let coalesce = a.coalesce(b).coalesce(c).prior();
        assert_eq!(coalesce.value(), &Some(2));
        assert_eq!(coalesce.extension(), &"B");
    }
    #[test]
    fn test_coalesce_result_extension() {
        let a = Ok(1).with_extension("A");
        let b = Ok(2).with_extension("B");
        let c = Err(3).with_extension("C");
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
    fn test_option_coalesce_matrix() {
        let option1 = Some(1);
        let option2 = Some(2);
        let prior3 = Some(3).prior();
        let prior4 = Some(4).prior();
        let posterior5 = Some(5).posterior();
        let posterior6 = Some(6).posterior();

        let opt_opt_prior = option1.clone().coalesce(option2.clone()).prior();
        assert_eq!(opt_opt_prior.unwrap(), 2);
        let opt_opt_posterior = option1.clone().coalesce(option2.clone()).posterior();
        assert_eq!(opt_opt_posterior.unwrap(), 1);

        let opt_prior = option1.clone().coalesce(prior3.clone());
        assert_eq!(opt_prior.unwrap(), 3);
        let opt_posterior = option1.clone().coalesce(posterior5.clone());
        assert_eq!(opt_posterior.unwrap(), 1);

        let prior_opt = prior3.clone().coalesce(option1.clone());
        assert_eq!(prior_opt.unwrap(), 1);
        let posterior_opt = posterior5.clone().coalesce(option1.clone());
        assert_eq!(posterior_opt.unwrap(), 5);

        let prior_prior = prior3.clone().coalesce(prior4.clone());
        assert_eq!(prior_prior.unwrap(), 4);
        let posterior_posterior = posterior5.clone().coalesce(posterior6.clone());
        assert_eq!(posterior_posterior.unwrap(), 5);
    }
}
