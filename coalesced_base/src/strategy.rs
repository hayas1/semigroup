use std::ops::{Deref, DerefMut};

use crate::extension::{Extension, WithExt};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Overwrite<T>(pub T);
impl<X, T> Extension<X> for Overwrite<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
impl<T> Deref for Overwrite<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Overwrite<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::coalesce::Coalesce;

    use super::*;

    #[test]
    fn test_overwrite_coalesce_integer() {
        assert_eq!(Overwrite(0), Overwrite(0).prior(Overwrite(0)));
        assert_eq!(Overwrite(1), Overwrite(0).prior(Overwrite(1)));
        assert_eq!(Overwrite(0), Overwrite(1).prior(Overwrite(0)));
        assert_eq!(Overwrite(2), Overwrite(1).prior(Overwrite(2)));
        assert_eq!(
            Overwrite(0),
            Overwrite(0)
                .prior(Overwrite(2))
                .prior(Overwrite(0))
                .prior(Overwrite(4))
                .prior(Overwrite(0))
        );

        assert_eq!(Overwrite(0), Overwrite(0).posterior(Overwrite(0)));
        assert_eq!(Overwrite(0), Overwrite(0).posterior(Overwrite(1)));
        assert_eq!(Overwrite(1), Overwrite(1).posterior(Overwrite(0)));
        assert_eq!(Overwrite(1), Overwrite(1).posterior(Overwrite(2)));
        assert_eq!(
            Overwrite(0),
            Overwrite(0)
                .posterior(Overwrite(2))
                .posterior(Overwrite(0))
                .posterior(Overwrite(4))
                .posterior(Overwrite(0))
        );
    }

    #[test]
    fn test_overwrite_coalesce_str() {
        assert_eq!(Overwrite(""), Overwrite("").prior(Overwrite("")));
        assert_eq!(Overwrite("foo"), Overwrite("").prior(Overwrite("foo")));
        assert_eq!(Overwrite(""), Overwrite("foo").prior(Overwrite("")));
        assert_eq!(Overwrite("bar"), Overwrite("foo").prior(Overwrite("bar")));
        assert_eq!(
            Overwrite(""),
            Overwrite("")
                .prior(Overwrite("bar"))
                .prior(Overwrite(""))
                .prior(Overwrite("baz"))
                .prior(Overwrite(""))
        );

        assert_eq!(Overwrite(""), Overwrite("").posterior(Overwrite("")));
        assert_eq!(Overwrite(""), Overwrite("").posterior(Overwrite("foo")));
        assert_eq!(Overwrite("foo"), Overwrite("foo").posterior(Overwrite("")));
        assert_eq!(
            Overwrite("foo"),
            Overwrite("foo").posterior(Overwrite("bar"))
        );
        assert_eq!(
            Overwrite(""),
            Overwrite("")
                .posterior(Overwrite("bar"))
                .posterior(Overwrite(""))
                .posterior(Overwrite("baz"))
                .posterior(Overwrite(""))
        );
    }

    #[test]
    fn test_overwrite_coalesce_ref() {
        assert_eq!(Overwrite(&0), Overwrite(&0).prior(Overwrite(&0)));
        assert_eq!(
            Overwrite(&false),
            Overwrite(&false).posterior(Overwrite(&true))
        );
    }
}
