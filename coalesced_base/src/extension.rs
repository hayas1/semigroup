use std::ops::{Deref, DerefMut};

use crate::coalesce::Coalesce;

pub trait Extension<X>: Sized {
    type WithExt<'a>
    where
        X: 'a;
    fn with_extension(self, extension: &X) -> Self::WithExt<'_>;
    fn from_extension(with_ext: Self::WithExt<'_>) -> Self;
    fn ex_prior<'a>(base: Self::WithExt<'a>, other: Self::WithExt<'a>) -> Self::WithExt<'a>;
    fn ex_posterior<'a>(base: Self::WithExt<'a>, other: Self::WithExt<'a>) -> Self::WithExt<'a>;
}
impl<'a, T: Extension<()>> Coalesce for T
where
    T::WithExt<'a>: Coalesce,
{
    fn prior(self, other: Self) -> Self {
        let (s, o) = (self.with_extension(&()), other.with_extension(&()));
        Self::from_extension(s.prior(o))
    }
    fn posterior(self, other: Self) -> Self {
        let (s, o) = (self.with_extension(&()), other.with_extension(&()));
        Self::from_extension(s.posterior(o))
    }
}
enum ExEither<T> {
    Base(T),
    Other(T),
}
impl<T, X> Extension<X> for Option<T> {
    type WithExt<'a>
        = WithExt<'a, Self, X>
    where
        X: 'a;
    fn with_extension(self, extension: &X) -> Self::WithExt<'_> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn from_extension(with_ext: Self::WithExt<'_>) -> Self {
        with_ext.value
    }
    fn ex_prior<'a>(base: Self::WithExt<'a>, other: Self::WithExt<'a>) -> Self::WithExt<'a> {
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
    fn ex_posterior<'a>(base: Self::WithExt<'a>, other: Self::WithExt<'a>) -> Self::WithExt<'a> {
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
impl<T, E, X> Extension<X> for Result<T, E> {
    type WithExt<'a>
        = WithExt<'a, Self, X>
    where
        X: 'a;
    fn with_extension(self, extension: &X) -> Self::WithExt<'_> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn from_extension(with_ext: Self::WithExt<'_>) -> Self {
        with_ext.value
    }
    fn ex_prior<'a>(base: Self::WithExt<'a>, other: Self::WithExt<'a>) -> Self::WithExt<'a> {
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
    fn ex_posterior<'a>(base: Self::WithExt<'a>, other: Self::WithExt<'a>) -> Self::WithExt<'a> {
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

/// A value with an extension
///
/// # Examples
/// An instance can be created with [Extension::with_extension].
/// ```
/// use coalesced_base::extension::Extension;
/// let ext = Some(100).with_extension(&"ext");
/// assert_eq!(*ext, Some(100));
/// assert_eq!(ext.extension, &"ext");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WithExt<'a, T, X> {
    pub value: T,
    pub extension: &'a X,
}
impl<T, X> Deref for WithExt<'_, T, X> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T, X> DerefMut for WithExt<'_, T, X> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
impl<'a, T: Extension<X, WithExt<'a> = Self>, X> Coalesce for WithExt<'a, T, X> {
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
        let file = Some(1).with_extension(&"file");
        let env = Some(10).with_extension(&"env");
        let cli = None.with_extension(&"cli");

        let coalesced = file.prior(env).prior(cli);
        assert_eq!(*coalesced, Some(10));
        assert_eq!(coalesced.unwrap(), 10);
        assert_eq!(coalesced.extension, &"env");
    }

    #[test]
    fn test_option_posterior_extension() {
        let file = Some(1).with_extension(&"file");
        let env = Some(10).with_extension(&"env");
        let cli = None.with_extension(&"cli");

        let coalesced = file.posterior(env).posterior(cli);
        assert_eq!(*coalesced, Some(1));
        assert_eq!(coalesced.unwrap(), 1);
        assert_eq!(coalesced.extension, &"file");
    }

    #[test]
    fn test_result_prior_extension() {
        let file = Ok(1).with_extension(&"file");
        let env = Ok(10).with_extension(&"env");
        let cli = Err(1).with_extension(&"cli");

        let coalesced = file.prior(env).prior(cli);
        assert_eq!(*coalesced, Ok(10));
        assert_eq!(coalesced.unwrap(), 10);
        assert_eq!(coalesced.extension, &"env");
    }

    #[test]
    fn test_result_posterior_extension() {
        let file = Ok(1).with_extension(&"file");
        let env = Ok(10).with_extension(&"env");
        let cli = Err(1).with_extension(&"cli");

        let coalesced = file.posterior(env).posterior(cli);
        assert_eq!(*coalesced, Ok(1));
        assert_eq!(coalesced.unwrap(), 1);
        assert_eq!(coalesced.extension, &"file");
    }
}
