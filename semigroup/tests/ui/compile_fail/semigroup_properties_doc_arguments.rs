use semigroup::{SemigroupOp, semigroup_properties_doc};

/// # Properties
/// <!-- properties -->
#[semigroup_properties_doc(selective)]
#[derive(Debug, Clone, PartialEq, SemigroupOp)]
#[semigroup_op(impl(semigroup = |_, _| {}))]
pub struct Construct<T>(pub T);

fn main() {}
