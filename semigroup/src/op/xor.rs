use std::ops::BitXorAssign;

use semigroup_derive::{CommutativeOp, SemigroupOp, semigroup_properties_doc};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that returns the exclusive or.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Xor, Construction, Semigroup};
///
/// let a = Xor(0b101);
/// let b = Xor(0b100);
///
/// assert_eq!(a.semigroup(b).into_inner(), 0b001);
#[semigroup_properties_doc]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOp, CommutativeOp,
)]
#[cfg_attr(feature = "monoid", derive(semigroup_derive::MonoidOp))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(
    impl(semigroup = |base, other| *base ^= other, monoid = Self(T::zero())),
    bound(monoid = "T: num::Zero"),
    crate = crate,
)]
pub struct Xor<T: BitXorAssign>(pub T);

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

    use super::*;

    #[test]
    fn test_xor_semigroup() {
        let (a, b, c) = (Xor(0b111), Xor(0b101), Xor(0b100));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_xor_monoid() {
        let (a, b, c) = (Xor(0b111), Xor(0b101), Xor(0b100));
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    fn test_xor_commutative() {
        let (a, b, c) = (Xor(0b111), Xor(0b101), Xor(0b100));
        crate::assert_commutative!(a, b, c);
    }

    #[test]
    fn test_xor() {
        let (a, b) = (Xor(0b101), Xor(0b100));
        assert_eq!(a.semigroup(b).into_inner(), 0b001);
        assert_eq!(b.semigroup(a).into_inner(), 0b001);
    }
}
