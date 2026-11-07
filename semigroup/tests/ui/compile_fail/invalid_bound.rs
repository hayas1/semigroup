use semigroup::SemigroupOp;

#[derive(Debug, Clone, PartialEq, SemigroupOp)]
#[semigroup_op(properties(monoid(identity = Self(T::default()))), bound(monoid = "T Default"))]
pub struct Construct<T>(pub T);

fn main() {}
