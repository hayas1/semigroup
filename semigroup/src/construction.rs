use std::ops::{Deref, DerefMut};

use crate::{Idempotent, Selected, Semigroup};

/// [`Construction`] represents [`crate::Semigroup`] as a [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).
///
/// # Examples
/// Simple example see [`crate::Semigroup#construction`].
pub trait Construction<T>: Sized + From<T> + Deref<Target = T> + DerefMut {
    /// Convert into inner type of [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).
    ///
    /// # Examples
    /// ```
    /// use semigroup::{Construction, Op, Semigroup};
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Op)]
    /// struct Coalesce<T>(Option<T>);
    /// impl<T> Op<Option<T>> for Coalesce<T> {
    ///     fn lift_op_assign(base: &mut Option<T>, other: Option<T>) {
    ///         match (&base, &other) {
    ///             (None, Some(_)) => *base = other,
    ///             _ => {},
    ///         }
    ///     }
    /// }
    ///
    /// let a = Coalesce(Some(1));
    /// assert_eq!(a.into_inner(), Some(1));
    /// ```
    fn into_inner(self) -> T;
}

/// [`Op`] represents [`crate::Semigroup`] as a [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) like [`Construction`].
///
/// Implement [`Op::lift_op_assign`] to define the semigroup operation on the inner type `T`.
/// The derive macro `#[derive(Op)]` automatically generates the [`Semigroup`](crate::Semigroup) and
/// [`Construction`] implementations from this single method.
///
/// # Examples
/// Simple example see [`crate::Semigroup#construction`].
pub trait Op<T>: Semigroup + Construction<T> {
    /// Assign-based semigroup operation on the inner type `T`.
    /// Required method for [`Op::lift_op`].
    fn lift_op_assign(base: &mut T, other: T);

    /// Semigroup operation between `base` and `other` with constructed type.
    /// When `T` does not implement [`crate::Semigroup`], this function can be used.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{Construction, Op, Semigroup};
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Op)]
    /// struct Coalesce<T>(Option<T>);
    /// impl<T> Op<Option<T>> for Coalesce<T> {
    ///     fn lift_op_assign(base: &mut Option<T>, other: Option<T>) {
    ///         match (&base, &other) {
    ///             (None, Some(_)) => *base = other,
    ///             _ => {},
    ///         }
    ///     }
    /// }
    ///
    /// let a = None;
    /// let b = Some(2);
    /// assert_eq!(Coalesce::lift_op(a, b), Some(2));
    /// ```
    fn lift_op(mut base: T, other: T) -> T {
        Self::lift_op_assign(&mut base, other);
        base
    }
}

/// [`IdempotentOp`] represents [`Idempotent`] as a [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) like [`Construction`].
///
/// Implement [`IdempotentOp::lift_select`] to declare which inner value is selected.
/// The derive macro `#[op(idempotent)]` automatically generates the [`Op`], [`Semigroup`], and
/// [`Idempotent`] implementations from this single method.
///
/// # Examples
/// ```
/// use semigroup::{Construction, IdempotentOp, Op, Selected, Semigroup, Annotate};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Op)]
/// #[op(idempotent)]
/// struct Coalesce<T>(Option<T>);
/// impl<T> IdempotentOp<Option<T>> for Coalesce<T> {
///     fn lift_select(base: &Option<T>, other: &Option<T>) -> Selected {
///         match (base, other) {
///             (None, Some(_)) => Selected::Other,
///             _ => Selected::Base,
///         }
///     }
/// }
///
/// let a = Coalesce(None).annotated("first");
/// let b = Coalesce(Some(2)).annotated("second");
/// let ab = a.semigroup(b);
/// assert_eq!(ab.value(), &Coalesce(Some(2)));
/// assert_eq!(ab.annotation(), &"second");
/// ```
pub trait IdempotentOp<T>: Idempotent + Construction<T> {
    /// Determine which of the two inner values is selected by the operation.
    fn lift_select(base: &T, other: &T) -> Selected;

    /// Assign-based idempotent semigroup operation on the inner type `T`.
    fn lift_select_assign(base: &mut T, other: T) {
        if let Selected::Other = Self::lift_select(base, &other) {
            *base = other;
        }
    }
}

/// [`MonoidOp`] represents [`crate::Monoid`] as a [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html). like [`Construction`].
///
/// # Examples
/// Simple example see [`crate::Monoid#construction`].
#[cfg(feature = "monoid")]
pub trait MonoidOp<T>: Op<T> + crate::Monoid {
    /// Get monoid *identity element* with constructed type.
    /// When `T` does not implement [`crate::Monoid`], this function can be used.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{Construction, MonoidOp, Op, Semigroup};
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Op)]
    /// #[op(monoid, identity = Self(None))]
    /// struct Coalesce<T>(Option<T>);
    /// impl<T> Op<Option<T>> for Coalesce<T> {
    ///     fn lift_op_assign(base: &mut Option<T>, other: Option<T>) {
    ///         match (&base, &other) {
    ///             (None, Some(_)) => *base = other,
    ///             _ => {},
    ///         }
    ///     }
    /// }
    ///
    /// let a: Option<u32> = Coalesce::lift_identity();
    /// assert_eq!(a, None);
    /// ```
    fn lift_identity() -> T {
        Self::identity().into_inner()
    }
}
