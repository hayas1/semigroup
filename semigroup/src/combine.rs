use semigroup_derive::OpPriv;

use crate::{Annotate, Annotated, AnnotatedOp, AnnotatedSemigroup, Lazy, Op, Semigroup};

/// Extensions for [`Iterator`]s that items implement [`Semigroup`].
/// Composed of a variety of the 3 main methods
/// - `fold`: requires an initial value, but return non-optional value
/// - `reduce`: does not require an initial value, but return optional value
/// - `combine`: does not require an initial value, and return non-optional value. (but, requires [`Monoid`](crate::Monoid))
pub trait CombineIterator: Sized + Iterator {
    /// Folds every [`Semigroup`] element. Given argument is the final value.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, CombineIterator, Semigroup};
    /// let v1 = vec![Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3))];
    /// assert_eq!(v1.into_iter().fold_final(Coalesce(Some(4))), Coalesce(Some(2)));
    ///
    /// let v2 = vec![Coalesce::<u32>(None), Coalesce(None), Coalesce(None)];
    /// assert_eq!(v2.into_iter().fold_final(Coalesce(Some(4))), Coalesce(Some(4)));
    /// ```
    fn fold_final(mut self, fin: Self::Item) -> Self::Item
    where
        Self::Item: Semigroup,
    {
        if let Some(init) = self.next() {
            self.chain(Some(fin)).fold(init, Semigroup::op)
        } else {
            fin
        }
    }

    /// Folds every [`Semigroup`] element in reverse order using [`Dual`]. Given argument is the final value.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, CombineIterator, Semigroup};
    /// let v1 = vec![Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3))];
    /// assert_eq!(v1.into_iter().rfold_final(Coalesce(Some(3))), Coalesce(Some(3)));
    ///
    /// let v2 = vec![Coalesce::<u32>(None), Coalesce(None), Coalesce(None)];
    /// assert_eq!(v2.into_iter().rfold_final(Coalesce(Some(4))), Coalesce(Some(4)));
    /// ```
    fn rfold_final(self, fin: Self::Item) -> Self::Item
    where
        Self::Item: Semigroup,
    {
        use crate::Construction;
        self.map(Dual).fold(Dual(fin), Semigroup::op).into_inner()
    }

    /// This method like [`CombineIterator::fold_final`], but no argument is required and return [`Option`].
    ///
    /// # Example
    /// ```
    /// use semigroup::{op::Coalesce, CombineIterator, Semigroup};
    /// let v1 = vec![Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3))];
    /// assert_eq!(v1.into_iter().lreduce(), Some(Coalesce(Some(2))));
    ///
    /// let v2 = vec![Coalesce::<u32>(None), Coalesce(None), Coalesce(None)];
    /// assert_eq!(v2.into_iter().lreduce(), Some(Coalesce(None)));
    ///
    /// let v3 = Vec::<Coalesce<u32>>::new();
    /// assert_eq!(v3.into_iter().lreduce(), None)
    /// ```
    fn lreduce(self) -> Option<Self::Item>
    where
        Self::Item: Semigroup,
    {
        self.reduce(Semigroup::op)
    }

    /// This method like [`CombineIterator::rfold_final`], but no argument is required and return [`Option`].
    ///
    /// # Example
    /// ```
    /// use semigroup::{op::Coalesce, CombineIterator, Semigroup};
    /// let v1 = vec![Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3))];
    /// assert_eq!(v1.into_iter().rreduce(), Some(Coalesce(Some(3))));
    ///
    /// let v2 = vec![Coalesce::<u32>(None), Coalesce(None), Coalesce(None)];
    /// assert_eq!(v2.into_iter().rreduce(), Some(Coalesce(None)));
    ///
    /// let v3 = Vec::<Coalesce<u32>>::new();
    /// assert_eq!(v3.into_iter().rreduce(), None)
    /// ```
    fn rreduce(self) -> Option<Self::Item>
    where
        Self::Item: Semigroup,
    {
        use crate::Construction;
        self.map(Dual).reduce(Semigroup::op).map(Dual::into_inner)
    }

    /// This method like [`CombineIterator::fold_final`], but no argument is required.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, CombineIterator, Semigroup};
    /// let v1 = vec![Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3))];
    /// assert_eq!(v1.into_iter().combine(), Coalesce(Some(2)));
    ///
    /// let v2 = vec![Coalesce::<u32>(None), Coalesce(None), Coalesce(None)];
    /// assert_eq!(v2.into_iter().combine(), Coalesce(None));
    /// ```
    #[cfg(feature = "monoid")]
    fn combine(self) -> Self::Item
    where
        Self::Item: crate::Monoid,
    {
        self.fold_final(crate::Monoid::identity())
    }

