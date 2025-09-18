use crate::annotate::Annotated;

pub trait Semigroup {
    fn semigroup_op(base: Self, other: Self) -> Self;
}
pub trait AnnotatedSemigroup<A>: Sized {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A>;
    fn default_semigroup_op(
        base: Self,
        other: Self,
        base_annotation: A,
        other_annotation: A,
    ) -> Self {
        Self::annotated_op(
            Annotated::new(base, base_annotation),
            Annotated::new(other, other_annotation),
        )
        .into_value()
    }
}
impl<T: AnnotatedSemigroup<A>, A> Semigroup for Annotated<T, A> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        AnnotatedSemigroup::annotated_op(base, other)
    }
}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use std::fmt::Debug;

    use crate::{
        lazy::tests::assert_lazy_evaluation,
        reverse::tests::{assert_reverse, assert_reversed_associative_law},
    };

    use super::*;

    #[macro_export]
    macro_rules! assert_semigroup_op {
        ($a:expr, $b: expr, $c: expr) => {
            $crate::semigroup::tests::assert_semigroup_op_impl($a, $b, $c)
        };
    }
    pub use assert_semigroup_op;

    pub fn assert_semigroup_op_impl<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_associative_law(a.clone(), b.clone(), c.clone());
        assert_reverse(a.clone(), b.clone(), c.clone());
        assert_reversed_associative_law(a.clone(), b.clone(), c.clone());
        assert_lazy_evaluation(a.clone(), b.clone(), c.clone());
    }

    pub fn associative_law<T: Semigroup + Clone>(a: T, b: T, c: T) -> (T, T) {
        let ab_c = T::semigroup_op(T::semigroup_op(a.clone(), b.clone()), c.clone());
        let a_bc = T::semigroup_op(a.clone(), T::semigroup_op(b.clone(), c.clone()));
        (ab_c, a_bc)
    }
    pub fn assert_associative_law<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let (ab_c, a_bc) = associative_law(a, b, c);
        assert_eq!(ab_c, a_bc);
    }
}
