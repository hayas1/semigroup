use semigroup::{AnnotateFields, Semigroup};

#[derive(Debug, Clone, PartialEq, Semigroup)]
#[semigroup(annotated)]
pub struct Container {
    #[semigroup(with = "semigroup::op::Sum")]
    pub value: u32,
    #[semigroup(with = "semigroup::op::Coalesce")]
    pub value2: Option<u32>,
}

fn main() {
    let a = Container {
        value: 1,
        value2: None,
    }
    .annotated("first");
    let b = Container {
        value: 2,
        value2: Some(2),
    }
    .annotated("second");
    let _ = Semigroup::op(a, b);
}
