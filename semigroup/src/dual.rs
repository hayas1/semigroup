use std::ops::{Deref, DerefMut};

use crate::{Construction, Op, Semigroup};

/// `Dual<T>` is a wrapper type that reverses the [`Semigroup`] operation of `T`.
///
/// If `T: Semigroup` with operation `op(a, b)`, then `Dual<T>` has operation `op(b, a)`.
///
/// This is analogous to `Dual` in Haskell's `Data.Monoid`.
///
/// The `Dual` of a [`crate::Monoid`] is also a `Monoid` with the same identity element,
/// since `op(identity, a) = a = op(a, identity)` implies the same for the reversed operation.
///
/// The `Dual` of a [`crate::Commutative`] semigroup is the same semigroup,
/// since `op(a, b) = op(b, a)` by definition.
///
/// # Using with `#[derive(Semigroup)]`
/// Use `#[semigroup(with = "Dual")]` on a field whose type is already a [`Construction`] type
/// (e.g. `Coalesce<T>`, `Overwrite<T>`, `Sum<T>`) to reverse its operation.
///
/// ```
/// use semigroup::{Dual, Semigroup};
/// use semigroup::op::{Coalesce, Overwrite};
///
/// #[derive(Debug, Clone, PartialEq, Semigroup)]
/// pub struct Config {
///     /// Last writer wins (reversed Coalesce = Overwrite semantics)
///     #[semigroup(with = "Dual")]
///     pub value: Coalesce<u32>,
/// }
///
/// let a = Config { value: Coalesce(Some(1)) };
/// let b = Config { value: Coalesce(Some(2)) };
/// // Normal Coalesce would keep Some(1), but Dual reverses it to keep Some(2)
/// assert_eq!(a.semigroup(b), Config { value: Coalesce(Some(2)) });
/// ```
///
/// # Examples
/// ## Reversing a non-commutative operation
/// ```
/// use semigroup::{op::Coalesce, Dual, Semigroup, Construction};
///
/// let a = Coalesce(Some(1));
/// let b = Coalesce(Some(2));
///
/// // Normal Coalesce: takes first non-None
/// assert_eq!(Semigroup::op(a, b), Coalesce(Some(1)));
///
/// // Dual<Coalesce>: reverses Coalesce, takes last non-None
/// assert_eq!(Semigroup::op(Dual(a), Dual(b)).into_inner(), Coalesce(Some(2)));
/// ```
///
/// ## Right fold via left fold
/// ```
/// # #[cfg(feature = "monoid")]
/// # {
/// use semigroup::{op::Coalesce, Dual, CombineIterator, Monoid, Semigroup};
///
/// let v = (1..100u32).map(Some).map(Coalesce).collect::<Vec<_>>();
///
/// // Left fold: first non-None wins → Some(1)
/// assert_eq!(v.iter().cloned().combine(), Coalesce(Some(1)));
///
/// // Right fold via Dual: last non-None wins → Some(99)
/// assert_eq!(v.iter().cloned().rcombine(), Coalesce(Some(99)));
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dual<T>(pub T);

impl<T> From<T> for Dual<T> {
    fn from(t: T) -> Self {
        Dual(t)
    }
}

impl<T> Deref for Dual<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Dual<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Construction<T> for Dual<T> {
    fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Semigroup> Op<T> for Dual<T> {
    /// Reversed operation: `lift_op_assign(base, other)` computes `*base = T::op(other, *base)`.
    ///
    /// # Safety
    /// Uses `ptr::read` / `ptr::write` to move out of `&mut T`.
    /// An abort-on-drop guard ensures soundness: if `T::op` panics, the process aborts
    /// rather than leaving `*base` in an invalid state that would cause a double-drop.
    fn lift_op_assign(base: &mut T, other: T) {
        // Guard that aborts the process if T::op panics, preventing a double-drop of *base.
        struct AbortOnDrop;
        impl Drop for AbortOnDrop {
            fn drop(&mut self) {
                std::process::abort();
            }
        }

        let _guard = AbortOnDrop;
        // SAFETY: We immediately overwrite `base` with a valid value via `ptr::write`.
        // If `T::op` panics, the AbortOnDrop guard above aborts the process,
        // so `*base` is never observed in its intermediate invalid state.
        let old = unsafe { std::ptr::read(base) };
        let new_val = T::op(other, old);
        unsafe { std::ptr::write(base, new_val) };
        std::mem::forget(_guard);
    }
}

impl<T: Semigroup> Semigroup for Dual<T> {
    fn op_assign(base: &mut Self, other: Self) {
        <Dual<T> as Op<T>>::lift_op_assign(&mut base.0, other.0);
    }
}

#[cfg(feature = "monoid")]
impl<T: crate::Monoid> crate::Monoid for Dual<T> {
    fn identity() -> Self {
        Dual(T::identity())
    }
}

impl<T: crate::Commutative> crate::Commutative for Dual<T> {}

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup, op::Coalesce};

    use super::*;

    #[test]
    fn test_dual_reverses_op() {
        let a = Coalesce(Some(1u32));
        let b = Coalesce(Some(2u32));

        // Normal Coalesce: first non-None wins
        assert_eq!(Semigroup::op(a, b), Coalesce(Some(1)));

        // Dual<Coalesce>: last non-None wins (reversed)
        assert_eq!(Semigroup::op(Dual(a), Dual(b)).into_inner(), Coalesce(Some(2)));
    }

    #[test]
    fn test_dual_semigroup() {
        let (a, b, c) = (Dual(Coalesce(Some(1u32))), Dual(Coalesce(Some(2))), Dual(Coalesce(Some(3))));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_dual_dual_is_original() {
        let a = Coalesce(Some(1u32));
        let b = Coalesce(Some(2u32));

        // Dual<Dual<T>> should behave the same as T
        assert_eq!(
            Semigroup::op(Dual(Dual(a)), Dual(Dual(b))).into_inner().into_inner(),
            Semigroup::op(a, b),
        );
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_dual_monoid() {
        let (a, b, c) = (Dual(Coalesce(Some(1u32))), Dual(Coalesce(Some(2))), Dual(Coalesce(Some(3))));
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    fn test_dual_lift_op() {
        let a = Coalesce(Some(1u32));
        let b = Coalesce(Some(2u32));

        // lift_op_assign reverses the operands
        let result = Dual::<Coalesce<u32>>::lift_op(a, b);
        assert_eq!(result, Coalesce(Some(2)));
    }
}
