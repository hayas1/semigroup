use num::{Integer, Unsigned};
use semigroup_derive::{SemigroupOpPriv, properties_priv};

use crate::SemigroupOp;

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that returns the greatest common divisor.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Gcd, Construction, Semigroup};
///
/// let a = Gcd(12u32);
/// let b = Gcd(18);
///
/// assert_eq!(a.semigroup(b).into_inner(), 6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(monoid, commutative, identity = Self(T::zero()))]
#[properties_priv(monoid, commutative)]
pub struct Gcd<T: Unsigned + Integer + Clone>(pub T);
impl<T: Unsigned + Integer + Clone> SemigroupOp<T> for Gcd<T> {
    fn lift_op_assign(base: &mut T, other: T) {
        *base = num::integer::gcd(base.clone(), other);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

    use super::*;

    #[test]
    fn test_gcd_semigroup() {
        let (a, b, c) = (Gcd(12u32), Gcd(18), Gcd(27));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_gcd_monoid() {
        let (a, b, c) = (Gcd(12u32), Gcd(18), Gcd(27));
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    fn test_gcd_commutative() {
        let (a, b, c) = (Gcd(12u32), Gcd(18), Gcd(27));
        crate::assert_commutative!(a, b, c);
    }

    #[test]
    fn test_gcd() {
        let (a, b) = (Gcd(57u32), Gcd(95));
        assert_eq!(a.semigroup(b).into_inner(), 19);
        assert_eq!(b.semigroup(a).into_inner(), 19);
    }
}
