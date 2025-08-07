use coalesced::{Coalesce, Extension, WithExt};

// #[derive(Coalesce)]
struct NamedStruct {
    u: Option<u32>,
    v: i32,
}
impl<X: Clone> Extension<X> for NamedStruct {
    type WithExt = NamedStructWithExt<X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        NamedStructWithExt {
            u: self.u.with_extension(extension.clone()),
            v: coalesced::strategy::Overwrite(self.v).with_extension(extension.clone()),
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        Self {
            u: Extension::unwrap_extension(with_ext.u),
            v: coalesced::strategy::Overwrite::unwrap_extension(with_ext.v).0,
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
    u: WithExt<Option<u32>, X>,
    v: WithExt<coalesced::strategy::Overwrite<i32>, X>,
}
impl<X> Coalesce for NamedStructWithExt<X> {
    fn prior(self, other: Self) -> Self {
        Self {
            u: Coalesce::prior(self.u, other.u),
            v: Coalesce::prior(self.v, other.v),
        }
    }
    fn posterior(self, other: Self) -> Self {
        Self {
            u: Coalesce::posterior(self.u, other.u),
            v: Coalesce::posterior(self.v, other.v),
        }
    }
}
impl<X: Clone> From<NamedStructWithExt<X>> for NamedStruct {
    fn from(with_ext: NamedStructWithExt<X>) -> Self {
        Extension::unwrap_extension(with_ext)
    }
}

// #[derive(Coalesce)]
struct UnnamedStruct(Option<u32>, i32);
impl<X: Clone> Extension<X> for UnnamedStruct {
    type WithExt = UnnamedStructWithExt<X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        UnnamedStructWithExt(
            self.0.with_extension(extension.clone()),
            coalesced::strategy::Overwrite(self.1).with_extension(extension.clone()),
        )
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        Self(
            Extension::unwrap_extension(with_ext.0),
            coalesced::strategy::Overwrite::unwrap_extension(with_ext.1).0,
        )
    }
    fn ex_prior(base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        base.prior(other)
    }
    fn ex_posterior(base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        base.posterior(other)
    }
}
struct UnnamedStructWithExt<X>(
    WithExt<Option<u32>, X>,
    WithExt<coalesced::strategy::Overwrite<i32>, X>,
);
impl<X> Coalesce for UnnamedStructWithExt<X> {
    fn prior(self, other: Self) -> Self {
        Self(
            Coalesce::prior(self.0, other.0),
            Coalesce::prior(self.1, other.1),
        )
    }
    fn posterior(self, other: Self) -> Self {
        Self(
            Coalesce::posterior(self.0, other.0),
            Coalesce::posterior(self.1, other.1),
        )
    }
}
impl<X: Clone> From<UnnamedStructWithExt<X>> for UnnamedStruct {
    fn from(with_ext: UnnamedStructWithExt<X>) -> Self {
        Extension::unwrap_extension(with_ext)
    }
}

// #[derive(Coalesce)]
struct UnitStruct;
impl<X: Clone> Extension<X> for UnitStruct {
    type WithExt = WithExt<UnitStruct, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        WithExt {
            value: other.value,
            extension: other.extension,
        }
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        WithExt {
            value: base.value,
            extension: base.extension,
        }
    }
}
impl<X: Clone> From<WithExt<UnitStruct, X>> for UnitStruct {
    fn from(with_ext: WithExt<UnitStruct, X>) -> Self {
        Extension::unwrap_extension(with_ext)
    }
}

