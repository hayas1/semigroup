use std::ops::{Deref, DerefMut};

use crate::coalesce::Coalesce;

pub trait Extension<T>: Sized {
    type Output<X>: Coalesce + Deref<Target = Self> + DerefMut<Target = Self>;
    fn with_extension<X>(self, extension: X) -> Self::Output<X>;
}
impl<T: Coalesce> Extension<T> for Option<T> {
    type Output<X> = Opt<T, X>;
    fn with_extension<X>(self, extension: X) -> Self::Output<X> {
        Opt {
            base: self,
            extension: Some(extension),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Opt<T, X> {
    base: Option<T>,
    extension: Option<X>, // TODO Option<X> -> X
}
impl<T: Coalesce, X> Coalesce for Opt<T, X> {
    fn prior(self, other: Self) -> Self {
        let s = self.base.map(|v| (v, self.extension));
        let o = other.base.map(|v| (v, other.extension));
        let coalesce = s.prior(o);
        match coalesce {
            Some((v, x)) => Opt {
                base: Some(v),
                extension: x,
            },
            None => Opt {
                base: None,
                extension: None,
            },
        }
    }
    fn posterior(self, other: Self) -> Self {
        let _ = other;
        todo!()
    }
}
impl<T: Coalesce, X> Deref for Opt<T, X> {
    type Target = Option<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<T: Coalesce, X> DerefMut for Opt<T, X> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
