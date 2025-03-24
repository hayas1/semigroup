use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::coalesce::Coalesce;

pub trait Extension<T>: Sized {
    type Output<X>: Coalesce + Deref<Target = Self> + DerefMut<Target = Self>;
    fn with_extension<X>(self, extension: X) -> Self::Output<X>;
}
impl<T> Extension<T> for Option<T> {
    type Output<X> = Extended<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::Output<X> {
        Extended {
            value: self,
            extension,
        }
    }
}
impl<T, E> Extension<T> for Result<T, E> {
    type Output<X> = Extended<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::Output<X> {
        Extended {
            value: self,
            extension,
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
impl<T, X> Coalesce for Extended<Option<T>, X> {
    fn prior(self, other: Self) -> Self {
        let (se, oe) = (Rc::new(self.extension), Rc::new(other.extension));
        let (s, o) = (
            self.value.map(|v| (v, se.clone())),
            other.value.map(|v| (v, oe.clone())),
        );

        match s.prior(o) {
            Some((v, x)) => {
                drop(se);
                drop(oe);
                Extended {
                    value: Some(v),
                    extension: Rc::try_unwrap(x).unwrap_or_else(|_| unreachable!()),
                }
            }
            None => Extended {
                value: None,
                extension: Rc::try_unwrap(oe).unwrap_or_else(|_| unreachable!()),
            },
        }
    }
    fn posterior(self, other: Self) -> Self {
        let (se, oe) = (Rc::new(self.extension), Rc::new(other.extension));
        let (s, o) = (
            self.value.map(|v| (v, se.clone())),
            other.value.map(|v| (v, oe.clone())),
        );

        match s.posterior(o) {
            Some((v, x)) => {
                drop(se);
                drop(oe);
                Extended {
                    value: Some(v),
                    extension: Rc::try_unwrap(x).unwrap_or_else(|_| unreachable!()),
                }
            }
            None => Extended {
                value: None,
                extension: Rc::try_unwrap(se).unwrap_or_else(|_| unreachable!()),
            },
        }
    }
}

impl<T, E, X> Coalesce for Extended<Result<T, E>, X> {
    fn prior(self, other: Self) -> Self {
        let (se, oe) = (Rc::new(self.extension), Rc::new(other.extension));
        let (s, o) = (
            self.value
                .map(|v| (v, se.clone()))
                .map_err(|e| (e, se.clone())),
            other
                .value
                .map(|v| (v, oe.clone()))
                .map_err(|e| (e, oe.clone())),
        );
        drop(se);
        drop(oe);

        match s.prior(o) {
            Ok((v, x)) => Extended {
                value: Ok(v),
                extension: Rc::try_unwrap(x).unwrap_or_else(|_| unreachable!()),
            },
            Err((e, x)) => Extended {
                value: Err(e),
                extension: Rc::try_unwrap(x).unwrap_or_else(|_| unreachable!()),
            },
        }
    }
    fn posterior(self, other: Self) -> Self {
        let (se, oe) = (Rc::new(self.extension), Rc::new(other.extension));
        let (s, o) = (
            self.value
                .map(|v| (v, se.clone()))
                .map_err(|e| (e, se.clone())),
            other
                .value
                .map(|v| (v, oe.clone()))
                .map_err(|e| (e, oe.clone())),
        );
        drop(se);
        drop(oe);

        match s.posterior(o) {
            Ok((v, x)) => Extended {
                value: Ok(v),
                extension: Rc::try_unwrap(x).unwrap_or_else(|_| unreachable!()),
            },
            Err((e, x)) => Extended {
                value: Err(e),
                extension: Rc::try_unwrap(x).unwrap_or_else(|_| unreachable!()),
            },
        }
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
