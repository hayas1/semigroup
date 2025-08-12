use crate::semigroup::{Proceeded, Provenance};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Annotated<T, P> {
    pub value: T,
    pub provenance: P,
}

impl<T, P> Annotated<T, P> {
    pub fn lift_with(value: T, provenance: P) -> Self {
        Self { value, provenance }
    }
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Annotated<U, P> {
        Annotated {
            value: f(self.value),
            provenance: self.provenance,
        }
    }
}

impl<T: Provenance, P> Provenance for Annotated<T, P> {
    fn op_prov(self, other: Self) -> (Self, Proceeded) {
        let Self {
            value: sv,
            provenance: sp,
        } = self;
        let Self {
            value: ov,
            provenance: op,
        } = other;
        let (value, provenance) = sv.op_prov(ov);
        (
            Self {
                value,
                provenance: match &provenance {
                    Proceeded::Base => sp,
                    Proceeded::Other => op,
                },
            },
            provenance,
        )
    }
}
