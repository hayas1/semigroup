//! TODO doc

use serde::{Deserialize, Serialize};

use crate::{
    priority::{sealed::Access, Accessor},
    Coalesced,
};

impl<C, A> Serialize for Coalesced<C, A>
where
    C: Serialize,
    A: Access<Accessor = Accessor<A>>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&**self).serialize(serializer)
    }
}

impl<'de, C, A> Deserialize<'de> for Coalesced<C, A>
where
    C: Deserialize<'de>,
    A: Access<Accessor = Accessor<A>>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let coalesce = C::deserialize(deserializer)?;
        Ok(Coalesced::new(coalesce))
    }
}

#[cfg(test)]
mod tests {
    use crate::{coalesce::Coalesce, Prior};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct Config<'a> {
        name: &'a str,
        number: Option<i64>,
    }
    #[test]
    fn test_deserialized_coalesce() {
        let file: Config = serde_json::from_str(r#"{"name":"file","number":1}"#).unwrap();
        let env: Config = serde_json::from_str(r#"{"name":"env","number":10}"#).unwrap();
        let cli: Config = serde_json::from_str(r#"{"name":"cli","number":100}"#).unwrap();

        let number: Coalesced<Option<i64>, Prior> = file
            .number
            .coalesce(env.number)
            .coalesce(cli.number)
            .into_single();

        assert_eq!(number.unwrap(), 100);
    }

    #[derive(Serialize, Deserialize)]
    struct CoalesceConfig<'a> {
        name: &'a str,
        number: Coalesced<Option<i64>, Prior>,
    }

    #[test]
    fn test_coalesced_serialize() {
        let file = CoalesceConfig {
            name: "file",
            number: Coalesced::new_prior(Some(1)),
        };
        let env = CoalesceConfig {
            name: "env",
            number: Coalesced::new_prior(Some(10)),
        };
        let cli = CoalesceConfig {
            name: "cli",
            number: Coalesced::new_prior(Some(100)),
        };

        let config = CoalesceConfig {
            name: "config",
            number: file
                .number
                .coalesce(env.number)
                .coalesce(cli.number)
                .into_single(),
        };

        let serialized = serde_json::to_string(&config).unwrap();
        assert_eq!(serialized, r#"{"name":"config","number":100}"#);
    }
    #[test]
    fn test_coalesced_deserialize() {
        let file: CoalesceConfig = serde_json::from_str(r#"{"name":"file","number":1}"#).unwrap();
        let env: CoalesceConfig = serde_json::from_str(r#"{"name":"env","number":10}"#).unwrap();
        let cli: CoalesceConfig = serde_json::from_str(r#"{"name":"cli","number":100}"#).unwrap();

        let config = CoalesceConfig {
            name: "config",
            number: file
                .number
                .coalesce(env.number)
                .coalesce(cli.number)
                .into_single(),
        };

        assert_eq!(config.number.unwrap(), 100);
    }

    #[test]
    fn test_coalesced_serialize_with_none() {
        let file = CoalesceConfig {
            name: "file",
            number: Coalesced::new_prior(Some(1)),
        };
        let env = CoalesceConfig {
            name: "env",
            number: Coalesced::new_prior(Some(10)),
        };
        let cli = CoalesceConfig {
            name: "cli",
            number: Coalesced::new_prior(None),
        };

        let config = CoalesceConfig {
            name: "config",
            number: file
                .number
                .coalesce(env.number)
                .coalesce(cli.number)
                .into_single(),
        };

        let serialized = serde_json::to_string(&config).unwrap();
        assert_eq!(serialized, r#"{"name":"config","number":10}"#);
    }
    #[test]
    fn test_coalesced_deserialize_with() {
        let file: CoalesceConfig = serde_json::from_str(r#"{"name":"file","number":1}"#).unwrap();
        let env: CoalesceConfig = serde_json::from_str(r#"{"name":"env","number":10}"#).unwrap();
        let cli: CoalesceConfig = serde_json::from_str(r#"{"name":"cli"}"#).unwrap();

        let config = CoalesceConfig {
            name: "config",
            number: file
                .number
                .coalesce(env.number)
                .coalesce(cli.number)
                .into_single(),
        };

        assert_eq!(config.number.unwrap(), 10);
    }
}
