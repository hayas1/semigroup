//! Arguments of `#[properties]` as written up to 0.6.0, kept to show the errors they get.
//! From 0.7.0 `#[properties]` takes the same content as `properties(..)` of the derives.
use semigroup::properties;

/// # Properties
/// <!-- properties -->
#[properties(selective, selective_where = "T: Clone")]
pub struct SelectiveWhere<T>(pub T);

/// # Properties
/// <!-- properties -->
#[properties(monoid, monoid_where = "T: Default")]
pub struct MonoidWhere<T>(pub T);

/// # Properties
/// <!-- properties -->
#[properties(commutative, commutative_where = "T: Ord")]
pub struct CommutativeWhere<T>(pub T);

fn main() {}
