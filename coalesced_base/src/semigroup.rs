use crate::lazy::LazySemigroup;

pub trait Semigroup {
    fn op(self, other: Self) -> Self;
    fn into_lazy(self) -> LazySemigroup<Self>
    where
        Self: Sized,
    {
        LazySemigroup::with(self)
    }
}
impl<P: Provenance> Semigroup for P {
    fn op(self, other: Self) -> Self {
        let (value, _) = self.op_prov(other);
        value
    }
}

pub trait Provenance: Sized {
    fn op_prov(self, other: Self) -> (Self, Proceeded);
}
pub enum Proceeded {
    Base,
    Other,
}