// #[derive(Coalesce)]
enum CompoundEnum {
    Unit,
    Named { u: Option<u32>, v: i32 },
    Unnamed(&'static str, usize),
}
impl<X: Clone> Extension<X> for CompoundEnum {
    type WithExt = CompoundEnumWithExt<X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        match self {
            CompoundEnum::Unit => CompoundEnumWithExt::Unit(().with_extension(extension)),
            CompoundEnum::Named { u, v } => CompoundEnumWithExt::Named {
                u: u.with_extension(extension.clone()),
                v: coalesced::strategy::Overwrite(v).with_extension(extension.clone()),
            },
            CompoundEnum::Unnamed(base_0, base_1) => CompoundEnumWithExt::Unnamed(
                base_0.with_extension(extension.clone()),
                base_1.with_extension(extension.clone()),
            ),
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        match with_ext {
            CompoundEnumWithExt::Unit(_) => CompoundEnum::Unit,
            CompoundEnumWithExt::Named { u, v } => CompoundEnum::Named {
                u: Extension::unwrap_extension(u),
                v: coalesced::strategy::Overwrite::unwrap_extension(v).0,
            },
            CompoundEnumWithExt::Unnamed(base_0, base_1) => CompoundEnum::Unnamed(
                Extension::unwrap_extension(base_0),
                Extension::unwrap_extension(base_1),
            ),
        }
    }
    fn ex_prior(base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        base.prior(other)
    }
    fn ex_posterior(base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        base.posterior(other)
    }
}
enum CompoundEnumWithExt<X> {
    Unit(WithExt<(), X>), // TODO ?
    Named {
        u: WithExt<Option<u32>, X>,
        v: WithExt<coalesced::strategy::Overwrite<i32>, X>,
    },
    Unnamed(WithExt<&'static str, X>, WithExt<usize, X>),
}
impl<X> Coalesce for CompoundEnumWithExt<X> {
    fn prior(self, other: Self) -> Self {
        match (self, other) {
            (Self::Unit(_), prior @ Self::Unit(_)) => prior,
            (
                Self::Named {
                    u: base_u,
                    v: base_v,
                },
                Self::Named {
                    u: other_u,
                    v: other_v,
                },
            ) => Self::Named {
                u: Coalesce::prior(base_u, other_u),
                v: Coalesce::prior(base_v, other_v),
            },
            (Self::Unnamed(base_0, base_1), Self::Unnamed(other_0, other_1)) => Self::Unnamed(
                Coalesce::prior(base_0, other_0),
                Coalesce::prior(base_1, other_1),
            ),
            (_, o) => o,
        }
    }
    fn posterior(self, other: Self) -> Self {
        match (self, other) {
            (posterior @ Self::Unit(_), Self::Unit(_)) => posterior,
            (
                Self::Named {
                    u: base_u,
                    v: base_v,
                },
                Self::Named {
                    u: other_u,
                    v: other_v,
                },
            ) => Self::Named {
                u: Coalesce::posterior(base_u, other_u),
                v: Coalesce::posterior(base_v, other_v),
            },
            (Self::Unnamed(base_0, base_1), Self::Unnamed(other_0, other_1)) => Self::Unnamed(
                Coalesce::posterior(base_0, other_0),
                Coalesce::posterior(base_1, other_1),
            ),
            (b, _) => b,
        }
    }
}
impl<X: Clone> From<CompoundEnumWithExt<X>> for CompoundEnum {
    fn from(with_ext: CompoundEnumWithExt<X>) -> Self {
        Extension::unwrap_extension(with_ext)
    }
}

#[test]
fn test_named_struct() {
    let a = NamedStruct { u: Some(1), v: -1 };
    let b = NamedStruct { u: Some(2), v: -2 };
    assert!(matches!(a.prior(b), NamedStruct { u: Some(2), v: -2 }));

    let ae = NamedStruct { u: Some(1), v: -1 }.with_extension("a");
    let be = NamedStruct { u: Some(2), v: -2 }.with_extension("b");
    let posterior = ae.posterior(be);
    assert_eq!(posterior.u.extension, "a");
    assert_eq!(posterior.v.extension, "a");
    assert!(matches!(
        posterior.into(),
        NamedStruct { u: Some(1), v: -1 }
    ));
}

#[test]
fn test_unnamed_struct() {
    let a = UnnamedStruct(Some(1), -1);
    let b = UnnamedStruct(None, -2);
    assert!(matches!(a.prior(b), UnnamedStruct(Some(1), -2)));
}

#[test]
fn test_unit_struct() {
    let a = UnitStruct;
    let b = UnitStruct;
    assert!(matches!(a.prior(b), UnitStruct));
}

#[test]
fn test_compound_enum() {
    let a_unit = CompoundEnum::Unit;
    let b_unit = CompoundEnum::Unit;
    assert!(matches!(a_unit.posterior(b_unit), CompoundEnum::Unit));

    let a_named = CompoundEnum::Named { u: None, v: -1 };
    let b_named = CompoundEnum::Named { u: Some(2), v: -2 };
    assert!(matches!(
        a_named.prior(b_named),
        CompoundEnum::Named { u: Some(2), v: -2 }
    ));

    let a_unnamed = CompoundEnum::Unnamed("one", 1);
    let b_unnamed = CompoundEnum::Unnamed("two", 2);
    assert!(matches!(
        a_unnamed.posterior(b_unnamed),
        CompoundEnum::Unnamed("one", 1)
    ));
}
