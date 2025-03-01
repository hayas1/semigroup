use crate::{Coalesced, Prior};

pub fn parser(s: &str) -> Result<Coalesced<Option<i64>, Prior>, String> {
    s.parse()
        .map(Some)
        .map(Coalesced::new_prior)
        .map_err(|e: std::num::ParseIntError| e.to_string())
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[derive(Debug, Clone, Parser)]
    #[clap(version, about, arg_required_else_help = true)]
    pub struct Cli {
        /// Some number
        #[arg(short, long, value_parser = parser)]
        pub number: Coalesced<Option<i64>, Prior>,
    }

    #[test]
    fn test_clap_parser() {
        let cli = Cli::try_parse_from(["coalesced", "--number", "100"]).unwrap();
        assert_eq!(cli.number.unwrap(), 100);
    }
}
