use semigroup::SemigroupOp;

#[derive(Debug, Clone, PartialEq, SemigroupOp)]
#[semigroup_op(monoid, identity = Self(T::default()), monoid_where = "T Default")]
pub struct Construct<T>(pub T);

fn main() {}
