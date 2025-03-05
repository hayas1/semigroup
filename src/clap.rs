use std::str::FromStr;

use crate::{
    priority::{
        sealed::{Access, Length},
        Accessor,
    },
    Coalesced,
};

// TODO not Option<T>, T instead ?
impl<T, A> FromStr for Coalesced<Option<T>, A>
where
    T: FromStr,
{
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Coalesced::new(None))
        } else {
            s.parse().map(Some).map(Coalesced::new)
        }
    }
}
// TODO not Option<T>, T instead ?
impl<T, A, E, L> ToString for Coalesced<Option<T>, A, E, L>
where
    T: ToString,
    A: Access<Accessor = Accessor<A>>,
    L: Length,
{
    fn to_string(&self) -> String {
        match self.value() {
            Some(s) => s.to_string(),
            None => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[derive(Debug, Clone, Parser)]
    #[clap(version, about)]
    pub struct Cli {
        /// Some number
        #[arg(short, long, default_value = "")] // TODO remove default_value = ""
        pub number: Coalesced<Option<i64>>,
    }

    #[test]
    fn test_clap_parser() {
        let cli = Cli::try_parse_from(["coalesced", "--number", "100"]).unwrap();
        assert_eq!(cli.number.unwrap(), 100);

        let cli = Cli::try_parse_from(["coalesced", "--number", ""]).unwrap();
        assert!(cli.number.is_none());

        let cli = Cli::try_parse_from(["coalesced"]).unwrap();
        assert!(cli.number.is_none());
    }
    #[test]
    fn test_coalesced_with_clap() {
        let cli = Cli::try_parse_from(["coalesced", "--number", "100"]).unwrap();
        let cli_number = cli.number.set_extension("cli");
        let number = Coalesced::new_prior_with(Some(10), "const");
        let coalesced = number.register(cli_number);
        assert_eq!(coalesced.value(), &Some(100));
        assert_eq!(coalesced.extension(), &"cli");
    }

    #[test]
    fn test_coalesced_with_clap_empty() {
        let cli = Cli::try_parse_from(["coalesced", "--number", ""]).unwrap();
        let cli_number = cli.number.set_extension("cli");
        let number = Coalesced::new_prior_with(Some(10), "const");
        let coalesced = number.register(cli_number);
        assert_eq!(coalesced.value(), &Some(10));
        assert_eq!(coalesced.extension(), &"const");
    }
}
