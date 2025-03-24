use std::ops::{Deref, DerefMut};

use crate::coalesce::Coalesce;

pub trait Extension: Sized {
    fn extension_prior<X>(base: Extended<Self, X>, other: Extended<Self, X>) -> Extended<Self, X>;
    fn extension_posterior<X>(
        base: Extended<Self, X>,
        other: Extended<Self, X>,
    ) -> Extended<Self, X>;

    fn with_extension<X>(self, extension: X) -> Extended<Self, X> {
        Extended {
            value: self,
            extension,
        }
    }
    fn ex_prior(self, other: Self) -> Self {
        let (s, o) = (self.with_extension(()), other.with_extension(()));
        Self::extension_prior(s, o).into()
    }
    fn ex_posterior(self, other: Self) -> Self {
        let (s, o) = (self.with_extension(()), other.with_extension(()));
        Self::extension_posterior(s, o).into()
    }
}
impl<T: Extension> Coalesce for T {
    fn prior(self, other: Self) -> Self {
        self.ex_prior(other)
    }
    fn posterior(self, other: Self) -> Self {
        self.ex_posterior(other)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum WrapPrim<T> {
    Base(T),
    Other(T),
}
impl<T> Extension for Option<T> {
    fn extension_prior<X>(base: Extended<Self, X>, other: Extended<Self, X>) -> Extended<Self, X> {
        let (s, o) = (
            base.value.map(WrapPrim::Base),
            other.value.map(WrapPrim::Other),
        );
        match s.or(o) {
            Some(WrapPrim::Base(v)) => Some(v).with_extension(base.extension),
            Some(WrapPrim::Other(v)) => Some(v).with_extension(other.extension),
            None => None.with_extension(other.extension),
        }
    }
    fn extension_posterior<X>(
        base: Extended<Self, X>,
        other: Extended<Self, X>,
    ) -> Extended<Self, X> {
        let (s, o) = (
            base.value.map(WrapPrim::Base),
            other.value.map(WrapPrim::Other),
        );
        match s.or(o) {
            Some(WrapPrim::Base(v)) => Some(v).with_extension(base.extension),
            Some(WrapPrim::Other(v)) => Some(v).with_extension(other.extension),
            None => None.with_extension(base.extension),
        }
    }
}
impl<T, E> Extension for Result<T, E> {
    fn extension_prior<X>(base: Extended<Self, X>, other: Extended<Self, X>) -> Extended<Self, X> {
        let (s, o) = (
            base.value.map(WrapPrim::Base).map_err(WrapPrim::Base),
            other.value.map(WrapPrim::Other).map_err(WrapPrim::Other),
        );
        match s.or(o) {
            Ok(WrapPrim::Base(v)) => Ok(v).with_extension(base.extension),
            Ok(WrapPrim::Other(v)) => Ok(v).with_extension(other.extension),
            Err(WrapPrim::Base(e)) => Err(e).with_extension(base.extension),
            Err(WrapPrim::Other(e)) => Err(e).with_extension(other.extension),
        }
    }
    fn extension_posterior<X>(
        base: Extended<Self, X>,
        other: Extended<Self, X>,
    ) -> Extended<Self, X> {
        let (s, o) = (
            base.value.map(WrapPrim::Base).map_err(WrapPrim::Base),
            other.value.map(WrapPrim::Other).map_err(WrapPrim::Other),
        );
        match s.or(o) {
            Ok(WrapPrim::Base(v)) => Ok(v).with_extension(base.extension),
            Ok(WrapPrim::Other(v)) => Ok(v).with_extension(other.extension),
            Err(WrapPrim::Base(e)) => Err(e).with_extension(other.extension),
            Err(WrapPrim::Other(e)) => Err(e).with_extension(base.extension),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Extended<C, X> {
    value: C,
    extension: X,
}
impl<C, X> Extended<C, X> {
    pub fn into(self) -> C {
        self.value
    }
    pub fn into_extension(self) -> X {
        self.extension
    }
    pub fn into_parts(self) -> (C, X) {
        (self.value, self.extension)
    }

    pub fn value_ref(&self) -> &C {
        &self.value
    }
    pub fn extension_ref(&self) -> &X {
        &self.extension
    }

    pub fn value_mut(&mut self) -> &mut C {
        &mut self.value
    }
    pub fn extension_mut(&mut self) -> &mut X {
        &mut self.extension
    }
}
impl<C, X> Deref for Extended<C, X> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        self.value_ref()
    }
}
impl<C, X> DerefMut for Extended<C, X> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value_mut()
    }
}
impl<T: Extension, X> Coalesce for Extended<T, X> {
    fn prior(self, other: Self) -> Self {
        T::extension_prior(self, other)
    }
    fn posterior(self, other: Self) -> Self {
        T::extension_posterior(self, other)
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
