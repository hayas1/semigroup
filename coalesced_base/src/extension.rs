use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::coalesce::Coalesce;

pub trait Extension<T>: Sized {
    type Output<X>: Coalesce + Deref<Target = Self> + DerefMut<Target = Self>;
    fn with_extension<X>(self, extension: X) -> Self::Output<X>;
}
impl<T: Coalesce> Extension<T> for Option<T> {
    type Output<X> = Extended<Option<T>, X>;
    fn with_extension<X>(self, extension: X) -> Self::Output<X> {
        Extended {
            base: self,
            extension,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Extended<C, X> {
    base: C,
    extension: X,
}
impl<T, X> Coalesce for Extended<Option<T>, X> {
    fn prior(self, other: Self) -> Self {
        let (se, oe) = (Rc::new(self.extension), Rc::new(other.extension));
        let (s, o) = (
            self.base.map(|v| (v, se.clone())),
            other.base.map(|v| (v, oe.clone())),
        );

        match s.prior(o) {
            Some((v, x)) => Extended {
                base: Some(v),
                extension: Rc::try_unwrap(x).unwrap_or_else(|_| unreachable!()),
            },
            None => Extended {
                base: None,
                extension: Rc::try_unwrap(se).unwrap_or_else(|_| unreachable!()),
            },
        }
    }
    fn posterior(self, other: Self) -> Self {
        let (se, oe) = (Rc::new(self.extension), Rc::new(other.extension));
        let (s, o) = (
            self.base.map(|v| (v, se.clone())),
            other.base.map(|v| (v, oe.clone())),
        );

        match s.posterior(o) {
            Some((v, x)) => Extended {
                base: Some(v),
                extension: Rc::try_unwrap(x).unwrap_or_else(|_| unreachable!()),
            },
            None => Extended {
                base: None,
                extension: Rc::try_unwrap(se).unwrap_or_else(|_| unreachable!()),
            },
        }
    }
}
impl<T, X> Deref for Extended<Option<T>, X> {
    type Target = Option<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<T, X> DerefMut for Extended<Option<T>, X> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
