use crate::{op::reverse::Reversed, semigroup::Semigroup};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Added<T>(pub T);
pub trait Add: Sized + Semigroup {
    fn add(self, other: Self) -> Self {
        Semigroup::semigroup_op(self, other)
    }
}
impl<T: std::ops::Add<Output = T>> Add for Added<T> {}
impl<T: std::ops::Add<Output = T>> Add for Reversed<Added<T>> {}

impl<T> From<T> for Added<T> {
    fn from(value: T) -> Self {
        Added(value)
    }
}
impl<T> Added<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: std::ops::Add<Output = T>> Semigroup for Added<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Added(base.0 + other.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::semigroup::tests::{assert_associative_law, assert_reversed_associative_law};

    use super::*;

    #[test]
    fn test_add_as_semigroup_op() {
        let (a, b, c) = (Added(1), Added(2), Added(3));
        assert_associative_law(a, b, c);
        assert_reversed_associative_law(a, b, c);
    }

    #[test]
    fn test_add() {
        let (a, b) = (Added(1), Added(2));
        assert_eq!(a.add(b).into_inner(), 3);

        let (ra, rb) = (Reversed(a), Reversed(b));
        assert_eq!(ra.add(rb).0.into_inner(), 3);
    }
}
