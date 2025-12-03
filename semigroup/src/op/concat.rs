use semigroup_derive::{OpPriv, properties_priv};

use crate::{Annotate, Annotated, AnnotatedOp};

/// A [`Semigroup`](crate::Semigroup) [op construction](crate::Op) that concatenates two values.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Concat, Construction, Semigroup};
///
/// let a = Concat(vec![1, 2]);
/// let b = Concat(vec![3, 4]);
///
/// assert_eq!(a.semigroup(b).into_inner(), vec![1, 2, 3, 4]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, OpPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[op(
    annotated,
    monoid,
    identity = Self(Default::default()),
    annotation_type_param = "A: Default + Extend<A::Item> + IntoIterator",
    annotation_where = "A::Item: Clone",
    unit_annotation = "vec![(); 0]",
    manual_annotate_impl
)]
#[properties_priv(
    annotated,
    monoid,
    annotation_where = "A: IntoIterator + FromIterator<A::Item>, A::Item: Clone"
)]
pub struct Concat<T: Default + Extend<T::Item> + IntoIterator>(pub T);
impl<T: Default + Extend<T::Item> + IntoIterator, A: Default + Extend<A::Item> + IntoIterator>
    AnnotatedOp<T, A> for Concat<T>
where
    A::Item: Clone,
{
    fn lift_annotated_op_assign(mut base: Annotated<&mut T, &mut A>, other: Annotated<T, A>) {
        let (other_value, other_annotation) = other.into_parts();
        base.value_mut().extend(other_value);
        base.annotation_mut().extend(other_annotation);
    }
}
impl<T: Default + Extend<T::Item> + IntoIterator, A: Default + Extend<A::Item> + IntoIterator>
    Annotate<A> for Concat<T>
where
    A::Item: Clone,
{
    type Annotation = A::Item;
    fn annotated(self, annotation: Self::Annotation) -> Annotated<Self, A> {
        let iter = self.0.into_iter();
        let (mut values, mut annotations) = (T::default(), A::default());
        let (lower, upper) = iter.size_hint();
        match upper.filter(|&u| u == lower) {
            Some(len) => {
                values.extend(iter);
                annotations.extend(std::iter::repeat_n(annotation, len));
            }
            None => {
                let (val, ann): (Vec<_>, Vec<_>) = iter.map(|v| (v, annotation.clone())).collect();
                values.extend(val);
                annotations.extend(ann);
            }
        }
        Annotated::new(Self(values), annotations)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

    use super::*;

    #[test]
    fn test_concat_semigroup() {
        let (a, b, c) = (Concat(vec![1]), Concat(vec![2]), Concat(vec![3]));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_concat_monoid() {
        let (a, b, c) = (Concat(vec![1]), Concat(vec![2]), Concat(vec![3]));
        crate::assert_monoid!(a, b, c)
    }

    #[test]
    fn test_concat() {
        let (a, b) = (Concat(vec![1]), Concat(vec![2]));
        assert_eq!(a.clone().semigroup(b.clone()).into_inner(), vec![1, 2]);
        assert_eq!(b.semigroup(a).into_inner(), vec![2, 1]);
    }
}
