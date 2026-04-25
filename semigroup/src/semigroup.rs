/// [`Semigroup`] represents a binary operation that satisfies the following properties
/// 1. *Closure*: `op: T × T → T`
/// 2. *Associativity*: `op(op(a, b), c) = op(a, op(b, c))`
///
/// # Examples
/// ## Deriving
/// When fields do not implement [`Semigroup`], use `with` attribute.
/// ```
/// use semigroup::Semigroup;
/// #[derive(Debug, Clone, PartialEq, Semigroup)]
/// #[semigroup(with = "semigroup::op::Coalesce")]
/// pub struct ExampleStruct<'a> {
///     pub str: Option<&'a str>,
///     #[semigroup(with = "semigroup::op::Overwrite")]
///     pub boolean: bool,
///     #[semigroup(with = "semigroup::op::Sum")]
///     pub sum: u32,
/// }
///
/// let a = ExampleStruct { str: None, boolean: true, sum: 1 };
/// let b = ExampleStruct { str: Some("ten"), boolean: false, sum: 10 };
/// let c = ExampleStruct { str: None, boolean: false, sum: 100 };
///
/// // #[test]
/// semigroup::assert_semigroup!(&a, &b, &c);
/// assert_eq!(a.semigroup(b).semigroup(c), ExampleStruct { str: Some("ten"), boolean: false, sum: 111 });
/// ```
///
/// ## Construction
/// [`Semigroup`] can be constructed as [`crate::Op`] by [`crate::Construction`].
///
/// Some operations are already provided by [`crate::op`].
/// ```
/// use semigroup::{Construction, Op, Semigroup};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Op)]
/// pub struct Sum(u64);
/// impl Op<u64> for Sum {
///     fn lift_op_assign(base: &mut u64, other: u64) {
///         *base += other;
///     }
/// }
///
/// let (a, b, c) = (Sum(1), Sum(2), Sum(3));
/// // #[test]
/// semigroup::assert_semigroup!(&a, &b, &c);
/// assert_eq!(a.semigroup(b).semigroup(c), Sum(6));
/// ```
///
/// # Testing
/// Use [`crate::assert_semigroup!`] macro.
///
/// The *closure* property is guaranteed by Rust’s type system,
/// but *associativity* must be verified manually using [`crate::assert_semigroup!`].
pub trait Semigroup: Sized {
    fn op_assign(base: &mut Self, other: Self);
    fn op(mut base: Self, other: Self) -> Self {
        Self::op_assign(&mut base, other);
        base
    }
    fn semigroup_assign(&mut self, other: Self) {
        Semigroup::op_assign(self, other);
    }
    fn semigroup(self, other: Self) -> Self {
        Semigroup::op(self, other)
    }
}

macro_rules! impl_tuple_semigroup {
    ($($idx:tt: $t:tt),+) => {
        impl<$($t: $crate::Semigroup),+> $crate::Semigroup for ($($t,)+) {
            fn op_assign(base: &mut Self, other: Self) {
                $(
                    $t::op_assign(&mut base.$idx, other.$idx)
                );+
            }
        }
    };
}
impl_tuple_semigroup!(0: A);
impl_tuple_semigroup!(0: A, 1: B);
impl_tuple_semigroup!(0: A, 1: B, 2: C);
impl_tuple_semigroup!(0: A, 1: B, 2: C, 3: D);
impl_tuple_semigroup!(0: A, 1: B, 2: C, 3: D, 4: E);
impl_tuple_semigroup!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F);
impl_tuple_semigroup!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G);
impl_tuple_semigroup!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H);
impl_tuple_semigroup!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I);
impl_tuple_semigroup!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J);
impl_tuple_semigroup!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K);
impl_tuple_semigroup!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L);
impl_tuple_semigroup!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L, 12: M);

/// [`Selected`] indicates which operand was chosen by an [`Idempotent`] operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selected {
    Base,
    Other,
}

/// [`Idempotent`] represents a binary operation that satisfies the following properties
/// 1. *Idempotency*: `op(a, a) = a`
///
/// This means that the result always equals one of the two operands unchanged,
/// i.e. `op(a, b) ∈ {a, b}` for all `a`, `b`.
///
/// When a type is both a [`Semigroup`] and [`Idempotent`],
/// [`Annotated`](crate::Annotated) can track which operand was selected.
pub trait Idempotent {
    fn select(base: &Self, other: &Self) -> Selected;
}

