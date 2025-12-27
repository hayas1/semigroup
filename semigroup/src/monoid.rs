use semigroup_derive::{OpPriv, properties_priv};

use crate::{Op, Semigroup};

/// [`Monoid`] represents a binary operation that satisfies the following properties
/// 1. *Closure*: `op: T × T → T`
/// 2. *Associativity*: `op(op(a, b), c) = op(a, op(b, c))`
/// 3. Existence of *identity element*: `op(identity(), a) = a = op(a, identity())`
///
/// # Examples
/// ## Deriving
/// [`Monoid`] can be derived like [`Semigroup`], use `monoid` attribute.
/// ```
/// use semigroup::{Semigroup, Monoid};
/// #[derive(Debug, Clone, PartialEq, Default, Semigroup)]
/// #[semigroup(monoid, with = "semigroup::op::Coalesce")]
/// pub struct ExampleStruct<'a> {
///     pub str: Option<&'a str>,
///     #[semigroup(with = "semigroup::op::Sum")]
///     pub sum: u32,
/// }
///
/// let a = ExampleStruct::identity();
/// let b = ExampleStruct { str: Some("ten"), sum: 10 };
/// let c = ExampleStruct { str: None, sum: 100 };
///
/// // #[test]
/// semigroup::assert_monoid!(&a, &b, &c);
/// assert_eq!(a.semigroup(b).semigroup(c), ExampleStruct { str: Some("ten"), sum: 110 });
/// ```
///
/// ## Construction
/// [`Monoid`] can be constructed by [`crate::MonoidOp`] like [`Semigroup`], use `monoid` attribute.
///
/// Some operations are already provided by [`crate::op`].
/// ```
/// use semigroup::{Semigroup, Op, Monoid};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Op)]
/// #[op(monoid, identity = Self(0))]
/// pub struct Sum(u64);
/// impl Op<u64> for Sum {
///     fn lift_op_assign(base: &mut u64, other: u64) {
///         *base += other;
///     }
/// }
/// let (a, b, c) = (Sum::identity(), Sum(2), Sum(3));
/// // #[test]
/// semigroup::assert_monoid!(&a, &b, &c);
/// assert_eq!(a.semigroup(b).semigroup(c), Sum(5));
/// ```
///
/// # Optional [`Semigroup`]
/// [`Option<Semigroup>`] can behave like [`Monoid`]. In such case, [`OptionMonoid`] is useful.
///
/// # Testing
/// Use [`crate::assert_monoid!`] macro.
///
/// The *closure* and *associativity* properties are same as [`Semigroup`],
/// so they are guaranteed by [`crate::assert_semigroup!`].
/// However, existence of *identity element* is not guaranteed the macro,
/// so it must be verified manually using [`crate::assert_monoid!`].
pub trait Monoid: Semigroup {
    fn identity() -> Self;
}

/// Construct [`Monoid`] from optional [`Semigroup`].
/// Some [`Semigroup`] lack a suitable *identity element* for extension to a [`Monoid`].
///
/// # Examples
/// In [`Semigroup`] operations of [`crate::op::Min`] and [`crate::op::Max`], [`std::time::Instant`] does not have a suitable *identity element* for extension to a [`Monoid`].
/// ```compile_fail
/// use std::time::{Duration, Instant};
/// use semigroup::{Semigroup, Monoid, OptionMonoid};
///
/// #[derive(Debug, Clone, PartialEq, Semigroup)]
/// #[semigroup(monoid, commutative)]
/// pub struct BoundingDuration {
///     #[semigroup(with = "semigroup::op::Min")]
///     start: Instant,
///     #[semigroup(with = "semigroup::op::Max")]
///     end: Instant,
/// }
/// impl BoundingDuration {
///      pub fn duration(&self) -> Duration {
///         self.end - self.start
///     }
/// }
/// ```
///
/// In such case, [`OptionMonoid`] is useful.
/// ```
/// use std::time::{Duration, Instant};
/// use semigroup::{Semigroup, Monoid, OptionMonoid};
///
/// #[derive(Debug, Clone, PartialEq, Semigroup)]
/// #[semigroup(commutative)]
/// pub struct BoundingDuration {
///     #[semigroup(with = "semigroup::op::Min")]
///     start: Instant,
///     #[semigroup(with = "semigroup::op::Max")]
///     end: Instant,
/// }
/// impl BoundingDuration {
///      pub fn duration(&self) -> Duration {
///         self.end - self.start
///     }
/// }
///
/// let (now, mut bd) = (Instant::now(), OptionMonoid::identity());
/// let v = vec! [
///     OptionMonoid::from(BoundingDuration { start: now + Duration::from_millis(50), end: now + Duration::from_millis(100) }),
///     OptionMonoid::from(BoundingDuration { start: now + Duration::from_millis(100), end: now + Duration::from_millis(200) }),
///     OptionMonoid::from(BoundingDuration { start: now + Duration::from_millis(150), end: now + Duration::from_millis(300) }),
/// ];
/// for vi in v {
///     bd = bd.semigroup(vi);
/// }
/// assert_eq!(bd.as_ref().unwrap().duration(), Duration::from_millis(250));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, OpPriv)]
#[op(monoid, commutative, identity = Self(None), commutative_where = "T: crate::Commutative")]
#[properties_priv(monoid, commutative, commutative_where = "T: crate::Commutative")]
pub struct OptionMonoid<T: Semigroup>(pub Option<T>);
impl<T: Semigroup> From<T> for OptionMonoid<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}
impl<T: Semigroup> Op<Option<T>> for OptionMonoid<T> {
    fn lift_op_assign(base: &mut Option<T>, other: Option<T>) {
        match (base, other) {
            (Some(b), Some(o)) => Semigroup::op_assign(b, o),
            (_, None) => {}
            (b @ None, o @ Some(_)) => *b = o,
        }
    }
}

