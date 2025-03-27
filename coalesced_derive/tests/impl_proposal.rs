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
struct UnitStruct;
impl Coalesce for UnitStruct {
    fn prior(self, other: Self) -> Self {
        let _ = self;
        other
    }
    fn posterior(self, other: Self) -> Self {
        let _ = other;
        self
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
            (Self::Named { value: base_value }, Self::Named { value: other_value }) => {
                Self::Named {
                    value: base_value.prior(other_value),
                }
            }
            (Self::Unnamed(base_0), Self::Unnamed(other_0)) => Self::Unnamed(base_0.prior(other_0)),
            (_, o) => o,
        }
    }

    fn posterior(self, other: Self) -> Self {
        match (self, other) {
            (Self::Unit, Self::Unit) => Self::Unit,
            (Self::Named { value: base_value }, Self::Named { value: other_value }) => {
                Self::Named {
                    value: base_value.posterior(other_value),
                }
            }
            (Self::Unnamed(base_0), Self::Unnamed(other_0)) => {
                Self::Unnamed(base_0.posterior(other_0))
            }
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
fn test_unit_struct() {
    let a = UnitStruct;
    let b = UnitStruct;
    assert!(matches!(a.prior(b), UnitStruct));
}

#[test]
fn test_compound_enum() {
    let a = CompoundEnum::Named { value: 1 };
    let b = CompoundEnum::Named { value: 2 };
    assert!(matches!(a.prior(b), CompoundEnum::Named { value: 2 }));
}
