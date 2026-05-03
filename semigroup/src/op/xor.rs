use std::ops::BitXorAssign;

use semigroup_derive::{SemigroupOpPriv, properties_priv};

use crate::SemigroupOp;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(monoid, commutative, identity = Self(T::zero()), monoid_where = "T: num::Zero")]
#[properties_priv(monoid, commutative, monoid_where = "T: num::Zero")]
pub struct Xor<T: BitXorAssign>(pub T);
impl<T: BitXorAssign> SemigroupOp<T> for Xor<T> {
    fn lift_op_assign(base: &mut T, other: T) {
        *base ^= other;
    }
}

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
