use crate::annotate::Annotated;

pub trait Semigroup {
    fn semigroup_op(base: Self, other: Self) -> Self;
}
pub trait AnnotatedSemigroup<A>: Sized {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A>;
}
impl<T: AnnotatedSemigroup<A>, A> Semigroup for Annotated<T, A> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        AnnotatedSemigroup::annotated_op(base, other)
    }
}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use std::fmt::Debug;

    use crate::op::reverse::Reverse;

    use super::*;

    pub fn associative_law<T: Semigroup + Clone>(a: T, b: T, c: T) -> (T, T) {
        let ab_c = T::semigroup_op(T::semigroup_op(a.clone(), b.clone()), c.clone());
        let a_bc = T::semigroup_op(a.clone(), T::semigroup_op(b.clone(), c.clone()));
        (ab_c, a_bc)
    }
    pub fn assert_associative_law<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let (ab_c, a_bc) = associative_law(a, b, c);
        assert_eq!(ab_c, a_bc);
    }

    pub fn reverse<T: Semigroup + Clone>(a: T, b: T) -> (T, T) {
        let (ra, rb) = (Reverse(a.clone()), Reverse(b.clone()));
        (T::semigroup_op(a, b), Reverse::<T>::semigroup_op(rb, ra).0)
    }
    pub fn assert_reversed_associative_law<T: Semigroup + Clone + PartialEq + Debug>(
        a: T,
        b: T,
        c: T,
    ) {
        let (ra, rb, rc) = (Reverse(a), Reverse(b), Reverse(c));
        assert_associative_law(ra, rb, rc);
    }
}
