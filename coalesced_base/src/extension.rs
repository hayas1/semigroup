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
    type Output<X> = Extended<Option<T>, X>;
    fn with_extension<X>(self, extension: X) -> Self::Output<X> {
        Extended {
            value: self,
            extension,
        }
    }
}
impl<T, E> Extension<T> for Result<T, E> {
    type Output<X> = Extended<Result<T, E>, X>;
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
            Some((v, x)) => Extended {
                value: Some(v),
                extension: Rc::try_unwrap(x).unwrap_or_else(|_| unreachable!()),
            },
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
            Some((v, x)) => Extended {
                value: Some(v),
                extension: Rc::try_unwrap(x).unwrap_or_else(|_| unreachable!()),
            },
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
    fn test_option_extension() {
        let file = Some(1).with_extension("file");
        let env = Some(10).with_extension("env");
        let cli = None.with_extension("cli");

        let coalesced = file.prior(env).prior(cli);
        assert_eq!(*coalesced, Some(10));
        assert_eq!(coalesced.unwrap(), 10);
        assert_eq!(coalesced.extension_ref(), &"env");
    }
}
