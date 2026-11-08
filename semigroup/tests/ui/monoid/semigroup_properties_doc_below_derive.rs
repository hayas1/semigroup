use semigroup::{SemigroupOp, semigroup_properties_doc};

/// # Properties
/// <!-- properties -->
#[derive(Debug, Clone, PartialEq, SemigroupOp)]
#[semigroup_op(impl(semigroup = |_, _| {}))]
#[semigroup_properties_doc]
pub struct Construct<T>(pub T);

fn main() {}
