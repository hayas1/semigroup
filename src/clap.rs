//! TODO doc

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::coalesce::{Coalesce, CoalesceExt};

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

        let cli = Cli::try_parse_from(["coalesced", "--number", ""]).unwrap();
        assert!(cli.number.is_none());

        let cli = Cli::try_parse_from(["coalesced"]).unwrap();
        assert!(cli.number.is_none());
    }

    #[test]
    fn test_coalesced_with_clap() {
        let cli = Cli::try_parse_from(["coalesced", "--number", "100"]).unwrap();
        let cli_number = cli.number.set_extension("cli");
        let number = Some(10).set_extension("const").prior();
        let coalesced = number.coalesce(cli_number);
        assert_eq!(coalesced.value(), &Some(100));
        assert_eq!(coalesced.extension(), &"cli");
    }
    #[test]
    fn test_coalesced_with_clap_empty() {
        let cli = Cli::try_parse_from(["coalesced", "--number", ""]).unwrap();
        let cli_number = cli.number.set_extension("cli");
        let number = Some(10).set_extension("const").prior();
        let coalesced = number.coalesce(cli_number);
        assert_eq!(coalesced.value(), &Some(10));
        assert_eq!(coalesced.extension(), &"const");
    }
    #[test]
    fn test_coalesced_with_clap_empty_arg() {
        let cli = Cli::try_parse_from(["coalesced"]).unwrap();
        let cli_number = cli.number.set_extension("cli");
        let number = Some(10).set_extension("const").prior();
        let coalesced = number.coalesce(cli_number);
        assert_eq!(coalesced.value(), &Some(10));
        assert_eq!(coalesced.extension(), &"const");
    }
}
