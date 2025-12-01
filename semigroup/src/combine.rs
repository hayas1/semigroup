use crate::{Lazy, Semigroup};

/// Extensions for [`Iterator`]s that items implement [`Semigroup`].
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

    /// Folds every [`Semigroup`] element in reverse order using [`Reversible`]. Given argument is the final value.
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
        self.fold(fin, Reversible::rev_op)
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
        self.reduce(Reversible::rev_op)
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

/// [`Reversible`] provides a reverse operation of [`Semigroup`], `rev_op(a, b) = op(b, a)`.
///
/// If `T` is [`Commutative`](crate::Commutative), then `op(a, b) = op(b, a)`, and thus [`Reversible`] is meaningless.
///
/// ## Calculate right fold by left fold algorithm
/// By using [`Reversible`], a right fold can be computed using a left fold algorithm.
/// - Let the underlying operation be `a ⊙ b := op(a, b)`, and `a ⊡ b := rev_op(a, b) = op(b, a) = b ⊙ a`
/// - `op` is [`Semigroup`], so that has associativity property: `a ⊙ b ⊙ c = a ⊙ (b ⊙ c) = (a ⊙ b) ⊙ c`
/// - Now, `c ⊙ b ⊙ a = rev_op(b, c) ⊙ a = rev_op(a, rev_op(b, c)) = rev_op(a, b ⊡ c) = a ⊡ b ⊡ c`
/// - This implies that right fold of `op`` `vn ⊙ vn-1 ⊙ ... ⊙ v1` is equal to left fold of `rev_op` `v1 ⊡ v2 ⊡ ... ⊡ vn`.
///
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ## Simple reverse two elements
/// ```
/// use semigroup::{op::Coalesce, Reversible, Construction, Semigroup};
///
/// let a = Coalesce(Some(1));
/// let b = Coalesce(Some(2));
///
/// assert_eq!(Semigroup::op(a, b), Coalesce(Some(1)));
/// assert_eq!(Reversible::rev_op(a, b), Coalesce(Some(2)));
/// ```
///
/// ## Calculate right fold by left fold algorithm
/// ```
/// # #[cfg(feature = "monoid")]
/// # {
/// use semigroup::{op::Coalesce, Reversible, Construction, Semigroup, Monoid};
///
/// let v = (1..100).map(Some).map(Coalesce).collect::<Vec<_>>();
///
/// assert_eq!(v.iter().cloned().fold(Monoid::identity(), Semigroup::op), Coalesce(Some(1)));
/// assert_eq!(v.iter().cloned().fold(Monoid::identity(), Reversible::rev_op), Coalesce(Some(99)));
/// # }
/// ```
pub trait Reversible: Semigroup {
    fn rev_op(base: Self, other: Self) -> Self {
        Semigroup::op(other, base)
    }
}
impl<T: Semigroup> Reversible for T {}

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

    pub fn assert_semigroup_reverse<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_reverse_reverse(a.clone(), b.clone(), c.clone());
        assert_reverse_associative_law(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_reverse_reverse<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_eq!(
            Semigroup::op(a.clone(), b.clone()),
            Reversible::rev_op(b.clone(), a.clone())
        );
        assert_eq!(
            Semigroup::op(b.clone(), c.clone()),
            Reversible::rev_op(c.clone(), b.clone())
        );
        assert_eq!(
            Semigroup::op(a.clone(), c.clone()),
            Reversible::rev_op(c.clone(), a.clone())
        );
    }
    pub fn assert_reverse_associative_law<T: Semigroup + Clone + PartialEq + Debug>(
        a: T,
        b: T,
        c: T,
    ) {
        let ab_c = Reversible::rev_op(Reversible::rev_op(a.clone(), b.clone()), c.clone());
        let a_bc = Reversible::rev_op(a.clone(), Reversible::rev_op(b.clone(), c.clone()));
        assert_eq!(ab_c, a_bc);
    }
}
