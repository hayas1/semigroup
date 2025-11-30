#![cfg(all(feature = "monoid", feature = "test"))]
use semigroup::{Construction, Op, Semigroup, assert_monoid, assert_semigroup, properties};

/// A semigroup construction that join two [`String`]s into a [`String`].
/// # Properties
/// <!-- properties -->
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Op)]
#[op(monoid, identity = Self(String::new()))]
#[properties(monoid)]
pub struct Join(pub String);
impl Op<String> for Join {
    fn lift_op_assign(base: &mut String, other: String) {
        base.push_str(&other);
    }
}

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