#[cfg(feature = "test")]
pub mod test_semigroup {
    use std::fmt::Debug;

    use rand::seq::IndexedRandom;

    use crate::{
        combine::test_combine::{assert_combine_iter, assert_semigroup_dual},
        lazy::test_lazy::assert_lazy,
    };

    use super::*;

    /// Assert that the given type satisfies the *semigroup* property.
    ///
    /// # Usage
    /// ```sh
    /// cargo add semigroup --dev --features test
    /// ```
    ///
    /// - 1 argument: iterator of more than 3 items that implements [`Semigroup`].
    /// - More than 3 arguments: items that implements [`Semigroup`].
    ///
    /// # Examples
    /// ```
    /// use semigroup::{assert_semigroup, op::Coalesce};
    ///
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(None);
    /// let c = Coalesce(Some(3));
    /// assert_semigroup!(a, b, c);
    ///
    /// let v = vec![a, b, c];
    /// assert_semigroup!(&v);
    /// ```
    ///
    /// # Panics
    /// - If the given function does not satisfy the *semigroup* property.
    /// ```should_panic
    /// use semigroup::{assert_semigroup, Construction, Op, Semigroup};
    /// #[derive(Debug, Clone, PartialEq, Op)]
    /// pub struct Sub(i32);
    /// impl Op<i32> for Sub {
    ///     fn lift_op_assign(base: &mut i32, other: i32) {
    ///         *base -= other;
    ///     }
    /// }
    /// let a = Sub(1);
    /// let b = Sub(2);
    /// let c = Sub(3);
    /// assert_semigroup!(a, b, c);
    /// ```
    ///
    /// - The input iterator has less than 3 items.
    /// ```compile_fail
    /// use semigroup::{assert_semigroup, op::Coalesce};
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(None);
    /// assert_semigroup!(a, b);
    /// ```
    /// ```should_panic
    /// use semigroup::{assert_semigroup, op::Coalesce};
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(None);
    /// assert_semigroup!(&vec![a, b]);
    /// ```
    #[macro_export]
    macro_rules! assert_semigroup {
        ($a:expr, $b: expr, $($tail: expr),*) => {
            {
                let v = vec![$a, $b, $($tail),*];
                $crate::assert_semigroup!(&v)
            }
        };
        ($v:expr) => {
            {
                let (a, b, c) = $crate::test_semigroup::pick3($v);
                $crate::test_semigroup::assert_semigroup_impl(a.clone(), b.clone(), c.clone());
            }
        };
    }

    pub fn pick3<T: Clone>(data: &[T]) -> (T, T, T) {
        data.choose_multiple_array(&mut rand::rng())
            .map(|[a, b, c]| (a, b, c))
            .expect("failed to pick 3 items")
    }

    pub fn assert_semigroup_impl<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_associative_law(a.clone(), b.clone(), c.clone());
        assert_combine_iter(a.clone(), b.clone(), c.clone());
        assert_semigroup_dual(a.clone(), b.clone(), c.clone());
        assert_lazy(a.clone(), b.clone(), c.clone());
        assert_semigroup_tuple(a.clone(), b.clone(), c.clone());
        #[cfg(feature = "monoid")]
        crate::test_monoid::assert_option_monoid(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_associative_law<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let ab_c = Semigroup::op(Semigroup::op(a.clone(), b.clone()), c.clone());
        let a_bc = Semigroup::op(a.clone(), Semigroup::op(b.clone(), c.clone()));
        assert_eq!(ab_c, a_bc);
    }

    pub fn assert_semigroup_tuple<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let abc = (a.clone(), b.clone(), c.clone());
        let bca = (b.clone(), c.clone(), a.clone());
        let cab = (c.clone(), a.clone(), b.clone());

        let (a_b_c, b_c_a, c_a_b) = abc.semigroup(bca).semigroup(cab);
        assert_eq!(a_b_c, a.clone().semigroup(b.clone()).semigroup(c.clone()));
        assert_eq!(b_c_a, b.clone().semigroup(c.clone()).semigroup(a.clone()));
        assert_eq!(c_a_b, c.clone().semigroup(a.clone()).semigroup(b.clone()));
    }
}
