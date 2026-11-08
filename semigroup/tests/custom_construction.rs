#![cfg(all(feature = "monoid", feature = "testing"))]
use semigroup::{
    Construction, MonoidOp, Semigroup, SemigroupOp, assert_monoid, assert_semigroup,
    semigroup_properties_doc,
};

/// A semigroup construction that join two [`String`]s into a [`String`].
/// # Properties
/// <!-- properties -->
#[semigroup_properties_doc]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOp, MonoidOp)]
#[semigroup_op(impl(semigroup = |base, other| base.push_str(&other), monoid = Self(String::new())))]
pub struct Join(pub String);

#[test]
fn test_join_semigroup() {
    let (a, b, c) = (
        Join("a".to_string()),
        Join("b".to_string()),
        Join("c".to_string()),
    );
    assert_semigroup!(a, b, c);
}

#[test]
fn test_join_monoid() {
    let (a, b, c) = (
        Join("a".to_string()),
        Join("b".to_string()),
        Join("c".to_string()),
    );
    assert_monoid!(a, b, c)
}

#[test]
fn test_join() {
    let (a, b, c) = (
        Join("a".to_string()),
        Join("b".to_string()),
        Join("c".to_string()),
    );
    assert_eq!(a.semigroup(b).semigroup(c).into_inner(), "abc");
}
