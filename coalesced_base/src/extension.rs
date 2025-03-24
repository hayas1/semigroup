use std::ops::{Deref, DerefMut};

use crate::coalesce::Coalesce;

pub trait CoalesceExtension: Sized {
    fn ex_prior<X>(base: Extension<Self, X>, other: Extension<Self, X>) -> Extension<Self, X>;
    fn ex_posterior<X>(base: Extension<Self, X>, other: Extension<Self, X>) -> Extension<Self, X>;

    fn with_extension<X>(self, extension: X) -> Extension<Self, X> {
        Extension {
            value: self,
            extension,
        }
    }
}
impl<T: CoalesceExtension> Coalesce for T {
    fn prior(self, other: Self) -> Self {
        let (s, o) = (self.with_extension(()), other.with_extension(()));
        s.prior(o).into()
    }
    fn posterior(self, other: Self) -> Self {
        let (s, o) = (self.with_extension(()), other.with_extension(()));
        s.posterior(o).into()
    }
}
enum ExEither<T> {
    Base(T),
    Other(T),
}
impl<T> CoalesceExtension for Option<T> {
    fn ex_prior<X>(base: Extension<Self, X>, other: Extension<Self, X>) -> Extension<Self, X> {
        let (s, o) = (
            base.value.map(ExEither::Base),
            other.value.map(ExEither::Other),
        );
        match o.or(s) {
            Some(ExEither::Base(v)) => Some(v).with_extension(base.extension),
            Some(ExEither::Other(v)) => Some(v).with_extension(other.extension),
            None => None.with_extension(other.extension),
        }
    }
    fn ex_posterior<X>(base: Extension<Self, X>, other: Extension<Self, X>) -> Extension<Self, X> {
        let (s, o) = (
            base.value.map(ExEither::Base),
            other.value.map(ExEither::Other),
        );
        match s.or(o) {
            Some(ExEither::Base(v)) => Some(v).with_extension(base.extension),
            Some(ExEither::Other(v)) => Some(v).with_extension(other.extension),
            None => None.with_extension(base.extension),
        }
    }
}
impl<T, E> CoalesceExtension for Result<T, E> {
    fn ex_prior<X>(base: Extension<Self, X>, other: Extension<Self, X>) -> Extension<Self, X> {
        let (s, o) = (
            base.value.map(ExEither::Base).map_err(ExEither::Base),
            other.value.map(ExEither::Other).map_err(ExEither::Other),
        );
        match o.or(s) {
            Ok(ExEither::Base(v)) => Ok(v).with_extension(base.extension),
            Ok(ExEither::Other(v)) => Ok(v).with_extension(other.extension),
            Err(ExEither::Base(e)) => Err(e).with_extension(base.extension),
            Err(ExEither::Other(e)) => Err(e).with_extension(other.extension),
        }
    }
    fn ex_posterior<X>(base: Extension<Self, X>, other: Extension<Self, X>) -> Extension<Self, X> {
        let (s, o) = (
            base.value.map(ExEither::Base).map_err(ExEither::Base),
            other.value.map(ExEither::Other).map_err(ExEither::Other),
        );
        match s.or(o) {
            Ok(ExEither::Base(v)) => Ok(v).with_extension(base.extension),
            Ok(ExEither::Other(v)) => Ok(v).with_extension(other.extension),
            Err(ExEither::Base(e)) => Err(e).with_extension(other.extension),
            Err(ExEither::Other(e)) => Err(e).with_extension(base.extension),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Extension<T, X> {
    value: T,
    extension: X,
}
impl<T, X> Extension<T, X> {
    pub fn into(self) -> T {
        self.value
    }
    pub fn into_extension(self) -> X {
        self.extension
    }
    pub fn into_parts(self) -> (T, X) {
        (self.value, self.extension)
    }

    pub fn value_ref(&self) -> &T {
        &self.value
    }
    pub fn extension_ref(&self) -> &X {
        &self.extension
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
    pub fn extension_mut(&mut self) -> &mut X {
        &mut self.extension
    }
}
impl<T, X> Deref for Extension<T, X> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.value_ref()
    }
}
impl<T, X> DerefMut for Extension<T, X> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value_mut()
    }
}
impl<T: CoalesceExtension, X> Coalesce for Extension<T, X> {
    fn prior(self, other: Self) -> Self {
        T::ex_prior(self, other)
    }
    fn posterior(self, other: Self) -> Self {
        T::ex_posterior(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_prior_extension() {
        let file = Some(1).with_extension("file");
        let env = Some(10).with_extension("env");
        let cli = None.with_extension("cli");

        let coalesced = file.prior(env).prior(cli);
        assert_eq!(*coalesced, Some(10));
        assert_eq!(coalesced.unwrap(), 10);
        assert_eq!(coalesced.extension_ref(), &"env");
    }

    #[test]
    fn test_option_posterior_extension() {
        let file = Some(1).with_extension("file");
        let env = Some(10).with_extension("env");
        let cli = None.with_extension("cli");

        let coalesced = file.posterior(env).posterior(cli);
        assert_eq!(*coalesced, Some(1));
        assert_eq!(coalesced.unwrap(), 1);
        assert_eq!(coalesced.extension_ref(), &"file");
    }

    #[test]
    fn test_result_prior_extension() {
        let file = Ok(1).with_extension("file");
        let env = Ok(10).with_extension("env");
        let cli = Err(1).with_extension("cli");

        let coalesced = file.prior(env).prior(cli);
        assert_eq!(*coalesced, Ok(10));
        assert_eq!(coalesced.unwrap(), 10);
        assert_eq!(coalesced.extension_ref(), &"env");
    }

    #[test]
    fn test_result_posterior_extension() {
        let file = Ok(1).with_extension("file");
        let env = Ok(10).with_extension("env");
        let cli = Err(1).with_extension("cli");

        let coalesced = file.posterior(env).posterior(cli);
        assert_eq!(*coalesced, Ok(1));
        assert_eq!(coalesced.unwrap(), 1);
        assert_eq!(coalesced.extension_ref(), &"file");
    }
}