macro_rules! impl_tuple_monoid {
    ($($idx:tt: $t:tt),+) => {
        impl<$($t: $crate::Monoid),+> $crate::Monoid for ($($t,)+) {
            fn identity() -> Self {
                ($($t::identity(),)+)
            }
        }
    };
}
impl_tuple_monoid!(0: A);
impl_tuple_monoid!(0: A, 1: B);
impl_tuple_monoid!(0: A, 1: B, 2: C);
impl_tuple_monoid!(0: A, 1: B, 2: C, 3: D);
impl_tuple_monoid!(0: A, 1: B, 2: C, 3: D, 4: E);
impl_tuple_monoid!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F);
impl_tuple_monoid!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G);
impl_tuple_monoid!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H);
impl_tuple_monoid!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I);
impl_tuple_monoid!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J);
impl_tuple_monoid!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K);
impl_tuple_monoid!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L);
impl_tuple_monoid!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L, 12: M);

#[cfg(feature = "test")]
pub mod test_monoid {
    use std::fmt::Debug;

    use crate::{
        combine::test_combine::assert_combine_iter_monoid,
        semigroup::test_semigroup::assert_associative_law,
    };

    use super::*;

    /// Assert that the given type satisfies the *monoid* property.
    ///
    /// # Usage
    /// Same to [`crate::assert_semigroup!`].
    ///
    /// # Examples
    /// ```
    /// use semigroup::{assert_monoid, op::Coalesce};
    ///
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(None);
    /// let c = Coalesce(Some(3));
    /// assert_monoid!(a, b, c);
    ///
    /// let v = vec![a, b, c];
    /// assert_monoid!(&v);
    /// ```
    ///
    /// # Panics
    /// - If the given function does not satisfy the *monoid* property.
    /// ```should_panic
    /// use semigroup::{assert_monoid, Op, Semigroup};
    /// #[derive(Debug, Clone, PartialEq, Op)]
    /// #[op(monoid, identity = Self(0))]
    /// pub struct Sub(i32);
    /// impl Op<i32> for Sub {
    ///     fn lift_op_assign(base: &mut i32, other: i32) {
    ///         *base -= other;
    ///     }
    /// }
    /// let a = Sub(1);
    /// let b = Sub(2);
    /// let c = Sub(3);
    /// assert_monoid!(a, b, c);
    /// ```
    ///
    /// - The input iterator has less than 3 items.
    /// ```compile_fail
    /// use semigroup::{assert_monoid, op::Coalesce};
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(None);
    /// assert_monoid!(a, b);
    /// ```
    /// ```should_panic
    /// use semigroup::{assert_monoid, op::Coalesce};
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(None);
    /// assert_monoid!(&vec![a, b]);
    /// ```
    #[macro_export]
    macro_rules! assert_monoid {
        ($a:expr, $b: expr, $($tail: expr),*) => {
            {
                let v = vec![$a, $b, $($tail),*];
                $crate::assert_monoid!(&v)
            }
        };
        ($v:expr) => {
            {
                let (a, b, c) = $crate::test_semigroup::pick3($v);
                $crate::test_monoid::assert_monoid_impl(a.clone(), b.clone(), c.clone());
            }
        };
    }

    pub fn assert_monoid_impl<T: Monoid + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_monoid_identity_associative_law(a.clone(), b.clone(), c.clone());
        assert_combine_iter_monoid(a.clone(), b.clone(), c.clone());
        assert_monoid_tuple(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_option_monoid<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_monoid_impl(
            OptionMonoid::<T>::from(a.clone()),
            OptionMonoid::<T>::from(b.clone()),
            OptionMonoid::<T>::from(c.clone()),
        );
    }
    pub fn assert_monoid_identity_associative_law<T: Monoid + Clone + PartialEq + Debug>(
        a: T,
        b: T,
        c: T,
    ) {
        assert_eq!(T::identity(), Semigroup::op(T::identity(), T::identity()));
        assert_eq!(a.clone(), Semigroup::op(a.clone(), Monoid::identity()));
        assert_eq!(a.clone(), Semigroup::op(Monoid::identity(), a.clone()));
        assert_eq!(b.clone(), Semigroup::op(b.clone(), Monoid::identity()));
        assert_eq!(b.clone(), Semigroup::op(Monoid::identity(), b.clone()));
        assert_eq!(c.clone(), Semigroup::op(c.clone(), Monoid::identity()));
        assert_eq!(c.clone(), Semigroup::op(Monoid::identity(), c.clone()));

        assert_associative_law(a.clone(), b.clone(), c.clone());
        assert_associative_law(Monoid::identity(), b.clone(), c.clone());
        assert_associative_law(a.clone(), Monoid::identity(), c.clone());
        assert_associative_law(a.clone(), b.clone(), Monoid::identity());
    }
    pub fn assert_monoid_tuple<T: Monoid + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let abi = (a.clone(), b.clone(), Monoid::identity());
        let cib = (c.clone(), Monoid::identity(), b.clone());
        let ica = (Monoid::identity(), c.clone(), a.clone());

        let (a_c_i, b_i_c, i_b_a) = abi.semigroup(cib).semigroup(ica);
        assert_eq!(a_c_i, a.clone().semigroup(c.clone()));
        assert_eq!(b_i_c, b.clone().semigroup(c.clone()));
        assert_eq!(i_b_a, b.clone().semigroup(a.clone()));
    }
}
