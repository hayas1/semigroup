use semigroup::SemigroupOp;

#[derive(Debug, Clone, PartialEq, SemigroupOp)]
#[semigroup_op(impl(semigroup = |_, _| {}), bound(semigroup = "T Default"))]
pub struct Construct<T>(pub T);

fn main() {}
