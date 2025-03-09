//! TODO doc

use std::str::FromStr;

use crate::{
    coalesce::IntoCoalesced,
    priority::{
        sealed::{Access, Length},
        Accessor,
    },
    Coalesced,
};

// impl<T, A> ValueParserFactory for Coalesced<Option<T>, A>
// where
//     Option<T>: Priority,
//     T: ValueParserFactory + Clone + Send + Sync,
//     <T as ValueParserFactory>::Parser: TypedValueParser<Value = T>,
//     A: Access<Accessor = Accessor<A>> + Clone + Send + Sync,
// {
//     type Parser = MapValueParser<<T as ValueParserFactory>::Parser, fn(T) -> Self>;

//     fn value_parser() -> Self::Parser {
//         T::value_parser().map(|t| Some(t).into_coalesced())
//     }
// }

impl<T, A> FromStr for Coalesced<Option<T>, A>
where
    T: FromStr,
    A: Access<Accessor = Accessor<A>>,
{
    type Err = <T as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(None.into_coalesced())
        } else {
            T::from_str(s).map(Some).map(IntoCoalesced::into_coalesced)
        }
    }
}
impl<T, A, L> ToString for Coalesced<Option<T>, A, (), L>
where
    T: ToString,
    A: Access<Accessor = Accessor<A>>,
    L: Length,
{
    fn to_string(&self) -> String {
        match &**self {
            Some(t) => t.to_string(),
            None => String::new(),
        }
    }
}
impl<T, A> Default for Coalesced<Option<T>, A> {
    fn default() -> Self {
        None.into_coalesced()
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::{
        coalesce::{Coalesce, CoalesceExt},
        Coalesced, Posterior,
    };

    #[derive(Debug, Clone, Parser)]
    #[clap(version, about)]
    pub struct Cli {
        /// Some number
        #[arg(short, long)]
        pub number: Option<i64>,
    }
    #[test]
    fn test_clap_parser() {
        let cli = Cli::try_parse_from(["coalesced", "--number", "100"]).unwrap();
        assert_eq!(cli.number.unwrap(), 100);

        let cli = Cli::try_parse_from(["coalesced"]).unwrap();
        assert!(cli.number.is_none());
    }
    #[test]
    fn test_coalesced_with_clap() {
        let cli = Cli::try_parse_from(["coalesced", "--number", "100"]).unwrap();
        let cli_number = cli.number.with_extension("cli");
        let number = Some(10).with_extension("const").prior();
        let coalesced = number.coalesce(cli_number);
        assert_eq!(coalesced.value(), &Some(100));
        assert_eq!(coalesced.extension(), &"cli");
    }
    #[test]
    fn test_coalesced_with_clap_empty_arg() {
        let cli = Cli::try_parse_from(["coalesced"]).unwrap();
        let cli_number = cli.number.with_extension("cli");
        let number = Some(10).with_extension("const").prior();
        let coalesced = number.coalesce(cli_number);
        assert_eq!(coalesced.value(), &Some(10));
        assert_eq!(coalesced.extension(), &"const");
    }

    #[derive(Debug, Clone, Parser)]
    #[clap(version, about)]
    pub struct CliPosterior {
        /// Some number
        #[arg(short, long, default_value = "")] // TODO default_value_t
        pub number: Coalesced<Option<i64>, Posterior>,
    }
    #[test]
    fn test_coalesced_clap_parser() {
        let cli = CliPosterior::try_parse_from(["coalesced", "--number", "100"]).unwrap();
        assert_eq!(cli.number.unwrap(), 100);

        let cli = CliPosterior::try_parse_from(["coalesced"]).unwrap();
        assert!(cli.number.is_none());
    }
    #[test]
    fn test_coalesced_posterior_with_clap() {
        let cli = CliPosterior::try_parse_from(["coalesced", "--number", "100"]).unwrap();
        let cli_number = cli.number.with_extension("cli");
        let number = Some(10).with_extension("const");
        let coalesced = number.coalesce(cli_number);
        assert_eq!(coalesced.value(), &Some(10));
        assert_eq!(coalesced.extension(), &"const");
    }
    #[test]
    fn test_coalesced_posterior_with_clap_empty_arg() {
        let cli = CliPosterior::try_parse_from(["coalesced"]).unwrap();
        let cli_number = cli.number.with_extension("cli");
        let number = Some(10).with_extension("const");
        let coalesced = number.coalesce(cli_number);
        assert_eq!(coalesced.value(), &Some(10));
        assert_eq!(coalesced.extension(), &"const");
    }
}
