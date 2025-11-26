use semigroup::Semigroup;

#[derive(Debug, Clone, PartialEq, Semigroup)]
pub struct NamedStruct<'a> {
    pub num: u32,
    #[semigroup(with = "semigroup::op::Coalesce")]
    pub str: Option<&'a str>,
}

fn main() {
    let a = NamedStruct { num: 1, str: None };
    let b = NamedStruct {
        num: 0,
        str: Some("ten"),
    };

    let c = a.semigroup(b);

    assert_eq!(
        c,
        NamedStruct {
            num: 1,
            str: Some("ten"),
        }
    );
}
