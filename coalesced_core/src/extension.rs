use std::cmp::Ordering;

use crate::{coalesce::Coalesce, coalesced::Coalesced, priority::sealed::Length};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Extension<T, E = ()> {
    pub value: T,
    pub extension: E,
}

impl<T, A, E, L> Coalesce<A, E, L> for Extension<T, E>
where
    T: Coalesce<A, E, L>,
    L: Length,
{
    type History = Coalesced<T, A, E, L>;
    fn order(&self, other: &Self) -> Ordering {
        self.value.order(&other.value)
    }
}

impl<T> Extension<T, ()> {
    // TODO impl as From trait ?
    pub fn new(value: T) -> Self {
        Self::new_with(value, ())
    }
}
impl<T, E> Extension<T, E> {
    pub fn new_with(value: T, extension: E) -> Self {
        Self { value, extension }
    }
}

#[cfg(test)]
mod tests {

    use crate::{coalesce::CoalesceExt, coalesced::Coalesced};

    use super::*;

    #[test]
    fn test_coalesced_with_extension_prior_history() {
        #[derive(Debug, PartialEq)]
        enum Context {
            File,
            Env,
            Cli,
        }
        let from_file = Coalesced::new_prior_with(Some("file"), Context::File);
        let from_env = Coalesced::new_prior_with(Some("env"), Context::Env);
        let from_cli = Coalesced::new_prior_with(Some("cli"), Context::Cli);

        let config = from_file.coalesce(from_env).coalesce(from_cli);
        assert_eq!(config.value(), &Some("cli"));
        assert_eq!(config.extension(), &Context::Cli);
        assert_eq!(
            config.priority(),
            &vec![
                Extension::new_with(Some("file"), Context::File),
                Extension::new_with(Some("env"), Context::Env),
                Extension::new_with(Some("cli"), Context::Cli),
            ],
        );
    }

    #[test]
    fn test_coalesced_set_extension() {
        let from_file = Coalesced::new_prior(Some(1));
        let from_env = Coalesced::new_prior(Some(10));
        let from_cli = Coalesced::new_prior(Some(100));
        assert_eq!(from_file.extension(), &());
        assert_eq!(from_env.extension(), &());
        assert_eq!(from_cli.extension(), &());

        let from_file = from_file.with_extension("file");
        let from_env = from_env.with_extension("env");
        let from_cli = from_cli.with_extension("cli");
        assert_eq!(from_file.extension(), &"file");
        assert_eq!(from_env.extension(), &"env");
        assert_eq!(from_cli.extension(), &"cli");

        let config = from_file.coalesce(from_env).coalesce(from_cli);
        assert_eq!(config.value(), &Some(100));
        assert_eq!(config.extension(), &"cli");
    }
}
