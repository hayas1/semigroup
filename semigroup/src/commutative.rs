use crate::Semigroup;

/// [`Commutative`] represents a binary operation that satisfies the following property
/// 1. *Commutativity*: `op(a, b) = op(b, a)`
///
/// The [*semigroup*](crate::Semigroup) set that satisfies the *commutativity* property is often called *commutative semigroup*.
///
/// And the [*monoid*](crate::Monoid) set that satisfies the *commutativity* property is often called *commutative monoid*.
///
/// This is marker trait.
///
/// # Examples
/// ## Deriving
/// [`Commutative`] can be derived like [`Semigroup`], use `commutative` attribute.
/// ```
/// use semigroup::Semigroup;
/// #[derive(Debug, Clone, PartialEq, Default, Semigroup)]
/// #[semigroup(commutative)]
/// pub struct ExampleStruct {
///     #[semigroup(with = "semigroup::op::Sum")]
///     pub sum: u32,
///     #[semigroup(with = "semigroup::op::Min")]
///     pub min: u32,
/// }
///
/// let a = ExampleStruct { sum: 1, min: 1 };
/// let b = ExampleStruct { sum: 10, min: 10 };
/// let c = ExampleStruct { sum: 100, min: 100 };
///
/// // #[test]
/// semigroup::assert_commutative!(&a, &b, &c);
/// assert_eq!(a.semigroup(b).semigroup(c), ExampleStruct { sum: 111, min: 1 });
/// ```
///
/// ## Construction
/// [`Commutative`] can be constructed like [`Semigroup`], use `commutative` attribute.
/// ```
/// use semigroup::{Op, Semigroup};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Op)]
/// #[op(commutative)]
/// pub struct Sum(u64);
/// impl Op<u64> for Sum {
///     fn lift_op_assign(base: &mut u64, other: u64) {
///         *base += other;
///     }
/// }
///
/// let (a, b, c) = (Sum(1), Sum(2), Sum(3));
/// // #[test]
/// semigroup::assert_commutative!(&a, &b, &c);
/// assert_eq!(a.semigroup(b).semigroup(c), Sum(6));
/// ```
///
/// # Testing
/// Use [`crate::assert_commutative!`] macro.
///
/// The *commutativity* property is not guaranteed by Rust’s type system,
/// so it must be verified manually using [`crate::assert_commutative!`].
pub trait Commutative: Semigroup {}

macro_rules! impl_tuple_commutative {
    ($($idx:tt: $t:tt),+) => {
        impl<$($t: $crate::Commutative),+> $crate::Commutative for ($($t,)+) {}
    };
}
impl_tuple_commutative!(0: A);
impl_tuple_commutative!(0: A, 1: B);
impl_tuple_commutative!(0: A, 1: B, 2: C);
impl_tuple_commutative!(0: A, 1: B, 2: C, 3: D);
impl_tuple_commutative!(0: A, 1: B, 2: C, 3: D, 4: E);
impl_tuple_commutative!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F);
impl_tuple_commutative!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G);
impl_tuple_commutative!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H);
impl_tuple_commutative!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I);
impl_tuple_commutative!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J);
impl_tuple_commutative!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K);
impl_tuple_commutative!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L);
impl_tuple_commutative!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L, 12: M);

#[cfg(feature = "test")]
pub mod test_commutative {
    use std::fmt::Debug;

    use crate::Dual;

    use super::*;

