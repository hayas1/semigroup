use semigroup::{SelectiveOp, SemigroupOp};

#[derive(Debug, Clone, PartialEq, SemigroupOp)]
pub struct Sum(pub u32);

#[derive(Debug, Clone, PartialEq, SemigroupOp, SelectiveOp)]
#[semigroup_op(impl(semigroup = Self::lift_select_assign))]
pub struct Max(pub u32);

fn main() {}
