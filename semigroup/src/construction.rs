use std::ops::{Deref, DerefMut};

use crate::{Selected, Selective, Semigroup};

/// [`Construction`] represents [`crate::Semigroup`] as a [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).
///
/// # Examples
/// Simple example see [`crate::Semigroup#construction`].
pub trait Construction<T>: Sized + From<T> + Deref<Target = T> + DerefMut {
    /// Convert into inner type of [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).
    ///
    /// # Examples
    /// ```
    /// use semigroup::{Construction, SemigroupOp, Semigroup};
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOp)]
    /// #[semigroup_op(impl(semigroup = |base, other| match (&base, &other) {
    ///         (None, Some(_)) => *base = other,
    ///         _ => {},
    ///     }))]
    /// struct Coalesce<T>(Option<T>);
    ///
    /// let a = Coalesce(Some(1));
    /// assert_eq!(a.into_inner(), Some(1));
    /// ```
    fn into_inner(self) -> T;
}

/// [`SemigroupOp`] represents [`crate::Semigroup`] as a [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) like [`Construction`].
///
/// Implement [`SemigroupOp::lift_op_assign`] to define the semigroup operation on the inner type `T`.
/// The derive macro `#[derive(SemigroupOp)]` automatically generates the [`Semigroup`](crate::Semigroup) and
/// [`Construction`] implementations from this single method.
///
/// # Examples
/// Simple example see [`crate::Semigroup#construction`].
pub trait SemigroupOp<T>: Semigroup + Construction<T> {
    /// Assign-based semigroup operation on the inner type `T`.
    /// Required method for [`SemigroupOp::lift_op`].
    fn lift_op_assign(base: &mut T, other: T);

    /// Semigroup operation between `base` and `other` with constructed type.
    /// When `T` does not implement [`crate::Semigroup`], this function can be used.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{Construction, SemigroupOp, Semigroup};
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOp)]
    /// #[semigroup_op(impl(semigroup = |base, other| match (&base, &other) {
    ///         (None, Some(_)) => *base = other,
    ///         _ => {},
    ///     }))]
    /// struct Coalesce<T>(Option<T>);
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

/// [`SelectiveOp`] represents [`Selective`] as a [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) like [`Construction`].
///
/// `#[derive(SelectiveOp)]` implements [`SelectiveOp::lift_select`] with the function given as `impl(selective = ..)`,
/// and [`Selective`] on it. A selective op usually derives [`SemigroupOp`] with `impl(semigroup = Self::lift_select_assign)`.
///
/// # Examples
/// ```
/// use semigroup::{Construction, SelectiveOp, SemigroupOp, Selected, Semigroup, Annotate};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOp, SelectiveOp)]
/// #[semigroup_op(impl(semigroup = Self::lift_select_assign, selective = |base, other| match (base, other) {
///         (None, Some(_)) => Selected::Other,
///         _ => Selected::Base,
///     }))]
/// struct Coalesce<T>(Option<T>);
///
/// let a = Coalesce(None).annotated("first");
/// let b = Coalesce(Some(2)).annotated("second");
/// let ab = a.semigroup(b);
/// assert_eq!(ab.value(), &Coalesce(Some(2)));
/// assert_eq!(ab.annotation(), &"second");
/// ```
pub trait SelectiveOp<T>: Selective + Construction<T> {
    /// Determine which of the two inner values is selected by the operation.
    fn lift_select(base: &T, other: &T) -> Selected;

    /// Assign-based selective semigroup operation on the inner type `T`.
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
pub trait MonoidOp<T>: SemigroupOp<T> + crate::Monoid {
    /// Get monoid *identity element* with constructed type.
    /// When `T` does not implement [`crate::Monoid`], this function can be used.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{Construction, MonoidOp, SemigroupOp, Semigroup};
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, SemigroupOp, MonoidOp)]
    /// #[semigroup_op(impl(semigroup = |base, other| match (&base, &other) {
    ///         (None, Some(_)) => *base = other,
    ///         _ => {},
    ///     }, monoid = Self(None)))]
    /// struct Coalesce<T>(Option<T>);
    ///
    /// let a: Option<u32> = Coalesce::lift_identity();
    /// assert_eq!(a, None);
    /// ```
    fn lift_identity() -> T {
        Self::identity().into_inner()
    }
}

/// [`CommutativeOp`] represents [`crate::Commutative`] as a [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) like [`MonoidOp`].
///
/// It has no method: `#[derive(CommutativeOp)]` declares that the op of [`SemigroupOp`] is commutative.
pub trait CommutativeOp<T>: SemigroupOp<T> + crate::Commutative {}
