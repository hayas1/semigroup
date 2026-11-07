use semigroup::{SemigroupOp, semigroup_properties_doc};

/// # Properties
/// <!-- properties -->
#[derive(Debug, Clone, PartialEq, SemigroupOp)]
#[semigroup_op(properties(selective))]
#[semigroup_properties_doc(selective)]
pub struct Construct<T>(pub T);

fn main() {}
