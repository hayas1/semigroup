use std::ops::{Deref, DerefMut};

use crate::{Annotate, Annotated, AnnotatedSemigroup, Semigroup};

/// [`Construction`] represents [`crate::Semigroup`] as a [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).
///
/// Use `#[derive(Op)]` to automatically implement this trait alongside [`Op`].
/// The `#[op(...)]` attribute supports `annotated`, `monoid`, and `commutative` options.
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
    ///         if base.is_none() && other.is_some() {
    ///             *base = other;
    ///         }
    ///     }
    /// }
    ///
    /// let a = Coalesce(Some(1));
    /// assert_eq!(a.into_inner(), Some(1));
    /// ```
    fn into_inner(self) -> T;
}
pub trait Op<T>: Semigroup + Construction<T> {
    /// Assign-based semigroup operation on the inner type `T`.
    ///
    /// This is the core method to implement when defining a new [`Op`].
    /// The result of combining `base` and `other` is stored back into `base`.
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
    ///         if base.is_none() && other.is_some() {
    ///             *base = other;
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

/// [`AnnotatedOp`] represents [`crate::AnnotatedSemigroup`] as a [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) like [`Construction`].
///
/// Use `#[derive(Op)]` with `#[op(annotated)]` to automatically implement this trait.
///
/// # Examples
/// Simple example see [`crate::Semigroup#construction`].
pub trait AnnotatedOp<T, A>: Op<T> + AnnotatedSemigroup<A> + Annotate<A> {
    /// Assign-based annotated semigroup operation on the inner types `T` and `A`.
    ///
    /// This is the core method to implement when defining an annotated [`Op`].
    /// The result of combining `base` and `other` (including annotations) is stored back into `base`.
    fn lift_annotated_op_assign(base: Annotated<&mut T, &mut A>, other: Annotated<T, A>);

    /// Semigroup operation between `base` and `other` with constructed type.
    /// When `T` does not implement [`crate::AnnotatedSemigroup`], this function can be used.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{Annotate, Annotated, AnnotatedOp, Construction, Op};
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Op)]
    /// #[op(annotated)]
    /// struct Coalesce<T>(Option<T>);
    /// impl<A, T> AnnotatedOp<Option<T>, A> for Coalesce<T> {
    ///     fn lift_annotated_op_assign(
    ///         mut base: Annotated<&mut Option<T>, &mut A>,
    ///         other: Annotated<Option<T>, A>,
    ///     ) {
    ///         if base.value().is_none() && other.value().is_some() {
    ///             base.replace(other);
    ///         }
    ///     }
    /// }
    ///
    /// let a = Annotated::new(None, "first");
    /// let b = Annotated::new(Some(2), "second");
    /// assert_eq!(Coalesce::lift_annotated_op(a, b), b);
    /// ```
    fn lift_annotated_op(base: Annotated<T, A>, other: Annotated<T, A>) -> Annotated<T, A> {
        AnnotatedSemigroup::annotated_op(base.map(Self::from), other.map(Self::from))
            .map(Self::into_inner)
    }
}

/// [`MonoidOp`] represents [`crate::Monoid`] as a [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html). like [`Construction`].
///
/// Use `#[derive(Op)]` with `#[op(monoid, identity = ...)]` to automatically implement this trait.
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
    ///         if base.is_none() && other.is_some() {
    ///             *base = other;
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
