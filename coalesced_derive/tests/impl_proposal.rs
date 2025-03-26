use coalesced::Coalesce;

// #[derive(Coalesce)]
struct NamedStruct {
    value: i32,
}
impl Coalesce for NamedStruct {
    fn prior(self, other: Self) -> Self {
        Self {
            value: self.value.prior(other.value),
        }
    }
    fn posterior(self, other: Self) -> Self {
        Self {
            value: self.value.posterior(other.value),
        }
    }
}

// #[derive(Coalesce)]
struct UnnamedStruct(i32);
impl Coalesce for UnnamedStruct {
    fn prior(self, other: Self) -> Self {
        Self(self.0.prior(other.0))
    }
    fn posterior(self, other: Self) -> Self {
        Self(self.0.posterior(other.0))
    }
}

// #[derive(Coalesce)]
enum CompoundEnum {
    Unit,
    Named { value: i32 },
    Unnamed(String),
}

impl Coalesce for CompoundEnum {
    fn prior(self, other: Self) -> Self {
        match (self, other) {
            (Self::Unit, Self::Unit) => Self::Unit,
            (Self::Named { value: base }, Self::Named { value: other }) => Self::Named {
                value: base.prior(other),
            },
            (Self::Unnamed(base), Self::Unnamed(other)) => Self::Unnamed(base.prior(other)),
            (_, o) => o,
        }
    }

    fn posterior(self, other: Self) -> Self {
        match (self, other) {
            (Self::Unit, Self::Unit) => Self::Unit,
            (Self::Named { value: base }, Self::Named { value: other }) => Self::Named {
                value: base.posterior(other),
            },
            (Self::Unnamed(base), Self::Unnamed(other)) => Self::Unnamed(base.posterior(other)),
            (b, _) => b,
        }
    }
}

#[test]
fn test_named_struct() {
    let a = NamedStruct { value: 1 };
    let b = NamedStruct { value: 2 };
    assert!(matches!(a.prior(b), NamedStruct { value: 2 }));
}

#[test]
fn test_unnamed_struct() {
    let a = UnnamedStruct(1);
    let b = UnnamedStruct(2);
    assert!(matches!(a.prior(b), UnnamedStruct(2)));
}

#[test]
fn test_compound_enum() {
    let a = CompoundEnum::Named { value: 1 };
    let b = CompoundEnum::Named { value: 2 };
    assert!(matches!(a.prior(b), CompoundEnum::Named { value: 2 }));
}
