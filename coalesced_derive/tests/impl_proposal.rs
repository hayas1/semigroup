use coalesced::{Coalesce, Extension, WithExt};

// #[derive(Coalesce)]
struct NamedStruct {
    u: u32,
    v: i32,
}
impl<X: Clone> Extension<X> for NamedStruct {
    type WithExt = NamedStructWithExt<X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        NamedStructWithExt {
            u: self.u.with_extension(extension.clone()),
            v: self.v.with_extension(extension.clone()),
        }
    }
    fn from_extension(with_ext: Self::WithExt) -> Self {
        let Self::WithExt { u, v } = with_ext;
        Self {
            u: Extension::from_extension(u),
            v: Extension::from_extension(v),
        }
    }
    fn ex_prior(base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        base.prior(other)
    }
    fn ex_posterior(base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        base.posterior(other)
    }
}
struct NamedStructWithExt<X> {
    u: WithExt<u32, X>,
    v: WithExt<i32, X>,
}
impl<X> Coalesce for NamedStructWithExt<X> {
    fn prior(self, other: Self) -> Self {
        Self {
            u: self.u.prior(other.u),
            v: self.v.prior(other.v),
        }
    }
    fn posterior(self, other: Self) -> Self {
        Self {
            u: self.u.posterior(other.u),
            v: self.v.posterior(other.v),
        }
    }
}

// #[derive(Coalesce)]
struct UnnamedStruct(u32, i32);
impl Coalesce for UnnamedStruct {
    fn prior(self, other: Self) -> Self {
        Self(self.0.prior(other.0), self.1.prior(other.1))
    }
    fn posterior(self, other: Self) -> Self {
        Self(self.0.posterior(other.0), self.1.posterior(other.1))
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
    Named { u: u32, v: i32 },
    Unnamed(&'static str, String),
}

impl Coalesce for CompoundEnum {
    fn prior(self, other: Self) -> Self {
        match (self, other) {
            (Self::Unit, Self::Unit) => Self::Unit,
            (
                Self::Named {
                    u: self_u,
                    v: self_v,
                },
                Self::Named {
                    u: other_u,
                    v: other_v,
                },
            ) => Self::Named {
                u: self_u.prior(other_u),
                v: self_v.prior(other_v),
            },
            (Self::Unnamed(self_0, self_1), Self::Unnamed(other_0, other_1)) => {
                Self::Unnamed(self_0.prior(other_0), self_1.prior(other_1))
            }
            (_, o) => o,
        }
    }

    fn posterior(self, other: Self) -> Self {
        match (self, other) {
            (Self::Unit, Self::Unit) => Self::Unit,
            (
                Self::Named {
                    u: self_u,
                    v: self_v,
                },
                Self::Named {
                    u: other_u,
                    v: other_v,
                },
            ) => Self::Named {
                u: self_u.posterior(other_u),
                v: self_v.posterior(other_v),
            },
            (Self::Unnamed(self_0, self_1), Self::Unnamed(other_0, other_1)) => {
                Self::Unnamed(self_0.posterior(other_0), self_1.posterior(other_1))
            }
            (b, _) => b,
        }
    }
}

#[test]
fn test_named_struct() {
    let a = NamedStruct { u: 1, v: -1 };
    let b = NamedStruct { u: 2, v: -2 };
    assert!(matches!(a.prior(b), NamedStruct { u: 2, v: -2 }));

    let ae = NamedStruct { u: 1, v: -1 }.with_extension("a");
    let be = NamedStruct { u: 2, v: -2 }.with_extension("b");
    let posterior = ae.posterior(be);
    assert_eq!(posterior.u.extension, "a");
    assert_eq!(posterior.v.extension, "a");
    // assert!(matches!(*posterior, NamedStruct { u: 2, v: -2 }));
}

#[test]
fn test_unnamed_struct() {
    let a = UnnamedStruct(1, -1);
    let b = UnnamedStruct(2, -2);
    assert!(matches!(a.prior(b), UnnamedStruct(2, -2)));
}

#[test]
fn test_unit_struct() {
    let a = UnitStruct;
    let b = UnitStruct;
    assert!(matches!(a.prior(b), UnitStruct));
}

#[test]
fn test_compound_enum() {
    let a = CompoundEnum::Named { u: 1, v: -1 };
    let b = CompoundEnum::Named { u: 2, v: -2 };
    assert!(matches!(a.prior(b), CompoundEnum::Named { u: 2, v: -2 }));
}
