//! Attributes of `#[derive(SemigroupOp)]` as written up to 0.6.0, kept to show the errors they get.
//! From 0.7.0 each property is a derive, such as `#[derive(Monoid)]`, with its function or value in `impl(..)`.
use semigroup::SemigroupOp;

#[derive(SemigroupOp)]
#[semigroup_op(selective)]
pub struct Selective<T>(pub T);

#[derive(SemigroupOp)]
#[semigroup_op(monoid, identity = Self(T::default()))]
pub struct Monoid<T>(pub T);

#[derive(SemigroupOp)]
#[semigroup_op(commutative)]
pub struct Commutative<T>(pub T);

#[derive(SemigroupOp)]
#[semigroup_op(semigroup_where = "T: Clone")]
pub struct SemigroupWhere<T>(pub T);

#[derive(SemigroupOp)]
#[semigroup_op(selective_where = "T: Clone")]
pub struct SelectiveWhere<T>(pub T);

#[derive(SemigroupOp)]
#[semigroup_op(monoid_where = "T: Default")]
pub struct MonoidWhere<T>(pub T);

#[derive(SemigroupOp)]
#[semigroup_op(commutative_where = "T: Ord")]
pub struct CommutativeWhere<T>(pub T);

#[derive(SemigroupOp)]
#[semigroup_op(manual_op_impl)]
pub struct ManualOpImpl<T>(pub T);

#[derive(SemigroupOp)]
#[semigroup_op(without_monoid_impl)]
pub struct WithoutMonoidImpl<T>(pub T);

fn main() {}