    /// This method like [`CombineIterator::rfold_final`], but no argument is required.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, CombineIterator, Semigroup};
    /// let v1 = vec![Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3))];
    /// assert_eq!(v1.into_iter().rcombine(), Coalesce(Some(3)));
    ///
    /// let v2 = vec![Coalesce::<u32>(None), Coalesce(None), Coalesce(None)];
    /// assert_eq!(v2.into_iter().rcombine(), Coalesce(None));
    /// ```
    #[cfg(feature = "monoid")]
    fn rcombine(self) -> Self::Item
    where
        Self::Item: crate::Monoid,
    {
        self.rfold_final(crate::Monoid::identity())
    }

    /// Collect into [`Lazy`]. If the iterator is empty, returns `None`.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, CombineIterator, Semigroup, Lazy};
    /// let v1 = vec![Coalesce(Some(1)), Coalesce(Some(2)), Coalesce(Some(3))];
    /// assert_eq!(
    ///     v1.into_iter().collect_lazy(),
    ///     Some(Lazy::from(Coalesce(Some(1))).semigroup(Coalesce(Some(2)).into()).semigroup(Coalesce(Some(3)).into()))
    /// );
    ///
    /// let v2 = Vec::<Coalesce<u32>>::new();
    /// assert_eq!(v2.into_iter().collect_lazy(), None);
    /// ```
    fn collect_lazy(self) -> Option<Lazy<Self::Item>>
    where
        Self::Item: Semigroup,
    {
        Lazy::from_iterator(self)
    }
}
impl<I: Iterator> CombineIterator for I {}

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
/// Use `#[semigroup(with = "Dual")]` on a field whose type is already a [`crate::Construction`]
/// type (e.g. `Coalesce<T>`, `Overwrite<T>`, `Sum<T>`) to reverse its operation.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, OpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[op(
    annotated,
    manual_op_impl,
    manual_annotate_impl,
    annotation_where = "T: crate::AnnotatedSemigroup<A>",
    monoid,
    identity = Self(T::identity()),
    monoid_where = "T: crate::Monoid",
    commutative,
    commutative_where = "T: crate::Commutative"
)]
pub struct Dual<T: Semigroup>(pub T);

impl<T: Semigroup, A> Annotate<A> for Dual<T> {
    type Annotation = A;
    fn annotated(self, annotation: A) -> Annotated<Self, A> {
        Annotated::new(self, annotation)
    }
}

impl<T: Semigroup> Op<T> for Dual<T> {
    /// Reversed operation: computes `*base = T::op(other, *base)`.
    ///
    /// Uses `ptr::read` / `ptr::write` to move out of `&mut T`.
    /// An abort-on-drop guard ensures soundness: if `T::op` panics, the process aborts
    /// rather than leaving `*base` in an invalid state that would cause a double-drop.
    fn lift_op_assign(base: &mut T, other: T) {
        struct AbortOnDrop;
        impl Drop for AbortOnDrop {
            fn drop(&mut self) {
                std::process::abort();
            }
        }
        let _guard = AbortOnDrop;
        // SAFETY: We immediately overwrite `base` with a valid value via `ptr::write`.
        // If `T::op` panics, `AbortOnDrop` aborts the process so `*base` is never
        // observed in its intermediate invalid state.
        let old = unsafe { std::ptr::read(base) };
        let new_val = T::op(other, old);
        unsafe { std::ptr::write(base, new_val) };
        std::mem::forget(_guard);
    }
}

impl<T: AnnotatedSemigroup<A>, A> AnnotatedOp<T, A> for Dual<T> {
    /// Reversed annotated operation: computes `*base = T::annotated_op(other, *base)`.
    ///
    /// Uses `ptr::read` / `ptr::write` to move both value and annotation out of `&mut`.
    /// An abort-on-drop guard ensures soundness if `T::annotated_op` panics.
    fn lift_annotated_op_assign(base: Annotated<&mut T, &mut A>, other: Annotated<T, A>) {
        struct AbortOnDrop;
        impl Drop for AbortOnDrop {
            fn drop(&mut self) {
                std::process::abort();
            }
        }
        let _guard = AbortOnDrop;
        let (base_val, base_ann) = base.into_parts();
        // SAFETY: We immediately overwrite both slots with valid values via `ptr::write`.
        // If `T::annotated_op` panics, `AbortOnDrop` aborts so neither slot is observed
        // in its intermediate invalid state.
        let old_val = unsafe { std::ptr::read(base_val) };
        let old_ann = unsafe { std::ptr::read(base_ann) };
        // Reversed: other op old_base (instead of old_base op other)
        let result = T::annotated_op(other, Annotated::new(old_val, old_ann));
        let (new_val, new_ann) = result.into_parts();
        unsafe { std::ptr::write(base_val, new_val) };
        unsafe { std::ptr::write(base_ann, new_ann) };
        std::mem::forget(_guard);
    }
}

