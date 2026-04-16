use semigroup_derive::{OpPriv, properties_priv};

use crate::{
    Annotate, Annotated, AnnotatedOp, AnnotatedSemigroup, Construction, Lazy, Op, Semigroup,
};

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
        self.map(Dual).fold(Dual(fin), Semigroup::op).into_inner()
    }

    /// This method like [`CombineIterator::fold_final`], but no argument is required and return [`Option`].
    ///
    /// # Examples
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
    /// # Examples
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

/// [`Dual`] provides a reverse operation of [`Semigroup`], `op(Dual(a), Dual(b)) = op(b, a)`.
///
/// The [`Dual`] of a [`Commutative`](crate::Commutative) semigroup is the same semigroup, since `op(a, b) = op(b, a)` by definition.
///
/// [`Dual`] can be used to calculate right fold by left fold algorithm.
/// This works because `Dual::op(a, b) = T::op(b, a)` reverses the argument order, turning left-associativity into right-associativity:
/// - `left_fold([a, b, c])` = `op(op(a, b), c)`
/// - `right_fold([a, b, c])` = `op(c, op(b, a))` = `op(op(Dual(a), Dual(b)), Dual(c))`
///
/// # Properties
/// <!-- properties -->
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
/// ## Using with `#[derive(Semigroup)]`
/// Use `#[semigroup(with = "Dual(Op(_))")]` to keep the field as a plain inner type while
/// composing [`Dual`] with an `Op` wrapper inline.  The `_` is a placeholder for the field value.
///
/// ```
/// use semigroup::{Dual, Semigroup};
/// use semigroup::op::Coalesce;
///
/// /// Config whose `value` field is a plain `Option<u32>` (no wrapper type in the struct).
/// #[derive(Debug, Clone, PartialEq, Semigroup)]
/// pub struct Config {
///     /// Use Coalesce semantics reversed by Dual: last `Some` wins.
///     #[semigroup(with = "Dual(Coalesce(_))")]
///     pub value: Option<u32>,
/// }
///
/// let a = Config { value: Some(1) };
/// let b = Config { value: Some(2) };
/// // Normal Coalesce keeps first Some, Dual reverses it → last Some wins
/// assert_eq!(a.semigroup(b), Config { value: Some(2) });
///
/// let c = Config { value: None };
/// let d = Config { value: Some(3) };
/// assert_eq!(c.semigroup(d), Config { value: Some(3) });
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, OpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[op(
    annotated,
    manual_op_impl,
    manual_annotate_impl,
    annotation_where = "T: crate::AnnotatedSemigroup<A> + Annotate<A>",
    monoid,
    identity = Self(T::identity()),
    monoid_where = "T: crate::Monoid",
    commutative,
    commutative_where = "T: crate::Commutative"
)]
#[properties_priv(
    annotated,
    annotation_where = "T: crate::AnnotatedSemigroup<A> + Annotate<A>",
    monoid,
    monoid_where = "T: crate::Monoid",
    commutative,
    commutative_where = "T: crate::Commutative"
)]
pub struct Dual<T: Semigroup>(pub T);

impl<T: Semigroup + Annotate<A>, A> Annotate<A> for Dual<T> {
    type Annotation = T::Annotation;
    fn annotated(self, annotation: Self::Annotation) -> Annotated<Self, A> {
        self.into_inner().annotated(annotation).map(Self)
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
        let base_owned = unsafe { std::ptr::read(base) };
        let result = Semigroup::op(other, base_owned);
        unsafe { std::ptr::write(base, result) };
        std::mem::forget(_guard);
    }
}

impl<T: AnnotatedSemigroup<A> + Annotate<A>, A> AnnotatedOp<T, A> for Dual<T> {
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
        let (base_val_owned, base_ann_owned) =
            unsafe { (std::ptr::read(base_val), std::ptr::read(base_ann)) };
        let base_owned = Annotated::new(base_val_owned, base_ann_owned);
        let result = Semigroup::op(other, base_owned);
        let (new_val, new_ann) = result.into_parts();
        unsafe {
            std::ptr::write(base_val, new_val);
            std::ptr::write(base_ann, new_ann);
        };
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
        assert_dual_dual_original(a.clone(), b.clone(), c.clone());
        assert_dual_associative_law(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_dual_reverse<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
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

    pub fn assert_dual_dual_original<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_eq!(
            Semigroup::op(a.clone(), b.clone()),
            Semigroup::op(Dual(Dual(a.clone())), Dual(Dual(b.clone())))
                .into_inner()
                .into_inner()
        );
        assert_eq!(
            Semigroup::op(b.clone(), c.clone()),
            Semigroup::op(Dual(Dual(b.clone())), Dual(Dual(c.clone())))
                .into_inner()
                .into_inner()
        );
        assert_eq!(
            Semigroup::op(c.clone(), a.clone()),
            Semigroup::op(Dual(Dual(c.clone())), Dual(Dual(a.clone())))
                .into_inner()
                .into_inner()
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
