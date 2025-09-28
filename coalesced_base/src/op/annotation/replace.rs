use coalesced_derive::ConstructionUse;

use crate::{
    annotate::{Annotate, Annotated},
    op::{Construction, ConstructionAnnotated},
    reverse::Reverse,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(annotated, op_trait = ReplaceExt)]
pub struct Replace<T>(pub T);
impl<T, A> AnnotatedSemigroup<A> for Replace<T> {
    fn annotated_op(_base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        other
    }
}

#[cfg(test)]
mod tests {
    use crate::semigroup::tests::assert_semigroup_op;

    use super::*;

    #[test]
    fn test_replace_as_semigroup_op() {
        let (a, b, c) = (Replace(1), Replace(2), Replace(3));
        assert_semigroup_op!(a, b, c);
    }

    #[test]
    fn test_replace() {
        let (a, b) = (Replace(1), Replace(2));
        assert_eq!(a.replace(b).into_inner(), 2);

        let (ra, rb) = (Reverse(a), Reverse(b));
        assert_eq!(ra.replace(rb).0.into_inner(), 1);
    }
}