#[cfg(feature = "test")]
pub mod test_combine {
    use std::fmt::Debug;

    use super::*;

    pub fn assert_combine_iter<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let ab = vec![a.clone(), b.clone()];
        assert_eq!(
            ab.into_iter().fold_final(c.clone()),
            Semigroup::op(Semigroup::op(a.clone(), b.clone()), c.clone())
        );

        let bc = vec![b.clone(), c.clone()];
        assert_eq!(
            bc.into_iter().rfold_final(a.clone()),
            Semigroup::op(Semigroup::op(c.clone(), b.clone()), a.clone())
        );
    }

    #[cfg(feature = "monoid")]
    pub fn assert_combine_iter_monoid<T: crate::Monoid + Clone + PartialEq + Debug>(
        a: T,
        b: T,
        c: T,
    ) {
        let abc = vec![a.clone(), b.clone(), c.clone()];
        assert_eq!(
            abc.clone().into_iter().combine(),
            Semigroup::op(Semigroup::op(a.clone(), b.clone()), c.clone())
        );
        assert_eq!(
            abc.clone().into_iter().rcombine(),
            Semigroup::op(Semigroup::op(c.clone(), b.clone()), a.clone())
        );
    }

    pub fn assert_semigroup_dual<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_dual_reverse(a.clone(), b.clone(), c.clone());
        assert_dual_associative_law(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_dual_reverse<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        use crate::Construction;
        assert_eq!(
            Semigroup::op(a.clone(), b.clone()),
            Semigroup::op(Dual(b.clone()), Dual(a.clone())).into_inner()
        );
        assert_eq!(
            Semigroup::op(b.clone(), c.clone()),
            Semigroup::op(Dual(c.clone()), Dual(b.clone())).into_inner()
        );
        assert_eq!(
            Semigroup::op(a.clone(), c.clone()),
            Semigroup::op(Dual(c.clone()), Dual(a.clone())).into_inner()
        );
    }

    pub fn assert_dual_associative_law<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let ab_c = Semigroup::op(
            Semigroup::op(Dual(a.clone()), Dual(b.clone())),
            Dual(c.clone()),
        );
        let a_bc = Semigroup::op(
            Dual(a.clone()),
            Semigroup::op(Dual(b.clone()), Dual(c.clone())),
        );
        assert_eq!(ab_c, a_bc);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Annotate, AnnotatedSemigroup, Construction, Semigroup, op::Coalesce};

    use super::*;

    #[test]
    fn test_dual_reverses_op() {
        let a = Coalesce(Some(1u32));
        let b = Coalesce(Some(2u32));

        // Normal Coalesce: first non-None wins
        assert_eq!(Semigroup::op(a, b), Coalesce(Some(1)));

        // Dual<Coalesce>: last non-None wins (reversed)
        assert_eq!(
            Semigroup::op(Dual(a), Dual(b)).into_inner(),
            Coalesce(Some(2))
        );
    }

    #[test]
    fn test_dual_semigroup() {
        let (a, b, c) = (
            Dual(Coalesce(Some(1u32))),
            Dual(Coalesce(Some(2))),
            Dual(Coalesce(Some(3))),
        );
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_dual_dual_is_original() {
        let a = Coalesce(Some(1u32));
        let b = Coalesce(Some(2u32));

        // Dual<Dual<T>> should behave the same as T
        assert_eq!(
            Semigroup::op(Dual(Dual(a)), Dual(Dual(b)))
                .into_inner()
                .into_inner(),
            Semigroup::op(a, b),
        );
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_dual_monoid() {
        let (a, b, c) = (
            Dual(Coalesce(Some(1u32))),
            Dual(Coalesce(Some(2))),
            Dual(Coalesce(Some(3))),
        );
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

    #[test]
    fn test_dual_annotated_op() {
        let a = Dual(Coalesce(Some(1u32))).annotated("first");
        let b = Dual(Coalesce(Some(2u32))).annotated("second");

        // Normal Dual<Coalesce> op(a, b): reverses Coalesce so last non-None wins → Some(2)
        // Annotation should follow the winning value → "second"
        let result = AnnotatedSemigroup::annotated_op(a, b);
        assert_eq!(result.value(), &Dual(Coalesce(Some(2))));
        assert_eq!(result.annotation(), &"second");

        // op(b, a): reverses Coalesce so last non-None wins → Some(1)
        // Annotation should follow the winning value → "first"
        let b2 = Dual(Coalesce(Some(2u32))).annotated("second");
        let a2 = Dual(Coalesce(Some(1u32))).annotated("first");
        let result2 = AnnotatedSemigroup::annotated_op(b2, a2);
        assert_eq!(result2.value(), &Dual(Coalesce(Some(1))));
        assert_eq!(result2.annotation(), &"first");
    }
}
