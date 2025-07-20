pub trait Coalesce {
    fn prior(self, other: Self) -> Self;
    fn posterior(self, other: Self) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coalesce_option() {
        assert_eq!(None::<Option<()>>, None.prior(None));
        assert_eq!(Some(1), None.prior(Some(1)));
        assert_eq!(Some(1), Some(1).prior(None));
        assert_eq!(Some(2), Some(1).prior(Some(2)));
        assert_eq!(
            Some(4),
            None.prior(Some(2)).prior(None).prior(Some(4)).prior(None)
        );

        assert_eq!(None::<Option<()>>, None.posterior(None));
        assert_eq!(Some(1), None.posterior(Some(1)));
        assert_eq!(Some(1), Some(1).posterior(None));
        assert_eq!(Some(1), Some(1).posterior(Some(2)));
        assert_eq!(
            Some(2),
            None.posterior(Some(2))
                .posterior(None)
                .posterior(Some(4))
                .posterior(None)
        );
    }

    #[test]
    fn test_coalesce_result() {
        assert_eq!(Err::<(), _>("one"), Err("one").prior(Err("two")));
        assert_eq!(Ok(2), Err("one").prior(Ok(2)));
        assert_eq!(Ok(1), Ok(1).prior(Err("two")));
        assert_eq!(Ok::<_, ()>(2), Ok(1).prior(Ok(2)));
        assert_eq!(
            Ok(4),
            Err("one")
                .prior(Ok(2))
                .prior(Err("three"))
                .prior(Ok(4))
                .prior(Err("five"))
        );

        assert_eq!(Err::<(), _>("two"), Err("one").posterior(Err("two")));
        assert_eq!(Ok(2), Err("one").posterior(Ok(2)));
        assert_eq!(Ok(1), Ok(1).posterior(Err("two")));
        assert_eq!(Ok::<_, ()>(1), Ok(1).posterior(Ok(2)));
        assert_eq!(
            Ok(2),
            Err("one")
                .posterior(Ok(2))
                .posterior(Err("three"))
                .posterior(Ok(4))
                .posterior(Err("five"))
        );
    }
}
