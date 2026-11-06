//! Attributes of `#[derive(Semigroup)]` as written up to 0.6.0, kept to show the errors they get.
//! From 0.7.0 every property goes in `properties(..)`.
use semigroup::Semigroup;

#[derive(Semigroup)]
#[semigroup(annotated, with = "semigroup::op::Coalesce")]
pub struct Annotated {
    pub num: Option<u32>,
}

#[derive(Semigroup)]
#[semigroup(annotation_param = B, with = "semigroup::op::Coalesce")]
pub struct AnnotationParam {
    pub num: Option<u32>,
}

#[derive(Semigroup)]
#[semigroup(monoid, with = "semigroup::op::Coalesce")]
pub struct Monoid {
    pub num: Option<u32>,
}

#[derive(Semigroup)]
#[semigroup(identity = Self { num: None }, with = "semigroup::op::Coalesce")]
pub struct Identity {
    pub num: Option<u32>,
}

#[derive(Semigroup)]
#[semigroup(commutative, with = "semigroup::op::Coalesce")]
pub struct Commutative {
    pub num: Option<u32>,
}

#[derive(Semigroup)]
#[semigroup(monoid_where = "T: Default")]
pub struct MonoidWhere<T> {
    pub value: T,
}

#[derive(Semigroup)]
#[semigroup(commutative_where = "T: Ord")]
pub struct CommutativeWhere<T> {
    pub value: T,
}

#[derive(Semigroup)]
#[semigroup(without_monoid_impl)]
pub struct WithoutMonoidImpl<T> {
    pub value: T,
}

fn main() {}
