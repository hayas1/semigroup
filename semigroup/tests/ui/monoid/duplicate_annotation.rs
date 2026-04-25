use semigroup::{Annotate, Construction, Semigroup, op::Coalesce};

fn main() {
    let a = Coalesce(Some(1)).annotated("a").annotated("b");

    assert_eq!(a.value().value(), &Coalesce(Some(1)));
}
