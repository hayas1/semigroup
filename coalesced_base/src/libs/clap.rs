//! TODO doc

// use std::str::FromStr;

// use clap::builder::{MapValueParser, TypedValueParser, ValueParserFactory};

// use crate::{
//     coalesced::{Coalesced, IntoHistory},
//     Coalesce,
// };

// impl<T> ValueParserFactory for Coalesced<T>
// where
//     T: Coalesce + ValueParserFactory + Clone + Send + Sync,
//     <T as ValueParserFactory>::Parser: TypedValueParser<Value = T>,
// {
//     type Parser = MapValueParser<<T as ValueParserFactory>::Parser, fn(T) -> Self>;

//     fn value_parser() -> Self::Parser {
//         T::value_parser().map(IntoHistory::into_history)
//     }
// }

// impl<T> FromStr for Coalesced<T>
// where
//     T: Coalesce + FromStr,
// {
//     type Err = <T as FromStr>::Err;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         T::from_str(s).map(IntoHistory::into_history)
//     }
// }
// impl<T> ToString for Coalesced<T>
// where
//     T: Coalesce + Clone + ToString,
// {
//     fn to_string(&self) -> String {
//         self.into_cloned().to_string()
//     }
// }

#[cfg(test)]
mod tests {
    use clap::Parser;
    use coalesced::Coalesce;

    // use super::*;

    #[derive(Debug, Clone, Parser, Coalesce)]
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
    fn test_clap_parser_without_history() {
        let cli1 = Cli::try_parse_from(["coalesced", "--number", "1"]).unwrap();
        let cli2 = Cli::try_parse_from(["coalesced", "--number", "10"]).unwrap();
        let cli3 = Cli::try_parse_from(["coalesced", "--number", "100"]).unwrap();

        let cli = cli1.prior(cli2).prior(cli3);
        assert_eq!(cli.number.unwrap(), 100);
    }
    #[test]
    fn test_clap_parser_with_history() {
        // TODO
    }
}