    /// Assert that the given type satisfies the *commutative* property.
    ///
    /// # Usage
    /// Same to [`crate::assert_semigroup!`].
    ///
    /// # Examples
    /// ```
    /// use semigroup::{assert_commutative, op::Sum};
    ///
    /// let a = Sum(1);
    /// let b = Sum(2);
    /// let c = Sum(3);
    /// assert_commutative!(a, b, c);
    ///
    /// let v = vec![a, b, c];
    /// assert_commutative!(&v);
    /// ```
    ///
    /// # Panics
    /// - If the given function does not satisfy the *commutative* property.
    /// ```should_panic
    /// use semigroup::{assert_commutative, Op, Semigroup};
    /// #[derive(Debug, Clone, PartialEq, Op)]
    /// #[op(commutative)]
    /// pub struct Sub(i32);
    /// impl Op<i32> for Sub {
    ///     fn lift_op_assign(base: &mut i32, other: i32) {
    ///         *base -= other;
    ///     }
    /// }
    /// let a = Sub(1);
    /// let b = Sub(2);
    /// let c = Sub(3);
    /// assert_commutative!(a, b, c);
    /// ```
    ///
    /// - The input iterator has less than 3 items.
    /// ```compile_fail
    /// use semigroup::{assert_commutative, op::Sum};
    /// let a = Sum(1);
    /// let b = Sum(2);
    /// assert_commutative!(a, b);
    /// ```
    /// ```should_panic
    /// use semigroup::{assert_commutative, op::Sum};
    /// let a = Sum(1);
    /// let b = Sum(2);
    /// assert_commutative!(&vec![a, b]);
    /// ```
    #[macro_export]
    macro_rules! assert_commutative {
        ($a:expr, $b: expr, $($tail: expr),*) => {
            {
                let v = vec![$a, $b, $($tail),*];
                $crate::assert_commutative!(&v)
            }
        };
        ($v:expr) => {
            {
                let (a, b, c) = $crate::test_semigroup::pick3($v);
                $crate::test_commutative::assert_commutative_impl(a.clone(), b.clone(), c.clone());
            }
        };
    }

    pub fn assert_commutative_impl<T: Commutative + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_commutative_law(a.clone(), b.clone(), c.clone());
        assert_commutative_dual(a.clone(), b.clone(), c.clone());
        assert_commutative_tuple(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_commutative_law<T: Commutative + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let abc = Semigroup::op(Semigroup::op(a.clone(), b.clone()), c.clone());
        let bca = Semigroup::op(Semigroup::op(b.clone(), c.clone()), a.clone());
        let cba = Semigroup::op(Semigroup::op(c.clone(), b.clone()), a.clone());
        let acb = Semigroup::op(Semigroup::op(a.clone(), c.clone()), b.clone());
        let bac = Semigroup::op(Semigroup::op(b.clone(), a.clone()), c.clone());
        let cab = Semigroup::op(Semigroup::op(c.clone(), a.clone()), b.clone());

        assert_eq!(abc, bca);
        assert_eq!(bca, cba);
        assert_eq!(cba, acb);
        assert_eq!(acb, bac);
        assert_eq!(bac, cab);
        assert_eq!(cab, abc);
    }

    /// Verifies that for a [`Commutative`] semigroup, `op(a, b) == Dual::op(Dual(a), Dual(b))`
    /// (i.e. reversing the arguments produces the same result).
    pub fn assert_commutative_dual<T: Commutative + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        use crate::Construction;
        assert_eq!(
            Semigroup::op(a.clone(), b.clone()),
            Semigroup::op(Dual(a.clone()), Dual(b.clone())).into_inner()
        );
        assert_eq!(
            Semigroup::op(b.clone(), c.clone()),
            Semigroup::op(Dual(b.clone()), Dual(c.clone())).into_inner()
        );
        assert_eq!(
            Semigroup::op(c.clone(), a.clone()),
            Semigroup::op(Dual(c.clone()), Dual(a.clone())).into_inner()
        );
    }

    pub fn assert_commutative_tuple<T: Commutative + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let abc = (a.clone(), b.clone(), c.clone());
        let bca = (b.clone(), c.clone(), a.clone());
        let cab = (c.clone(), a.clone(), b.clone());

        let (a_b_c, b_c_a, c_a_b) = abc.semigroup(bca).semigroup(cab);
        assert_eq!(a_b_c, b_c_a);
        assert_eq!(b_c_a, c_a_b);
        assert_eq!(c_a_b, a_b_c);
    }
}
