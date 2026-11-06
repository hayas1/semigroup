use semigroup::Semigroup;

#[derive(Debug, Clone, PartialEq, Semigroup)]
#[semigroup(properties(monoid), with = "semigroup::op::Coalesce")]
pub struct NamedStruct {
    pub num: Option<u32>,
    #[semigroup(with = "semigroup::op::Last")]
    pub boolean: bool,
}

fn main() {}
