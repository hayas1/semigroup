use semigroup::Semigroup;

#[derive(Debug, Clone, PartialEq, Semigroup)]
pub struct NamedStruct<'a, T> {
    pub num: Option<T>,
    #[semigroup(with = "semigroup::op::Coalesce")]
    pub str: Option<&'a str>,
}

fn main() {
    let a = NamedStruct {
        num: Some(1),
        str: None,
    };
    let b = NamedStruct {
        num: None,
        str: Some("ten"),
    };

    let c = a.semigroup(b);

    assert_eq!(
        c,
        NamedStruct {
            num: Some(1),
            str: Some("ten"),
        }
    );
}
