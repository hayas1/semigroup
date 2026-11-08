use num::{Integer, Unsigned};
use semigroup_derive::{CommutativeOp, MonoidOp, SemigroupOp, semigroup_properties_doc};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::SemigroupOp) that returns the least common multiple.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Lcm, Construction, Semigroup};
///
/// let a = Lcm(12u32);
/// let b = Lcm(18);
///
/// assert_eq!(a.semigroup(b).into_inner(), 36);
/// ```
#[semigroup_properties_doc]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Hash,
    SemigroupOp,
    MonoidOp,
    CommutativeOp,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[semigroup_op(
    impl(
        semigroup = |base, other| *base = num::integer::lcm(base.clone(), other),
        monoid = Self(T::one()),
    ),
    crate = crate,
)]
pub struct Lcm<T: Unsigned + Integer + Clone>(pub T);

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

    use super::*;

    #[test]
    fn test_lcm_semigroup() {
        let (a, b, c) = (Lcm(4u32), Lcm(6), Lcm(9));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_lcm_monoid() {
        let (a, b, c) = (Lcm(4u32), Lcm(6), Lcm(9));
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    fn test_lcm_commutative() {
        let (a, b, c) = (Lcm(4u32), Lcm(6), Lcm(9));
        crate::assert_commutative!(a, b, c);
    }

    #[test]
    fn test_lcm() {
        let (a, b) = (Lcm(12u32), Lcm(18));
        assert_eq!(a.semigroup(b).into_inner(), 36);
        assert_eq!(b.semigroup(a).into_inner(), 36);
    }
}
