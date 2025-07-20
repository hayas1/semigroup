pub mod overwrite {
    use crate::extension::{Extension, WithExt};

    pub fn prior<T>(_base: T, other: T) -> T {
        other
    }

    pub fn posterior<T>(base: T, _other: T) -> T {
        base
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_override_coalesce() {
        assert_eq!(0, overwrite::prior(0, 0));
        assert_eq!(1, overwrite::prior(0, 1));
        assert_eq!(0, overwrite::prior(1, 0));
        assert_eq!(2, overwrite::prior(1, 2));
        assert_eq!(0, overwrite::posterior(0, 0));
        assert_eq!(0, overwrite::posterior(0, 1));
        assert_eq!(1, overwrite::posterior(1, 0));
        assert_eq!(1, overwrite::posterior(1, 2));

        assert_eq!("", overwrite::prior("", ""));
        assert_eq!("foo", overwrite::prior("", "foo"));
        assert_eq!("", overwrite::prior("foo", ""));
        assert_eq!("bar", overwrite::prior("foo", "bar"));
        assert_eq!("", overwrite::posterior("", ""));
        assert_eq!("", overwrite::posterior("", "foo"));
        assert_eq!("foo", overwrite::posterior("foo", ""));
        assert_eq!("foo", overwrite::posterior("foo", "bar"));
    }

    #[test]
    fn test_override_coalesce_ref() {
        assert_eq!(&0, overwrite::prior(&0, &0));
        assert_eq!(&false, overwrite::posterior(&false, &true));
    }
}
