use crate::Coalesce;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Extension<T, E = ()> {
    pub value: T,
    pub extension: E,
}

impl<T, E> Coalesce for Extension<T, E>
where
    T: Coalesce,
{
    fn straight(&self, other: &Self) -> bool {
        self.value.straight(&other.value)
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

    use crate::Coalesced;

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

        let config = from_file.extend_prior(from_env).extend_prior(from_cli);
        assert_eq!(config.value(), &Some("cli"));
        assert_eq!(config.extension(), &Context::Cli);
        assert_eq!(
            config.priority,
            vec![
                Extension::new_with(Some("file"), Context::File),
                Extension::new_with(Some("env"), Context::Env),
                Extension::new_with(Some("cli"), Context::Cli),
            ],
        );
    }
}
